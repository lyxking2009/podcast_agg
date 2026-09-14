# Podcast Summaries — 2026-09-13

- **Episodes in vault:** 3
- **Run:** 2026-09-13 15:00 PT cron (two concurrent runs of this job reconciled into one vault; 47 subscriptions synced)
- **Coverage:** 3/3 episodes (100%)
- **Prior-date diff:** 2026-09-12 re-fetched, 2 episodes — both already in state (no late additions)
- **Catch-up note:** TIP846 published 2026-09-13T00:00Z, i.e. after the 09-12 run fetched (22:11Z) but before midnight PT — a "day after last_run_date" window start (00:00 PT) misses such episodes. Start the window from the previous run's completion timestamp instead.

## Episodes

- [[the-investors-podcast-we-study-billionaires-the-investors-podcast-network__tip846-stock-picker-how-to-live-off-your-portfolio-w-ian-cassel]]  ·  `web` (podscripts.co full transcript)
- [[y-combinator-startup-podcast__8-ways-to-improve-your-outbound-sales]]  ·  `youtube_autocaptions` (yt wr6PMD06hP0, 12:41)
- [[西西弗高速__30-一个人要消化多少信息-才能成为一个社会人-西西弗驿站]]  ·  `show_notes` (fireside/xiaoyuzhou show notes with full 「你将听到」 timestamp map)

## Feed errors

All five errored feeds were resolved by direct-curl checks — **no in-window episodes missed**, no web fallback needed:

- **Bankless** — Flightcast ~167-byte error; direct fetch OK (21.9MB), latest = 2026-09-11 ROLLUP (already in the 09-11 vault)
- **Latent Space: The AI Engineer Podcast** — Flightcast ~167-byte error; direct fetch OK (13.2MB), latest = 2026-08-26 — none in window
- **Chalk Radio** — timeout; direct fetch OK, latest = 2026-03-05 — none in window
- **Critics at Large | The New Yorker** — timeout; direct fetch OK, latest = 2026-09-03 — none in window
- **The Edge** — timeout; direct fetch OK, latest = 2026-07-07 — none in window

## Unresolved failures

- None added today.

## Notes

- Sunday run: 3 episodes across all 47 feeds (weekend pattern — low volume, expected). A verification pass over every feed's newest 8 items found zero missing/undated entries.
- yt-dlp hit a stray `/tmp/inspect.py` shadowing the stdlib (module name collision) — fixed by running yt-dlp from a non-`/tmp` cwd.

## Late additions (recovered 2026-09-14)

Both episodes published after this date's 15:00 PT RSS fetch. Recovered by the 2026-09-14 prior-date diff.

- [[acquired__the-home-depot]]  ·  `rss_transistor` (full transcript from transistor)
- [[the-rest-is-history__705-queen-victoria-s-revenge-the-mad-emperor-of-abyssinia-part-1]]  ·  `show_notes`
