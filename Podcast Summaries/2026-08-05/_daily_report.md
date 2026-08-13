---
title: "Daily Podcast Report — 2026-08-05"
date: 2026-08-05
---

# 📻 Daily Podcast Report — 2026-08-05

## Summary

| Metric | Value |
|--------|-------|
| Episodes processed (published 2026-08-05) | 7 (RSS) + 2 (web-search fallback) |
| Successes | 5 RSS + 2 fallback = 7 |
| Failures | 2 (The Rest Is History, Y Combinator Startup Podcast — Data Centers in Space) |
| Feed errors (catch-up window 2026-08-04/08-05) | 5 (Bankless, Chalk Radio, Critics at Large, Latent Space, The Edge) |
| Sources used | 5 youtube_autocaptions, 1 rss_omny_srt, 2 web_search_fallback |

Run window: 2026-08-04 and 2026-08-05 (17 RSS episodes across both dates + 2 web-search-fallback episodes). See [[../2026-08-04/_daily_report|2026-08-04 report]] for that date's 8 episodes.

## Episodes

### 1. [[animal-spirits-podcast__when-genius-failed-again-ep-476|When Genius Failed, Again (EP. 476)]]
**Animal Spirits Podcast** · youtube_autocaptions

Michael Batnick and Ben Carlson unpack the collapse of Leopold Aschenbrenner's Situational Awareness fund — from $9.3B to a $45B peak on 4x leverage, then a forced Citadel liquidation — alongside South Korea's 40% KOSPI crash and a Guggenheim/Mark Walter fraud probe.

### 2. [[big-technology-podcast__how-the-ai-bet-pays-off-ai-lab-strategy-game-with-david-cahn|How The AI Bet Pays Off + AI Lab Strategy Game — With David Cahn]]
**Big Technology Podcast** · youtube_autocaptions

Sequoia's David Cahn updates his AI CapEx math to ~$3 trillion since ChatGPT and argues "nothing short of AGI" justifies it, then rates every major AI lab's resource-allocation strategy as a game — from Anthropic's talent-hoarding to Microsoft's two-way hedge via its OpenAI stake.

### 3. [[in-good-company-with-nicolai-tangen__susan-cain-why-introverts-make-great-leaders-staying-human-in-the-ai-age-and-the|Susan Cain: Why Introverts Make Great Leaders, Staying Human in the AI Age and the Bittersweet Life]]
**In Good Company with Nicolai Tangen** · youtube_autocaptions

Susan Cain tells Nicolai Tangen that "highly sensitive people" will have an authenticity edge in the AI era, cites Jim Collins's finding that all 11 top Good to Great CEOs were quiet and unassuming, and explains how she precisely calibrates solitude versus socializing to avoid burnout.

### 4. [[masters-in-business__bonus-future-standard-president-cio-mike-kelly|BONUS: Future Standard President & CIO Mike Kelly]]
**Masters in Business** · rss_omny_srt

Mike Kelly traces his path from cold-calling Lee Cooperman to leading Future Standard's transformation into a $90B alternatives platform, argues private credit isn't a bubble, and predicts the term "alternatives" will eventually disappear in favor of a growth/income/real-assets framework.

### 5. [[riskreversal-pod__the-ai-fomo-feeding-frenzy-with-amish-jani-of-firstmark-capital|The AI "FOMO Feeding Frenzy" with Amish Jani of FirstMark Capital]]
**RiskReversal Pod** · youtube_autocaptions

FirstMark's Amish Jani calls the current AI shift the fastest, largest-magnitude change of his 20-year VC career, flags architectural disruption and circular chip financing as real risks, and predicts an Anthropic/OpenAI IPO will end the private-market "FOMO feeding frenzy."

### 6. [[bankless__early-access-we-want-to-be-bigger-than-the-cme-kalshi-s-john-wang|EARLY ACCESS - "We Want to Be Bigger Than the CME" | Kalshi's John Wang]]
**Bankless** · web_search_fallback

*(Recovered via feed-error fallback — Bankless's Flightcast RSS feed failed.)* Kalshi's John Wang details the exchange's regulated-onshore strategy versus Polymarket, its new perpetual futures product, and its ambition to become the world's largest exchange.

### 7. [[bankless__ansem-robinhood-chain-why-sol-still-beats-eth-in-2026-mike-dudas|$ANSEM, Robinhood Chain, & Why SOL Still Beats ETH in 2026 | Mike Dudas]]
**Bankless** · web_search_fallback

*(Recovered via feed-error fallback — Bankless's Flightcast RSS feed failed.)* Six Man Ventures' Mike Dudas traces meme coins back to 2022's Bonk airdrop, argues Robinhood Chain's timing beat Base's, and explains why his fund is structurally bullish on SOL over ETH for the next five years.

## Failures (2)

| Episode | Show | Reason |
|---------|------|--------|
| 694. Elizabeth I vs The Catholics: A Treacherous Conspiracy (Part 4) | The Rest Is History | No public transcript found; the only YouTube upload is members-only early access content, blocked for both youtube-transcript-api and yt-dlp. |
| Building the First Data Centers in Space | Y Combinator Startup Podcast | Episode too recent to be indexed by transcript sites or web search. Matching YouTube cross-post found but auto-caption download was persistently blocked by HTTP 429 rate-limiting across multiple retries/clients. |

## Feed Errors (5)

Web search fallback was run for each show below against the combined window (2026-08-04..2026-08-05).

| Feed | Error | Fallback Result |
|------|-------|------------------|
| Bankless | XML: mismatched tag (Flightcast intermittent) | **2 episodes recovered**, both published 2026-08-05: "We Want to Be Bigger Than the CME" (Kalshi's John Wang) and "$ANSEM, Robinhood Chain, & Why SOL Still Beats ETH in 2026" (Mike Dudas) — both summarized above with full web transcripts. |
| Chalk Radio | timeout | No episode published on 2026-08-04 or 2026-08-05 — latest episode found was 2026-03-05 (low, irregular cadence). |
| Critics at Large \| The New Yorker | timeout | No episode published on 2026-08-04 or 2026-08-05 — show publishes weekly on Thursdays; nearest episodes were 2026-07-30 and 2026-08-06 (subscriber early access), neither landing in the window. |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag (Flightcast intermittent) | No audio episode published on 2026-08-04 or 2026-08-05 — an Aug 4 Substack post was written-only (no audio). Nearest actual audio episode was 2026-08-03. |
| The Edge | timeout | No episode published on 2026-08-04 or 2026-08-05 — latest was Episode 35, 2026-07-07 (low, irregular cadence). |

## Notes

- Run covered 17 RSS episodes across 2026-08-04 and 2026-08-05 (13 successes, 4 failures) plus 2 web-search-fallback episodes from Bankless (both successes) — 19 total processed, 15 successes / 4 failures.
- Masters in Business again delivered a fast, reliable RSS `<podcast:transcript>` SRT transcript via Omny.
- YouTube auto-captions remained the dominant successful rung across both dates; two shows (Training Data, 硅谷101) were recovered via full written web transcripts instead.
- All four failures were transcript-availability issues (paywall, members-only content, or rate-limiting on very recently published episodes) rather than content problems — worth a backfill retry in a day or two once YouTube uploads/rate limits clear.
