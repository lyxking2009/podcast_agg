---
date: 2026-10-07
episodes_processed: 5
episodes_found_in_rss: 5
late_additions: 2
failures: 0
generated_at: "2026-10-07T22:13:00Z"
generated_by: "Hermes cron (podcast aggregation pipeline)"
---

# Podcast Summary - 2026-10-07

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-07 (Wednesday) |
| Window | 2026-10-07 only (last_run_date 2026-10-06 + 1 → 2026-10-07) |
| Subscriptions synced | 47 (Apple Podcasts) |
| Feeds fetched | 47 (1 excluded by config: Sticky Notes) — 0 feed errors |
| In-window episodes | 5 (2026-10-07) |
| Episodes summarized | 5 |
| Late additions recovered | 2 (2026-10-06 episodes published after the 10-06 run finished) |
| Failures | 0 |
| Transcript coverage | 5/5 substantial (100%) |
| Transcript sources | `youtube_autocaptions` 3, `web_podscripts` 1, `rss_omny_srt` 1 |
| Note | A parallel duplicate run of this same job wrote competing files; duplicates were de-duplicated, keeping the higher-quality transcript-grounded versions |
| Note | The concurrent duplicate run also corroborated both 2 late additions |

## Episodes - 2026-10-07 (5 summarized)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| RiskReversal Pod | [[riskreversal-pod__trading-from-the-short-side-with-danny-moses-vincent-daniel-porter-collins]] | 49m11s | `youtube_autocaptions` | YouTube `CD2ZdM39Ywg` "Trading From The Short Side" (RiskReversal Media); ~48K-char captions |
| Animal Spirits Podcast | [[animal-spirits-podcast__sudden-wealth-syndrome-ep-485]] | 1h3m55s | `youtube_autocaptions` | YouTube `MhRHngKYa_8` "Sudden Wealth Syndrome \| Animal Spirits 485" (The Compound); ~68K-char captions |
| Big Technology Podcast | [[big-technology-podcast__can-ai-keep-growing-exponentially-let-s-ask-semianalysis-with-dylan-patel-and-jo]] | 1h2m4s | `web_podscripts` | podscripts.co episode transcript (~64K chars) |
| Empire | [[empire__jeff-yan-on-hyperliquid-s-plan-to-bring-all-finance-onchain]] | 30m35s | `youtube_autocaptions` | YouTube `e0xi_NeVk4w` (Empire); English VTT recovered via yt-dlp after the YouTube API exposed only auto-translated tracks |
| Masters in Business | [[masters-in-business__at-the-money-what-the-best-ceos-actually-do]] | 20m16s | `rss_omny_srt` | Omny SubRip transcript declared in the RSS feed |

## Late additions (2026-10-06, published after the 10-06 run completed)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| In Good Company with Nicolai Tangen | [[in-good-company-with-nicolai-tangen__john-armitage-how-he-invests-building-egerton-and-what-worries-him-about-ai]] | 54m25s | `youtube_autocaptions` | YouTube `6431POnuLKw`; RSS pubDate 2026-10-06 20:00 PT |
| Unchained | [[unchained__thorchain-says-it-can-t-block-north-korean-laundering-is-that-true]] | 1h11m43s | `show_notes` | unchainedcrypto.com episode page; RSS pubDate 2026-10-07 03:54 UTC = 2026-10-06 20:54 PT |

## Notes

- **Timezone note:** Unchained's feed stamps `-0000` but means UTC. `Wed, 07 Oct 2026 03:54:00 -0000` = 2026-10-06 20:54 PT, corroborated by the episode page ("October 6, 2026 at 11:56 pm ET"). Correctly classified as a 10-06 late addition, not an in-window episode.
- **Duplicate run:** A second instance of this cron job ran concurrently and emitted alternate-slug files for Big Technology and RiskReversal built from show notes (YouTube captions had 429'd for it). Those were removed in favour of the transcript-grounded versions above; the two 10-06 late additions it produced were retained and recorded in `state.json`.

## Late additions (recovered 2026-10-08)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| The Rest Is History | [[the-rest-is-history__712-the-terror-the-fall-of-robespierre-part-6]] | 1h36m55s | `show_notes` | therestishistory.com episode page (description only — no transcript published); YouTube early-access upload is members-only |

_Recovered on 2026-10-08 by re-fetching the 2026-10-07 window and diffing GUIDs against `state.json` — this episode was published after the 2026-10-07 run's RSS fetch._
