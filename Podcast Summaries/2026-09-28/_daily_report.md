---
date: 2026-09-28
episodes_processed: 8
episodes_found_in_rss: 9
feed_fallbacks_recovered: 1
trailers_skipped: 0
late_additions: 2
generated_at: "2026-09-28T22:55:00Z"
generated_by: "Hermes cron (podcast aggregation pipeline, manual fallback)"
---

# Podcast Summary — 2026-09-28

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-28 (Monday) |
| Window | 2026-09-28 (last_run_date 2026-09-27 + 1) plus a prior-date diff of 2026-09-27 |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 5 (Bankless, Chalk Radio, Critics at Large, Latent Space, The Edge) |
| New in-window episodes | 9 RSS + 1 feed-fallback recovery + 2 late additions (09-27) |
| Episodes summarized | 8 on 09-28 + 2 late additions on 09-27 |
| Failures | 0 new |
| Transcript coverage (09-28) | 8/8 = 100% |
| Final transcript sources (10 eps) | `rss_omny_srt` 1 · `web` (podscripts.co) 2 · `youtube_autocaptions` 6 · `show_notes` 1 |

## Episodes — 2026-09-28 (8)

| Show | Episode | Duration | Source |
|---|---|---|---|
| Odd Lots | Why Building a Crosswalk in LA Is Kafkaesque | 21m58s | rss_omny_srt |
| The Peterman Pod | Amazon VP: Questions On Corporate Politics But They Get Increasingly Darker \| Ethan Evans | 3h07m53s | youtube_autocaptions |
| Bankless | Ben Cowen Says You Have Permission to be Bullish | 57m33s | youtube_autocaptions (feed-failure recovery) |
| Animal Spirits Podcast | Talk Your Book: The Next Generation of Income Strategies | 42m15s | web (podscripts.co) |
| Invest Like the Best | Noah Shinn - Building Instinct: The Personal Agent - [EP.493] | 1h26m13s | youtube_autocaptions (full transcript) |
| RiskReversal Pod | U.S. Bonds Are Trading Like an Emerging Market with Liz Thomas | 24m57s | youtube_autocaptions (full transcript) |
| The Compound and Friends | Erik Hirsch, CEO of Hamilton Lane, on the Explosive Growth of Private Markets | 45m26s | youtube_autocaptions (full transcript) |
| The Master Investor Podcast | Michael Hartnett: I coined “Mag7” - Here’s What Ends Their Run | 1h00m26s | show_notes |

## Late additions — 2026-09-27 (2)

| Show | Episode | Duration | Source |
|---|---|---|---|
| The Rest Is History | 709. The Terror: The Execution of Marie Antoinette (Part 3) | 1h11m41s | web (podscripts.co) |
| 硅谷101 | E253｜谁在给大模型出题、卖题、判卷？聊聊AI数据行业的野蛮生长 | 58m04s | youtube_autocaptions (full transcript) |

Both were recovered by the mandatory prior-date diff: re-fetching 2026-09-27 returned 4 episodes, 2 of which were not in `state.json`. Vault files were written into the **2026-09-27** directory and the 09-27 report was appended to (not rewritten).

## Key content

- **AI agents / startup strategy:** Noah Shinn (Instinct) — all software collapsing into one ambient interface, understandability over capability, time-to-first-credit-card as the core trust metric, and a compute problem that doubles weekly with multi-month procurement lead times.
- **Markets:** Michael Hartnett on the twin threat to the AI trade (bond market discipline and midterm voter backlash) and his "Buy Humiliation, Sell Hubris" contrarian playbook; Liz Thomas on Treasuries repricing into a new inflationary regime (5-year above 5%) and a "balance by extremes" allocation.
- **Income products:** Janus Henderson's Mike Laughlin on autocallables as contingent barrier puts and the ETF wrapper absorbing the 40-year-old structured-note market.
- **Private markets:** Hamilton Lane's Erik Hirsch on correlation, private credit redemption risk, secondaries and day-one markups, and widening retail access.
- **Real estate / policy:** a live Odd Lots from Hollywood on why an LA crosswalk costs about a million dollars and takes years, and why roughly 95% of the city being residentially zoned is the binding constraint.
- **Crypto:** Ben Cowen concedes his Q4-flush call was wrong, argues the bear market's brevity is explained by having far fewer "sins" to pay for than 2022, and sets $83K as the line below which bearishness becomes defensible.
- **Careers:** Ethan Evans on corporate politics — speak first, claim what is legitimately yours, be your own "color guy," and how far the gray area of self-promotion can legitimately stretch.
- **History / AI data:** Rest Is History part 3 on Marie Antoinette's imprisonment and execution; 硅谷101 E253 on what AI data vendors actually sell (rubrics, RL environments) and how much of the benchmark economy is score-gaming.

## Notes

