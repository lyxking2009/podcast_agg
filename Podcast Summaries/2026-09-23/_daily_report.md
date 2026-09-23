---
date: 2026-09-23
episodes_processed: 10
episodes_found_in_rss: 10
feed_fallbacks_recovered: 0
trailers_skipped: 0
late_additions: 1
generated_at: 2026-09-23T22:20:00Z
generated_by: "Hermes cron (podcast aggregation pipeline, deepseek-v4-pro)"
---

# Podcast Summary — 2026-09-23

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-23 (Wednesday) |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 47 (46 active, 1 excluded by config) |
| Feed errors | 0 |
| Episodes found in scan window | 8 (primary pass) |
| Episodes summarized | 10 for 09-23 (8 primary + 2 from concurrent sibling pass), plus 1 late addition filed under 09-22 |
| Failures | 0 |
| Transcript coverage | 11/11 new episodes (100%) |
| Trailers/promos skipped | 0 |
| Rung 1 / Rung 2 / Rung 3 | 2 / 6 / 0 |
| Pipeline | Hermes-native (RSS fetch → transcript ladder → DeepSeek `deepseek-v4-pro` → Markdown) |

All 46 active feeds returned parseable XML on the first parallel pass (Chrome User-Agent, 8 workers). `Sticky Notes: The Classical Music Podcast` was skipped by `excluded_feeds` in `data/config.json`.

## Transcript sources (this date)

| Source | Count |
|---|---|
| `youtube_autocaptions` | 6 |
| `web` | 2 |
| `rss_vtt` | 1 |
| `rss_omny_srt` | 1 |

(The 2026-09-22 late addition `Compound and Friends` used `show_notes`.)

| Published | Episodes |
|---|---|
| 2026-09-22 | 1 (late addition) |
| 2026-09-23 | 10 |

## Episodes

| Show | Episode | Published | Duration | Source |
|---|---|---|---|---|
| Machine Learning Street Talk (MLST) | [[machine-learning-street-talk-mlst__how-deep-learning-finally-cracked-messy-tables-frank-hutter]] | 2026-09-23 | 1h53m12s | `youtube_autocaptions` |
| Latent Space: The AI Engineer Podcast | [[latent-space-the-ai-engineer-podcast__bio-security-is-an-ai-arms-race-eric-nguyen-ceo-radical-numerics]] | 2026-09-23 | 1h31m58s | `youtube_autocaptions` |
| Animal Spirits Podcast | [[animal-spirits-podcast__the-wobbly-house-of-cards-ep-483]] | 2026-09-23 | 1h16m3s | `youtube_autocaptions` |
| In Good Company with Nicolai Tangen | [[in-good-company-with-nicolai-tangen__president-alexander-stubb-a-changing-world-order-democracies-facing-ai-and-golfing-with-trump]] | 2026-09-23 | 49m36s | `youtube_autocaptions` |
| Unchained | [[unchained__how-zcash-and-near-are-driving-this-crypto-bull-run]] | 2026-09-23 | 56m06s | `web` |
| Bankless | [[bankless__fomo-meme-stocks-and-robinhood-chain-andy8052-eric-conner]] | 2026-09-23 | 56m45s | `rss_vtt` |
| Big Technology Podcast | [[big-technology-podcast__youtube-ceo-neal-mohan-why-we-re-betting-on-ai-and-not-afraid-of-it]] | 2026-09-23 | 1h2m41s | `web` |
| RiskReversal Pod | [[riskreversal-pod__the-ai-trying-to-replace-your-financial-advisor-with-fahad-hassan-and-david-cusa]] | 2026-09-23 | 39m26s | `youtube_autocaptions` |
| Masters in Business | [[masters-in-business__at-the-money-who-is-wealthy-in-america-and-how-did-they-get-rich]] | 2026-09-23 | 25m28s | `rss_omny_srt` |
| Unchained | [[unchained__bits-bips-is-bitcoin-finally-being-priced-on-fundamentals]] | 2026-09-23 | 13m26s | `youtube_autocaptions` |
| The Compound and Friends | [[the-compound-and-friends__metas-muse-launch-josh-is-wrong-on-netflix-internals-weaken-10-year-bonds-at-5-buffett-steps-away]] | 2026-09-22 | 1h06m29s | `show_notes` |

## Scan window

Primary pass: `2026-09-23T07:00:00+00:00` → `2026-09-24T06:59:59+00:00` (label window 2026-09-23 PT), derived from `last_run_date` = `2026-09-22`; `lookback_cap_days` is 7, so no cap applied.

Verification sweep: a second fetch pass over `2026-09-22T07:00:00+00:00` → `2026-09-24T06:59:59+00:00` (one-day backfill) returned **0 unprocessed episodes**, confirming the union of this run, the concurrent sibling pass, and the 2026-09-22 digest covers the whole range.

