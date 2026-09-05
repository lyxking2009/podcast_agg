# Podcast Summaries — 2026-08-17

## Episodes (5)
| Show | Episode | Transcript source | Vault |
|---|---|---|---|
| Odd Lots | What the OpenAI-Hugging Face Hack Really Tells Us About AI Danger | rss_omny_srt | [[odd-lots__what-the-openai-hugging-face-hack-really-tells-us-about-ai-danger]] |
| Empire | Uniswap Is Building The Liquidity Network For Everything \| Hayden Adams | youtube_autocaptions | [[empire__uniswap-is-building-the-liquidity-network-for-everything-hayden-adams]] |
| RiskReversal Pod | Peter Boockvar: Inflation Is The Core Disease | web_substack | [[riskreversal-pod__peter-boockvar-inflation-is-the-core-disease]] |
| Animal Spirits Podcast | Talk Your Book: The Most Important Thing Nobody Owns | web | [[animal-spirits-podcast__talk-your-book-the-most-important-thing-nobody-owns]] |
| The Peterman Pod | Creator of TypeScript: 10x Faster Typescript, Why AI Won't Replace SWEs \| Anders Hejlsberg | youtube_autocaptions | [[the-peterman-pod__creator-of-typescript-10x-faster-typescript-why-ai-wont-replace-swes-anders-hejlsberg]] |

## Feed errors (5)
| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ Direct feed fetch found in-window EARLY ACCESS episode: "Why Raising Rates Would Actually Calm Markets" (Jim Bianco, 08-17). Premium-only — no public transcript / full YouTube upload → logged as failure (same pattern as 08-14 Jim Bianco). |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window — direct feed fetch: 219 items, latest is "The BioAI Phase Shift" (Chai Discovery, Aug 11, outside window) |
| Chalk Radio | timeout | ⚠️ Not re-searched (manual-mode budget; feed sporadic) |
| Critics at Large (The New Yorker) | timeout | ⚠️ Not re-searched (manual-mode budget; weekly Thursdays) |
| The Edge | timeout | ⚠️ Not re-searched (manual-mode budget; latest #35, ~July) |

## Notes
- Mode: Claude Code OAuth expired (10th consecutive day, 08-08 → 08-17) → Manual Pipeline Fallback, 5/5 episodes, 100% coverage.
- Monday run — 5 episodes is a normal weekday count.
- Prior-date diff (08-16): re-fetch found 3 episodes for 08-16; 2 already in state; **recovered late-published The Rest Is History 697 "The Troubles: Murder in Belfast (Part 1)"** into the 08-16 vault as a late addition (show_notes; Rest Is History publishes description-only pages).
- Transcript sources: Odd Lots via Omny SRT (Rung 1); Empire + Peterman via YouTube auto-captions (Peterman video ID cywK3XYYJ2o via podcasters.spotify.com link hub; Empire 9d7wamjeHtU, duration matched RSS 4561s); RiskReversal via Substack structured summary; Animal Spirits via awealthofcommonsense.com show-notes blog (Talk Your Book w/ Steve Schoffstall, Sprott ETFs).
- Feed errors verified via direct-curl recovery: Latent Space definitively no in-window episode; Bankless definitively has today's Jim Bianco EARLY ACCESS (premium, no public transcript).
- Pipeline's internal re-fetch reported 8 feed errors vs. standalone fetch's 5 (known count variability across runs — Megaphone transient timeouts); all 5 episodes' feeds succeeded.
- Pending backfill candidate (unchanged): Latent Space "The BioAI Phase Shift" (Chai Discovery, 08-11) — not in state.processed.
- State updated: last_run_date → 2026-08-17, +5 processed, +1 failure (Bankless Jim Bianco EARLY ACCESS, no_transcript_found).
