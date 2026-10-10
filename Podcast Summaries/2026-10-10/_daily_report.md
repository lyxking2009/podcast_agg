---
date: 2026-10-10
episodes_processed: 2
episodes_found_in_rss: 1
late_additions: 0
failures: 0
generated_at: "2026-10-10T22:04:13Z"
generated_by: "Hermes cron (podcast aggregation pipeline)"
---

# Podcast Summary - 2026-10-10

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-10 (Saturday) |
| Window | 2026-10-10 only (last_run_date 2026-10-09 + 1) + late check of 2026-10-09 |
| Subscriptions synced | 47 (Apple Podcasts) |
| In-window episodes | 1 (RSS) + 1 (Latent Space feed-fallback) = 2 |
| Episodes summarized | 2 |
| Failures | 0 |
| Transcript coverage | 2/2 (100%) |
| Transcript sources | `youtube_autocaptions` 1, `web_substack` 1 |

> Note: Claude Code OAuth session was still expired this run (`run_podcast_pipeline.py` exited after ~30 s with `Failed to authenticate: OAuth session expired and could not be refreshed`), so the day was completed via the Manual Pipeline Fallback. A concurrent sibling run processed the Latent Space episode; the two runs were reconciled to one file per GUID (see Notes).

## Episodes - 2026-10-10 (2 summarized)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| Latent Space: The AI Engineer Podcast | [[latent-space-the-ai-engineer-podcast__why-alphafold-didnt-solve-protein-folding-pushmeet-kohli-google-deepmind-and-sal-candido-biohub]] | 31m55s | `web_substack` | latent.space/p/biohub-deepmind (full verbatim transcript); feed failed (Flightcast) |
| Machine Learning Street Talk (MLST) | [[machine-learning-street-talk-mlst__what-most-people-get-wrong-about-evolution-akarsh-kumar]] | 47m54s | `youtube_autocaptions` | YouTube `615B5nyMFPk` (2,875 s ≈ RSS 2,874 s) |

## Feed errors (verified with browser-UA curl)

- **Bankless** — Flightcast `mismatched tag` in the parallel fetch; browser-UA curl OK (21.9 MB, 1,349 items). Its 2026-10-09 ROLLUP was already processed on 10-09. ✅ verified
- **Latent Space** — Flightcast `mismatched tag`; browser-UA curl OK (14.5 MB, 233 items). Its in-window 2026-10-10 episode (AlphaFold/Biohub panel, guid `substack:post:219666029`) was recovered via the direct feed fetch. ✅ recovered
- **Critics at Large | The New Yorker** — timeout; browser-UA curl OK (1.4 MB, 149 items). Latest in-window episode (2026-10-08) already processed on 10-08. ✅ verified
- **Chalk Radio** — timeout; browser-UA curl OK (628 KB, 60 items). No in-window episode. ✅ verified
- **Lex Fridman Podcast** — truncated download (`unclosed CDATA section`); browser-UA curl OK (2.1 MB, 502 items). No in-window episode. ✅ verified
- **The Edge** — timeout; browser-UA curl OK (131 KB, 37 items). No in-window episode. ✅ verified

## Notes

- No `failures` entries: both in-window episodes yielded acceptable transcripts.
- A prior-date diff of 2026-10-09 against `state.json` found no late-published 10-09 episodes (all 12 feed items already processed).
- **Concurrent sibling run:** a sibling session processed the Latent Space episode in parallel (its file dated `published: 2026-10-10`, model `deepseek-v4-pro`). The two runs were reconciled — one file per GUID, its (verified) content retained under a single canonical filename, my duplicate from the `2026-10-09/` directory removed. The episode is dated by the feed's own offset (`Sat, 10 Oct 2026 00:31:26 GMT`), matching `fetch_episodes_parallel.py`'s `pub_date` convention.
- Feed errors were re-checked with a browser User-Agent before concluding — all six returned HTTP 200 with the UA, confirming the failures were transient (Flightcast error pages / timeouts without a UA).
- `last_run_date` advanced 2026-10-09 → 2026-10-10; state records `substack:post:219666029` (date 2026-10-09) and `387cf19d-15d2-40f5-9752-7893a16aa830` (date 2026-10-10).

## Duplicate run note

- A second instance of this cron job ran concurrently (~15:03–15:04 PT) and wrote an alternate-slug copy of the Latent Space episode. The two runs were reconciled to one file per GUID: the episode is kept once, in `2026-10-10/`, under the canonical filename listed above; the alternate copy was removed.
- The second instance identified the Bankless feed's real GUID `flightcast:01M4F5C2ZMR7V1J2DXVA9XTPAX` and recorded it in `state.json` as an alias of the already-processed `bankless:rollup-2026-10-09` (the 10-09 Flightcast fetch failed and supplied no GUID then), preventing a future re-processing of that ROLLUP.
