---
date: 2026-09-26
episodes_processed: 2
episodes_found_in_rss: 3
feed_fallbacks_recovered: 1
trailers_skipped: 0
late_additions: 2
generated_at: "2026-09-26T22:10:00Z"
generated_by: "Hermes cron (podcast aggregation pipeline, deepseek-v4-flash manual fallback)"
---

# Podcast Summary — 2026-09-26

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-26 (Saturday) |
| Window | 2026-09-25 15:00 PT → 2026-09-26 15:00 PT |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 47 (46 active, 1 excluded by config) |
| Feed errors | 5 (Bankless, Chalk Radio, Critics at Large, Latent Space, The Edge) |
| Episodes returned by RSS scan | 14 across both dates (12 dated 2026-09-25, 2 dated 2026-09-26) |
| New in-window episodes | 3 (MLST + YC dated 09-26; Masters in Business dated 09-25) |
| Feed-fallback episodes recovered | 1 (Latent Space `OpenRouter: from Seed to Stripe`) |
| Episodes summarized for 2026-09-26 | 2 |
| Late additions to 2026-09-25 vault | 2 (Masters in Business; Latent Space) |
| Failures | 0 |
| Transcript coverage | 2/2 for 2026-09-26 = 100% |
| Rung 1 / Rung 2 / Rung 3 / Rung 4 | 1 / 3 / 0 / 0 |
| Pipeline | Hermes manual fallback (Claude Code OAuth expired — 19th consecutive day) |

**Run note.** Saturday, so the RSS scan was light: 2 episodes dated 2026-09-26 and 1 late-published episode dated 2026-09-25. `run_podcast_pipeline.py --dates "2026-09-25 2026-09-26"` exited after **24 seconds** with `Failed to authenticate: OAuth session expired and could not be refreshed` (Claude Code CLI, `/Users/yuxinglin/.local/bin/claude`). Per the skill's Manual Pipeline Fallback this run was completed inline: RSS fetch, browser-UA feed recovery, transcript ladder, vault writes, report and state update. 3 in-window items + 4 episodes total (≤16 threshold) made this viable.

