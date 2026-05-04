use crate::{Episode, TranscriptRef, TranscriptSource};
use anyhow::{anyhow, Context, Result};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

const MAX_TRANSCRIPT_CHARS: usize = 600_000;

pub struct TranscriptResult {
    pub text: String,
    pub source: TranscriptSource,
}

pub async fn fetch_transcript(
    client: &reqwest::Client,
    episode: &Episode,
    show_title: &str,
    show_web_url: Option<&str>,
    web_search_timeout: Duration,
) -> Result<TranscriptResult> {
    // 1. RSS rung
    if !episode.transcript_refs.is_empty() {
        match try_rss_refs(client, &episode.transcript_refs).await {
            Ok(text) if !text.trim().is_empty() => {
                return Ok(TranscriptResult {
                    text: truncate(text),
                    source: TranscriptSource::Rss,
                });
            }
            Ok(_) => tracing::warn!(guid = %episode.guid, "RSS transcript was empty"),
            Err(e) => tracing::warn!(guid = %episode.guid, error = %e, "RSS transcript fetch failed"),
        }
    }

    // 2. Claude web-search rung: invoke `claude -p` with WebSearch+WebFetch tools enabled.
    match try_claude_web_search(episode, show_title, show_web_url, web_search_timeout).await {
        Ok(Some(text)) if looks_like_transcript(&text) => {
            return Ok(TranscriptResult {
                text: truncate(text),
                source: TranscriptSource::Web,
            });
        }
        Ok(_) => tracing::info!(guid = %episode.guid, "claude web search found no transcript"),
        Err(e) => tracing::warn!(guid = %episode.guid, error = %e, "claude web search failed"),
    }

    // 3. Description rung
    if let Some(desc) = episode.description.as_deref() {
        let plain = strip_html(desc);
        if !plain.trim().is_empty() {
            return Ok(TranscriptResult {
                text: truncate(plain),
                source: TranscriptSource::Description,
            });
        }
    }

    Err(anyhow!("no transcript and no description available"))
}

async fn try_rss_refs(client: &reqwest::Client, refs: &[TranscriptRef]) -> Result<String> {
    let mut last_err: Option<anyhow::Error> = None;
    for r in refs {
        match fetch_one_ref(client, r).await {
            Ok(t) => return Ok(t),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.unwrap_or_else(|| anyhow!("no transcript refs")))
}

async fn fetch_one_ref(client: &reqwest::Client, r: &TranscriptRef) -> Result<String> {
    let resp = client
        .get(&r.url)
        .timeout(Duration::from_secs(60))
        .send()
        .await
        .with_context(|| format!("GET {}", r.url))?
        .error_for_status()
        .with_context(|| format!("status from {}", r.url))?;
    let bytes = resp.bytes().await?;
    let body = String::from_utf8_lossy(&bytes).into_owned();
    let mime = r.mime.as_deref().unwrap_or("");
    let lower = mime.to_ascii_lowercase();
    if lower.contains("srt") {
        Ok(srt_to_text(&body))
    } else if lower.contains("vtt") {
        Ok(vtt_to_text(&body))
    } else if lower.contains("json") {
        Ok(json_transcript_to_text(&body).unwrap_or(body))
    } else if lower.contains("html") {
        Ok(strip_html(&body))
    } else {
        Ok(body)
    }
}

pub fn srt_to_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len() / 2);
    for line in s.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        if l.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        if l.contains("-->") {
            continue;
        }
        out.push_str(l);
        out.push('\n');
    }
    out
}

pub fn vtt_to_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len() / 2);
    for line in s.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        if l.starts_with("WEBVTT") || l.starts_with("NOTE") || l.contains("-->") {
            continue;
        }
        if l.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        out.push_str(l);
        out.push('\n');
    }
    out
}

fn json_transcript_to_text(s: &str) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(s).ok()?;
    // Common podcast 2.0 transcript JSON: { "version": ..., "segments": [{ "speaker": ..., "body": ... }] }
    if let Some(segs) = v.get("segments").and_then(|s| s.as_array()) {
        let mut out = String::new();
        for seg in segs {
            if let Some(body) = seg.get("body").and_then(|b| b.as_str()) {
                if let Some(speaker) = seg.get("speaker").and_then(|s| s.as_str()) {
                    out.push_str(speaker);
                    out.push_str(": ");
                }
                out.push_str(body);
                out.push('\n');
            }
        }
        if !out.is_empty() {
            return Some(out);
        }
    }
    None
}