- **Pipeline:** `run_podcast_pipeline.py --dates "2026-09-28"` exited after ~2 minutes with `Failed to authenticate: OAuth session expired and could not be refreshed` (Claude Code CLI, **19th consecutive day**). Completed via Manual Pipeline Fallback.
- **Bankless feed failure hid two in-window releases.** The parallel fetcher reported the usual Flightcast 167-byte `mismatched tag` error. A browser-UA direct fetch returned **HTTP 200 / 22.2 MB / 1,377 items**, and the newest item's `<pubDate>` is **Mon, 28 Sep 2026 10:30 UTC** (= 03:30 PT) — squarely inside the window. Per the documented recovery pattern, the episode was found on the official Bankless YouTube channel as "Bitcoin Broke the Bear Case. What Happens Next?" (`gYyhed2KEV8`) at **3,453 s against an RSS duration of 3,453 s — an exact match** — and captions were downloaded and summarized. The YouTube title differs completely from the RSS title, so a title-based search would have missed it.
- **The other four feed errors were verified clean.** With a browser UA all four returned HTTP 200 with full payloads (Chalk Radio 628 KB, Critics at Large 1.4 MB, Latent Space 14.0 MB, The Edge 131 KB) and their newest `<pubDate>`s are **Thu 24 Sep** (Critics at Large, The Edge), **Fri 25 Sep** (Latent Space) and **5 Mar 2026** (Chalk Radio) — the first three already covered in the 09-24 and 09-25 vaults, so **no in-window episode was lost by those four**.
- **Two RiskReversal items re-surfaced with a 2026-09-28 `pubDate` but were already processed.** "Anthony Scaramucci at Hunt & Fish Club Restaurant | Standing Table Podcast Episode #1" and "Rick Heitzmann at Manhatta | Standing Table Podcast Episode #2" carry GUIDs already in `state.json` under **2026-05-12** and **2026-05-19** respectively (the feed appears to refresh their pubDates). Deduped by GUID — not re-processed and not counted in the 8.
- **Two episodes were YouTube-caption recoveries rather than RSS transcripts** (Peterman 1.59 MB VTT, Bankless 577 KB VTT), and the Master Investor captions hit **HTTP 429 on two attempts** even after the other downloads finished; that record therefore relies on the official Podbean show notes plus auto-caption excerpts, and the limitation is stated inside the vault file itself.
- **Chalk Radio, The Edge and Critics at Large were not re-searched** for fallback episodes this run — each was positively confirmed to have no in-window release by direct feed inspection, which is stronger evidence than a search would have produced.
- Vault written: `Podcast Summaries/2026-09-28/` (8 files, one per GUID, verified) + 2 files in `Podcast Summaries/2026-09-27/`. No duplicate GUIDs across the 09-28 directory.

## Reconciliation — concurrent sibling run

A second instance of this job ran concurrently and finished after this one. It re-derived the same 10-GUID window, then **replaced five records with full-transcript versions**, because it recovered real auto-captions where the first pass had fallen back to show notes / recaps:

| Record | Before | After |
|---|---|---|
| The Compound and Friends — Erik Hirsch | `show_notes` | `youtube_autocaptions` (47 KB transcript) — 13 key points with figures ($146B AUM, >$1T advisory, ~2% private-credit defaults, Apollo Debt Solutions capping redemptions at 5% after 14.7% requests, ~13% secondary discounts, ~$640M quarterly net inflows) |
| Invest Like the Best — Noah Shinn | `web` (structured summary) | `youtube_autocaptions` (172 KB) |
| RiskReversal Pod — Liz Thomas | `web_substack` (recap) | `youtube_autocaptions` (24 KB) — replaces two paraphrased "quotes" that violated the verbatim rule |
| 硅谷101 — E253 | `show_notes` (fireside.fm) | `youtube_autocaptions` (64 KB, zh-Hans) |
| Bankless — Ben Cowen | `youtube_autocaptions` | unchanged (already a full transcript) |

All five re-summaries were generated with `deepseek-v4-pro`; filenames, GUIDs and vault dates were preserved, so no duplicate records were created and `state.json` still holds exactly 1,027 processed GUIDs.

**YouTube throttling was the binding constraint.** `www.youtube.com/api/timedtext` returned HTTP 429 on roughly 95% of requests; the signed URLs had to be pulled via `yt-dlp --print "%(automatic_captions)j"` and fetched separately, and success only came after a ~4-minute quiet period between attempts. Third-party transcript services (youtubetotranscript.com, Tactiq, NoteGPT, Kome) were all blocked (403/401/522), and `web_extract` refuses signed URLs as credential-like.

**Remaining weak record:** *The Master Investor Podcast — Michael Hartnett* still rests on Podbean show notes plus caption excerpts. Its caption fetch failed 20+ attempts across two processes and every cooldown cycle; the limitation is stated inside that vault file. This is the only non-transcript record of the ten.
