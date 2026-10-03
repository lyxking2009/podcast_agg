---
date: 2026-10-03
episodes_processed: 1
episodes_found_in_rss: 1
feed_fallbacks_recovered: 0
feed_fallbacks_identified: 5
trailers_skipped: 0
late_additions: 0
generated_at: "2026-10-03T22:04:30Z"
generated_by: "Hermes cron (podcast aggregation pipeline) - Manual Pipeline Fallback (Claude Code OAuth expired, 22nd consecutive day)"
---

# Podcast Summary - 2026-10-03

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-03 (Saturday) |
| Window | 2026-10-03 only (last_run_date 2026-10-02 + 1) |
| Subscriptions synced | cached (Apple Podcasts DB query timed out at 30s) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 5 on the parallel pass: Bankless, Chalk Radio, Critics at Large, Latent Space, The Edge |
| In-window episodes | 1 from RSS |
| Episodes summarized | 1 |
| Failures | 0 at episode level |
| Transcript coverage | 1/1 substantial transcript (100%) |
| Final transcript sources | `rss_omny_srt` 1 |

## Episodes - 2026-10-03 (1)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| Everybody's Business | [[everybodys-business__are-you-feeling-the-streaming-fatigue]] | 35:32 | `rss_omny_srt` | Omny SRT |

## Feed errors (5)

| Feed | Error | Resolution |
|---|---|---|
| Bankless | XML mismatched tag (Flightcast 167-byte error page) | Verified via browser-UA curl (22.2 MB, 1,379 items) - in-window ROLLUP (10-02) already in state |
| Latent Space | XML mismatched tag (Flightcast 167-byte error page) | Verified via browser-UA curl (14.4 MB, 231 items) - no new in-window episode; "Academia is for Ambition" (10-01) already in state |
| Chalk Radio | timeout | Verified via browser-UA curl (628 KB, 60 items) - latest 2026-03-05, no in-window episode |
| Critics at Large (New Yorker) | timeout | Verified via browser-UA curl (1.44 MB, 148 items) - "Tom Cruise-a-Palooza" (10-01) already processed |
| The Edge | timeout (Buzzsprout) | Verified via browser-UA curl (131 KB, 37 items) - latest 2026-09-24, no in-window episode |

## Prior-date late-publishing check

Re-fetched 2026-10-02 and diffed GUIDs against `state.json` `processed`: 11 of 12 already processed; **1 late addition recovered** - Big Technology Podcast "Anthropic's IPO Leak, OpenAI's Dots vs. Meta's Muse, Visual Turing Test" (pubDate 2026-10-02 21:59Z, about a minute before the previous run's fetch). Summarized from the full podscripts.co transcript and written into the 2026-10-02 vault, with a late-additions note appended to that date's report.

## Notes

- Saturday run: only Everybody's Business (Bloomberg's weekend show) published; expected low volume.
- Claude Code OAuth expired again (**22nd consecutive day**) - `run_podcast_pipeline.py` exits ~2 min after launch with `Failed to authenticate`. Confirmed with a direct test prompt before choosing Manual Pipeline Fallback (1-2 episodes, far inside the 16-episode budget).
- All 5 parallel-fetch feed errors were re-verified with a browser-UA curl; every one returned HTTP 200 with a full payload, confirming the failures were fetcher-side truncation/rate-limiting rather than broken feeds. No in-window episodes were missed.
- Everybody's Business transcript came from the Omny `<podcast:transcript>` SRT (Rung 1).
- Big Technology is Megaphone-hosted with no RSS transcript and no same-day YouTube upload; the full verbatim transcript was obtained from podscripts.co (Rung 2).

## Wikilinks

- Everybody's Business: [[everybodys-business__are-you-feeling-the-streaming-fatigue]]