pub fn strip_html(s: &str) -> String {
    let opts = html2text::config::plain();
    opts.string_from_read(s.as_bytes(), 100)
        .unwrap_or_else(|_| s.to_string())
}

fn truncate(s: String) -> String {
    if s.len() <= MAX_TRANSCRIPT_CHARS {
        s
    } else {
        let mut t = s;
        t.truncate(MAX_TRANSCRIPT_CHARS);
        t.push_str("\n\n[...transcript truncated...]");
        t
    }
}

/// Shells out to `claude -p` with WebSearch + WebFetch enabled and asks it to find and return
/// a transcript on stdout. Permission prompts are bypassed (headless). Output capped by `deadline`.
async fn try_claude_web_search(
    episode: &Episode,
    show_title: &str,
    show_web_url: Option<&str>,
    deadline: Duration,
) -> Result<Option<String>> {
    let claude = match which_claude() {
        Some(p) => p,
        None => {
            tracing::info!("`claude` CLI not on PATH; skipping web-search rung");
            return Ok(None);
        }
    };

    let pub_date = episode.published.format("%Y-%m-%d").to_string();
    let audio = episode.audio_url.as_deref().unwrap_or("(unknown)");
    let episode_url = episode.episode_url.as_deref().unwrap_or("(unknown)");

    let show_site = show_web_url.unwrap_or("(unknown)");
    let prompt = format!(
        "Use WebSearch and WebFetch to find a full transcript for this podcast episode:\n\
         \n\
         Show: {show}\n\
         Show website: {show_site}\n\
         Episode title: {title}\n\
         Published: {pub_date}\n\
         Episode page: {episode_url}\n\
         Audio URL: {audio}\n\
         \n\
         Strategy (be efficient — try in this order, stop at the first hit):\n\
         1. WebFetch the episode page directly. Many shows include the transcript inline.\n\
         2. WebFetch the show website and look for a transcripts section/page for this episode.\n\
         3. Run a focused WebSearch: '\"{title}\" transcript {show}' — pick the top result \
            that's clearly a transcript (not a summary, not a comment).\n\
         4. Try transcript aggregators (podscribe.ai, listennotes.com, podsearch.com, \
            snipd.com) only if step 1-3 yield nothing.\n\
         \n\
         Output rules — STRICT:\n\
         - If you find a real transcript, output the transcript text PLAIN. No preamble, no \
           commentary, no markdown fences, no '\"Here is the transcript:\"', nothing — just the \
           transcript itself, plain text, possibly with speaker labels.\n\
         - If you cannot find one, output exactly the token NOT_FOUND on its own line and \
           nothing else.\n\
         - Do not fabricate or paraphrase the transcript. If you only found a summary or \
           description, output NOT_FOUND.\n",
        show = show_title,
        show_site = show_site,
        title = episode.title,
        pub_date = pub_date,
        episode_url = episode_url,
        audio = audio,
    );

    let mut cmd = Command::new(&claude);
    cmd.arg("-p").arg(&prompt);
    cmd.arg("--allowedTools").arg("WebSearch WebFetch");
    cmd.arg("--permission-mode").arg("bypassPermissions");
    cmd.arg("--output-format").arg("text");
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.kill_on_drop(true);

    let run = cmd.output();
    let output = match timeout(deadline, run).await {
        Ok(r) => r.context("running claude CLI")?,
        Err(_) => {
            tracing::warn!("claude web-search subprocess timed out");
            return Ok(None);
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::warn!(status = ?output.status, stderr = %stderr, "claude exited non-zero");
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let trimmed = stdout.trim();
    if trimmed.is_empty() || trimmed == "NOT_FOUND" || trimmed.contains("NOT_FOUND") {
        return Ok(None);
    }
    Ok(Some(trimmed.to_string()))
}

/// Heuristic: a real transcript is at least ~600 chars and has multiple line breaks. Filters
/// out cases where Claude returned a brief "I couldn't find one" prose response.
fn looks_like_transcript(s: &str) -> bool {
    let t = s.trim();
    if t.len() < 600 {
        return false;
    }
    let lines = t.lines().filter(|l| !l.trim().is_empty()).count();
    lines >= 5
}

fn which_claude() -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_default();
    for dir in path.split(':') {
        let candidate = std::path::Path::new(dir).join("claude");
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    None
}

