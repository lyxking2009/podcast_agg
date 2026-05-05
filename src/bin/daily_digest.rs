use anyhow::{Context, Result};
use chrono::{NaiveDate, Utc};
use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use podcast_agg::config::Config;
use podcast_agg::state::{compute_window, State, Window};
use podcast_agg::summarize::Summarizer;
use podcast_agg::writer::{write_summary, WriteRequest};
use podcast_agg::{git_sync, rss, sqlite_reader, transcript, Episode, Subscription, TranscriptSource};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

#[derive(Debug, Parser)]
#[command(name = "daily-digest", about = "Daily podcast summary pipeline")]
struct Args {
    /// Project root (defaults to cwd).
    #[arg(long)]
    repo: Option<PathBuf>,

    /// Override "today" (YYYY-MM-DD, in config timezone). Useful for backfills/tests.
    #[arg(long)]
    date: Option<NaiveDate>,

    /// Only process the show with this title (case-insensitive substring match).
    #[arg(long)]
    only: Option<String>,

    /// Skip git commit/push at the end.
    #[arg(long, default_value_t = false)]
    no_commit: bool,

    /// Skip git push but still commit.
    #[arg(long, default_value_t = false)]
    no_push: bool,

    /// Skip refreshing data/subscriptions.json from the Apple Podcasts app at the start of the run.
    #[arg(long, default_value_t = false)]
    no_refresh_subs: bool,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();
    let repo = args
        .repo
        .unwrap_or_else(|| std::env::current_dir().expect("cwd"));

    // Load .env from project root if present (best-effort).
    let _ = dotenvy::from_path(repo.join(".env"));
    let cfg = Config::load(&repo.join("data/config.json"))
        .context("loading data/config.json")?;
    let tz = cfg.tz()?;

    let mut state = State::load(&repo.join("data/state.json"))?;
    let today_local = args.date.unwrap_or_else(|| {
        Utc::now().with_timezone(&tz).date_naive()
    });
    let window = compute_window(&state, today_local, cfg.lookback_cap_days, tz);
    tracing::info!(
        since = %window.since_date,
        until = %window.until_date,
        "catch-up window"
    );

    if !args.no_refresh_subs {
        let library = sqlite_reader::default_library_path();
        let subs_out = repo.join("data/subscriptions.json");
        match sqlite_reader::refresh_to_file(&library, &subs_out) {
            Ok(n) => tracing::info!(count = n, "refreshed subscriptions from Apple Podcasts"),
            Err(e) => tracing::warn!(
                error = %e,
                "refreshing subscriptions failed; continuing with existing data/subscriptions.json"
            ),
        }
    }

    let subs: Vec<Subscription> = serde_json::from_str(
        &std::fs::read_to_string(repo.join("data/subscriptions.json"))
            .context("reading data/subscriptions.json")?,
    )?;
    let subs = filter_subs(subs, args.only.as_deref());
    if subs.is_empty() {
        tracing::warn!("no subscriptions to process");
        return Ok(());
    }
    tracing::info!(count = subs.len(), "processing subscriptions");

