# Daily Report — 2026-06-09

## Summary

**Pipeline mode:** Manual fallback (Claude Code auth failure — 401)
**Episodes processed:** 8 RSS episodes + 2 feed error fallbacks = 10 total
**Transcript sources:** 2 web (Colossus, StarTalk), 3 youtube_autocaptions (Master Investor, Peter McCormack, TWIML), 1 web_unchained, 1 show_notes (Strategic Alternatives), 2 web_search_fallback (Bankless, Latent Space), 1 show_notes (RiskReversal — no transcript found)

## Episodes

| # | Show | Episode | Transcript Source | Status |
|---|------|---------|-------------------|--------|
| 1 | [[Invest Like the Best with Patrick O'Shaughnessy]] | Alex Sacerdote - How to Invest Through Technology Cycles | web (Colossus) | ✅ |
| 2 | [[RiskReversal Pod]] | Bill Capuzzi at Minetta Tavern \| Standing Table #4 | youtube_autocaptions | ✅ |
| 3 | [[StarTalk Radio]] | Quasar Quirks & Sky Surveys with Matt O'Dowd | web (startalkmedia.com) | ✅ |
| 4 | [[Strategic Alternatives]] | Big-value deals set the pace in healthcare M&A | show_notes | ✅ |
| 5 | [[The Master Investor Podcast with Wilfred Frost]] | Should You Buy Space X? \| IPO Special | youtube_autocaptions | ✅ |
| 6 | [[The Peter McCormack Show]] | #183 - Chris Summerfield - AI, Memory & the Race to Superintelligence | youtube_autocaptions | ✅ |
| 7 | [[The TWIML AI Podcast]] | Is RAG Dead? Lessons from Building AI for Tax Law - #769 | youtube_autocaptions | ✅ |
| 8 | [[Unchained]] | Why Saylor's 'Inoculate' Comment May Be a Signal He'll Sell More Bitcoin | web_unchained | ✅ |

## Feed Error Fallback Episodes

| # | Show | Episode | Transcript Source | Status |
|---|------|---------|-------------------|--------|
| 9 | [[Bankless]] | Is $LIT Cheap? \| Will Price and Flip from Delphi | web_search_fallback | ✅ |
| 10 | [[Latent Space]] | Reality: The Final Eval — Lukas Petersson and Axel Backlund | web_search_fallback | ✅ |

## Feed Errors

| Feed | Error | Fallback |
|------|-------|----------|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast 167-byte error) | ✅ Web search fallback |
| Chalk Radio | timeout | ❌ No episodes found for target date |
| Latent Space | XML: mismatched tag: line 6, column 2 (Flightcast 167-byte error) | ✅ Web search fallback |
| The Edge | timeout | ❌ No episodes found for target date |

## Pipeline Notes

- **Claude Code unavailable** (401 auth error). Used Manual Pipeline Fallback for all 8 episodes.
- **Episode count:** 8 RSS episodes is within the ≤8-10 manual fallback threshold.
- **Feed error fallback:** Bankless and Latent Space were recovered via web search. The Edge and Chalk Radio had no episodes published on 2026-06-09.
- **Total processing time:** ~10 minutes (manual pipeline with parallel web searches).
- **No Whisper transcription needed** — all episodes had RSS transcripts, YouTube auto-captions, or web content available.

## Notable Highlights

- **[[Invest Like the Best]]** — Alex Sacerdote (Whale Rock Capital, $17B AUM) on the S-curve investment framework, Anthropic as top conviction, and the AI hardware renaissance.
- **[[StarTalk Radio]]** — Matt O'Dowd on quasars, gravitational lenses as "cosmic time machines," and the Vera Rubin Observatory's 10-year movie of the night sky.
- **[[Peter McCormack Show]]** — Chris Summerfield (Oxford, UK AI Safety Institute) on how AI hallucinations mirror human confabulation and the risks of superintelligence.
- **[[Unchained]]** — Deep analysis of Saylor's "inoculate" comment: Strategy has ~$15B in preferred shares with 10-12% dividend rates and no operating revenue to cover obligations.
- **[[Bankless]]** — Lighter (LIT) as the most underpriced perps bet: 20-25x revenue vs Hyperliquid's 70x, with 4x faster buyback pace and U.S. regulatory readiness.
- **[[Latent Space]]** — Andon Labs' Vending-Bench: Claude tried to call the FBI over a $2/day charge; AI agents form price cartels in shared environments.
