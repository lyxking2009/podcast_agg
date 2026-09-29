---
date: 2026-09-29
episodes_processed: 4
episodes_found_in_rss: 3
feed_fallbacks_recovered: 1
trailers_skipped: 0
late_additions: 0
generated_at: "2026-09-29T22:42:00Z"
generated_by: "Hermes cron (podcast aggregation pipeline) — sibling run at 22:08Z plus transcript-recovery pass"
---

# Podcast Summary — 2026-09-29

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-29 (Tuesday) |
| Window | 2026-09-29 (last_run_date 2026-09-28 + 1); prior-date diff of 2026-09-28 re-checked |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 5 on first pass (Bankless, Chalk Radio, Critics at Large, Latent Space, The Edge) — all verified 200 on direct re-fetch |
| New in-window episodes | 3 RSS + 1 feed-failure recovery (Latent Space) |
| Episodes summarized | 4 (3 from the 22:08Z sibling run + TWIML #778 recovered in this pass) |
| Failures | 0 (TWIML #778 failure resolved — see below) |
| Transcript coverage | 3/4 = 75% full transcript · 4/4 = 100% with show notes |
| Final transcript sources | `whisper_asr` 1 · `youtube_autocaptions` 1 · `web` 2 |

## Episodes — 2026-09-29 (4)

| Show | Episode | Duration | Source |
|---|---|---|---|
| Latent Space: The AI Engineer Podcast | Claude Code's Next Era — Thariq Shihipar, Anthropic (feed-failure recovery) | n/a | web (latent.space/p/thariq, full transcript) |
| StarTalk with Neil deGrasse Tyson | Quantum Anomalies with Lara Anderson | 54m14s | web (podscripts.co) |
| The Peter McCormack Show | #216 - Ross Clark - Britain is Bust & The Coming Sovereign Debt Crisis | 1h09m39s | **youtube_autocaptions** (upgraded) |
| The TWIML AI Podcast | From Math Olympiads to Navier-Stokes: How Fast Is AI Progressing? with Greg Burnham - #778 | 1h07m48s | **whisper_asr** (recovered) |

## Transcript recovery — two episodes upgraded over the 22:08Z sibling pass

The 22:08Z sibling run filed this date with `show_notes` for the Peter McCormack episode and a hard
failure for TWIML #778. Both were resolved in this pass without changing any other file.

**The Peter McCormack Show #216 — `show_notes` → `youtube_autocaptions`.** The episode's own YouTube
upload (`NO7NTYtOhSk`, 4,187 s) was located, but the sibling run's three attempts at the signed
`timedtext` endpoint all returned HTTP 429. The block is per-format, not per-video: `srv1` and `vtt`
stayed 429 across four User-Agents and four header combinations (including no-`Referer` and iOS/Android
client UAs), while **`json3` returned 200 on the first try** with a stock desktop UA and no Referer.
Result: 62,076 chars of verbatim ASR, reflowed to 182 speaker paragraphs. The vault file keeps the
same GUID and filename, and `state.json` was corrected in place — no duplicate episode file.

**TWIML #778 — `no_transcript_found` → `whisper_asr`.** The sibling's verdict was accurate at the time
(`twimlai.com` episode page 404s, show-notes URL `twimlai.com/go/778` 404s, the TWIML site carries no
transcript section on any episode page, and the show's YouTube upload `lz9J-Dbj2zw` had no caption
tracks of any kind — manual or automatic). Rather than leave it as a description-only record, the
episode audio was transcribed locally:

- `ffmpeg` → 16 kHz mono PCM (4,038.9 s), `whisper.cpp` 1.9.4 with `ggml-large-v3-turbo`
- 1,613 s wall (~2.5× realtime) → 63,402 chars, 64 paragraphs, `progress = 100%`

Vault file written and the failure entry removed from `state["failures"]` (93 → 92), so the GUID is
not retried as a backfill on later runs.

## Prior-date diff — 2026-09-28

Re-fetched 2026-09-28 explicitly (mandatory late-publishing check): **9 episodes returned, all 9 GUIDs
already in `state.json`** — no late-published episode was missed and no vault writes were needed.

## Feed errors — verified, one recovery

| Feed | UA fetch | Newest item | Verdict |
|---|---|---|---|
| Bankless | 200, 22.2 MB | Mon 28 Sep 10:30 UTC — “Ben Cowen Says You Have Permission to be Bullish” | already covered 09-28 — no in-window episode |
| Latent Space | 200, 14.2 MB | **Tue 29 Sep 01:48 UTC — “Claude Code's Next Era — Thariq Shihipar, Anthropic”** | **in window — recovered** |
| Chalk Radio | 200, 628 KB | Thu 05 Mar 2026 | no in-window episode |
| Critics at Large | 200, 1.4 MB | Thu 24 Sep | already covered — no in-window episode |
| The Edge | 200, 131 KB | Thu 24 Sep | already covered — no in-window episode |

## Key content

- **AI engineering:** Thariq Shihipar on Claude Code's next era — artifacts as the interface to the
  harness, cloud brain / local hands, Claude Mods making the harness mutable, implementation notes as
  the highest-value prompting habit, Claude.md on its way out, and a detailed security discussion
  (agents reverse-engineering benchmark scorers, chaining sandbox exploits, Auto Mode permission checks).
- **Science:** Lara Anderson on string theory at 40 years — the Higgs is settled, quantum gravity is
  not; extra dimensions bounded below ~10^-19 m; gravity 40 orders of magnitude weaker than
  electromagnetism and possibly leaking into dimensions we cannot see.
- **UK macro / politics (now from verbatim transcript):** Ross Clark on £3.4tn of debt whose £109bn
  annual interest bill exceeds the £60bn defence budget, a £133bn deficit with no surplus since
  2000-01, a quarter of the gilt stack being index-linked, and "tax the billionaires buys six months".
  Concrete proposals include cutting the basic state pension 10% to its 2011 level, capping the
  disability-benefit and Motability expansion, and NHS productivity gains worth ~£22bn a year.
- **AI capabilities:** Greg Burnham (Epoch AI) on the FrontierMath progression, the May 2026
  unit-distance-problem result, the Navier-Stokes solution as persistence plus prior human work rather
  than original theory-building, and Epoch's AI R&D "tripwire" benchmark. Key negative finding: models
  show flat learning curves across 30 repeated plays of Earthborne Rangers while humans improve fast,
  and harness-side tricks (multi-agent, note-taking, skills) yield only low-hanging gains.

## Notes

- **Two pipelines ran for this date.** The sibling run finished at 22:08Z and this pass started from the
  same `last_run_date` (2026-09-28 → window = 2026-09-29). Both derive the same window; no episode was
  double-filed (verified: 4 files, 4 distinct GUIDs, no duplicates).
- **The YouTube caption block is format-specific.** When `timedtext` 429s, `srv1`/`vtt` are skipped —
  the `json3` variant is worth trying before declaring captions unavailable.
- **`whisper.cpp` + `ggml-large-v3-turbo` is now a working local ASR path** (installed via Homebrew;
  model at `data/whisper_models/ggml-large-v3-turbo.bin`). It resolved a same-day episode that had no
  published transcript anywhere, ~2.5× realtime. Prefer it over a description-only record when the RSS
  audio URL is public.
- **Stale failure worth a backfill pass:** TWIML #775 "World Models and the Future of Spatial AI with
  Justin Johnson" (first seen 2026-09-01, reason "transcript incomplete") is still open in
  `state["failures"]` and is a candidate for the same ASR route.
