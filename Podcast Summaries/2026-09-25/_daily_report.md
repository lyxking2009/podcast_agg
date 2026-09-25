---
date: 2026-09-25
episodes_processed: 12
episodes_found_in_rss: 11
feed_fallbacks_recovered: 2
trailers_skipped: 0
late_additions: 1
generated_at: 2026-09-25T22:07:41Z
generated_by: "Hermes cron (podcast aggregation pipeline, deepseek-v4-flash manual fallback)"
---

# Podcast Summary — 2026-09-25

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-25 (Friday) |
| Window | 2026-09-24 15:00 PT → 2026-09-25 15:00 PT |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 47 (46 active, 1 excluded by config) |
| Feed errors | 5 (Bankless, Chalk Radio, Critics at Large, Latent Space, The Edge) |
| Episodes found in RSS scan | 11 (2026-09-25) + 6 (2026-09-24 re-fetch, all already in state) |
| Feed-fallback episodes recovered | 2 (Bankless ROLLUP, Latent Space) |
| Episodes summarized for 2026-09-25 | 12 |
| Late additions to 2026-09-24 vault | 1 (Latent Space) |
| Failures | 0 |
| Transcript coverage | 12/12 for 2026-09-25 = 100% |
| Rung 1 / Rung 2 / Rung 3 / Rung 4 | 3 / 3 / 0 / 8 |
| Pipeline | Hermes manual fallback (Claude Code OAuth expired — 18th consecutive day) |

**Run note.** `run_podcast_pipeline.py --dates "2026-09-25"` exited after ~7 seconds with `Failed to authenticate: OAuth session expired and could not be refreshed` (Claude Code CLI, `/Users/yuxinglin/.local/bin/claude`). Per the skill’s Manual Pipeline Fallback this run was completed inline: RSS fetch, transcript ladder, batch vault generation, report and state update. 11 RSS episodes + 2 feed fallbacks (13 items, ≤16 threshold) made this viable.

**Prior-date diff (late-publishing check).** Re-fetched 2026-09-24 alongside today: 6 episodes returned, all 6 already present in `state.json` `processed` — no missed late episodes from the prior day.

**YouTube rate-limiting.** Every `yt-dlp --write-auto-sub` call returned `HTTP Error 429: Too Many Requests` (retried once after a 45 s pause, and with alternate `--extractor-args` player clients — same result). Pending YouTube caption sources were therefore routed to the publisher’s own web/show-note sources instead of being skipped. Two videos were positively identified with exact RSS duration matches before the block: Empire `-iN23QvBj-k` (3,064 s vs RSS 3,063 s) and Unchained `PQy2WhgAMDE` (2,943 s vs RSS 2,942 s) — both unusable because their captions could not be downloaded.

## Feed errors (5)

| Feed | Error | Outcome |
|---|---|---|
| Bankless | `XML: mismatched tag: line 6, column 2` (Flightcast 167-byte error) | Browser-UA fetch: HTTP 200, 22.2 MB. In-window item **ROLLUP: The Bull Market is On? \| Zcash & NEAR \| Kalshi Wash Trading \| BlackRock Goes Onchain** (Fri 25 Sep 10:30 -0000) — **recovered**: free ROLLUP, full transcript extracted from bankless.com (`web_search_fallback`). |
| Latent Space: The AI Engineer Podcast | `XML: mismatched tag: line 6, column 2` (Flightcast) | Browser-UA fetch: HTTP 200, 13.9 MB. In-window item **Runway’s WorldPrompt and the Engineering of Real-Time Worlds** (Fri 25 Sep 01:30 GMT = 24 Sep 18:30 PT) — **recovered** as a **late addition to the 2026-09-24 vault** (`web_search_fallback`). |
| Chalk Radio | timeout | Browser-UA fetch: HTTP 200, 628 KB. Latest episode is 5 Mar 2026 — **no episode in window**. |
| Critics at Large \| The New Yorker | timeout | Browser-UA fetch: HTTP 200, 1.4 MB. Latest episode 20 Sep 2026 — **already covered in the 2026-09-24 vault**; no new in-window episode. |
| The Edge | timeout (Buzzsprout) | Browser-UA fetch: HTTP 200, 131 KB. Latest episode #36 (7 Jul 2026) — **already covered in the 2026-09-24 vault**; no new in-window episode. |

