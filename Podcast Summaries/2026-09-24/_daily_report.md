---
date: 2026-09-24
episodes_processed: 8
episodes_found_in_rss: 6
feed_fallbacks_recovered: 2
trailers_skipped: 0
late_additions: 0
generated_at: 2026-09-25T00:14:21Z
generated_by: "Hermes cron (podcast aggregation pipeline, deepseek-v4-flash manual fallback)"
---

# Podcast Summary — 2026-09-24

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-24 (Thursday) |
| Subscriptions synced | 47 (from `MTLibrary.sqlite`) |
| Feeds fetched | 47 (46 active, 1 excluded by config) |
| Feed errors | 5 |
| Episodes found in RSS scan window | 6 |
| Feed-fallback episodes recovered | 2 (Critics at Large, The Edge) |
| Episodes summarized for 2026-09-24 | 8 |
| Failures | 0 |
| Transcript coverage | 8/8 (2026-09-24) = 100% |
| Trailers/promos skipped | 0 |
| Rung 1 / Rung 2 / Rung 3 | 2 / 6 / 0 |
| Pipeline | Hermes manual fallback (Claude Code OAuth expired — 18th consecutive day) |

**Run note.** `run_podcast_pipeline.py` exited after ~75 seconds with `Failed to authenticate: OAuth session expired and could not be refreshed` (Claude Code CLI, `/Users/yuxinglin/.local/bin/claude`). Per the skill's Manual Pipeline Fallback this run was completed inline: RSS fetch, three-rung transcript ladder, batch vault generation, report and state update. 8 episodes for 2026-09-24 (≤16 threshold) made this viable.

## Feed errors (5)

| Feed | Error | Outcome |
|---|---|---|
| Bankless | `XML: mismatched tag: line 6, column 2` (Flightcast 167-byte error) | Recovered via browser-UA direct fetch: HTTP 200, 22.2 MB, 1,373 items. No in-window episode for 2026-09-24; only in-window item was 09-23 "FOMO, Meme Stocks and Robinhood Chain" — **already covered in the 2026-09-23 vault** (`bankless__fomo-meme-stocks-and-robinhood-chain-andy8052-eric-conner.md`). Deduped, no re-processing. |
| Latent Space: The AI Engineer Podcast | `XML: mismatched tag: line 6, column 2` (Flightcast) | Browser-UA fetch: HTTP 200, 13.8 MB. In-window item was 09-23 "Bio-security is an AI Arms Race (Eric Nguyen)" — **already covered in the 2026-09-23 vault**. Deduped. |
| Chalk Radio | timeout | Browser-UA fetch: HTTP 200, 628 KB. Latest episode is 5 Mar 2026 — **no episode in window**. |
| Critics at Large \| The New Yorker | timeout | Browser-UA fetch: HTTP 200, 1.4 MB. In-window episode "2026-09-24: What Survival Stories Tell Us (and Should We Listen?)" — **recovered via feed-error fallback**. |
| The Edge | timeout (Buzzsprout) | Browser-UA fetch: HTTP 200, 131 KB. In-window episode "2026-09-24: #36 Normalizing Inequality with G. Cristina Mora and Tianna S. Paschel" — **recovered via feed-error fallback**. |

The browser-UA sequence (parallel fetch → plain curl → Chrome-UA curl) held for a third consecutive run: all five failing feeds returned HTTP 200 with full payloads under a browser User-Agent, and `<pubDate>` inspection settled in-window existence for every one of them with zero wasted fallback searches.

## Transcript sources (2026-09-24)

| Source | Count | Episodes |
|---|---|---|
| `rss_omny_srt` | 1 | Odd Lots |
| `rss_vtt` | 1 | Practical AI |
| `youtube_autocaptions` | 1 | Unchained (Uneasy Money) |
| `web` | 1 | The Investor's Podcast (TIP848, via podscripts.co) |
| `show_notes` | 2 | No Priors, The Peter McCormack Show |
| `web_search_fallback` | 2 | Critics at Large, The Edge |

## Episodes

| # | Show | Episode | Source | File |
|---|---|---|---|---|
| 1 | Odd Lots | AI Is Upending the Lives of People Who Do Social Media Professionally | `rss_omny_srt` | [[odd-lots__ai-is-upending-the-lives-of-people-who-do-social-media-professionally]] |
| 2 | Practical AI | From AGENTS.md to Enterprise Deployment | `rss_vtt` | [[practical-ai__from-agents-md-to-enterprise-deployment]] |
| 3 | Unchained | Why Kalshi's Timeline Fight Put Its ETH Perp Volume Under a Microscope: Uneasy Money | `youtube_autocaptions` | [[unchained__why-kalshis-timeline-fight-put-its-eth-perp-volume-under-a-microscope-uneasy-money]] |
| 4 | The Investor's Podcast (We Study Billionaires) | TIP848: Meta (META): What the Market Misses? w/ Daniel Mahncke & Shawn O'Malley | `web` | [[the-investors-podcast-we-study-billionaires-the-investors-podcast-network__tip848-meta-meta-what-the-market-misses-w-daniel-mahncke-and-shawn-omalley]] |
| 5 | No Priors | Re-Founding Incumbents for the AI Era with Sequence Holdings Co-Founder and CEO Michael Lee | `show_notes` | [[no-priors-artificial-intelligence-technology-startups__re-founding-incumbents-for-the-ai-era-with-sequence-holdings-co-founder-and-ceo-michael-le]] |
| 6 | The Peter McCormack Show | #215 - Nick Bostrom - "We Are Creating Superintelligence": The Existential Risk is Real | `show_notes` | [[the-peter-mccormack-show__215-nick-bostrom-we-are-creating-superintelligence-the-existential-risk-is-real]] |
| 7 | Critics at Large \| The New Yorker | What Survival Stories Tell Us (and Should We Listen?) | `web_search_fallback` | [[critics-at-large-the-new-yorker__what-survival-stories-tell-us-and-should-we-listen]] |
| 8 | The Edge | #36 Normalizing Inequality with G. Cristina Mora and Tianna S. Paschel | `web_search_fallback` | [[the-edge__36-normalizing-inequality-with-g-cristina-mora-and-tianna-s-paschel]] |

