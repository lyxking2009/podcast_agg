---
date: 2026-09-27
episodes_processed: 2
episodes_found_in_rss: 2
feed_fallbacks_recovered: 0
trailers_skipped: 0
late_additions: 1
generated_at: "2026-09-27T22:10:00Z"
generated_by: "Hermes cron (podcast aggregation pipeline)"
---

# Podcast Summary — 2026-09-27

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-27 (Sunday) |
| Window | 2026-09-26T22:04:44Z (previous run completion) → 2026-09-28T07:00:00Z |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 0 |
| Items parsed across all feeds | 19,946 |
| Unprocessed items in trailing 7 days | 2 |
| New in-window episodes | 2 |
| Episodes summarized | 2 |
| Failures | 0 |
| Transcript coverage | 2/2 = 100% |
| Rung 1 / Rung 2 / Rung 3 | 0 / 2 / 0 |

## Episodes (2)

| Show | Episode | Duration | Source |
|---|---|---|---|
| Lenny's Podcast: Product \| Career \| Growth | The grief, loneliness, and burnout sweeping through the tech industry right now \| Molly Graham | 1h34m14s | web (podscripts.co) |
| The Investor's Podcast (We Study Billionaires) | TIP849: Average Returns Can Still Make You Wealthy w/ David Fagan | 1h11m27s | web (podscripts.co) |

## Notes

- **Sunday window, zero feed errors.** All 46 active feeds returned HTTP 200 with well-formed XML on the parallel fetch.
- **Late addition (1).** TIP849 carries `<pubDate>` 2026-09-27T00:00:00Z = 2026-09-26 17:00 PT — two hours *after* the previous run completed (2026-09-26T22:04:44Z) but belonging to the 09-26 PT calendar day. The window start was therefore anchored to the previous run's completion timestamp rather than midnight, so it was not lost. It is filed here under the run date.
- **Rung 1 empty, Rung 2 clean.** Neither episode declares `podcast:transcript` in RSS. Both full transcripts were recovered from `podscripts.co` on the first search query (`"{episode_title}" transcript`) and parsed out of the page's `transcript-text` spans with timestamps: Lenny's 98,846 chars, TIP 76,119 chars.
- TIP849's own episode page carries only a ~3-minute preview transcript (8 paragraphs, 3,012 chars) before the paywall — the podscripts source supplied the full 71-minute transcript.
- **Zero failures.** No `failures` entries added; existing `failures` list unchanged.
- Vault written: `Podcast Summaries/2026-09-27/` (2 files) — one file per GUID, verified.
- **Feed-error variance (independent re-fetch).** A second fetch of 2026-09-27 completed during this run returned **5 feed errors** (Bankless, Latent Space, Chalk Radio, Critics at Large, The Edge) where the primary fetch saw none — the documented run-to-run variability of `fetch_episodes_parallel.py`. All five were re-checked with a browser-UA `curl` (**HTTP 200 with full payloads**, 22.2 MB / 14.0 MB / 628 KB / 1.4 MB / 131 KB) and their `<pubDate>`s inspected: newest items are Bankless ROLLUP **Fri 25 Sep** (already covered in the 2026-09-25 vault), Latent Space **Fri 25 Sep 23:14 GMT** (= 16:14 PT, already recovered as a 09-25 late addition), Critics at Large **Thu 24 Sep** and The Edge **Thu 24 Sep** (both already covered in the 09-24 vault), and Chalk Radio's latest is **5 Mar 2026**. **No in-window episode was lost** — the 0-feed-error outcome stands.
- **Concurrent sibling run (reconciled).** A second instance of this cron job ran the same window in parallel and completed first (its files `model: deepseek-v4-pro`, `generated_at` 22:04:39Z; its state commit `last_run_completed_at` 22:04:39Z). This agent wrote two richer variants of the same episodes before the sibling's files were visible. Per the skill's concurrent-run reconciliation: **deduped by GUID, one file per GUID kept** — the sibling's two files were retained (they carry specific transcript detail and timestamped verbatim quotes) and this agent's two slug variants were removed (`...molly-graham.md`, `...the-investors-podcast-we-study-billionaires...tip849...md`). No episode was re-summarized or double-counted; `state.json` already held exactly the 2 correct 09-27 entries, so no merge was needed. One factual slip in the sibling's Lenny's note was corrected (survey burnout: 44% in **2025** → 55% in **2026**; the note had the years shifted by one).
- **Pipeline:** `run_podcast_pipeline.py --dates "2026-09-27"` exited after ~2 minutes with `Failed to authenticate: OAuth session expired and could not be refreshed` (Claude Code CLI, 18th consecutive day). Completed via Manual Pipeline Fallback.
