---
date: 2026-05-10
pipeline: podcast-aggregation
version: 1.5.0
---

# Daily Podcast Report — 2026-05-10

## Metrics

- **Episodes with transcripts:** 1 (manual via YouTube Rung 3)
- **Episodes summarized:** 1
- **Summarization engine:** Claude Sonnet 4 (primary) — $3.00 budget, actual ~$2-3 for 132K-char transcript
- **Episodes skipped:** —
- **Feed errors:** —

## Episodes

| # | Show | Episode | Guest | Transcript | Summary |
|---|---|---|---|---|---|
| 1 | [[lennys-podcast__eric-ries-incorruptible\|Lenny's Podcast]] | How to build a company that withstands any era | Eric Ries | YouTube (Rung 3) | Claude Sonnet 4 |

## Pipeline Updates

- **v1.5.0 (today):** Summarization engine switched to **Claude Sonnet 4** (via `claude -p`) as primary. DeepSeek v4 Pro is the fallback if Claude fails. End-to-end validated on this episode.
- **v1.4.0 (today):** Output format changed to match Claude `/summarize --depth page` style: Key Points + Overview + Implications.
- **Rung 3 (today):** YouTube auto-captions via `youtube-transcript-api` for paywalled/403-blocked transcripts. First use: Lenny's Podcast (Substack paywall + musixmatch.com 403).

## Successful test

```bash
claude -p "Read transcript, output raw JSON" \
  --add-dir /tmp \
  --model claude-sonnet-4-20250514 \
  --max-budget-usd 3.00
# → Valid JSON (4222 bytes), all 7 fields present
```
