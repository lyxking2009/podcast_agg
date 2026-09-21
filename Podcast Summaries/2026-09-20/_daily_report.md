---
date: 2026-09-20
episodes_processed: 3
episodes_found_in_rss: 3
feed_fallbacks_recovered: 1
trailers_skipped: 1
late_additions: 0
generated_at: 2026-09-20T22:05:19Z
generated_by: "Hermes cron (manual pipeline fallback — Claude Code OAuth expired, 16th consecutive day)"
---

# Podcast Summary — 2026-09-20

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-20 (Sunday) |
| Episodes found via RSS | 3 |
| Episodes from feed-error fallback | 1 (Critics at Large) |
| Episodes processed | 3 |
| Transcript coverage | 3/3 (100%) |
| Feed errors | 5 |
| Trailers/promos skipped | 1 (Odd Lots “Introducing: Bloomberg Money”) |
| Late additions to 2026-09-19 | 0 |
| Pipeline | Manual fallback — Claude Code OAuth session expired (exit 1, 2 min after launch) |

Normal Sunday volume. Three in-window items across all 46 subscribed feeds, plus one full-length Critics at Large episode recovered from a timed-out feed.

## Transcript sources

| Source | Count |
|---|---|
| youtube_autocaptions | 1 |
| web | 2 |

## Episodes

| Show | Episode | Source |
|---|---|---|
| The Investor's Podcast (We Study Billionaires) | [[the-investor-s-podcast-we-study-billionaires-the-investor-s-podcast-network__rwh072-the-making-of-a-money-master-w-rob-vinall]] | `youtube_autocaptions` |
| Lenny's Podcast: Product \| Career \| Growth | [[lenny-s-podcast-product-career-growth__90-minutes-of-unfiltered-product-advice-from-snap-and-discord-s-product-chief-pe]] | `web` |
| Critics at Large \| The New Yorker | [[critics-at-large-the-new-yorker__introducing-the-journey-from-in-the-dark]] | `web` |

## Skipped (not episodes)

| Item | Detail |
|---|---|
| Odd Lots — “Introducing: Bloomberg Money” | 30-second promo for a new Bloomberg show. An Omny SRT track exists, but the item is an announcement rather than an episode; recorded in state as `skipped_trailer`. |

## Feed errors (handled)

| Feed | Error | Resolution |
|---|---|---|
| Bankless | Flightcast 167-byte error (`XML: mismatched tag: line 6, column 2`) on both the parallel fetch AND a plain direct curl | ✅ Retried with a **browser User-Agent** → 22.1 MB, 1,373 items. **No in-window episode**; latest is the 2026-09-18 ROLLUP, already vaulted on 09-18 |
| Latent Space: The AI Engineer Podcast | Flightcast 167-byte error (same) | ✅ Browser-UA fetch → 13.5 MB, 223 items. **No in-window episode**; latest 2026-09-16 (Rune Kvist, AIUC), already processed |
| Chalk Radio | timeout | ✅ Browser-UA fetch → 628 KB, 60 items, **0 in-window** — dormant (latest 2025-11-05) |
| Critics at Large \| The New Yorker | timeout | ✅ Browser-UA fetch → 1.4 MB, 146 items, **1 in-window**: “Introducing: ‘The Journey,’ from In the Dark” (Sun 20 Sep 10:00 UTC), a full 56m57s episode → vaulted |
| The Edge | timeout (empty response) | ✅ Browser-UA fetch → 126 KB, 36 items, **0 in-window** — dormant (latest #35, 2026-04-07) |

The browser User-Agent was the decisive change this run: a plain `curl -sL` direct fetch returned 167 bytes (Flightcast) or 0 bytes (Buzzsprout / Simplecast / publicfeeds) for all five feeds, while `curl -sSL -A "<Chrome UA>"` returned HTTP 200 with full payloads for every one. No fallback web searches were needed — `<pubDate>` inspection settled in-window existence for all five.

## Late-publishing check (prior date)

Re-fetched 2026-09-19 independently and diffed GUIDs against `state.json` `processed`: **2/2 already processed, zero late additions.** The 09-19 run completed at 22:08Z (15:08 PDT), immediately after the cron fire.

## Notes

- **Concurrent sibling run reconciled.** A separate instance of this cron job (`cron:9de3a67a1a37:0dba7691417e4ab4aae8aada991b2928`) had already written the Lenny's Podcast and Critics at Large vault files at 22:03Z with `model: deepseek-v4-pro`, and had added their state entries, before this run's own pipeline launch. This run's `run_podcast_pipeline.py` then exited after ~2 minutes with `Failed to authenticate: OAuth session expired and could not be refreshed`, leaving TIP RWH072 unprocessed. Checked before writing: no duplicate GUIDs, no slug variants for any 09-20 episode (verified by parsing `guid:` out of every file's frontmatter). The two writers covered disjoint episodes, so all three survive — one file per GUID.
- **TIP RWH072** has no RSS transcript tag and the publishers' own page is login-gated ("the full transcript is only available to logged-in users" — first ~1,000 words public). Rung 3 delivered: YouTube `MMyfAJRU6o4` (“The Making of a Star Investor w/ Rob Vinall (RWH072)”) matched the RSS duration (6,492 s), auto-captions downloaded cleanly (1.07 MB VTT → 113.6 K chars). This is a rare in-depth interview — Vinall's first podcast appearance in several years.
- **Cross-show convergence:** Vinall's “weirdest market” framing (index at highs, most stocks 50%+ below highs, gains concentrated in semiconductor hardware) restates the frontier-AI/momentum deceleration theme running through the 2026-09-18 and 09-19 episodes.
- Claude Code OAuth remains expired (16th consecutive day, 2026-08-08 → 2026-09-20) — manual pipeline fallback used as documented.

## Late additions (recovered 2026-09-21)

Re-fetching 2026-09-20 during the 09-21 run surfaced one episode published on 09-20 after this report's fetch:

| Show | Episode | Source | Detail |
|---|---|---|---|
| The Rest Is History | [[the-rest-is-history__707-the-terror-an-assassination-in-paris-part-1]] | `web_podscripts` | Ep. 707, “The Terror: An Assassination in Paris (Part 1)”, guid `0322655e-b118-11f1-b8eb-f79d9c6ca5d1`, 1h25m33s. Opens the six-part Terror arc. No RSS `podcast:transcript` tag and the show's own episode pages are description-only, but PodScripts hosts a full machine transcript → 85.8 K chars with per-segment timestamps, so **upgraded from `show_notes` to `web_podscripts`** during 09-21 reconciliation. |

This raises 2026-09-20 coverage from 3 to 4 episodes (episodes_found_in_rss for that date was 4 — the original run's fetch missed this one because it published later in the day).

**Upgrade note (2026-09-21):** the late addition was first written from the publisher's episode description (`show_notes`, no quotes section). A subsequent pass in the 09-21 run found a full PodScripts transcript, so the vault file was rewritten with `transcript_source: web_podscripts`, a restored `## Notable quotes` section, and the state entry corrected. One file per GUID — the interim `show_notes` version was overwritten, not duplicated.
