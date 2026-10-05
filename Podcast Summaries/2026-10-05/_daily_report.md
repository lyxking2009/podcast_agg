---
date: 2026-10-05
episodes_processed: 10
episodes_found_in_rss: 10
feed_fallbacks_recovered: 5
feed_fallbacks_identified: 15
trailers_skipped: 0
late_additions: 1
generated_at: "2026-10-05T22:12:02Z"
generated_by: "Hermes cron (podcast aggregation pipeline) — manual fallback"
---

# Podcast Summary - 2026-10-05

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-05 (Monday) |
| Window | 2026-10-05 only (last_run_date 2026-10-04 + 1); 2026-10-04 re-fetched for late episodes |
| Subscriptions synced | 47 written (Apple Podcasts) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 15 on first parallel pass — mass truncation, all re-fetched directly |
| In-window episodes | 10 (2026-10-05) + 1 late addition (2026-10-04) |
| Episodes summarized | 11 total |
| Failures | 0 |
| Transcript coverage | 11/11 substantial (100%) |
| Final transcript sources | `youtube_autocaptions` 6, `rss` 2, `web_podscripts` 1, `whisper_transcription` 1 (plus 1 `web` for the late addition) — see upgrade pass below |

## Episodes - 2026-10-05 (10 summarized)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| Big Technology Podcast | [[big-technology-podcast__who-wins-the-ai-assistant-wars-why-didn-t-google-build-muse]] | 1:04:08 | `show_notes` | RSS description (no YT upload at publish time) |
| Empire | [[empire__compute-is-a-trillion-dollar-market-trading-in-group-chats-brett-harrison-and-andrawes-bah]] | 1:05:37 | `youtube_autocaptions` | YouTube NWMAOLQRQoI (3938s = RSS 3937s) |
| The Master Investor Podcast with Wilfred Frost | [[the-master-investor-podcast-with-wilfred-frost__the-bond-pivot-jim-bianco-on-why-it-s-time-to-start-]] | 58:57 | `web` | Podbean episode page — 14 chapters + full summary |
| The Peter McCormack Show | [[the-peter-mccormack-show__218-narinder-kaur-divided-and-looted-how-the-political-system-plays-us]] | 2:07:57 | `show_notes` | Acast episode page — 21 chapters |
| The Peterman Pod | [[the-peterman-pod__neetcode-google-vs-amazon-programming-interviews-quitting-big-tech]] | 34:43 | `youtube_autocaptions` | YouTube K15dND_Kfvg (2083s = RSS 2083s) |
| Unchained | [[unchained__paid-partnership-how-nexo-wants-you-to-build-wealth-using-crypto]] | 24:26 | `show_notes` | YouTube UPXpZIeokJ8 description with timestamps (subs 429) |
| Animal Spirits Podcast | [[animal-spirits-podcast__talk-your-book-how-higher-rates-are-re-shaping-commercial-real-estate]] | 26:34 | `show_notes` | theirrelevantinvestor.com + awealthofcommonsense.com show notes |
| Bankless | [[bankless__why-the-near-nrr-etf-is-different-hunter-horsley-and-sal-ternus]] | 43:47 | `rss` | Flightcast VTT (recovered via direct feed re-fetch) |
| Odd Lots | [[odd-lots__why-treasuries-are-risky-again]] | 50:39 | `rss` | Omny SubRip transcript (recovered via direct feed re-fetch) |
| RiskReversal Pod | [[riskreversal-pod__consumers-feel-awful-but-keep-spending-with-mastercard-s-chief-economist-michelle-]] | 28:21 | `web` | riskreversal.substack.com structured summary |

## Feed errors and recovery

The first `fetch_episodes_parallel.py` pass hit **15 feed parse errors** — the documented mass-truncation pattern (large feeds cut mid-stream, `unclosed CDATA`, `unclosed token`, `no element found` at varying line numbers), not per-feed breakage. All 15 were re-fetched directly with a browser UA and a 150s timeout in a 6-worker pool; every one returned HTTP 200 with a full payload.

