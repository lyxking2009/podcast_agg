# Daily Podcast Report — 2026-07-26

## Summary

| Metric | Value |
|--------|-------|
| Episodes processed | 3 |
| Transcript coverage | 100% (3/3) |
| Feed errors | 5 |
| Transcript sources | 1 Omny SRT, 2 YouTube auto-captions |

## Episodes

| # | Show | Episode | Transcript | Source |
|---|------|---------|------------|--------|
| 1 | Odd Lots | How Financial Advisors Can Grow During the Great Wealth Transfer | ✅ 24K | Omny SRT |
| 2 | Lenny's Podcast | Anthropic's first technical PM on token maxing, the jagged edge, and living in the future \| Dianne Penn | ✅ 87K | YouTube auto-captions |
| 3 | The Investor's Podcast (RWH) | RWH070: Hunting For Hidden Treasures w/ Christopher Begg | ✅ 113K | YouTube auto-captions |

## Feed Errors

| Feed | Error |
|------|-------|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast intermittent) |
| Chalk Radio | timeout |
| Critics at Large \| The New Yorker | timeout |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast intermittent) |
| The Edge | timeout |

## Highlights

- **[[Odd Lots — How Financial Advisors Can Grow During the Great Wealth Transfer]]** — Sponsored episode on the $100T+ wealth transfer. Only 19% of inheritors stay with their parents' financial advisor. Panel discusses behavioral finance dynamics, gender gaps in advisory, and team-based approaches.

- **[[Lenny's Podcast — Anthropic's first technical PM on token maxing | Dianne Penn]]** — Deep dive with Anthropic's Head of Product for AI Research and Labs. Key insight: "Evals are the new PRDs" — the product management role is shifting from requirements documents to evaluation frameworks. The "token maxing" philosophy: spending $100K/year on tokens now = living like someone in 2028.

- **[[The Investor's Podcast — RWH070: Hunting For Hidden Treasures w/ Christopher Begg]]** — William Green interviews the East Coast Asset Management CIO on graph theory investing. Begg's framework: 3 pillars (moats, secular tailwinds, human element) + 8 layers of competitive advantage. Detailed analyses of Alphabet, Tesla (likely SpaceX merger within 12 months), and SpaceX/Starlink.

## Notes

- Claude Code OAuth expired — used **Manual Pipeline Fallback**
- Lenny's Podcast page is paywalled but YouTube auto-captions provided 87K chars
- TIP RWH070 transcript was login-gated on the website; YouTube auto-captions provided 113K chars
- yt-dlp installed via brew (cffi version conflict with Hermes venv resolved by using brew python directly)
