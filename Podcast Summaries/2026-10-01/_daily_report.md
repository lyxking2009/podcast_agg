---
date: 2026-10-01
episodes_processed: 10
episodes_found_in_rss: 10
feed_fallbacks_recovered: 0
feed_fallbacks_identified: 1
trailers_skipped: 0
late_additions: 0
generated_at: "2026-10-01T23:05:00Z"
generated_by: "Hermes cron (podcast aggregation pipeline) - Manual Pipeline Fallback (Claude Code OAuth expired)"
---

# Podcast Summary - 2026-10-01

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-01 (Thursday) |
| Window | 2026-09-30 to 2026-10-01 (last_run_date 2026-09-29 + 1), 2 dates, 18 RSS episodes total |
| Subscriptions synced | 47 (`MTLibrary.sqlite` query timed out at 30s, cached `subscriptions.json` reused) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 5 on the parallel pass: Bankless, Chalk Radio, Critics at Large, Latent Space, The Edge |
| In-window episodes | 10 (RSS), including 1 promotional trailer |
| Episodes summarized | 10 |
| Failures | 0 at episode level; 1 identified-but-unrecovered feed fallback (see below) |
| Transcript coverage | 5/10 substantial transcript (50%) - 10/10 with structured content (100%) |
| Final transcript sources | `web` 2 - `rss_omny_json` 1 - `rss_transistor_vtt` 1 - `show_notes` 6 |

## Episodes - 2026-10-01 (10)

| Show | Episode | Duration | Source |
|---|---|---|---|
| Dwarkesh Podcast | Si Sheppard - How did a few hundred Spanish soldiers topple two empires? | 1h37m23s | web (dwarkesh.com full public transcript) |
| Machine Learning Street Talk (MLST) | How a Voice Agent Learns the Rhythm of Conversation - Shawn Wen | 1h10m09s | web (podcasters.spotify.com link hub, chapters + description) |
| Odd Lots | Everything in Markets Is Now Moving Incredibly Fast | 44m18s | rss_omny_json (Omny transcript API) |
| Practical AI | Open models and the future of Physical AI with NVIDIA | 47m19s | rss_transistor_vtt (RSS transcript tag -> Transistor VTT) |
| Sold a Story | Sold a Story 2: What Happened to Teaching **(trailer, 3m10s)** | 3m10s | show_notes (APM Reports show page) |
| The Investor's Podcast (We Study Billionaires) | TIP850: Walmart (WMT): From Discount Retailer to eCommerce Powerhouse | 1h16m27s | show_notes (theinvestorspodcast.com episode page) |
| The Peter McCormack Show | #217 - Lee Cronin - AI Will Never Be Conscious. This Is How We Create Real Life. | 1h35m52s | show_notes (Acast episode page, full chapter list) |
| Unchained | Uneasy Money: The $388M Bitget Hack Started With a Security Vendor | 1h23m05s | show_notes (YouTube description + timestamps) |
| Unchained | Is Hunter Biden's $LAPTOP Coin a Scam? He Says the Wallets Prove It Isn't | 55m08s | show_notes (YouTube description + timestamps) |
| Unchained | DEX in the City: The Hosts Sign Off With Their Best Advice for Women in Crypto | 56m19s | show_notes (YouTube description + timestamps) |

## Feed errors and fallback status

Three of the five failed feeds were re-verified with a browser User-Agent request (documented recovery sequence:
parallel fetch -> direct curl -> UA curl). One produced an in-window episode:

| Feed | Bytes (UA fetch) | Latest pubDate | In window | Status |
|---|---|---|---|---|
| Critics at Large (The New Yorker) | 1.39 MB (148 items) | Thu, 01 Oct 2026 10:00 -0000 | **yes** | identified, not recovered (manual-mode budget) |
| Bankless | 21.97 MB | Wed, 30 Sep 2026 10:30 -0000 | no (2026-09-30) | logged on 09-30 |
| Latent Space | 14.16 MB | Wed, 30 Sep 2026 22:23 GMT | no (2026-09-30) | logged on 09-30 |
| Chalk Radio | 0.62 MB | Thu, 05 Mar 2026 | no | clean |
| The Edge | 0.13 MB | Thu, 24 Sep 2026 | no | clean |

**Identified but not summarized in this run (manual fallback budget):**