| Feed | Result of direct re-fetch | In-window episodes recovered |
|---|---|---|
| Animal Spirits Podcast | 200, 5.8 MB | 1 — Talk Your Book (commercial real estate) |
| Bankless | 200, 21.7 MB | 1 — Near $NRR ETF (plus $VVV, already covered 2026-10-04) |
| Chalk Radio | 200, 612 KB | 0 — no episode in window |
| Critics at Large | 200, 1.4 MB | 0 — latest 2026-09-14 |
| Invest Like the Best | 200, 7.2 MB | 0 |
| Latent Space | 200, 14.0 MB | 0 |
| Masters in Business | 200, 3.1 MB | 0 (latest 2026-10-05 08:00Z investigated — no separate in-window item returned) |
| Odd Lots | 200, 6.3 MB | 1 — Why Treasuries Are 'Risky' Again |
| Practical AI | 200, 3.7 MB | 0 |
| RiskReversal Pod | 200, 5.7 MB | 1 — Michelle Meyer |
| StarTalk | 200, 5.0 MB | 0 |
| The Edge | 200, 127 KB | 0 |
| The Investor's Podcast (We Study Billionaires) | 200, 26.7 MB | 0 new (TIP851 already covered 2026-10-04) |
| The Meb Faber Show | 200, 5.3 MB | 0 |
| 硅谷101 | 200, 3.7 MB | 0 |

**Takeaway:** without the direct re-fetch, the run would have reported only 8 episodes and missed Bankless and Odd Lots entirely. Episode counts from the parallel fetcher are unreliable on truncated days — trust the direct fetch.

## Late additions (recovered 2026-10-05)

The prior-date diff of **2026-10-04** found 4 in-window episodes, 3 already in the vault (`tip851`, `vvv-community-call`, Lenny's/Tibo Sottiaux). The fourth — **The Rest Is History 711. The Terror: Killing God (Part 5)** — published 2026-10-04 23:05 UTC (16:05 PT), after the 2026-10-04 run's 22:18Z fetch, and was written into the 2026-10-04 vault directory as a late addition.

## Notes

- **Claude Code OAuth is expired (21st consecutive day).** A direct probe (`claude -p`) returned `Failed to authenticate: OAuth session expired and could not be refreshed`, so the pipeline was not launched and the run was completed via the Manual Pipeline Fallback (v1.26.x). No time was wasted waiting on the pipeline.
- **All 11 episodes have substantial sources.** Two came free from RSS (`Bankless` Flightcast VTT, `Odd Lots` Omny SubRip); two from duration-verified YouTube auto-captions (Empire 3938 vs RSS 3937; Peterman 2083 vs RSS 2083); two from web sources (Master Investor Podbean chapters, RiskReversal Substack); four from show notes.
- **Unchained (Nexo) captions were 429 rate-limited**, so the record uses the YouTube description, which carries the full topic breakdown and guest details. Labelled `show_notes`; no verbatim quotes presented.
- **Big Technology has no YouTube upload yet** (published ~12:21 PT). Recorded from the RSS episode description as `show_notes` — flagged as such rather than presented as a transcript.
- The 30-year Treasury yield at ~5.59%, its highest since 2002, links the Odd Lots episode to the Master Investor (Jim Bianco) episode — both frame the same rates regime, from opposite ends of the practitioner/academic divide.
- Feed-error count (15) reflects the truncated parallel pass; this is the mass-truncation pattern, not 15 independently broken feeds.

## Upgrade pass (post-run reconciliation)

A second Hermes cron pass (same job) ran after the manual fallback and re-summarised six episodes that the first pass had recorded from `show_notes` / non-verbatim web summaries, using full verbatim transcripts. YouTube subtitles were 429 rate-limited on the timedtext endpoint during the first pass; the second pass pulled them via the innertube API (`youtube_transcript_api`), and used local Whisper (`faster-whisper small.en`) where no caption track existed. Files were overwritten in place (matched by GUID); state updated.

| Episode | Was | Now | Transcript | Key points |
|---|---|---|---|---|
| Big Technology — AI Assistant Wars | `show_notes` (RSS blurb) | `whisper_transcription` | 67k chars, Whisper ASR | 13 |
| Animal Spirits — Talk Your Book: CRE | `show_notes` | `web_podscripts` | 28k chars, verbatim | 14 |
| The Peter McCormack Show #218 | `show_notes` (Acast chapters) | `youtube_autocaptions` | 130k chars | 18 |
| Unchained — Nexo | `show_notes` (YT description) | `youtube_autocaptions` | 25k chars | 12 |
| Master Investor — Jim Bianco | `web` (Podbean chapters) | `youtube_autocaptions` | 57k chars | 12 |
| RiskReversal — Michelle Meyer | `web` (Substack summary) | `youtube_autocaptions` | 26k chars | 12 |

Empire, Peterman Pod (`youtube_autocaptions`), Bankless and Odd Lots (`rss`) already had full transcripts and were left unchanged. After the pass, all 11 episodes carry verbatim transcripts with real timestamps and quotes — no `show_notes`-only records remain.
