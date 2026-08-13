# Daily Report — 2026-08-07

**Run type:** Manual pipeline fallback (Claude Code OAuth session expired — `Failed to authenticate: OAuth session expired and could not be refreshed`; pipeline exited at 15:53 PT without processing)

**Episodes found (RSS):** 12 | **Feed errors:** 5 | **Processed:** 13 (12 RSS + 1 web-search fallback) | **Skipped:** 0

## Transcript source breakdown
| Source | Count |
|---|---|
| rss_omny_srt (Bloomberg Omny) | 3 |
| youtube_autocaptions | 4 |
| web (Dwarkesh page, RiskReversal Substack, Meb Faber profile) | 3 |
| show_notes (Acast, startalkmedia.com) | 2 |
| web_search_fallback (Bankless ROLLUP) | 1 |

## Episodes

| Show | Episode | Source | Links |
|---|---|---|---|
| Dwarkesh Podcast | 8 Predictions for the Era of Continual Learning | web | [[dwarkesh-podcast__8-predictions-for-the-era-of-continual-learning]] |
| Empire | Venture Mandates, Cloudflare Wallets, Circle Earnings & Saddam Hussein Patek Philippe | youtube_autocaptions | [[empire__venture-mandates-cloudflare-wallets-circle-earnings-saddam-hussein-patek-philippe]] |
| Everybody's Business | A Gut Check on Political Spending | rss_omny_srt | [[everybodys-business__a-gut-check-on-political-spending]] |
| In Good Company with Nicolai Tangen | HIGHLIGHTS: Susan Cain | show_notes | [[in-good-company-with-nicolai-tangen__highlights-susan-cain]] |
| Masters in Business | Optimizing Life and Finances in Your Twenties with Jack Raines | rss_omny_srt | [[masters-in-business__optimizing-life-and-finances-in-your-twenties-with-jack-raines]] |
| Odd Lots | How a Sardine Gets From the Ocean to a Can | rss_omny_srt | [[odd-lots__how-a-sardine-gets-from-the-ocean-to-a-can]] |
| RiskReversal Pod | Mike Wilson: The AI Trade Has a Breaking Point, We're Just Nowhere Near It | web | [[riskreversal-pod__mike-wilson-the-ai-trade-has-a-breaking-point-were-just-nowhere-near-it]] |
| StarTalk Radio | Stars Ask Neil To Save The Universe | show_notes | [[startalk-radio__stars-ask-neil-to-save-the-universe]] |
| The Compound and Friends | The Man Who Called the Roaring 2020s with Ed Yardeni | youtube_autocaptions | [[the-compound-and-friends__the-man-who-called-the-roaring-2020s-with-ed-yardeni]] |
| The Meb Faber Show - Better Investing | Cambria Fund Profile – Cambria Global Value ETF (GVAL) | web | [[the-meb-faber-show-better-investing__cambria-fund-profile-cambria-global-value-etf-gval]] |
| Unchained | Inside the Coldcard Hack That Drained Over $100 Million in Bitcoin: Uneasy Money | youtube_autocaptions | [[unchained__inside-the-coldcard-hack-that-drained-over-100-million-in-bitcoin-uneasy-money]] |
| Y Combinator Startup Podcast | How To Design In The Agent Era | youtube_autocaptions | [[y-combinator-startup-podcast__how-to-design-in-the-agent-era]] |
| Bankless | ROLLUP: The ETH Issuance War \| $130M Coldcard Exploit \| Saylor Sells Again \| Uniswap Launchpad | web_search_fallback | [[bankless__rollup-the-eth-issuance-war-130m-coldcard-exploit-saylor-sells-again-uniswap-launchpad]] |

## Feed errors (5)

| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ✅ Recovered via web search — ROLLUP: The ETH Issuance War ($130M Coldcard Exploit) with full transcript from bankless.com |
| Chalk Radio | timeout | ⚠️ No episode found in window (feed intermittent) |
| Critics at Large (The New Yorker) | timeout | ⚠️ No episode confirmed in window (publishes Thursdays; couldn't confirm 08-06 episode within budget) |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window (latest published Aug 3) |
| The Edge | timeout | ⚠️ No episode in window (latest #35, July 7) |

## Notes
- Manual fallback executed because Claude Code auth failed (OAuth expired, non-retryable). All 12 RSS episodes + Bankless ROLLUP processed with 100% transcript coverage.
- Unchained episode is The Chopping Block content (Haseeb Qureshi, Tom Schmidt, Tarun Chitra, Robert Leshner on the Coldcard hack) published on the Unchained feed; transcript from the full YouTube episode.
- Meb Faber GVAL episode summarized from the Cambria Fund Profile Series page (same fund-profile content).
- State updated: last_run_date → 2026-08-07; 13 processed entries added. No failures.
