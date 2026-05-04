use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;

const SYSTEM_PROMPT: &str = r#"You summarize podcast episodes for a busy listener who wants to skim daily.

You will be given the show title, episode title, and a transcript (or an episode description if a transcript was unavailable). Produce a structured JSON object with EXACTLY these fields and nothing else:

{
  "tldr": "2-3 sentence headline of the episode.",
  "key_points": ["5-10 short bullets covering the substantive content."],
  "notable_quotes": [{"speaker": "Name or null", "quote": "verbatim quote", "approx_timestamp": "HH:MM:SS or null"}],
  "people_mentioned": ["names of notable people discussed or interviewed"],
  "topics": ["3-7 topic tags"],
  "listen_recommendation": "skip" | "skim" | "listen"
}

Rules:
- Be concrete. Prefer specific names, numbers, and claims over vague summaries.
- Quotes must be verbatim. If you cannot extract verbatim quotes (e.g. transcript is only a description), return an empty array.
- "listen_recommendation" reflects how worth listening to the episode is for someone who has read your summary: "skip" means the summary covers it, "listen" means the audio adds significant value.
- Output ONLY the JSON object. No prose before or after, no Markdown fences.
"#;

const DEEPSEEK_ENDPOINT: &str = "https://api.deepseek.com/v1/chat/completions";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub tldr: String,
    pub key_points: Vec<String>,
    #[serde(default)]
    pub notable_quotes: Vec<Quote>,
    #[serde(default)]
    pub people_mentioned: Vec<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    pub listen_recommendation: ListenRec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub speaker: Option<String>,
    pub quote: String,
    pub approx_timestamp: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ListenRec {
    Skip,
    Skim,
    Listen,
}

pub struct Summarizer {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

impl Summarizer {
    pub fn new(api_key: String, model: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300))
            .connect_timeout(Duration::from_secs(20))
            .build()?;
        Ok(Self {
            client,
            api_key,
            model,
        })
    }

    pub async fn summarize(
        &self,
        show: &str,
        episode_title: &str,
        transcript: &str,
    ) -> Result<Summary> {
        // DeepSeek V4 context is 1M tokens. Map-reduce is essentially never needed for podcasts;
        // 2M chars (~600-700K tokens) is a generous safety margin.
        const SOFT_LIMIT_CHARS: usize = 2_000_000;
        if transcript.len() > SOFT_LIMIT_CHARS {
            return self.map_reduce(show, episode_title, transcript).await;
        }
        self.summarize_once(show, episode_title, transcript).await
    }

    async fn summarize_once(
        &self,
        show: &str,
        episode_title: &str,
        transcript: &str,
    ) -> Result<Summary> {
        let mut last_err: Option<anyhow::Error> = None;
        for attempt in 1..=2u32 {
            match self.summarize_once_inner(show, episode_title, transcript).await {
                Ok(s) => return Ok(s),
                Err(e) => {
                    let msg = e.to_string();
                    let retryable = msg.contains("error decoding response body")
                        || msg.contains("connection")
                        || msg.contains("timed out");
                    tracing::warn!(attempt, error = %msg, retryable, "summarize attempt failed");
                    last_err = Some(e);
                    if !retryable {
                        break;
                    }
                }
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow!("summarize failed with no recorded error")))
    }

    async fn summarize_once_inner(
        &self,
        show: &str,
        episode_title: &str,
        transcript: &str,
    ) -> Result<Summary> {
        let user = format!(
            "Show: {show}\nEpisode: {episode_title}\n\n--- TRANSCRIPT BEGIN ---\n{transcript}\n--- TRANSCRIPT END ---\n\nReturn the JSON object now."
        );

        let body = json!({
            "model": self.model,
            "max_tokens": 8192,
            "temperature": 0.3,
            "response_format": { "type": "json_object" },
            "messages": [
                { "role": "system", "content": SYSTEM_PROMPT },
                { "role": "user", "content": user }
            ]
        });

        let resp = self
            .client
            .post(DEEPSEEK_ENDPOINT)
            .bearer_auth(&self.api_key)
            .header("content-type", "application/json")
            .timeout(Duration::from_secs(300))
            .json(&body)
            .send()
            .await
            .context("calling DeepSeek chat/completions API")?;

        let status = resp.status();
        let raw = resp.text().await.context("reading DeepSeek response body")?;
        if !status.is_success() {
            return Err(anyhow!("DeepSeek API {}: {}", status, raw));
        }

        let v: serde_json::Value = serde_json::from_str(&raw)?;
        let text = v["choices"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|c| c["message"]["content"].as_str())
            .ok_or_else(|| anyhow!("no choices[0].message.content in response: {}", raw))?
            .to_string();

        let cleaned = strip_code_fences(&text);
        let summary: Summary = serde_json::from_str(&cleaned)
            .with_context(|| format!("parsing summary JSON: {cleaned}"))?;
        Ok(summary)
    }

    async fn map_reduce(
        &self,
        show: &str,
        episode_title: &str,
        transcript: &str,
    ) -> Result<Summary> {
        const CHUNK_CHARS: usize = 100_000;
        let chunks = split_on_boundaries(transcript, CHUNK_CHARS);
        let total = chunks.len();
        tracing::info!(chunks = total, "map-reduce summarization");

        let mut chunk_summaries = Vec::with_capacity(total);
        for (i, chunk) in chunks.into_iter().enumerate() {
            let label = format!("{episode_title} (part {}/{total})", i + 1);
            let s = self.summarize_once(show, &label, &chunk).await?;
            chunk_summaries.push(s);
        }

        let merged = serde_json::to_string_pretty(&chunk_summaries)?;
        let synthesis = format!(
            "These are summaries of consecutive parts of the same episode. Merge them into ONE \
             coherent summary using the same JSON schema. Deduplicate. Preserve specific \
             names/numbers and quotes.\n\n{merged}"
        );
        self.summarize_once(show, episode_title, &synthesis).await
    }
}

fn strip_code_fences(s: &str) -> String {
    let t = s.trim();
    if let Some(rest) = t.strip_prefix("```json") {
        return rest.trim_end_matches("```").trim().to_string();
    }
    if let Some(rest) = t.strip_prefix("```") {
        return rest.trim_end_matches("```").trim().to_string();
    }
    t.to_string()
}

fn split_on_boundaries(s: &str, target: usize) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut start = 0;
    while start < bytes.len() {
        let mut end = (start + target).min(bytes.len());
        if end < bytes.len() {
            let lookback = end.saturating_sub(2_000);
            if let Some(off) = bytes[lookback..end]
                .iter()
                .rposition(|&b| b == b'\n' || b == b'.' || b == b'?' || b == b'!')
            {
                end = lookback + off + 1;
            }
            while end < bytes.len() && (bytes[end] & 0b1100_0000) == 0b1000_0000 {
                end += 1;
            }
        }
        out.push(String::from_utf8_lossy(&bytes[start..end]).into_owned());
        start = end;
    }
    out
}