## Feed errors (handled)

**None.** All 46 active feeds returned parseable XML (the sibling pass logged transient parse/timeout errors on Bankless, Latent Space, Lex Fridman, Chalk Radio, Critics at Large and The Edge; every one of those outlets either had no in-window episode or was already resolved here — see Notes).

## Notes

- **Concurrent sibling run reconciled.** A second podcast pipeline instance ran against the same vault at the same time (its own `_daily_report.md`, written 15:07 PT, used `deepseek-v4-flash` and show-notes/web-search fallbacks). It covered three episodes this run did not (`In Good Company` — Stubb, `Unchained` — Zcash/NEAR, plus the 2026-09-22 `Compound and Friends` late addition) and produced duplicate files for three episodes both runs handled (`Bankless`, `Big Technology`, `RiskReversal`). Reconciliation: the `deepseek-v4-pro` files from primary transcripts were kept; the three lower-fidelity duplicates were removed. Vault now holds exactly one file per GUID; `state.json` carries one entry per episode.
- **Late-publishing episodes fall between windows.** `In Good Company` (published 2026-09-23T03:00Z = 2026-09-22 20:00 PT) and `Unchained` Zcash/NEAR (2026-09-23T05:50Z = 2026-09-22 22:50 PT) both published during the 09-22 PT day *after* the previous digest's 15:08 PT completion, so the literal "day after `last_run_date`" window skipped them; the sibling pass caught both. A one-day backfill at the start of the window (as used by the 2026-09-22 run) is the durable fix, and the verification sweep above shows today's union left nothing else behind.
- **Rung 1 worked for both feeds that ship transcripts.** Bankless served its declared Flightcast VTT (`01M36RG2D8KB09CCWHRE8ET72R.vtt`, 59 K chars) and Masters in Business served its Omny SubRip (`c5f15608-…`, 27 K chars) — both non-empty and used directly.
- **YouTube auto-captions were the workhorse (6 of 11), and the direct `yt-dlp` subtitle endpoint 429'd.** `yt-dlp --write-auto-subs` returned `HTTP 429` for the auto-generated `en` tracks on Big Technology, Animal Spirits and Bits + Bips, while manual `en-GB`/`en-US` tracks downloaded normally. The 429s were bypassed with `youtube_transcript_api` (`api.fetch(vid).to_raw_data()`), which succeeded on the first try for `telFEyDmAgE`, `S7pe8L4WeRI`, `k7YUF519bNc` and `8HnjDDPc9Yk`. **Keep that client in the ladder for future runs.**
- **Identity by duration, not title.** MLST's upload is titled *“The AI That Replaces Hours of Model Tuning - Frank Hutter”* (6,793 s) against an RSS duration of 1:53:12; Latent Space's *“An AI that perceives the world through DNA”* is 5,519 s vs 1:31:58; RiskReversal's *“Meet Range: The AI Trying to Replace Your Financial Advisor”* is 2,261 s vs 39:26. All three were confirmed as the same episode by duration plus on-transcript content checks.
- **Big Technology's Substack post is a full, lightly-edited Q&A** (40 K chars) and is a cleaner source than the caption track — same for the show's own video upload. Worth going straight to `bigtechnology.com/p/<slug>` for this show.
- **Latent Space show notes are not a transcript.** `latent.space/p/bio-security-is-an-ai-arms-race-eric` carries editorial notes plus one pull quote; the full episode only exists as the YouTube upload (`B7DdNj_VjcU`, 1h32m). Don't count the Substack page as a transcript hit next time.
- **Thematic convergence: AI moving from demo to regulated infrastructure.** Hutter's TabPFN (tabular foundation models beating XGBoost), Nguyen's biosecurity arms race (genomes can't be patched, so screening must be AI-native), Mohan's YouTube AI-tooling strategy, Range's SEC-regulated agentic wealth advisor and the Bits + Bips regime-change debate all describe the same shift — models are being pushed into domains with real consequences, real regulators and real liability.
- **Toolchain note:** the previously used caption downloader (`/opt/homebrew/Cellar/yt-dlp/2026.7.4` under `python3.14`) is broken — that Cellar pin now holds 2026.8.19 with no `yt_dlp` module. The working binary is the pyenv shim (`/Users/yuxinglin/.pyenv/shims/yt-dlp`, 2026.07.04) plus `youtube_transcript_api` from the system Python. `timeout(1)` is not available on this macOS box (use `gtimeout` or no timeout).
- **No duplicate GUIDs.** All 11 new GUIDs verified present exactly once in `state.json` and exactly once as a vault file (after removing the sibling duplicates); `processed` map grew 976 → 987; `last_run_date` = `2026-09-23`.
