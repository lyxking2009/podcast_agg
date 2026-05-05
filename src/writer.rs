use crate::summarize::Summary;
use crate::{Episode, TranscriptSource};
use anyhow::{Context, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};

pub struct WriteRequest<'a> {
    pub root: &'a Path,
    pub show: &'a str,
    pub episode: &'a Episode,
    pub summary: &'a Summary,
    pub source: TranscriptSource,
    pub model: &'a str,
}

pub fn write_summary(req: WriteRequest<'_>) -> Result<PathBuf> {
    let date = req.episode.published.format("%Y-%m-%d").to_string();
    let dir = req.root.join("Podcast Summaries").join(&date);
    std::fs::create_dir_all(&dir).with_context(|| format!("mkdir {}", dir.display()))?;

    let show_slug = slug::slugify(req.show);
    let ep_slug = slug::slugify(&req.episode.title);
    let mut filename = format!("{show_slug}__{ep_slug}.md");
    if filename.len() > 200 {
        filename.truncate(200);
        filename.push_str(".md");
    }
    let path = dir.join(&filename);

    let body = render(&req);
    std::fs::write(&path, body).with_context(|| format!("writing {}", path.display()))?;
    Ok(path)
}

fn render(req: &WriteRequest<'_>) -> String {
    let mut s = String::new();
    s.push_str("---\n");
    s.push_str(&format!("podcast: {}\n", yaml_str(req.show)));
    s.push_str(&format!("episode: {}\n", yaml_str(&req.episode.title)));
    s.push_str(&format!(
        "published: {}\n",
        req.episode.published.format("%Y-%m-%d")
    ));
    if let Some(d) = req.episode.duration_secs {
        s.push_str(&format!("duration: {}\n", fmt_duration(d)));
    }
    if let Some(u) = &req.episode.audio_url {
        s.push_str(&format!("audio_url: {}\n", yaml_str(u)));
    }
    if let Some(u) = &req.episode.episode_url {
        s.push_str(&format!("episode_url: {}\n", yaml_str(u)));
    }
    s.push_str(&format!("transcript_source: {}\n", req.source.as_str()));
    s.push_str(&format!(
        "generated_at: {}\n",
        Utc::now().to_rfc3339()
    ));
    s.push_str(&format!("model: {}\n", yaml_str(req.model)));
    s.push_str(&format!("guid: {}\n", yaml_str(&req.episode.guid)));
    s.push_str("---\n\n");

    s.push_str(&format!("# {} — {}\n\n", req.episode.title, req.show));

    if matches!(req.source, TranscriptSource::Description) {
        s.push_str("> ⚠️ Summary based on episode description only — full transcript was unavailable.\n\n");
    }

    s.push_str("## TL;DR\n");
    s.push_str(&req.summary.tldr);
    s.push_str("\n\n");

    if !req.summary.key_points.is_empty() {
        s.push_str("## Key points\n");
        for p in &req.summary.key_points {
            s.push_str(&format!("- {p}\n"));
        }
        s.push('\n');
    }

    if !req.summary.notable_quotes.is_empty() {
        s.push_str("## Notable quotes\n");
        for q in &req.summary.notable_quotes {
            let attribution = match (&q.speaker, &q.approx_timestamp) {
                (Some(sp), Some(ts)) => format!(" — {sp} (~{ts})"),
                (Some(sp), None) => format!(" — {sp}"),
                (None, Some(ts)) => format!(" (~{ts})"),
                (None, None) => String::new(),
            };
            s.push_str(&format!("> \"{}\"{attribution}\n>\n", q.quote));
        }
        s.push('\n');
    }

    if !req.summary.people_mentioned.is_empty() {
        s.push_str("## People mentioned\n");
        for p in &req.summary.people_mentioned {
            s.push_str(&format!("- {p}\n"));
        }
        s.push('\n');
    }

    if !req.summary.topics.is_empty() {
        s.push_str("## Topics\n");
        s.push_str(
            &req.summary
                .topics
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(" "),
        );
        s.push_str("\n\n");
    }

    s
}

fn yaml_str(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn fmt_duration(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{h}h{m:02}m{s:02}s")
    } else {
        format!("{m}m{s:02}s")
    }
}
