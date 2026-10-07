---
date: 2026-10-06
episodes_processed: 5
episodes_found_in_rss: 5
feed_fallbacks_recovered: 4
feed_fallbacks_identified: 40
trailers_skipped: 0
late_additions: 0
generated_at: "2026-10-07T00:40:23Z"
generated_by: "Hermes cron (podcast aggregation pipeline) — manual fallback"
---

# Podcast Summary - 2026-10-06

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-06 (Tuesday) |
| Window | 2026-10-06 only (last_run_date 2026-10-05 + 1); 2026-10-05 re-fetched for late episodes — none found |
| Subscriptions synced | 47 (Apple Podcasts) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 40 on the first parallel pass — mass truncation (throughput collapse), not per-feed breakage |
| Feed recovery | All 40 re-fetched directly with a browser User-Agent: 24 recovered on the first pass, the remaining 16 via curl — 0 left unrecovered |
| In-window episodes | 5 (2026-10-06); 0 late additions from 2026-10-05 |
| Episodes summarized | 5 |
| Failures | 0 |
| Transcript coverage | 5/5 substantial (100%) |
| Transcript sources | `youtube_autocaptions` 4, `show_notes` 1 |
| Note | Claude Code OAuth expired (21st consecutive day) — run completed via Manual Pipeline Fallback |

## Episodes - 2026-10-06 (5 summarized)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| The TWIML AI Podcast | [[the-twiml-ai-podcast__why-jev-is-changing-how-we-build-with-ai-with-diogo-almeida-779]] | 1:30:09 | `youtube_autocaptions` | YouTube `9Aato-NfjoU` "Why Jev Is Changing How We Build With AI" (5380s, exact title match vs RSS 5409s) |
| Invest Like the Best with Patrick O'Shaughnessy | [[invest-like-the-best-with-patrick-oshaughnessy__andrew-huberman-the-frontier-of-neurotechnology-ep-494]] | 1:12:28 | `youtube_autocaptions` | YouTube `_QSX3BF9UX0` "Why Every AI Lab Will Become a Biotech Company \| Andrew Huberman" (4775s); Colossus episode page for cross-reference |
| Training Data | [[training-data__google-s-ai-infrastructure-chief-amin-vahdat-on-the-physics-and-economics-of-frontier-ai]] | 1:04:26 | `youtube_autocaptions` | YouTube `bGph8GwB3Sk` "Google's AI Infrastructure Chief, Amin Vahdat, on the Physics & Economics of Frontier AI" (3867s ≈ RSS 3866s); full captions recovered on retry — upgraded from a structured `web` summary |
| The Compound and Friends | [[the-compound-and-friends__sell-side-indicator-bofa-is-almost-on-sell-vix-is-still-asleep-the-bear-case-revisited-fico-massacred-wayt]] | 1:02:20 | `youtube_autocaptions` | YouTube `IYqPOO7a0T4` "One Step Away From Sell \| WAYT?" (3726s vs RSS 3740s); full captions recovered on retry — upgraded from show notes |
| StarTalk with Neil deGrasse Tyson | [[startalk-with-neil-degrasse-tyson__cosmic-queries-light-sails-and-quantum-scales]] | 1:00:04 | `show_notes` | startalkmedia.com episode page (full transcript Patreon-gated); no YouTube upload at publish time |

## Feed errors (40 — all recovered, 0 lost episodes)

The first parallel fetch hit a mass-truncation event: 40 of 46 feeds returned partial XML (`XML: unclosed CDATA section`, `no element found`, `unclosed token` at wildly different offsets) and only 1 episode was found. Per the documented recovery pattern this was NOT per-feed breakage — the feeds were re-fetched directly with a browser User-Agent. 24 feeds returned full payloads immediately (Bankless 22.2 MB, Unchained 13.4 MB, Lenny's 3.1 MB, Odd Lots 6.5 MB, The Investor's Podcast 27.3 MB, etc.); the remaining 16 (Bankless, Latent Space, MLST, RiskReversal, Practical AI, Chalk Radio, Flirting with Models, ACQ2, Acquired, The Rest Is History, In Good Company, The Memo by Howard Marks, Odd Lots, Lenny's, The Peter McCormack Show, TIP Network) were recovered via `curl` with the same UA. Net: **4 additional in-window episodes** surfaced that the parallel fetch missed.

Feed error list: ACQ2 by Acquired, Acquired, Animal Spirits, anything goes with emma chamberlain, Bankless, BG2Pod, Big Technology Podcast, Chalk Radio, Critics at Large, Dwarkesh Podcast, Empire, Flirting with Models, In Good Company, Invest Like the Best, Latent Space, Lenny's Podcast, Lex Fridman, MLST, Masters in Business, No Priors, Odd Lots, Philosophize This!, Practical AI, RiskReversal, StarTalk, The Compound and Friends, The Edge, TIP Network, The Master Investor, The Meb Faber Show, The Memo, The Peter McCormack Show, The Peterman Pod, The Rest Is History, TWIML, Unchained, Y Combinator, 李诞, 硅谷101, 西西弗高速.

## Notes

- 2026-10-05 was re-fetched and diffed against `state.json` `processed` — all 10 episodes were already covered, so no late additions were written to the prior day's directory.
- StarTalk's episode page confirms Season 17 Episode 60, published October 6, 2026; the full transcript requires a StarTalk+ Patreon tier, so the structured "About This Episode" notes were used (`show_notes`).
- TWIML #779 and The Compound's WAYT? published at 21:43Z and 21:00Z respectively (2:43 PM / 2:00 PM PDT) — inside the catch-up window.
- Post-run upgrade: full YouTube auto-caption transcripts were recovered on retry for **Training Data** (`bGph8GwB3Sk`) and **The Compound and Friends** (`IYqPOO7a0T4`), which the first pass had left as `web` / `show_notes` after hitting HTTP 429 on captions. Both summaries were re-generated from the verbatim transcripts (`transcript_source: youtube_autocaptions`).
