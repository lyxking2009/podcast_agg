---
date: 2026-09-19
episodes_processed: 3
episodes_found_in_rss: 2
feed_fallbacks_recovered: 0
late_additions: 1
generated_at: 2026-09-19T22:06:00Z
revised_at: 2026-09-19T22:08:35Z
generated_by: "Hermes cron (manual pipeline fallback — Claude Code OAuth expired, 15th consecutive day)"
---

# Podcast Summary — 2026-09-19

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-19 (Saturday) |
| Episodes found via RSS | 2 |
| Episodes from feed-error fallback | 0 |
| Episodes processed | 2 |
| Transcript coverage | 2/2 (100%) |
| Feed errors | 5 (Bankless, Latent Space, Chalk Radio, Critics at Large, The Edge) |
| Late additions to 2026-09-18 | 0 |
| Pipeline | Manual fallback — Claude Code OAuth session expired |

Normal Saturday volume: most shows do not publish at weekends. The two episodes below were the only in-window items across all 46 subscribed feeds.

## Transcript sources

| Source | Count |
|---|---|
| rss_omny_srt (Bloomberg/Omny) | 1 |
| local_whisper_asr (local faster-whisper `small.en`, CPU int8, on the published MP3) | 1 |

## Episodes

| Show | Episode | Source |
|---|---|---|
| Big Technology Podcast | [[big-technology-podcast__ai-doom-backlash-arrives-anthropic-openai-ipo-outlook-frontier-business-momentum]] | `local_whisper_asr` |
| Odd Lots | [[odd-lots__a-goldman-m-and-a-banker-helped-bring-the-olympics-to-los-angeles]] | `rss_omny_srt` |

## Feed errors (handled)

| Feed | Error | Resolution |
|---|---|---|
| Bankless | Flightcast `XML: mismatched tag` (167-byte error response) | ✅ Direct feed fetch succeeded (21.9 MB) — **no in-window episode**; latest is the 2026-09-18 ROLLUP, already vaulted on 09-18 |
| Latent Space: The AI Engineer Podcast | Flightcast `XML: mismatched tag` | ✅ Direct feed fetch succeeded (13.5 MB) — **no in-window episode**; latest is 2026-09-14 (Rune Kvist, AIUC) |
| Chalk Radio | timeout | ✅ Direct feed fetch succeeded — **dormant**; latest episode 2025-11-05 |
| Critics at Large \| The New Yorker | timeout | ✅ Direct feed fetch succeeded (1.4 MB) — **no in-window episode**; latest 2026-09-03 |
| The Edge | timeout (empty response) | ✅ Direct feed fetch succeeded — **dormant**; latest episode #35, 2026-04-07 |

All five recovered with the documented direct-curl pattern (`curl -sL --max-time 90 -H "Accept-Encoding: identity"`), which succeeded where the parallel fetcher reported timeout/parse errors. `<pubDate>` inspection settled in-window existence for every one — no fallback web searches were needed, which kept this run short.

## Late-publishing check (prior date)

Re-fetched 2026-09-18 independently and diffed every GUID against `state.json` `processed`: **14/14 already processed, zero late additions.** The previous run completed at 22:32Z (15:32 PDT) on 09-18, so no episode published after that day's fetch was missed.

## Notes

- **Odd Lots** was an on-stage recording at the Future Proof conference in Huntington Beach (32m51s) — the Omny SRT transcript downloaded and cleaned cleanly (38.4K chars, 7,032 words). Guest Gene Sykes (Goldman Sachs global M&A co-head / USOPC chair / IOC member) is unusually quotable on both Olympic financing and the AI-driven M&A cycle.
- **Big Technology Podcast** is a same-day Megaphone-hosted news roundtable (Ranjan Roy, Margins) with no RSS transcript tag and no published transcript or YouTube upload at run time. The first pass summarised the publisher's ten-point running order (`show_notes`); it was then replaced with a **real transcript obtained locally**: the published MP3 (99 MB, 67m57s) was transcribed with faster-whisper `small.en` (CPU, int8, 1,370 segments, 11,593 words, 273 s wall time) and the episode re-summarised from it at 22:08Z — hence `transcript_source: local_whisper_asr` and four verbatim, speaker-attributed quotes (spot-checked against the transcript). This is the Rung-4 escalation for hosts that ship neither an RSS transcript track nor a timely third-party transcript.
- **Arc context worth noting:** this Big Technology episode continues the show's four-week AI-risk arc (2026-09-11 "Will AI Wipe Out Humanity? (And Who Profits)", 2026-09-16 Nate Soares) and its final topic — "the frontier AI business shows signs of slowing" — is the same slowdown theme that dominated the 2026-09-18 In Good Company wrap-up. Two independent shows converging on frontier-AI revenue deceleration in the same week.
- **Revision (22:08Z):** the Big Technology entry above was re-generated from the local ASR transcript; the earlier `show_notes`-based file (slugged `…anthropic-and-openai…`) was removed so the vault holds exactly one file per GUID. Post-revision the 2026-09-19 directory contains exactly two episode files — one per in-window GUID — plus this report (verified by listing after the rewrite).
- **Late addition (2026-09-20 run):** The Investor's Podcast — *RWH072: The Making of A Money Master w/ Rob Vinall* published 2026-09-19 17:00 PT, after this run's feed fetch, and was processed by the 2026-09-20 run (full 22,079-word podscripts transcript). Its summary is filed in this 2026-09-19 directory by published date.
- Claude Code OAuth remains expired (15th consecutive day, 2026-08-08 → 2026-09-19) — manual pipeline fallback used as documented.
