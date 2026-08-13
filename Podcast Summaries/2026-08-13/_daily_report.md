# Daily Report — 2026-08-13

**Run type:** Cron pipeline (deepseek-v4-pro), single pass.

**Episodes found (RSS):** 5 | **Feed errors:** 5 | **Processed:** 5 (3 YouTube auto-captions + 1 RSS SRT + 1 show notes) | **Failures:** 0

## Transcript source breakdown
| Source | Count |
|---|---|
| youtube_autocaptions | 3 |
| rss_srt (Omny) | 1 |
| description (show notes) | 1 |

## Episodes

| Show | Episode | Source | Links |
|---|---|---|---|
| No Priors: Artificial Intelligence \| Technology \| Startups | What Chess.com Teaches US About Superhuman Capabilities, with CEO Erik Allebest | youtube_autocaptions | [[no-priors-artificial-intelligence-technology-startups__what-chess-com-teaches-us-about-superhuman-capabilities-with-ceo-erik-allebest]] |
| Odd Lots | Trucking Is Booming Again, And Drivers Aren't Happy About It | rss_srt (Omny) | [[odd-lots__trucking-is-booming-again-and-drivers-aren-t-happy-about-it]] |
| RiskReversal Pod | Sonali Basak: Building "The Bridge" Investors Can Walk Across | description (show notes) | [[riskreversal-pod__sonali-basak-building-the-bridge-investors-can-walk-across]] |
| Unchained | I Interviewed a North Korean Hacker Posing as a Crypto Dev | youtube_autocaptions | [[unchained__i-interviewed-a-north-korean-hacker-posing-as-a-crypto-dev]] |
| Y Combinator Startup Podcast | Chelsea Finn: This is the State of the Art in Robotics | youtube_autocaptions | [[y-combinator-startup-podcast__chelsea-finn-this-is-the-state-of-the-art-in-robotics]] |

## Feed errors (5)
| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast) | ⚠️ Feed unparseable; no episode in window |
| Chalk Radio | timeout | ⚠️ No episode in window (feed sporadic) |
| Critics at Large (The New Yorker) | timeout | ⚠️ No episode in window (weekly Thursdays) |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast) | ⚠️ Feed unparseable; no episode in window |
| The Edge | timeout | ⚠️ No episode in window |

## Notes
- Window: 2026-08-13T07:00:00Z .. 2026-08-14T07:00:00Z (Aug 13 PDT). Subscriptions synced from Apple Podcasts DB: 47 subscribed feeds (1 excluded by config: Sticky Notes).
- Transcripts: Odd Lots via Omny SRT from RSS (`rss_omny_srt`, 58K chars). No Priors (Erik Allebest), Unchained (North Korean hacker undercover interview), and YC (Chelsea Finn robotics) via YouTube auto-captions matched by duration. RiskReversal (Sonali Basak) had no transcript anywhere (search engines bot-blocked; no-priors.com down; Apple/Spotify/aggregators empty) → summarized from show notes (description).
- Rung 2 web search was partially degraded today: DuckDuckGo HTML served a challenge page, Bing RSS returned unrelated results, podscripts/listennotes/snipd/tapesearch are JS-rendered or 403/404. Direct host checks (Apple Podcasts episode page, riskreversal.com, podscribe) found no transcript. YouTube caption extraction worked without the usual 429 rate-limiting.
- State updated: last_run_date → 2026-08-13, +5 processed entries (total 749), 0 new failures.