## Transcript notes

- **Odd Lots** (Omny SRT, 32.9K chars): live show recorded in Hollywood, Taylor Lorenz + Rachel Carton on AI and the creator/marketing economy.
- **Practical AI** (Transistor VTT, 50.4K chars): guest "Nick" (Broadcom Tanzu/VMware) on shipping agents with cloud-native platform discipline.
- **Unchained / Uneasy Money** (YouTube auto-captions, 66.9K chars): video `li3bGJYmaxQ` "Is Kalshi's $500M Day of ETH Perp Volume Fake or Just Paid For?" — duration 4,198 s vs RSS `duration` 4,197 s (exact match). Austin Griffith's first episode as permanent third co-host.
- **TIP848** (podscripts.co transcript, 108.3K chars): Meta deep dive, Daniel Mahncke & Shawn O'Malley.
- **No Priors**: YouTube auto-captions for `TCpRwJBQvW0` were HTTP 429 rate-limited on two attempts (yt-dlp default client, then `player_client=web,mweb`). Summarized from the Apple Podcasts chapter list + episode description + press coverage of the Sequence/Bankless-adjacent transaction. Recorded as `show_notes`; no speaker quotes were invented.
- **The Peter McCormack Show**: Acast episode page carries the full timestamped running order but no verbatim transcript. Recorded as `show_notes`.
- **Critics at Large**: recovered via the episode page's official New Yorker transcript download link (`transcripts.condenastdigital.com`, 45.6K chars).
- **The Edge**: the `alumni.berkeley.edu` episode slug for #36 was not resolvable (three URL patterns returned 404), so the summary uses UC Berkeley News' direct interview feature on the same book by the same two guests, which carries verbatim quotes. Recorded as `web_search_fallback` with that dependency noted in the vault file.

## Late additions recovered into 2026-09-23

A prior-date re-fetch (`--dates "2026-09-23"`) returned 10 episodes; two GUIDs were absent from `state.json` and were processed as late additions into the **2026-09-23** vault directory:

| Show | Episode | Source | Guid |
|---|---|---|---|
| The Rest Is History | 708. The Terror: The Reign of Robespierre (Part 2) | `web` (podscripts.co full transcript) | `f3fe2076-b67c-11f1-ba50-83337ba68545` |
| 硅谷101 | E252｜硅谷睡眠外挂：富人的玩具，还是预防医疗的入口?｜对话Eight Sleep创始人 | `youtube_autocaptions` (English original interview, 42.8K chars) | `833a6b89-e6a1-4d61-a3b3-46f4d664894e` |

See the appended `## Late additions (recovered 2026-09-24)` section in `Podcast Summaries/2026-09-23/_daily_report.md`.

## Failures

None. All 8 in-window episodes for 2026-09-24 and both recovered late additions produced vault files.

## Independent verification pass (2026-09-25T00:22Z)

A scheduled Hermes cron run (`deepseek-v4-pro`) executed the same pipeline independently and reconciled against this report. Findings:

- **Window covered:** `2026-09-23T22:10:54Z` → `2026-09-24T22:05Z` (start derives from the previous run's completion timestamp rather than its date, so episodes published after the 09-23 cutoff are not lost).
- **Subscriptions re-synced** from `MTLibrary.sqlite`: 47 (unchanged).
- **Feed pass:** 9 in-window episodes on the first parallel pass; 2 Megaphone feeds (`TIFM6133783130`, `PPLLC8974708240`) failed with `ConnectionResetError` and were recovered via direct curl → **TIP848 (Meta) recovered**, the other had no in-window episode. Total 10 in-window episodes — 2 more than the 8 recorded above.
- **The 2 extra episodes** are exactly the late additions already filed into the 2026-09-23 vault (The Rest Is History #708, 硅谷101 E252). No new vault files were needed; the independent run produced no duplicates.
- **One upgrade applied:** 硅谷101 E252 was re-summarized from the **English original interview transcript** (YouTube auto-captions of the un-dubbed video `wqI7kJQEnV8`, 42.8K chars) instead of the fireside.fm show notes. Source updated to `youtube_autocaptions`; the file in `2026-09-23/` was replaced in place, and its `state.json` entry updated. The new summary is grounded in verbatim quotes from Matteo Franceschetti and Alexandra Zatarain.
- **State hygiene:** The `Critics at Large` entry used a synthetic key (`critics-at-large-2026-09-24-survival-stories`), so the real RSS guid `4902f8c2-b79e-11f1-86d2-a72bb864ff96` would not dedupe on future runs and the episode would be re-summarized. The guid is now mapped in `state.json` alongside the synthetic key.
- **Final coverage:** 10/10 in-window episodes present in `state.json` with a vault file. Transcript ladder for this window: rung 1 = 3, rung 2 = 7, rung 3 = 0 (no description fallbacks); no failures.

