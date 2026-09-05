# Podcast Summaries — 2026-08-16

## Episodes (2)
| Show | Episode | Transcript source | Vault |
|---|---|---|---|
| Lenny's Podcast | OpenAI's Head of Design: This is the best time in history to be a designer \| Ian Silber | youtube_autocaptions | [[lennys-podcast-product-career-growth__openais-head-of-design-this-is-the-best-time-in-history-to-be-a-designer-ian-silber]] |
| The Investor's Podcast (WSB) | TIP838: Google, Reddit, Amazon – Are Our Biggest Winners Still a Buy? w/ Daniel Mahncke & Shawn O'Malley | show_notes | [[the-investors-podcast-we-study-billionaires-the-investors-podcast-network__tip838-google-reddit-amazon-are-our-biggest-winners-still-a-buy-w-daniel-mahncke-shawn-omalley]] |

## Feed errors (5)
| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window — direct feed fetch: latest is "EtherFi's Next Act" (Mike Silagadze, Aug 14), already covered in the 08-14 vault |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window — direct feed fetch: latest is "The BioAI Phase Shift" (Chai Discovery, Aug 11, outside window) |
| Chalk Radio | timeout | ⚠️ Not re-searched (manual-mode budget; feed sporadic) |
| Critics at Large (The New Yorker) | timeout | ⚠️ Not re-searched (manual-mode budget; weekly Thursdays) |
| The Edge | timeout | ⚠️ Not re-searched (manual-mode budget; latest #35, ~July) |

## Notes
- Mode: Claude Code OAuth expired (9th consecutive day, 08-08 → 08-16) → Manual Pipeline Fallback, 2/2 episodes, 100% coverage.
- Sunday run — most shows don't publish weekends, so 2 episodes is expected (normal weekend pattern).
- Prior-date diff (08-15): re-fetched 0 episodes for 08-15 — clean; no late-published episodes to recover.
- Transcript sources: Lenny's via YouTube auto-captions (video BV0hy6NET-U, full 72-min transcript); TIP838 via public show notes + opening excerpt (full transcript login-gated, per the known TIP pattern).
- Flightcast feeds verified via direct-curl recovery — definitively no in-window episodes (Bankless latest 08-14; Latent Space latest 08-11).
- Pending backfill candidate (unchanged from 08-15): Latent Space "The BioAI Phase Shift" (Chai Discovery, 08-11) — not in state.processed.
- State updated: last_run_date → 2026-08-16, +2 processed, 0 new failures.

## Late additions (recovered 2026-08-17)
| Show | Episode | Transcript source | Vault |
|---|---|---|---|
| The Rest Is History | 697. The Troubles: Murder in Belfast (Part 1) | show_notes | [[the-rest-is-history__697-the-troubles-murder-in-belfast-part-1]] |

- Late-published episode: published 08-16 after the prior run's fetch (prior run completed 22:11Z; this episode was not in state.processed). Recovered via prior-date RSS re-fetch + episode page show notes (Rest Is History publishes description-only pages — no full transcript exists).
