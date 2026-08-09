# Daily Report — 2026-08-09

**Run type:** Scheduled cron (Hermes Agent)

**Episodes found (RSS):** 2 | **Feed errors:** 0 | **Processed:** 1 (1 web) | **Failed:** 1 (trailer)

## Transcript source breakdown
| Source | Count |
|---|---|
| web (YouTube auto-captions) | 1 |

## Episodes

| Show | Episode | Source | Links |
|---|---|---|---|
| Lenny's Podcast | The playbook for building high talent density teams \| Adam Ward, Head of Talent at Cursor | web | [[lenny-s-podcast-product-career-growth__the-playbook-for-building-high-talent-density-teams-adam-ward-head-of-talent-at]] |

## Failures (1)

| Show | Episode | Reason |
|---|---|---|
| Odd Lots | Introducing: Our Town | Trailer for Bloomberg's "Our Town" (1m53s promo, RSS SRT available but only 376 words / promotional content). DeepSeek returned the all-null error case: "Transcript is incomplete; only a promotional trailer was provided, not a full episode." Recorded in failures (attempts=1). |

## Notes
- Window: 2026-08-09T07:00:00Z .. 2026-08-10T07:00:00Z (Aug 9, America/Los_Angeles). 47 subscriptions synced from Apple Podcasts (1 excluded: Sticky Notes).
- Lenny's Podcast (90m46s, Adam Ward — Head of Talent at Cursor): no RSS-declared transcript; web search found the YouTube cross-post (video zegYJ6dhIg4, duration matches exactly) and auto-captions were pulled via yt-dlp (907 KB VTT → 95,914 chars clean text). Summarized with deepseek-v4-pro: 17 key points, 5 verbatim quotes, 9 people, 7 topics.
- Odd Lots "Introducing: Our Town" is a promotional trailer for the new Bloomberg Big Take show — no full episode content exists yet, so no summary was produced (recorded as failure, not processed).
- State updated: last_run_date → 2026-08-09; 1 processed entry added; 1 failure added. No existing failures retried.
