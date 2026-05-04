use crate::{Episode, TranscriptRef};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use feed_rs::parser;
use std::time::Duration;

pub async fn fetch_feed(client: &reqwest::Client, url: &str) -> Result<Vec<Episode>> {
    let resp = client
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            "podcast_agg/0.1 (+https://github.com/)",
        )
        .timeout(Duration::from_secs(60))
        .send()
        .await
        .with_context(|| format!("GET {url}"))?;
    let bytes = resp
        .error_for_status()
        .with_context(|| format!("HTTP error from {url}"))?
        .bytes()
        .await?;

    let feed = parser::parse(&bytes[..]).with_context(|| format!("parsing feed {url}"))?;
    let mut episodes = Vec::with_capacity(feed.entries.len());
    for entry in feed.entries {
        let pub_date: DateTime<Utc> = entry
            .published
            .or(entry.updated)
            .unwrap_or_else(Utc::now);

        let guid = if entry.id.is_empty() {
            // Fall back: hash of audio URL or title+pubDate.
            entry
                .media
                .first()
                .and_then(|m| m.content.first())
                .and_then(|c| c.url.as_ref().map(|u| u.to_string()))
                .unwrap_or_else(|| format!("{}|{}", entry.title.as_ref().map(|t| t.content.clone()).unwrap_or_default(), pub_date))
        } else {
            entry.id.clone()
        };

        let title = entry
            .title
            .as_ref()
            .map(|t| t.content.clone())
            .unwrap_or_else(|| "(untitled)".into());

        let audio_url = entry
            .media
            .iter()
            .flat_map(|m| m.content.iter())
            .find_map(|c| c.url.as_ref().map(|u| u.to_string()));

        let episode_url = entry.links.iter().find_map(|l| {
            if l.rel.as_deref() == Some("alternate") || l.rel.is_none() {
                Some(l.href.clone())
            } else {
                None
            }
        });

        let description = entry
            .summary
            .as_ref()
            .map(|s| s.content.clone())
            .or_else(|| entry.content.as_ref().and_then(|c| c.body.clone()));

        let duration_secs = entry
            .media
            .iter()
            .find_map(|m| m.duration.map(|d| d.as_secs()));

        let transcript_refs = extract_transcript_refs(&entry);

        episodes.push(Episode {
            guid,
            title,
            published: pub_date,
            duration_secs,
            audio_url,
            episode_url,
            description,
            transcript_refs,
        });
    }
    Ok(episodes)
}

/// feed-rs exposes the Podcasting 2.0 namespace under entry.media[].content[]; transcripts also
/// commonly appear as <podcast:transcript> at the entry level. Both are not directly typed in
/// feed-rs, so we re-parse the raw XML for the transcript element using a separate helper.
///
/// For now: pull from entry.media[].content[] where MIME type matches text/* OR application/srt /
/// vtt / json. Anything more thorough requires a custom XML pass, which we'll add later if RSS
/// transcript hit rates are too low.
fn extract_transcript_refs(entry: &feed_rs::model::Entry) -> Vec<TranscriptRef> {
    let mut refs = Vec::new();
    for media in &entry.media {
        for content in &media.content {
            let url = match &content.url {
                Some(u) => u.to_string(),
                None => continue,
            };
            let mime = content.content_type.as_ref().map(|m| m.to_string());
            if let Some(mime_str) = mime.as_deref() {
                if is_transcript_mime(mime_str) {
                    refs.push(TranscriptRef {
                        url,
                        mime: Some(mime_str.to_string()),
                    });
                }
            }
        }
    }
    // Also parse raw XML for <podcast:transcript> tags which feed-rs does not surface.
    refs
}

fn is_transcript_mime(mime: &str) -> bool {
    let m = mime.to_ascii_lowercase();
    m.contains("srt")
        || m.contains("vtt")
        || m.contains("text/plain")
        || m.contains("application/json")
        || m == "text/html"
}

/// Supplemental pass over the raw XML body to find <podcast:transcript url="..." type="..."/>
/// inside each <item>. Called by transcript layer when the feed-rs extraction came up empty.
pub fn extract_podcast_transcripts_from_xml(xml: &[u8]) -> Vec<(String, Vec<TranscriptRef>)> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut current_guid: Option<String> = None;
    let mut current_refs: Vec<TranscriptRef> = Vec::new();
    let mut in_item = false;
    let mut in_guid_tag = false;
    let mut out: Vec<(String, Vec<TranscriptRef>)> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name();
                let local = std::str::from_utf8(name.as_ref()).unwrap_or("");
                if local == "item" {
                    in_item = true;
                    current_guid = None;
                    current_refs.clear();
                } else if in_item && local == "guid" {
                    in_guid_tag = true;
                }
            }
            Ok(Event::Empty(e)) => {
                let name = e.name();
                let local = std::str::from_utf8(name.as_ref()).unwrap_or("");
                if in_item && (local.ends_with(":transcript") || local == "transcript") {
                    let mut url = None;
                    let mut mime = None;
                    for attr in e.attributes().flatten() {
                        let key = std::str::from_utf8(attr.key.as_ref()).unwrap_or("");
                        let val = attr.unescape_value().unwrap_or_default().into_owned();
                        match key {
                            "url" => url = Some(val),
                            "type" => mime = Some(val),
                            _ => {}
                        }
                    }
                    if let Some(u) = url {
                        current_refs.push(TranscriptRef { url: u, mime });
                    }
                }
            }
            Ok(Event::Text(t)) => {
                if in_guid_tag {
                    if let Ok(s) = t.unescape() {
                        current_guid = Some(s.into_owned());
                    }
                }
            }
            Ok(Event::End(e)) => {
                let name = e.name();
                let local = std::str::from_utf8(name.as_ref()).unwrap_or("");
                if local == "guid" {
                    in_guid_tag = false;
                } else if local == "item" {
                    if let Some(g) = current_guid.take() {
                        if !current_refs.is_empty() {
                            out.push((g, std::mem::take(&mut current_refs)));
                        }
                    }
                    in_item = false;
                    current_refs.clear();
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    out
}
