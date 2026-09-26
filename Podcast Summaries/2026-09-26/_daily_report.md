---
date: 2026-09-26
episodes_processed: 2
episodes_found_in_rss: 2
feed_fallbacks_recovered: 0
trailers_skipped: 0
late_additions: 0
generated_at: 2026-09-26T22:10:00Z
generated_by: "Hermes cron (podcast aggregation pipeline, deepseek-v4-pro)"
---

# Podcast Summary — 2026-09-26

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-26 (Saturday) |
| Window | 2026-09-26 00:00 PT → 2026-09-26 23:59 PT (`2026-09-26T07:00Z` → `2026-09-27T06:59Z`) |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 46 (1 excluded by config: *Sticky Notes: The Classical Music Podcast*) |
| Feed errors | 0 |
| Episodes found in RSS scan | 2 |
| Episodes summarized | 2 |
| Failures | 0 |
| Transcript coverage | 2/2 = 100% |
| Transcript sources | `web` 1 · `youtube_autocaptions` 1 · `description` 0 |
| Model | `deepseek-v4-pro` |

**Run note.** `last_run_date` was 2026-09-25, so the catch-up window resolved to today only (well inside the 7-day `lookback_cap_days`). Weekend release schedule produced a light day: two episodes, both on the AI-research/robotics track.

**Transcript ladder detail.** Neither episode exposed a `podcast:transcript` element in its RSS feed, so Rung 1 was unavailable for both.

- **Y Combinator Startup Podcast** — *Robot-Use Agents* resolved on Rung 2 via `web_search` → `podscripts.co` (33,961 chars, full 29:24 of audio, transcript complete through the closing credits).
- **MLST** — *When AI Research Starts Moving Faster Than Human Research* resolved on Rung 2 via YouTube auto-captions (`yt-dlp`, video `yB6_iFGTq9k`, duration 2,623 s matched the RSS 43:42 exactly). The episode's own show notes pointed at a `rescript.info` PDF, but that endpoint returned HTTP 403 to non-browser clients.

**Quality pass.** Quotes in both files were verified verbatim against the source transcripts (all matched exactly). The YC summary initially listed Chollet/Amodei/Altman under *People mentioned*; none appear anywhere in the transcript, so those three were removed and the list corrected to the names actually spoken (Isola, Francois, Hamei, Vincent, Jay).

## Episodes

| Show | Episode | Duration | Source |
|---|---|---|---|
| Y Combinator Startup Podcast | Robot-Use Agents: Why General-Purpose Models May Win in Robotics | 29m49s | web (podscripts.co) |
| Machine Learning Street Talk (MLST) | When AI Research Starts Moving Faster Than Human Research - Zhengyao Jiang | 43m42s | youtube_autocaptions |
