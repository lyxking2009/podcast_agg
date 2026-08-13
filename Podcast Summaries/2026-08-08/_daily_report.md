# Daily Report — 2026-08-08

**Run type:** Manual pipeline fallback (Claude Code OAuth session expired — `Failed to authenticate: OAuth session expired and could not be refreshed`; pipeline exited ~3 min after launch without processing)

**Episodes found (RSS):** 2 | **Feed errors:** 5 | **Processed:** 2 (2 RSS) | **Skipped:** 0

## Transcript source breakdown
| Source | Count |
|---|---|
| show_notes (iHeart episode description, Apple Podcasts) | 2 |

## Episodes

| Show | Episode | Source | Links |
|---|---|---|---|
| Big Technology Podcast | Demis Steps Down, Apple's Memory Problem, Microsoft's Clever Trick | show_notes | [[big-technology-podcast__demis-steps-down-apples-memory-problem-microsofts-clever-trick]] |
| Philosophize This! | Episode #251 ... You Must Change Your Life, Pt. 2 - The Worlds That Make Us | show_notes | [[philosophize-this__episode-251-you-must-change-your-life-pt-2-the-worlds-that-make-us]] |

## Feed errors (5)

| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ✅ Already covered — ROLLUP: The ETH Issuance War ($130M Coldcard Exploit) was processed 2026-08-07 (web_search_fallback); no new episode today (EARLY ACCESS "The EIP That Destroys DeFi" goes public Aug 10) |
| Chalk Radio | timeout | ⚠️ No episode in window (feed sporadic; nothing new in years) |
| Critics at Large (The New Yorker) | timeout | ⚠️ No episode in window (weekly Thursdays; latest was Jul 30 "The Irony and the Ecstasy of Colson Whitehead"; no Aug 6 episode found) |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window (latest published Aug 3 — Databricks cofounders) |
| The Edge | timeout | ⚠️ No episode in window (latest #35, ~July) |

## Notes
- Manual fallback executed because Claude Code auth failed (OAuth expired, non-retryable). Both RSS episodes processed with show-notes sources: both are same-day episodes with no YouTube upload or published transcript yet.
- Big Technology "Demis Steps Down..." (55 min, M.G. Siegler) summarized from the iHeart episode description — Demis Hassabis stepping down as DeepMind CEO, Apple's memory crunch and iPhone pricing questions, Microsoft's AI spend reduction with cloud growth driven by OpenAI/Anthropic, Google AI divisions.
- Philosophize This! #251 (35 min) summarized from Apple Podcasts show notes — Part 2 of the Sloterdijk "You Must Change Your Life" series, reworking anthropology and viewing culture as a monastic rule.
- State updated: last_run_date → 2026-08-08; 2 processed entries added. No new failures.