The browser-UA recovery sequence (parallel fetch → plain curl → Chrome-UA curl) held for a fourth consecutive run: all five failing feeds returned HTTP 200 with full payloads under a browser User-Agent, and `<pubDate>` inspection settled in-window existence for every one of them. This run it also **recovered two full episodes** that the parallel fetcher lost — Bankless recovered completely from its public transcript; Latent Space was fully recoverable but had actually published the previous evening.

## Episodes (12 for 2026-09-25)

| Show | Episode | Duration | Source |
|---|---|---|---|
| Big Technology Podcast | Meta’s Muse Revival, Frontier AI Under Threat, The Rise Of Dopamine Sites | 57m16s | show_notes |
| Empire | Markets Sending, Just Buy Consensus, Meta's AI Moment & Who Wins Tokenization? \| Weekly Roundup | 51m03s | show_notes |
| Everybody's Business | Why Are Employers Dropping GLP-1 Coverage? | 37m30s | rss_omny_srt |
| In Good Company with Nicolai Tangen | Friday Wrap-Up: A President's View on Global Power and Learning from Big Tech | 11m08s | show_notes |
| Money Stuff: The Podcast | Re-run: Midnight Madness: Ryan Patch and Jon Seale | 40m46s | rss_omny_srt |
| Odd Lots | Hollywood Was Cooked Before AI, and Now It's Only Getting Worse | 32m07s | rss_omny_srt |
| RiskReversal Pod | Jim Chanos & Gary Marcus: Circular AI Financing, The Agentic Economy & Doomsday Scenarios | 1h37m28s | web_substack |
| StarTalk with Neil deGrasse Tyson | Cosmic Queries – Space Junk! with Moriba Jah | 54m51s | web |
| The Compound and Friends | People Hate These 5 Stocks But They'll Be Wrong with Jonathan Boyar | 1h07m43s | show_notes |
| The Meb Faber Show | Dave Iben: The World Is on Sale While America Trades at a Premium \| #652 | 37m21s | show_notes |
| Unchained | Why the Crypto Market Cap Could Reach $50 Trillion This Cycle | 49m02s | show_notes |
| Bankless | ROLLUP: The Bull Market is On? \| Zcash & NEAR \| Kalshi Wash Trading \| BlackRock Goes Onchain | 1h00m57s | web_search_fallback |

## Late additions to 2026-09-24

| Show | Episode | Source | Note |
|---|---|---|---|
| Latent Space: The AI Engineer Podcast | Runway’s WorldPrompt and the Engineering of Real-Time Worlds | web_search_fallback | Published 2026-09-24 18:30 PT — after the 2026-09-24 3 PM run. Feed errored in both runs (Flightcast 167-byte). Recovered via browser-UA fetch today; file written to the 2026-09-24 vault. |

## Notes

- **Zero failures**: all 13 in-window items were summarized. The 5 feed errors produced no unprocessed episodes — two were recovered (Bankless, Latent Space) and three carry no in-window episode (Chalk Radio, Critics at Large, The Edge).
- **Coverage gap source note**: 8 of 13 items were summarized from publisher show notes or structured summaries rather than verbatim transcripts (`show_notes` × 6, plus the Compound and Meb Faber chaptered notes counted there, RiskReversal Substack structured summary, In Good Company Acast notes). This is a direct consequence of the YouTube 429 block plus those shows not publishing transcripts; each summary cites its source in the vault frontmatter.
- The 3 Omny SRT episodes (Everybody's Business, Money Stuff, Odd Lots) and StarTalk’s published S12E21 transcript are full verbatim sources.
- State updated: `last_run_date` = 2026-09-25; 12 processed entries dated 2026-09-25 plus 1 late-addition entry dated 2026-09-24; `failures` unchanged.
