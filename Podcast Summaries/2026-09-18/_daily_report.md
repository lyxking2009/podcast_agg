---
date: 2026-09-18
episodes_processed: 15
episodes_found_in_rss: 14
feed_fallbacks_recovered: 1
generated_at: 2026-09-18T22:32:42Z
generated_by: "Hermes cron (manual pipeline fallback — Claude Code OAuth expired, 14th consecutive day)"
---

# Podcast Summary — 2026-09-18

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-18 (Friday) |
| Episodes found via RSS | 14 |
| Episodes from feed-error fallback | 1 (Bankless ROLLUP) |
| Episodes processed | 15 |
| Transcript coverage | 15/15 (100%) |
| Feed errors | 5 (Bankless, Latent Space, Chalk Radio, Critics at Large, The Edge) |
| Pipeline | Manual fallback — Claude Code OAuth session expired |

## Transcript sources

| Source | Count |
|---|---|
| rss_omny_srt (Bloomberg/Omny) | 4 |
| youtube_autocaptions | 7 |
| show_notes | 3 |
| web_substack | 1 |
| web_search_fallback | 1 |

## Episodes

| Show | Episode | Source |
|---|---|---|
| Empire | [[empire__clarity-failed-many-stablecoins-debate-circle-launches-arc-and-meta-s-ai-edge-we]] | `youtube_autocaptions` |
| Everybody's Business | [[everybody-s-business__are-you-in-a-cognitive-deficit]] | `rss_omny_srt` |
| In Good Company with Nicolai Tangen | [[in-good-company-with-nicolai-tangen__friday-wrap-up-a-guru-of-simplification-and-the-ai-slowdown-debate]] | `show_notes` |
| Masters in Business | [[masters-in-business__investing-for-the-ai-boom-masters-in-business-with-glen-kacher]] | `rss_omny_srt` |
| Money Stuff: The Podcast | [[money-stuff-the-podcast__trying-to-save-the-world-is-no-excuse]] | `rss_omny_srt` |
| No Priors | [[no-priors__why-diffusion-will-win-ai-inference-with-inception-co-founder-and-ceo-stefano-er]] | `youtube_autocaptions` |
| Odd Lots | [[odd-lots__there-s-a-mind-boggling-number-of-rich-people-in-america]] | `rss_omny_srt` |
| RiskReversal Pod | [[riskreversal-pod__dan-niles-this-isn-t-the-top-it-s-a-speed-bump]] | `web_substack` |
| StarTalk Radio | [[startalk-radio__cosmic-queries-the-fractal-universe-with-charles-liu]] | `show_notes` |
| The Compound and Friends | [[the-compound-and-friends__tom-lee-and-dan-ives-explain-everything]] | `youtube_autocaptions` |
| The Meb Faber Show - Better Investing | [[the-meb-faber-show-better-investing__roger-ibbotson-why-isn-t-everyone-rich-651]] | `youtube_autocaptions` |
| The Peter McCormack Show | [[the-peter-mccormack-show__213-russell-napier-they-are-stealth-stealing-your-monday-financial-repression-is]] | `show_notes` |
| Unchained | [[unchained__crypto-s-clarity-act-collapses-two-days-later-the-sec-introduces-its-innovation-]] | `youtube_autocaptions` |
| Y Combinator Startup Podcast | [[y-combinator-startup-podcast__the-state-of-startups-in-2026]] | `youtube_autocaptions` |
| Bankless | [[bankless__rollup-the-bull-market-test-clarity-dies-sec-opens-the-door-hyperliquid-comes-on]] | `web_search_fallback` |

## Feed errors (handled)

| Feed | Error | Resolution |
|---|---|---|
| Bankless | Flightcast `XML: mismatched tag` (167-byte error response) | ✅ Recovered via direct feed fetch + public bankless.com episode page — same-day ROLLUP "The Bull Market Test" processed as `web_search_fallback` |
| Latent Space: The AI Engineer Podcast | Flightcast `XML: mismatched tag` | ✅ Direct feed fetch confirmed **no in-window episode** — latest is 2026-09-16 (already processed) |
| Chalk Radio | timeout | ⏭️ No fallback search run (manual-mode budget); low-frequency publisher |
| Critics at Large \| The New Yorker | timeout | ⏭️ No fallback search run (manual-mode budget); last covered 2026-09-17 |
| The Edge | timeout (empty response) | ✅ Direct feed fetch confirmed **no episode since 2026-07-07** — nothing missed |

All three direct-curl recoveries used the documented pattern (`curl -sL --max-time 120 -H "Accept-Encoding: identity"`), which succeeded where the parallel fetcher reported parse errors.

## Notes

- **Late addition to 2026-09-17:** the prior-date diff re-fetched 2026-09-17 and found 2 episodes published after that day's fetch — Unchained "Uneasy Money: What Crypto's Gensler Years Say About the AI Slowdown Debate" and 矽谷101 "外滩大会线下圆桌｜敢把钱包交给AI吗？". Both were processed into the 2026-09-17 vault and recorded against 2026-09-17 in state.
- No duplicate vault files were detected (sibling cron-instance warnings were raised on `/tmp` writes, but the date directories contained only files written by this run — verified by `os.listdir` before the report was written).
- **Data-integrity observation (verified and corrected 2026-09-18 by follow-up run):** `state.json` records 2026-09-17 `processed` entries for TIP847 (We Study Billionaires), TWIML #777 and Lex Fridman #502 with no vault file in `Podcast Summaries/2026-09-17/` — but all three files DO exist in `Podcast Summaries/2026-09-16/`. Verified by matching `guid:` frontmatter against `state.json` across the 09-16/09-17/09-18 directories: every processed guid for those dates has exactly one vault file. The apparent gap was a label artifact — the `state.date` field records the run date while files are filed under the PT publish date. **No backfill needed; do not re-process these guids.**
- Claude Code OAuth remains expired (14th consecutive day, 2026-08-08 → 2026-09-18), so the manual pipeline fallback was used as documented.
