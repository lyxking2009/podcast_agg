---
date: 2026-09-22
episodes_processed: 7
episodes_found_in_rss: 7
feed_fallbacks_recovered: 0
trailers_skipped: 0
late_additions: 0
generated_at: 2026-09-22T22:09:47Z
generated_by: "Hermes cron (podcast aggregation pipeline, deepseek-v4-pro)"
---

# Podcast Summary — 2026-09-22

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-22 (Tuesday) |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 47 (46 active, 1 excluded by config) |
| Feed errors | 0 |
| Episodes found in scan window | 7 |
| Episodes summarized | 7 |
| Failures | 0 |
| Transcript coverage | 7/7 (100%) |
| Trailers/promos skipped | 0 |
| Rung 1 / Rung 2 / Rung 3 | 1 / 6 / 0 |
| Pipeline | Hermes-native (RSS fetch → transcript ladder → DeepSeek → Markdown) |

Normal Tuesday volume: seven episodes published across 09-21/09-22 were picked up. No RSS feed declared a `podcast:transcript` element this run, so Rung 1 resolved only where the publisher embedded the transcript in the feed payload itself (Latent Space's Substack `content:encoded`). Everything else was solved on Rung 2 via YouTube auto-captions (4), PodScripts (1), and the publisher's own PDF (1). No episode needed the episode-description last resort.

## Transcript sources (this date)

| Source | Count |
|---|---|
| `youtube_autocaptions` | 4 |
| `rss_substack` | 1 |
| `web_podscripts` | 1 |
| `web_oaktree_pdf` | 1 |

| Published | Episodes |
|---|---|
| 2026-09-21 | 1 |
| 2026-09-22 | 6 |

## Episodes

| Show | Episode | Published | Duration | Source |
|---|---|---|---|---|
| Latent Space: The AI Engineer Podcast | [[latent-space-the-ai-engineer-podcast__jev-system-one-models-for-prod-not-god-with-diogo-almeida-ceo-typesafe-ai]] | 2026-09-21 | 2h20m53s | `rss_substack` |
| The Master Investor Podcast with Wilfred Frost | [[the-master-investor-podcast-with-wilfred-frost__if-global-trade-breaks-it-s-worse-than-2008-ed-conway]] | 2026-09-22 | 1h3m2s | `youtube_autocaptions` |
| StarTalk with Neil deGrasse Tyson | [[startalk-with-neil-degrasse-tyson__lost-in-space-game-night]] | 2026-09-22 | 42m14s | `youtube_autocaptions` |
| Invest Like the Best with Patrick O'Shaughnessy | [[invest-like-the-best-with-patrick-o-shaughnessy__gabe-stengel-building-investing-superintelligence-invest-like-the-best-ep-492]] | 2026-09-22 | 1h4m24s | `web_podscripts` |
| The Memo by Howard Marks | [[the-memo-by-howard-marks__shall-we-repeal-the-laws-of-economics-part-iii]] | 2026-09-22 | 27m16s | `web_oaktree_pdf` |
| The Peter McCormack Show | [[the-peter-mccormack-show__214-prof-richard-werner-they-are-traitors-how-central-banks-sabotage-wealth-crea]] | 2026-09-22 | 1h6m19s | `youtube_autocaptions` |
| Latent Space: The AI Engineer Podcast | [[latent-space-the-ai-engineer-podcast__an-oscar-two-asteroids-and-the-algorithm-in-your-sklearn-john-platt-on-ai-for-sc]] | 2026-09-22 | 2h1m26s | `youtube_autocaptions` |

## Scan window

`2026-09-21T07:00:00+00:00` → `2026-09-23T07:00:00+00:00` (label window 2026-09-22 PT; the start carries a one-day backfill so late-publishing items from the prior date are not lost). `last_run_date` was `2026-09-21`; `lookback_cap_days` is 7, so no cap applied.

## Feed errors (handled)

**None.** All 46 active feeds returned parseable XML on the first parallel pass (Chrome User-Agent, 8 workers). One feed (`Sticky Notes: The Classical Music Podcast`) was skipped by `excluded_feeds` in `data/config.json`.

## Notes

- **No feed shipped a `podcast:transcript` tag this run.** All seven episodes were resolved without Rung 1's declared-transcript path except Latent Space, whose Substack `content:encoded` payload *is* the full transcript (188 K chars, speaker-labelled with `[HH:MM:SS]` cues). Two of the feeds that historically ship transcripts (Odd Lots / Omny, Bankless / Flightcast) published nothing in this window.
- **YouTube auto-captions carried four of seven episodes.** Duration matching is the reliable identity check: the Latent Space John Platt upload (`2xBSGluFkG0`) is an exact 7,286 s match to the RSS `itunes:duration`, and the Peter McCormack Richard Werner upload (`uVlx2en1RRc`) is 3,992 s against an RSS 1:06:19. Both were uploaded 2026-09-22. YouTube titles often diverge from RSS titles (Platt's upload is titled *“Google's AI Scientist Started as an Attempt to Automate Kaggle”*), so match on duration, not title.
- **VTT prefix-dedup was clean this run.** After stripping rolling-caption repeats, output density landed at ~975–1,070 chars/minute with no adjacent 5-gram duplication (max 5-gram frequency 5–7 over 12–25 K words). No need for the `json3` workaround this time, but it remains the safer default for karaoke-style VTT.
- **The Howard Marks memo is a written document, not just audio.** The podcast episode is Marks reading his September 22, 2026 memo *Shall We Repeal the Laws of Economics – Part III*; the canonical text is the Oaktree PDF, which is a cleaner source than any caption track. Worth going straight to `oaktreecapital.com/docs/default-source/memos/` for this show in future runs.
- **Thematic convergence: fiscal/financial plumbing.** Marks on Treasury buybacks failing to hold down long yields, Werner on central banks engineering credit creation, and Conway on the fragility of just-in-time global trade all describe the same substrate — the institutions that move money and goods are the ones under strain. Stengel's Rogo episode is the counterpoint: AI-native software attacking exactly that plumbing from below.
- **Claude Code OAuth remains expired** (since 2026-08-08, now 18th consecutive day); the Hermes-native pipeline was used end-to-end as documented.
- **No duplicate GUIDs.** Each of the seven episodes produced exactly one vault file and one `state.json` entry; a frontmatter scan confirms one file per GUID for this date.


---

## Late additions (recovered 2026-09-23)

- **[[the-compound-and-friends__metas-muse-launch-josh-is-wrong-on-netflix-internals-weaken-10-year-bonds-at-5-buffett-steps-away\|The Compound and Friends — Meta’s Muse launch, Josh is wrong on Netflix, internals weaken, 10-year bonds at 5%%, Buffett steps away]]** — `show_notes`. This episode (guid `020e7f50-b69e-11f1-92e2-33e6a76bb77b`) published after the 09-22 run’s fetch window; it was caught by the prior-date re-fetch on 2026-09-23 and is recorded in state with `date: 2026-09-22`.
