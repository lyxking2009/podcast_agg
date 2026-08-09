# Daily Report — 2026-08-09

**Run type:** Manual pipeline fallback (Claude Code OAuth session expired — `Failed to authenticate: OAuth session expired and could not be refreshed`; pipeline exited ~1 min after launch without processing). Reconciled with a concurrent partial run (22:14Z) that had processed Lenny's only.

**Episodes found (RSS):** 4 | **Feed errors:** 5 | **Processed:** 4 (3 YouTube auto-captions + 1 RSS SRT) | **Skipped:** 0 | **Failures:** 0

## Transcript source breakdown
| Source | Count |
|---|---|
| youtube_autocaptions | 3 |
| rss_omny_srt | 1 |

## Episodes

| Show | Episode | Source | Links |
|---|---|---|---|
| Lenny's Podcast | The playbook for building high talent density teams \| Adam Ward, Head of Talent at Cursor | youtube_autocaptions | [[lennys-podcast-product-career-growth__the-playbook-for-building-high-talent-density-teams-adam-ward-head-of-talent-at-cursor]] |
| Odd Lots | Introducing: Our Town (trailer) | rss_omny_srt | [[odd-lots__introducing-our-town]] |
| The Investor's Podcast | RWH071: Risk, Ruin, Reinvention & Resilience w/ Victor Haghani | youtube_autocaptions | [[the-investors-podcast-we-study-billionaires-the-investors-podcast-network__rwh071-risk-ruin-reinvention-resilience-w-victor-haghani]] |
| Unchained | Could Some Vaults Trigger Securities Law? Yes, but It's Case by Case | youtube_autocaptions | [[unchained__could-some-vaults-trigger-securities-law-yes-but-its-case-by-case]] |

## Feed errors (5)

| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window — Friday ROLLUP "The ETH Issuance War" already covered 08-07; next public episode (EARLY ACCESS "The EIP That Destroys DeFi") drops Aug 10 |
| Chalk Radio | timeout | ⚠️ No episode in window (feed sporadic; nothing new in years) |
| Critics at Large (The New Yorker) | timeout | ⚠️ No episode in window (weekly Thursdays; latest Jul 30) |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window (latest Aug 3 — Databricks cofounders) |
| The Edge | timeout | ⚠️ No episode in window (latest #35, ~July) |

## Notes
- Manual fallback executed because Claude Code auth failed (OAuth expired, non-retryable). All 4 RSS episodes processed with full transcripts: 3 via YouTube auto-captions (Lenny's zegYJ6dhIg4, TIP rVFfGc-6U1Q, Unchained ASSnFypVdzk), 1 via RSS Omny SRT (Odd Lots trailer).
- Lenny's (1h30m, Adam Ward — Head of Talent at Cursor): the "funnel of doom" critique of funnel-based recruiting, the scoping → mapping → relentless pursuit playbook, the rise of the forward deployed engineer, and the return of the power IC.
- Odd Lots "Introducing: Our Town" (1m53s): trailer for Bloomberg News / The Big Take's new documentary series about right-wing-linked developers buying up land in Gainsboro, Tennessee. Summarized from the RSS SRT (a concurrent partial run had logged it as a failure; reconciled — vault file exists, stale failure entry removed).
- TIP RWH071 (2h, Victor Haghani — Elm Wealth, LTCM co-founder): Salomon Brothers arbitrage desk, Liars Poker and edge-based betting, the founding and 1998 collapse of LTCM, position sizing, low-probability high-consequence events, and lessons from "The Missing Billionaires".
- Unchained (34m, Sun Ragupathi — Veda CEO): how onchain vaults work ($67B TVL), the three layers of risk (smart contract / operational / economic), SEC Commissioner Hester Peirce's Howey-test statement on vaults, and the Kraken DeFi Earn partnership ($600M+ across 80K+ users).
- Concurrent partial run (22:14Z) saw only 2 episodes / 0 feed errors and processed Lenny's; this run's fetch saw 4 episodes / 5 feed errors (transient Megaphone/Flightcast variability). Reconciliation: duplicate Lenny's vault file removed, state merged (3 additional processed entries), stale trailer failure removed.
- State updated: last_run_date → 2026-08-09; 4 processed entries; 0 new failures (1 stale removed).