**Prior-date diff (late-publishing check).** Re-fetched 2026-09-25 alongside today: 12 episodes returned, **1 not in state** — Masters in Business "Investing In The Great Wealth Transfer" (raw `<pubDate>` Fri, 25 Sep 2026 22:07:33 +0000 = 15:07 PT, six seconds before the previous run's state commit at 22:07:41Z, i.e. after its RSS fetch). Recovered into the 2026-09-25 vault as a late addition.

**Concurrent sibling run.** A second instance of the same 3 PM cron job was active during this run: it wrote `y-combinator-startup-podcast__robot-use-agents-why-general-purpose-models-may-win-in-robotics.md` (model `deepseek-v4-pro`, `generated_at` 2026-09-26T22:03:12Z) and its state entry **37 seconds before** this agent's own file writes. Verified by GUID before writing: no duplicate files (one file per GUID in both date directories), and the sibling's YC summary spot-checks against the episode's actual themes (RT2, code-as-policies, Voyager tool creation, Waddle Labs harness design, Platonic representation hypothesis) — kept as-is rather than re-written. Reconciliation was merge-only: no episode re-summarized, no file overwritten.

**YouTube caption block (19th consecutive day).** Every `yt-dlp --write-auto-sub` call returned `HTTP Error 429: Too Many Requests` on both videos, retried once across `--sub-lang en` and `en.*`/`--sub-format vtt`. Both videos were positively identified with **exact duration matches** before the block — MLST `yB6_iFGTq9k` (2,623 s vs RSS 2,622 s) and YC `Jv5B5CEaPJI` (1,789 s vs RSS 1,789 s) — but their captions could not be downloaded. Both episodes were therefore summarized from the publisher's structured episode notes via the `podcasters.spotify.com` link hub, consistent with the global block recorded in the 2026-09-25 report.

## Feed errors (5)

| Feed | Error | Outcome |
|---|---|---|
| Bankless | `XML: mismatched tag: line 6, column 2` (Flightcast 167-byte error) | Browser-UA fetch: HTTP 200, 22.2 MB. Newest item **ROLLUP: The Bull Market is On? \| Zcash & NEAR \| Kalshi Wash Trading \| BlackRock Goes Onchain** (Fri 25 Sep 10:30 -0000) — **already covered** in the 2026-09-25 vault; no new in-window episode. |
| Latent Space: The AI Engineer Podcast | `XML: mismatched tag: line 6, column 2` (Flightcast) | Browser-UA fetch: HTTP 200, 14.0 MB. In-window item **OpenRouter: from Seed to Stripe — with OpenRouter's Alex Atallah & AMP's Anjney Midha** (Fri 25 Sep 23:14 GMT = **16:14 PT**, after the 2026-09-25 3 PM run) — **recovered** as a late addition to the 2026-09-25 vault (`web`). |
| Chalk Radio | timeout | Browser-UA fetch: HTTP 200, 628 KB. Latest episode is 5 Mar 2026 — **no episode in window**. |
| Critics at Large \| The New Yorker | timeout | Browser-UA fetch: HTTP 200, 1.4 MB. Latest episode 24 Sep 2026 — **already covered in the 2026-09-24 vault**; no new in-window episode. |
| The Edge | timeout (Buzzsprout) | Browser-UA fetch: HTTP 200, 131 KB. Latest episode #36 (24 Sep 2026 pubDate, already covered 2026-09-24) — **no new in-window episode**. |

The browser-UA recovery sequence (parallel fetch → Chrome-UA curl) held for a fifth consecutive run: all five failing feeds returned HTTP 200 with full payloads under a browser User-Agent, and `<pubDate>` inspection settled in-window existence for every one of them with zero wasted fallback searches. This run it also **recovered one full episode** (Latent Space) that the parallel fetcher lost entirely.

## Episodes (2 for 2026-09-26)

| Show | Episode | Duration | Source |
|---|---|---|---|
| Machine Learning Street Talk (MLST) | When AI Research Starts Moving Faster Than Human Research - Zhengyao Jiang | 43m42s | web |
| Y Combinator Startup Podcast | Robot-Use Agents: Why General-Purpose Models May Win in Robotics | 29m49s | web |

## Late additions to 2026-09-25

| Show | Episode | Duration | Source | Note |
|---|---|---|---|---|
| Masters in Business | Investing In The Great Wealth Transfer: Masters in Business with Adam Frank | 1h10m54s | rss_omny_srt | Published 2026-09-25 15:07 PT — after the previous run's RSS fetch (state committed 22:07:41Z). Full Omny SRT transcript. Found via prior-date diff. |
| Latent Space: The AI Engineer Podcast | OpenRouter: from Seed to Stripe — with OpenRouter's Alex Atallah & AMP's Anjney Midha | 1h20m43s | web | Published 2026-09-25 16:14 PT — after the 3 PM run. Feed errored (Flightcast 167-byte) in both runs; recovered via browser-UA fetch today. |

## Notes

- **Zero failures**: all 3 new in-window items were summarized. The 5 feed errors produced no unprocessed episodes — one was recovered (Latent Space) and four carry no new in-window episode (Bankless already covered, Chalk Radio / Critics at Large / The Edge have no new episode).
- **Coverage caveat**: 3 of 4 items this run were summarized from publisher structured content rather than a verbatim transcript (MLST episode notes + Weco's AIDE² write-up + arXiv:2609.26457; Latent Space's public post, which does carry transcript excerpts; YC's episode page). Only Masters in Business is verbatim (Omny SRT). This is a direct consequence of the sustained YouTube 429 block.
- MLST and YC `web` sources are the `podcasters.spotify.com` link hubs, which carry the description, chapter timestamps and references; the MLST summary is additionally grounded in Weco's published AIDE² write-up and the arXiv technical report, and cites both inline.
- State updated: `last_run_date` = 2026-09-26; 2 processed entries dated 2026-09-26 and 2 late-addition entries dated 2026-09-25; `failures` unchanged at 92.
