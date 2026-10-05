---
date: 2026-10-04
episodes_processed: 3
episodes_found_in_rss: 3
feed_fallbacks_recovered: 0
feed_fallbacks_identified: 0
trailers_skipped: 0
late_additions: 0
generated_at: "2026-10-04T22:18:30Z"
generated_by: "Hermes cron (podcast aggregation pipeline)"
---

# Podcast Summary - 2026-10-04

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-04 (Sunday) |
| Window | 2026-10-04 only (last_run_date 2026-10-03 + 1) |
| Subscriptions synced | ❌ cached (Apple Podcasts DB read blocked by macOS TCC — see Notes) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 1 (excluded feed only) — 46/47 parsed cleanly |
| In-window episodes | 3 |
| Episodes summarized | 3 |
| Failures | 0 |
| Transcript coverage | 3/3 substantial (100%) |
| Final transcript sources | `web` 2, `rss` 1 |

## Episodes - 2026-10-04 (3 summarized)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| The Investor's Podcast (We Study Billionaires) | [[the-investors-podcast-we-study-billionaires-the-investors-podcast-network__tip851-heico-vs-transdigm-whose-aerospace-monopoly-is-better-w-kyle-grieve-and-shawn-omalley]] | 1:17:30 | `web` | podscripts.co full verbatim transcript |
| Lenny's Podcast | [[lenny-s-podcast-product-career-growth__openais-head-of-chatgpt-were-entering-a-new-era-of-ai-again-tibo-sottiaux]] | 37:22 | `web` | BigGo Finance structured summary + YouTube full chapter description |
| Bankless | [[bankless__vvv-community-call-september]] | 1:04:41 | `rss` | RSS-declared VTT transcript (flightcast) |

## Notes

- Sunday run: low volume as expected. Only 3 in-window items across 46 feeds; all 3 summarized.
- **Bankless "$VVV Community Call | September" was recovered.** An earlier pass this day logged it as `no_transcript_found`; the episode ships a `podcast:transcript` VTT element (`https://rss.flightcast.com/transcripts/01M42EJXQZKN83N6TH38XWYXM6.vtt`, 12,529 words) which was downloaded, converted to plain text (Rung 1) and summarized normally. Not a failure.
- **Apple Podcasts DB sync is blocked.** Reading `MTLibrary.sqlite` (514 MB) hangs — both a direct `cat`/Python read and an `osascript` read block for 20-25 s+ with 0 bytes transferred, consistent with a macOS TCC (Full Disk Access) denial for this process. Fell back to cached `data/subscriptions.json` (47 subs, last synced 2026-10-02) per pipeline spec. The 2026-10-03 run logged the same symptom ("DB query timed out at 30s"). Needs a one-time Full Disk Access grant for the Hermes process to resume live subscription sync.
- Claude Code OAuth remains expired (23rd+ consecutive day); this run used the Hermes-native pipeline directly.
- Prior-date late-publishing check: the 2026-10-03 in-window episode (Everybody's Business "Are You Feeling the Streaming Fatigue?") was already processed — 0 late additions.

## Wikilinks

- The Investor's Podcast (We Study Billionaires): [[the-investors-podcast-we-study-billionaires-the-investors-podcast-network__tip851-heico-vs-transdigm-whose-aerospace-monopoly-is-better-w-kyle-grieve-and-shawn-omalley]]
- Lenny's Podcast: [[lenny-s-podcast-product-career-growth__openais-head-of-chatgpt-were-entering-a-new-era-of-ai-again-tibo-sottiaux]]
- Bankless: [[bankless__vvv-community-call-september]]

## Late additions (recovered 2026-10-05)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| The Rest Is History | [[the-rest-is-history__711-the-terror-killing-god-part-5]] | 1:16:57 | `web` | podscripts.co full transcript |

Published 2026-10-04 23:05 UTC (16:05 PT) — after the 2026-10-04 run's 22:18Z RSS fetch, so it was missed by the same-day run and recovered by the next day's prior-date diff.