    let api_key = std::env::var("DEEPSEEK_API_KEY").context("DEEPSEEK_API_KEY not set")?;
    let summarizer = Arc::new(Summarizer::new(api_key, cfg.model.clone())?);
    let http = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(120))
        .build()?;

    // Step 1: fetch all feeds in parallel, gather candidate episodes.
    let candidates = gather_candidates(&http, &subs, &window, cfg.max_concurrent_feeds).await;
    tracing::info!(
        episodes = candidates.iter().map(|(_, eps)| eps.len()).sum::<usize>(),
        "candidate episodes in window"
    );

    // Step 2: process each (sub, episode), serially to keep API costs predictable.
    let web_search_timeout = Duration::from_secs(cfg.web_search_timeout_secs);
    let mut written: Vec<PathBuf> = Vec::new();
    for (sub, episodes) in candidates {
        for ep in episodes {
            if state.is_processed(&ep.guid) {
                continue;
            }
            match process_episode(
                &http,
                &summarizer,
                &repo,
                &sub,
                &ep,
                &cfg.model,
                web_search_timeout,
            )
            .await
            {
                Ok((path, source)) => {
                    state.mark_processed(&ep.guid, today_local, source);
                    written.push(path);
                    // Save state after each success so partial runs preserve progress.
                    state
                        .save(&repo.join("data/state.json"))
                        .unwrap_or_else(|e| tracing::warn!(error = %e, "saving state failed"));
                }
                Err(e) => {
                    tracing::error!(guid = %ep.guid, title = %ep.title, error = %e, "episode failed");
                    let attempts =
                        state.record_failure(&ep.guid, &ep.title, &e.to_string(), today_local);
                    if attempts >= cfg.max_attempts_per_episode {
                        tracing::warn!(
                            guid = %ep.guid,
                            attempts,
                            "max attempts reached, marking as skipped"
                        );
                        state.mark_processed(&ep.guid, today_local, TranscriptSource::Skipped);
                    }
                    let _ = state.save(&repo.join("data/state.json"));
                }
            }
        }
    }

    // Advance last_run_date now that the main pass is done.
    state.last_run_date = Some(today_local);
    state.last_run_completed_at = Some(Utc::now());
    state.save(&repo.join("data/state.json"))?;

    let summary_msg = format!(
        "digest: {} ({} episodes, window {})",
        today_local,
        written.len(),
        window.span_str(),
    );
    tracing::info!(message = %summary_msg, "run complete");

    if !args.no_commit {
        let summaries_dir = repo.join("Podcast Summaries");
        let state_path = repo.join("data/state.json");
        let paths = [summaries_dir.as_path(), state_path.as_path()];
        let _ = git_sync::commit_and_push(git_sync::CommitOptions {
            repo: &repo,
            message: &summary_msg,
            paths: &paths,
            push: !args.no_push,
        });
    }

    Ok(())
}

fn filter_subs(subs: Vec<Subscription>, only: Option<&str>) -> Vec<Subscription> {
    match only {
        Some(needle) => {
            let n = needle.to_ascii_lowercase();
            subs.into_iter()
                .filter(|s| s.title.to_ascii_lowercase().contains(&n))
                .collect()
        }
        None => subs,
    }
}

async fn gather_candidates(
    http: &reqwest::Client,
    subs: &[Subscription],
    window: &Window,
    concurrency: usize,
) -> Vec<(Subscription, Vec<Episode>)> {
    let sem = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut tasks = FuturesUnordered::new();
    for sub in subs {
        let permit = sem.clone();
        let sub = sub.clone();
        let http = http.clone();
        let since = window.since_utc;
        let until = window.until_utc;
        tasks.push(tokio::spawn(async move {
            let _p = permit.acquire_owned().await;
            match rss::fetch_feed(&http, &sub.feed_url).await {
                Ok(eps) => {
                    let in_window: Vec<Episode> = eps
                        .into_iter()
                        .filter(|e| e.published >= since && e.published <= until)
                        .collect();
                    Some((sub, in_window))
                }
                Err(e) => {
                    tracing::warn!(feed = %sub.feed_url, error = %e, "feed fetch failed");
                    None
                }
            }
        }));
    }
    let mut out = Vec::new();
    while let Some(res) = tasks.next().await {
        if let Ok(Some(pair)) = res {
            if !pair.1.is_empty() {
                out.push(pair);
            }
        }
    }
    out
}

async fn process_episode(
    http: &reqwest::Client,
    summarizer: &Summarizer,
    repo: &Path,
    sub: &Subscription,
    ep: &Episode,
    model: &str,
    web_search_timeout: Duration,
) -> Result<(PathBuf, TranscriptSource)> {
    tracing::info!(show = %sub.title, episode = %ep.title, "processing");
    let t = transcript::fetch_transcript(
        http,
        ep,
        &sub.title,
        sub.web_url.as_deref(),
        web_search_timeout,
    )
    .await?;
    let summary = summarizer.summarize(&sub.title, &ep.title, &t.text).await?;
    let path = write_summary(WriteRequest {
        root: repo,
        show: &sub.title,
        episode: ep,
        summary: &summary,
        source: t.source,
        model,
    })?;
    tracing::info!(path = %path.display(), source = %t.source.as_str(), "wrote summary");
    Ok((path, t.source))
}