- **Critics at Large (The New Yorker)** - “Tom Cruise-a-Palooza” (guid `59ed12f8-7973-11f1-bb33-9b5caa6e4f02`)

Logged in `state.json` under `failures` with reason `feed_error_not_researched (manual-mode budget)`. The New Yorker
publishes a Condé Nast transcript for each episode, and this one was not surfaced in a single search attempt - it
remains a clean backfill candidate.

## Late-publishing check

The prior date (2026-09-30) was re-fetched in the same parallel run and diffed against `state.json`, per the
late-publishing recovery procedure. All 8 episodes returned for 2026-09-30 were new (no GUID already present in
`processed`), so no prior-date late additions were missed. The previous run completed at 2026-09-29T22:40:11Z.

## Notes

- **Claude Code was unavailable** - the pipeline exited ~2 minutes after launch with
  `Failed to authenticate: OAuth session expired and could not be refreshed` (21st consecutive day). Switched to the
  Manual Pipeline Fallback per the skill's documented procedure. 18 episodes across 2 dates, both within the <=16
  per-date limit.
- **YouTube auto-captions returned HTTP 429** for every episode attempted during this run (`yt-dlp --write-auto-sub`),
  while `--print description` succeeded normally - consistent with the documented per-endpoint rate limit that affects
  the subtitle endpoint but not the metadata API. The four Unchained-family episodes were each matched to their full
  YouTube upload by exact duration (within 1 second of the RSS duration) before using their descriptions.
- **Omny transcript API**: `?format=srt` and `?format=vtt` both return HTTP 400 (`The value 'srt' is not valid` /
  `'vtt'`). Omitting the format parameter returns the complete JSON transcript (speakers, segments, per-word timings),
  which was converted to plain text locally. This is a change worth recording in the skill: the documented SRT path is
  no longer accepted by the Omny API.
- **Sold a Story** published a 3-minute Season 2 trailer on 2026-10-01 rather than a full episode; it is summarized and
  labelled as a trailer rather than skipped, since it does carry a legitimate RSS transcript-free description.

## Late additions — recovery pass (2026-10-01 16:30 PDT)

A second pass re-synced `subscriptions.json` (47) and re-fetched all 46 active feeds. Every feed succeeded (0 errors),
which cleared the five failures logged by the 15:08 run. Five previously-unsummarized episodes were recovered and
summarized; all five had real transcripts, so no description-only fallbacks were needed.

| Show | Episode | Published | Duration | Transcript source |
|---|---|---|---|---|
| 硅谷101 | E254｜超级厄尔尼诺来了，我们的日常所需真会因它涨价吗？ | 2026-09-29 | 1h02m03s | web (YouTube auto-captions, `zh-Hans`) |
| The Compound and Friends | Bad feeling, weak internals, confidence collapse, Nvidia breaking out \| WAYT? | 2026-09-29 | 1h14m59s | web (YouTube auto-captions, `en`) |
| Bankless | Is Variational the Next Hyperliquid? \| CEO Lucas Schuermann and Justin Bram | 2026-09-30 | 49m36s | rss (flightcast VTT) |
| Latent Space | Why Dwarkesh is Wrong about Computer Use + How OpenAI shipped its Jev competitor in 1 Week | 2026-09-30 | 39m12s | rss (substack `content:encoded` full transcript) |
| Critics at Large (The New Yorker) | Tom Cruise-a-Palooza | 2026-10-01 | 56m40s | web (Condé Nast transcript) |

- **Critics at Large** — resolved as predicted: the Condé Nast transcript exists at
  `cn-static-sites.s3.amazonaws.com/transcripts.condenastdigital.com/tny/2026-09-30/...-cal-130-tomcruise-transcript.txt`
  (56 KB, speaker-labelled). Now recorded in `state.json` as processed.
- **Bankless** — Rung 1 (RSS `podcast:transcript` VTT) succeeded directly.
- **Latent Space** — the Substack `content:encoded` field embeds the full speaker-labelled transcript with timestamps
  (358 labelled turns); the show-notes preamble was trimmed before summarization.
- **硅谷101 / The Compound and Friends** — matched to their YouTube uploads (durations within 3 s of the RSS values);
  `--write-auto-subs` returned captions normally, so the HTTP 429 subtitle rate limit noted above had cleared.

State: `last_run_date` = 2026-10-01, `processed` = 1054 entries, `failures` unchanged (0 new failures this pass).
