use crate::TranscriptSource;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    pub last_run_date: Option<NaiveDate>,
    pub last_run_completed_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub processed: BTreeMap<String, ProcessedEntry>,
    #[serde(default)]
    pub failures: Vec<FailureEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedEntry {
    pub date: NaiveDate,
    pub transcript_source: TranscriptSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureEntry {
    pub guid: String,
    pub title: String,
    pub reason: String,
    pub first_seen: NaiveDate,
    pub attempts: u32,
}

impl State {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading state from {}", path.display()))?;
        let s: State = serde_json::from_str(&text).context("parsing state.json")?;
        Ok(s)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let text = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, text)?;
        Ok(())
    }

    pub fn is_processed(&self, guid: &str) -> bool {
        self.processed.contains_key(guid)
    }

    pub fn mark_processed(&mut self, guid: &str, date: NaiveDate, source: TranscriptSource) {
        self.processed.insert(
            guid.to_string(),
            ProcessedEntry {
                date,
                transcript_source: source,
            },
        );
        self.failures.retain(|f| f.guid != guid);
    }

    pub fn record_failure(&mut self, guid: &str, title: &str, reason: &str, today: NaiveDate) -> u32 {
        if let Some(f) = self.failures.iter_mut().find(|f| f.guid == guid) {
            f.attempts += 1;
            f.reason = reason.to_string();
            f.attempts
        } else {
            self.failures.push(FailureEntry {
                guid: guid.to_string(),
                title: title.to_string(),
                reason: reason.to_string(),
                first_seen: today,
                attempts: 1,
            });
            1
        }
    }
}

/// Catch-up window in UTC for filtering episodes by `pubDate`. The dates are interpreted
/// in the user's timezone, then converted to UTC bounds.
pub struct Window {
    pub since_date: NaiveDate,
    pub until_date: NaiveDate,
    pub since_utc: DateTime<Utc>,
    pub until_utc: DateTime<Utc>,
}

impl Window {
    pub fn span_str(&self) -> String {
        if self.since_date == self.until_date {
            self.until_date.to_string()
        } else {
            format!("{}..{}", self.since_date, self.until_date)
        }
    }
}

pub fn compute_window(state: &State, today_local: NaiveDate, cap_days: i64, tz: Tz) -> Window {
    let lower_bound = today_local - Duration::days(cap_days - 1);
    let since = match state.last_run_date {
        Some(last) => {
            let candidate = last + Duration::days(1);
            std::cmp::max(candidate, lower_bound)
        }
        None => today_local - Duration::days(1), // first run: yesterday + today
    };
    // Clamp: never start in the future.
    let since = std::cmp::min(since, today_local);
    let since_local_start = tz
        .from_local_datetime(&since.and_hms_opt(0, 0, 0).expect("valid midnight"))
        .single()
        .unwrap_or_else(|| Utc::now().with_timezone(&tz));
    let until_local_end = tz
        .from_local_datetime(&today_local.and_hms_opt(23, 59, 59).expect("valid end-of-day"))
        .single()
        .unwrap_or_else(|| Utc::now().with_timezone(&tz));
    Window {
        since_date: since,
        until_date: today_local,
        since_utc: since_local_start.with_timezone(&Utc),
        until_utc: until_local_end.with_timezone(&Utc),
    }
}
