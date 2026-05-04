pub mod config;
pub mod git_sync;
pub mod rss;
pub mod sqlite_reader;
pub mod state;
pub mod summarize;
pub mod transcript;
pub mod writer;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub uuid: String,
    pub title: String,
    pub author: Option<String>,
    pub feed_url: String,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub guid: String,
    pub title: String,
    pub published: chrono::DateTime<chrono::Utc>,
    pub duration_secs: Option<u64>,
    pub audio_url: Option<String>,
    pub episode_url: Option<String>,
    pub description: Option<String>,
    pub transcript_refs: Vec<TranscriptRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptRef {
    pub url: String,
    pub mime: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptSource {
    Rss,
    Web,
    Description,
    Skipped,
}

impl TranscriptSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            TranscriptSource::Rss => "rss",
            TranscriptSource::Web => "web",
            TranscriptSource::Description => "description",
            TranscriptSource::Skipped => "skipped",
        }
    }
}
