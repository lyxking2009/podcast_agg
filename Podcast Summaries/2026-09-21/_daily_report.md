---
date: 2026-09-21
episodes_processed: 5
episodes_found_in_rss: 5
feed_fallbacks_recovered: 0
trailers_skipped: 0
late_additions: 1
generated_at: 2026-09-21T22:09:13Z
generated_by: "Hermes cron (manual pipeline fallback — Claude Code OAuth expired, 17th consecutive day)"
---

# Podcast Summary — 2026-09-21

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-21 (Monday) |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`; up from 46) |
| Feeds fetched | 47 |
| Feed errors | 0 |
| Episodes found in scan window | 6 |
| Episodes vaulted to this date | 5 |
| Episode filed to 2026-09-20 (late addition) | 1 |
| Transcript coverage | 5/5 (100%) for this date; 6/6 including the late addition |
| Trailers/promos skipped | 0 |
| Rung 1 / Rung 2 / Rung 3 | 2 / 4 / 0 |
| Pipeline | Manual fallback — Claude Code OAuth session expired (17th consecutive day) |

Normal Monday volume: five in-window episodes published on 09-21, plus one 09-20 episode (The Rest Is History 707) recovered by the scan window's one-day backfill and filed as a late addition to 2026-09-20.

## Transcript sources (this date)

| Source | Count |
|---|---|
| `rss_omny_srt` | 1 |
| `rss_vtt` | 1 |
| `youtube_autocaptions` | 1 |
| `web_podscripts` | 1 |
| `web_substack` | 1 |

Four of five came from the RSS-declared tag or a publisher-hosted transcript — no episode needed the episode-description last resort.

## Episodes

| Show | Episode | Duration | Source |
|---|---|---|---|
| Odd Lots | [[odd-lots__how-la-is-quietly-becoming-america-s-new-industrial-tech-hub]] | 36m4s | `rss_omny_srt` |
| Animal Spirits Podcast | [[animal-spirits-podcast__talk-your-book-price-is-the-ultimate-factor]] | 28m55s | `web_podscripts` |
| The Peterman Pod | [[the-peterman-pod__openai-meta-distinguished-eng-ic9-the-psychology-behind-tech-career-peaks-philip]] | 44m21s | `web_substack` |
| Bankless | [[bankless__how-robinhood-is-using-the-sec-s-new-innovation-exemption-johann-kerbrat]] | 37m46s | `rss_vtt` |
| Machine Learning Street Talk (MLST) | [[machine-learning-street-talk-mlst__why-scaling-prediction-cannot-create-intelligence-alexander-mattick]] | 2h14m20s | `youtube_autocaptions` |

## Late additions (recovered for 2026-09-20)

| Show | Episode | Source | Detail |
|---|---|---|---|
| The Rest Is History | [[the-rest-is-history__707-the-terror-an-assassination-in-paris-part-1]] | `web_podscripts` | Ep. 707, published Sun 20 Sep, guid `0322655e-b118-11f1-b8eb-f79d9c6ca5d1`, 1h25m33s. Filed into the 2026-09-20 folder and flagged `late_addition` in state. See that date's report for the `show_notes` → `web_podscripts` upgrade note. |

## Feed errors (handled)

**None.** All 47 subscription feeds returned HTTP 200 with full payloads on the first parallel pass (browser User-Agent, `curl -sL --max-time 30`, 8 workers).

This differs from the concurrent sibling run's report, which logged five feed errors (Bankless, Latent Space, Chalk Radio, Critics at Large, The Edge) on a first pass with a plain default User-Agent, then recovered each with a Chrome UA. The browser-UA header was applied to **every** request in this run from the outset, so the Flightcast 167-byte `XML: mismatched tag` response and the Buzzsprout/Simplecast/publicfeeds empty responses never occurred. Confirms the earlier finding: for these hosts the User-Agent is the decisive variable, not the feed.

## Scan window

`2026-09-20T07:00:00Z` → `2026-09-22T06:59:59Z` (label window 2026-09-21 PT; the start carries a one-day backfill so late-publishing items from the prior date are not lost). `last_run_date` was `2026-09-20`; `lookback_cap_days` is 7, so no cap applied.

## Notes

- **Concurrent sibling run reconciled — no duplicate GUIDs left behind.** A second instance of this cron job fired at ~15:03 PDT and wrote state entries for all six episodes plus a 09-21 daily report and a Peterman Pod vault file, while this run was still fetching. At reconciliation three collisions were found and resolved, all in favour of the version backed by a real transcript:
  1. **The Peterman Pod** — two vault files for the same guid (`c7969f5b-…`): the sibling's `…openai-and-meta-distinguished-eng-…-philip-su.md` (YouTube auto-captions, 86-char slug) and this run's `…openai-meta-distinguished-eng-…-philip.md` (publisher transcript, 80-char truncation per the pipeline's slug rule). This run's file was kept and the sibling's deleted, bringing the episode-slug convention back in line with the rest of the vault.
  2. **The Rest Is History 707** — written to *both* the 09-20 and 09-21 folders. Kept the 09-20 copy (it is a 09-20-published late addition) and removed the 09-21 copy.
  3. **Transcript sources were upgraded** where the sibling had fallen back: Bankless `web_search_fallback` → `rss_vtt`, Animal Spirits `show_notes` → `web_podscripts`, Peterman `youtube_autocaptions` → `web_substack`, TRIH `show_notes` → `web_podscripts`. `state.json` was rewritten to match the files that actually survive.
  A full-vault frontmatter scan now shows one file per GUID for all six of today's episodes. (Nine older duplicate-GUID pairs from 2026-06 onward remain elsewhere in the vault — pre-existing slug-variant artifacts, e.g. `fomo-s`/`fomos`, `kyc-d`/`kycd` — and were left untouched.)
- **Bankless was not actually a premium dead end.** The sibling report concluded the episode was gated on bankless.com and substituted a duration-matched YouTube upload. The RSS feed *does* carry a `podcast:transcript` element — `https://rss.flightcast.com/transcripts/01M31NN7M7AFMPFJSG8VBD53Z0.vtt` — which downloaded cleanly (35.8 K chars, 37m46s). Rung 1 resolves Bankless whenever the publisher ships the tag; check it before reaching for Rung 2/3.
- **Animal Spirits “Talk Your Book” has a transcript after all.** The publisher's own page is show-notes only, but PodScripts carries a full transcript (28.9 K chars), so this run upgraded the episode from the `show_notes` fallback and restored the quotes section. Worth trying PodScripts for this sponsored series in future runs.
- **MLST — YouTube title differs from the RSS title again.** RSS: “Why Scaling Prediction Cannot Create Intelligence - Alexander Mattick”; the upload for the same video id `1S1B4XkFCD8` is titled differently and its duration is a 1-second-exact match to the RSS (8,060 s vs 2h14m20s). Downloaded as `json3` rather than VTT: the VTT is karaoke word-timed and prefix-dedup leaves ~2× duplication (283 K chars with repeated phrases), whereas concatenating `events[].segs[].utf8` yields a clean 141 K chars. **Use `--sub-format json3` for yt-dlp auto-captions.**
- **Cross-show convergence.** Kerbrat's Robinhood-Chain numbers (tokenised equities trading mostly outside exchange hours) and the Odd Lots live show on reindustrialising Los Angeles both land on the theme running through 09-18 → 09-21: market infrastructure and physical production capacity being rebuilt outside the incumbent venues. MLST's Mattick episode is the odd one out — a theory-of-deep-learning argument that scaling prediction alone cannot yield intelligence, with the “Bitter Lesson” revisited.
- **Claude Code OAuth remains expired** (2026-08-08 → 2026-09-21, 17th consecutive day); `run_podcast_pipeline.py` exits ~75 s after launch with `Failed to authenticate: OAuth session expired and could not be refreshed`. Manual Hermes-native fallback used, as documented.
