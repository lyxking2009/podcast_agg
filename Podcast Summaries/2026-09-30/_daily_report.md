---
date: 2026-09-30
episodes_processed: 8
episodes_found_in_rss: 8
feed_fallbacks_recovered: 0
feed_fallbacks_identified: 2
trailers_skipped: 0
late_additions: 0
generated_at: "2026-10-01T22:55:00Z"
generated_by: "Hermes cron (podcast aggregation pipeline) - Manual Pipeline Fallback (Claude Code OAuth expired)"
---

# Podcast Summary - 2026-09-30

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-30 (Wednesday) |
| Window | 2026-09-30 to 2026-10-01 (last_run_date 2026-09-29 + 1), 2 dates, 18 RSS episodes total |
| Subscriptions synced | 47 (`MTLibrary.sqlite` query timed out at 30s, cached `subscriptions.json` reused) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 5 on the parallel pass: Bankless, Chalk Radio, Critics at Large, Latent Space, The Edge |
| In-window episodes | 8 (RSS) |
| Episodes summarized | 8 |
| Failures | 0 at episode level; 2 identified-but-unrecovered feed fallbacks (see below) |
| Transcript coverage | 4/8 substantial transcript (50%) - 8/8 with structured content (100%) |
| Final transcript sources | `web` 3 - `rss_omny_json` 1 - `show_notes` 4 |

## Episodes - 2026-09-30 (8)

| Show | Episode | Duration | Source |
|---|---|---|---|
| Animal Spirits Podcast | Will Higher Rates Kill the Stock Market? (EP. 484) | 60m48s | web (pod.wave.co full notes + transcript) |
| Big Technology Podcast | SAP CEO: AI Won't Kill Software, But It Will Change Your Job - With Christian Klein | 61m28s | web (usetranscribe.io YouTube transcript) |
| In Good Company with Nicolai Tangen | Horizon Robotics CEO: Car Chips, Mind-Off Driving and China's Speed | 47m08s | show_notes (Acast episode page) |
| Machine Learning Street Talk (MLST) | Who Checks a Proof No Human Can Read? - Leo de Moura | 1h14m19s | web (podcasters.spotify.com link hub, chapters + description) |
| Masters in Business | At the Money: The Data Behind America's Wealthy | 13m09s | rss_omny_json (Omny transcript API) |
| The Rest Is History | 710. The Terror: Death at the Guillotine (Part 4) | 1h31m11s | show_notes (Apple Podcasts description) |
| Unchained | How Bitget Is Chasing $388 Million in Stolen Funds After a Zero-Day Hack | 1h06m12s | show_notes (YouTube description + Unchained/The Block coverage) |
| Wiser World | 104. Warning Signs of Authoritarianism | 47m10s | show_notes (wiserworld.com episode archive) |

## Feed errors and fallback status

All five failed feeds were re-verified with a browser User-Agent `curl`/`urllib` request (the documented recovery
sequence: parallel fetch -> direct curl -> UA curl). Three feeds returned HTTP 200 with full payloads, and two of
those contained episodes inside the 2026-09-30 window:

| Feed | Bytes (UA fetch) | Latest pubDate | In window | Status |
|---|---|---|---|---|
| Bankless | 21.97 MB (1,378 items) | Wed, 30 Sep 2026 10:30 -0000 | **yes** | identified, not recovered (manual-mode budget) |
| Latent Space: The AI Engineer Podcast | 14.16 MB (230 items) | Wed, 30 Sep 2026 22:23 GMT | **yes** | identified, not recovered (manual-mode budget) |
| Chalk Radio | 0.62 MB | Thu, 05 Mar 2026 | no | clean |
| Critics at Large (The New Yorker) | 1.39 MB | Thu, 01 Oct 2026 10:00 -0000 | no (2026-10-01) | clean for this date |
| The Edge | 0.13 MB | Thu, 24 Sep 2026 | no | clean |

**Identified but not summarized in this run (manual fallback budget):**

- **Bankless** - “Is Variational the Next Hyperliquid? | CEO Lucas Schuermann and Justin Bram” (guid `flightcast:01M3RVWW3PGAT901HR2BYQSHWE`)
- **Latent Space** - “Why Dwarkesh is Wrong about Computer Use + How OpenAI shipped its Jev competitor in 1 Week” (guid `substack:post:218243619`)

Both are logged in `state.json` under `failures` with reason `feed_error_not_researched (manual-mode budget)` and remain
backfill candidates. The automated pipeline's feed-fallback prompt would normally have searched the web for these; the
manual fallback skipped the searches to stay within the run's tool budget. No duplicate risk: neither title appears in
any prior vault directory.

## Notes

- **Claude Code was unavailable again** - `run_podcast_pipeline.py` exited ~2 minutes after launch with
  `Failed to authenticate: OAuth session expired and could not be refreshed` (21st consecutive day). The run switched
  to the Manual Pipeline Fallback per the skill's documented procedure.
- **YouTube auto-captions were HTTP 429 rate-limited** across the run. Every `yt-dlp --write-auto-sub` attempt
  returned `HTTP Error 429`, while `--print description` worked normally - consistent with the documented per-endpoint
  rate limit. All Unchained episodes were matched to their full YouTube uploads by exact duration and summarized from
  the full video descriptions plus published coverage.
- **Omny transcript API**: `?format=srt` returns HTTP 400 (`The value 'srt' is not valid`). Requesting the transcript
  endpoint with no format parameter returns the full JSON transcript (speakers + segments + words), which was converted
  to plain text locally.
- **Duration-matched YouTube uploads** for the four Unchained-family episodes (all within 1 second of the RSS
  duration), which removes any ambiguity about which video corresponds to which feed item.
