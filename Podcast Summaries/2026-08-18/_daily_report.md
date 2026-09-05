# Podcast Summaries — 2026-08-18

## Episodes (4)
| Show | Episode | Transcript source | Vault |
|---|---|---|---|
| Invest Like the Best | Ben Thompson on Big Tech, China, and the AI Boom Running Out of Money (EP.487) | youtube_autocaptions | [[invest-like-the-best__ben-thompson-on-big-tech-china-and-the-ai-boom-running-out-of-money]] |
| The Peter McCormack Show | #202 - Barry S. Strauss - "We Are In The Final Stages Of Western Collapse" | youtube_autocaptions | [[the-peter-mccormack-show__202-barry-s-strauss-we-are-in-the-final-stages-of-western-collapse]] |
| StarTalk Radio | It's Bugs' World and We're Just Living In It with Jessica Ware | show_notes | [[startalk-radio__its-bugs-world-and-were-just-living-in-it-with-jessica-ware]] |
| Training Data | Rich Sutton and Khurram Javed: Why AI Models Stop Learning, and How to Start It Again | web | [[training-data__rich-sutton-and-khurram-javed-why-ai-models-stop-learning-and-how-to-start-it-again]] |

## Feed errors (5)
| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ✅ No new episode in window — direct feed fetch (21.9MB, 1360 items): latest is 08-17 "Why Raising Rates Would Actually Calm Markets" (Jim Bianco, EARLY ACCESS) — already logged as premium no_transcript_found failure on 08-17; no 08-18 publication. |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ✅ No episode in window — direct feed fetch (13.1MB, 219 items): latest is "The BioAI Phase Shift" (Chai Discovery, Aug 11, outside window). |
| Chalk Radio | timeout | ⚠️ Not re-searched (manual-mode budget; feed sporadic) |
| Critics at Large (The New Yorker) | timeout | ⚠️ Not re-searched (manual-mode budget; weekly Thursdays) |
| The Edge | timeout | ⚠️ Not re-searched (manual-mode budget; latest #35, ~July) |

## Notes
- Mode: Claude Code OAuth expired (**11th consecutive day**, 08-08 → 08-18; pre-check confirmed "OAuth session expired") → Manual Pipeline Fallback, 4/4 episodes, 100% coverage.
- Tuesday run — 4 episodes is a normal weekday count.
- Prior-date diff (08-17): re-fetch found 5 episodes for 08-17, ALL already in state.processed → no late additions needed (clean diff).
- Transcript sources: ILTB EP.487 via YouTube auto-captions (video h-0NZ-oIjlk, linked directly from the Colossus episode page; YT title "What Happens When the AI Boom Runs Out of Money"); McCormack #202 via YouTube auto-captions (o0jBSkFELxI, 4525s ≈ RSS 1:15:17; published ~4h before run); StarTalk S17E49 via official show-notes page (Patreon-gated transcript, no same-day YouTube upload — channel listing checked); Training Data via BigGo Finance structured summary (web, verbatim quotes — same pattern as 08-13 No Priors).
- yt-dlp first subtitle-download attempt silently produced no files (output was piped through grep); re-run without the pipe downloaded both VTTs fine (832KB + 674KB — full-episode sizes, not clips).
- Feed errors verified via direct-curl recovery: both Flightcast feeds fetched directly and parsed by regex — Bankless definitively has no 08-18 episode; Latent Space definitively no in-window episode.
- State updated: last_run_date → 2026-08-18, +4 processed, no new failures.

## Backlog
- Pending backfill candidate (unchanged): Latent Space "The BioAI Phase Shift" (Chai Discovery, 08-11) — not in state.processed.
