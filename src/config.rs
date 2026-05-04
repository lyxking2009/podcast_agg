use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub timezone: String,
    pub lookback_cap_days: i64,
    pub model: String,
    #[serde(default = "default_concurrency")]
    pub max_concurrent_feeds: usize,
    #[serde(default = "default_max_attempts")]
    pub max_attempts_per_episode: u32,
    #[serde(default = "default_websearch_timeout", alias = "agent_browser_timeout_secs")]
    pub web_search_timeout_secs: u64,
}

fn default_concurrency() -> usize {
    8
}
fn default_max_attempts() -> u32 {
    3
}
fn default_websearch_timeout() -> u64 {
    300
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading config from {}", path.display()))?;
        let cfg: Config =
            serde_json::from_str(&text).with_context(|| "parsing config.json".to_string())?;
        Ok(cfg)
    }

    pub fn tz(&self) -> Result<chrono_tz::Tz> {
        self.timezone
            .parse::<chrono_tz::Tz>()
            .map_err(|e| anyhow::anyhow!("invalid timezone {}: {}", self.timezone, e))
    }
}
