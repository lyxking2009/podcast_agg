---
date: 2026-10-08
episodes_processed: 7
episodes_found_in_rss: 6
late_additions: 0
failures: 0
generated_at: "2026-10-08T22:09:15Z"
generated_by: "Hermes cron (podcast aggregation pipeline)"
---

# Podcast Summary - 2026-10-08

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-08 (Thursday) |
| Window | 2026-10-08 only (last_run_date 2026-10-07 + 1) + late check of 2026-10-07 |
| Subscriptions synced | 47 (Apple Podcasts) |
| In-window episodes | 7 (2026-10-08) |
| Episodes summarized | 7 |
| Failures | 0 |
| Transcript coverage | 7/7 (100%) |
| Transcript sources | `rss_omny_srt` 1, `rss_transistor_vtt` 1, `web_substack` 1, `web` 3, `web_podscripts` 1 |

> Note: Claude Code OAuth session was expired this run (auth failure on `run_podcast_pipeline.py`), so the day was completed via manual fallback. A concurrent sibling run processed 6 of the 7 episodes; this run added TIP852 and the 2026-10-07 late addition, then deduplicated by GUID and authored the reports and state.

## Episodes - 2026-10-08 (7 summarized)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| Critics at Large | The New Yorker | [[critics-at-large-the-new-yorker__the-social-reckoning-shows-how-it-all-went-wrong]] | 50m0s | `web` | newyorker.com episode page |
| Latent Space: The AI Engineer Podcast | [[latent-space-the-ai-engineer-podcast__synthesis-superintelligence-from-semiconductors-to-superconductors-periodic-labs]] | 1h24m0s | `web_substack` | Full public Substack transcript at latent.space/p/periodic |
| Odd Lots | [[odd-lots__what-everyone-gets-wrong-about-the-economic-problems-in-europe]] | 52m39s | `rss_omny_srt` | Omny SubRip transcript declared in the RSS feed |
| Practical AI | [[practical-ai__narrative-intelligence-and-the-human-advantage]] | 44m24s | `rss_transistor_vtt` | Transistor WebVTT transcript from the RSS feed |
| The Investor's Podcast (We Study Billionaires) | [[the-investors-podcast-we-study-billionaires__tip852-hermes-and-lvmh-stock-time-to-buy-luxury-w-daniel-mahncke-and-s]] | 1h15m48s | `web_podscripts` | TIP public transcript preview + podscripts.co structured summary |
| The Peter McCormack Show | [[the-peter-mccormack-show__219-paul-morland-why-the-next-global-financial-crisis-is-mathematically-inevitab]] | 1h23m11s | `web` | Acast episode page (timestamped chapters + description); YouTube 3kvuK93Z15I captions rate-limited (429) |
| Unchained (Uneasy Money) | [[unchained__how-openai-s-math-dump-has-crypto-rethinking-its-cryptography-uneasy-money]] | 1h19m40s | `web` | unchainedcrypto.com episode page; YouTube mD7UY-1xQL0 captions rate-limited (429) |

## Feed errors (recovered / verified)

- **Latent Space** — Flightcast 167-byte error in the parallel fetch; direct browser-UA curl retrieved the feed (13.5 MB) and its in-window episode was processed. ✅ recovered
- **Critics at Large** — timeout in the parallel fetch; direct UA curl retrieved the feed and its 2026-10-08 episode was processed. ✅ recovered
- **Bankless** — Flightcast 167-byte error; direct UA curl OK (21.8 MB). Latest episode 2026-10-05 — **no in-window episode**. ✅ verified, nothing missed
- **Lex Fridman Podcast** — truncated download; direct UA curl OK. Latest episode 2026-09-17 — **no in-window episode**. ✅ verified
- **Chalk Radio** — timeout; direct UA curl OK. Latest episode 2026-03-05 — **no in-window episode**. ✅ verified
- **The Edge** — timeout; direct UA curl OK. Latest episode 2026-09-24 — **no in-window episode**. ✅ verified

## Notes

- Odd Lots guest name rendered as **Dominik Leusder** in the vault note (transcript auto-caption mangles the spelling).
- Latent Space's RSS `pubDate` is 2026-10-08 16:27 UTC (09:27 PT) — an in-window episode that the initial parallel fetch had marked as a feed error.
- No `failures` entries: every in-window episode yielded an acceptable transcript or structured source.

## Late additions (recovered 2026-10-09)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| 硅谷101 | [[硅谷101__e255-yue-moxing-yuelai-yueqiang-zhang-kuo]] | 46m13s | `web` | fireside show notes (sv101.fireside.fm/269) + BigGo structured summary with verbatim quotes |

- 硅谷101 E255 (published 2026-10-08) was missed by the 10-08 run and recovered during the 2026-10-09 catch-up via a prior-date RSS diff against state.
