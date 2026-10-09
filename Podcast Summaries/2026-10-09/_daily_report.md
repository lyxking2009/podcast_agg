---
date: 2026-10-09
episodes_processed: 13
episodes_found_in_rss: 12
late_additions: 1
failures: 0
generated_at: "2026-10-09T22:06:28Z"
generated_by: "Hermes cron (podcast aggregation pipeline)"
---

# Podcast Summary - 2026-10-09

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-09 (Friday) |
| Window | 2026-10-09 only (last_run_date 2026-10-08 + 1) + late check of 2026-10-08 |
| Subscriptions synced | 47 (Apple Podcasts) |
| In-window episodes | 12 (RSS) + 1 (Bankless feed-fallback) = 13 |
| Episodes summarized | 13 |
| Late additions (2026-10-08) | 1 (硅谷101 E255) |
| Failures | 0 |
| Transcript coverage | 13/13 (100%) |
| Transcript sources | `rss_omny_srt` 4, `web` 4, `web_substack` 1, `show_notes` 2, `web_search_fallback` 1 |

> Note: Claude Code OAuth session was expired this run (`run_podcast_pipeline.py` exited after ~2 min with `Failed to authenticate: OAuth session expired and could not be refreshed`), so the day was completed via the Manual Pipeline Fallback (13 episodes, within the <=16 limit). YouTube auto-captions were HTTP-429 rate-limited, so web sources (BigGo, bankless.com, unchainedcrypto.com, thecompoundnews.com, acast/nbim, megaphone RSS description) carried the non-transcript episodes.

## Episodes - 2026-10-09 (13 summarized)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| Bankless | [[bankless__rollup-ai-threatens-crypto-security-tom-lee-stops-buying-eth-ethereum-l2s-shut-down-zcash-etfs]] | ~60m | `web_search_fallback` | bankless.com ROLLUP episode page (transcript); RSS feed failed (Flightcast) |
| Big Technology Podcast | [[big-technology-podcast__meta-and-microsoft-s-claude-slowdown-his-agent-leaked-his-banking-info-don-t-bully-your-ai]] | 55m39s | `show_notes` | Megaphone RSS description (10-point running order) |
| Everybody's Business | [[everybody-s-business__the-nfl-enters-its-flag-era]] | 35m44s | `rss_omny_srt` | Omny SubRip transcript declared in the RSS feed |
| In Good Company with Nicolai Tangen | [[in-good-company-with-nicolai-tangen__friday-wrap-up-lessons-from-a-former-boss-and-the-decline-of-shareholder-rights]] | 12m03s | `web` | Acast episode page (show notes) |
| Masters in Business | [[masters-in-business__balancing-conviction-and-risk-masters-in-business-with-maria-vassalou]] | 69m16s | `rss_omny_srt` | Omny SubRip transcript |
| Money Stuff: The Podcast | [[money-stuff-the-podcast__jeremy-maletz]] | 54m25s | `rss_omny_srt` | Omny SubRip transcript |
| No Priors | [[no-priors__beam-the-great-american-open-model-with-reflectionai-co-founder-and-ceo-misha-laskin]] | 69m44s | `web` | BigGo structured summary + verbatim quotes |
| Odd Lots | [[odd-lots__how-ai-is-upending-the-world-of-mathematics]] | 60m21s | `rss_omny_srt` | Omny SubRip transcript |
| RiskReversal Pod | [[riskreversal-pod__bond-market-dominoes-the-path-to-6-rates-with-peter-boockvar]] | 44m24s | `web_substack` | riskreversal.substack.com structured recap |
| The Compound and Friends | [[the-compound-and-friends__this-is-where-the-rubber-meets-the-road-with-jurrien-timmer]] | 75m01s | `show_notes` | podcasts.thecompoundnews.com episode page |
| The Meb Faber Show | [[the-meb-faber-show-better-investing__andrew-ang-should-ai-agents-run-your-asset-allocation-653]] | 45m52s | `web` | ai-street.co / arXiv paper ('The Self-Driving Portfolio') coverage |
| Unchained | [[unchained__how-near-intents-identified-a-suspect-and-got-its-stolen-funds-back]] | 46m00s | `web` | unchainedcrypto.com episode page |
| Unchained (Chopping Block) | [[unchained__the-chopping-block-live-at-token2049-feat-balaji-srinivasan-and-arthur-hayes-on-kazakhstan-crypto-bunker-mode-and-the-l2-shutdowns]] | 42m03s | `web` | YouTube description (chapters) + Apple Podcasts episode summary |

## Feed errors (recovered / verified)

- **Bankless** — Flightcast `mismatched tag` error in the parallel fetch; direct browser-UA curl OK (21.9 MB, 1,348 items). Its **2026-10-09 ROLLUP** was recovered via web fallback. ✅ recovered
- **Latent Space** — Flightcast `mismatched tag`; direct UA curl OK (14.5 MB). Latest in-window episode (2026-10-08, Synthesis Superintelligence) was **already processed on 10-08**. ✅ verified, nothing missed
- **Critics at Large** — timeout in the parallel fetch; direct UA curl OK. Latest in-window episode (2026-10-08, "The Social Reckoning") was **already processed on 10-08**. ✅ verified
- **Lex Fridman Podcast** — truncated download (`unclosed CDATA section`); direct UA curl OK (2.1 MB). Latest episode 2026-09-17 — **no in-window episode**. ✅ verified
- **Chalk Radio** — timeout; direct UA curl OK. Latest episode 2026-03-05 — **no in-window episode**. ✅ verified
- **The Edge** — timeout; direct UA curl OK. Latest episode 2026-09-24 — **no in-window episode**. ✅ verified

## Late additions - 2026-10-08

| Show | Episode | Duration | Source |
|---|---|---|---|
| 硅谷101 | [[硅谷101__e255-yue-moxing-yuelai-yueqiang-zhang-kuo]] | 46m13s | `web` |

- 硅谷101 E255 (published 2026-10-08) was missed by the 10-08 run; recovered during the 2026-10-09 catch-up via a prior-date RSS diff against `state.json`. Written into the `2026-10-08/` directory with a `## Late additions (recovered 2026-10-09)` section appended to that day's report.

## Notes

- No `failures` entries: every in-window episode yielded an acceptable transcript or structured source.
- YouTube auto-captions for No Priors (`up4sG9RM20M`), Unchained NEAR (`SHFrG_QOzQM`) and Chopping Block (`f1R55panuL4`) all returned HTTP 429 (rate-limited) — web sources used instead.
- Bankless uses a non-RSS GUID key (`bankless:rollup-2026-10-09`) because its feed failed and supplied no GUID.

