# Podcast Backfill Report — 2026-05-23

> These episodes were backfilled after the Flightcast gzip XML parse bug was fixed. The bug affected `rss.flightcast.com` feeds and prevented all Flightcast-hosted episodes from being parsed. Bankless and Latent Space both host on Flightcast. Episodes span 2026-05-14 through 2026-05-22.

| Metric | Count |
|---|---|
| Episodes backfilled | 8 |
| Successes | 8 |
| Failures | 0 |
| Bankless episodes | 4 |
| Latent Space episodes | 4 |
| Full transcript obtained | 4 (Latent Space via Substack) |
| Web search summary only | 4 (Bankless — premium-gated) |

## Summarized Episodes

### Bankless

| Date | Episode | File |
|---|---|---|
| 2026-05-22 | ROLLUP: David Sold His ETH \| EF Exodus \| Hyperliquid's Breakout \| Stagflation Fears | [[bankless__rollup-david-sold-his-eth-ef-exodus-hyperliquid-stagflation]] |
| 2026-05-21 | Bitcoin's $300T Credit Market Opportunity \| Jeff Walton | [[bankless__bitcoin-300t-credit-market-jeff-walton]] |
| 2026-05-19 | "Crypto Without Privacy Isn't Crypto" - The Zcash Bull Case \| Tushar Jain & Mert Mumtaz | [[bankless__zcash-bull-case-tushar-jain-mert-mumtaz]] |
| 2026-05-18 | Clarity Act Odds Jump to 75% After Surprise Senate Vote \| Alex Thorn | [[bankless__clarity-act-75pct-senate-vote-alex-thorn]] |

### Latent Space

| Date | Episode | File |
|---|---|---|
| 2026-05-21 | Giving Agents Computers — Ivan Burazin, Daytona | [[latent-space__giving-agents-computers-ivan-burazin-daytona]] |
| 2026-05-20 | Railway: The Agent-Native Cloud — Jake Cooper | [[latent-space__railway-agent-native-cloud-jake-cooper]] |
| 2026-05-18 | The Autonomous Drone Tech Stack & Economics of Drones — Yaroslav Azhnyuk | [[latent-space__autonomous-drone-tech-stack-yaroslav-azhnyuk]] |
| 2026-05-14 | AI-Native Healthcare: 100M Doctor Visits, 10–20 Hours Saved, Prior Auth in Minutes — Abridge | [[latent-space__ai-native-healthcare-abridge-janie-lee-chai-asawa]] |

## Transcript Sources

- **Latent Space** (4 episodes): Full content fetched from public Substack pages (`latent.space/p/{slug}`). No paywall encountered.
- **Bankless** (4 episodes): Premium-gated on bankless.com. Web search used to reconstruct summaries from news coverage, X/Twitter threads, and show notes. Summaries marked `⚠️ Summary based on web search only`.

## Key Themes This Period

- **Ethereum Foundation crisis**: David Hoffman sold all ETH, 8+ EF senior departures, Bankless leadership transition — the Ethereum advocacy landscape is in flux.
- **Bitcoin financial layer**: Jeff Walton / Strive making the case that Bitcoin-backed credit products can disrupt the $300T fixed-income market.
- **CLARITY Act momentum**: Alex Thorn raised passage odds to 75% after the surprise May 14 Senate committee vote; August presidential signature now the working assumption.
- **Privacy renaissance**: Multicoin built a significant ZEC position; Zcash framed as the forgotten cypherpunk asset for private wealth storage.
- **Agent infrastructure**: Both Railway and Daytona independently converging on bare metal ownership, stateful sandboxing, and the claim that agent workloads are structurally different from human developer workloads.
- **Defense AI**: Yaroslav Azhnyuk (The Fourth Law) on autonomous drone tech stack — terminal guidance alone 3.5x'd mission success rates; 70–80% of Ukraine casualties now from FPV drones.
- **Healthcare AI moat**: Abridge's three-act strategy (save time → save money → save lives) and 100M+ medical conversation dataset as the primary defensible advantage.

## Infrastructure Note

Episodes were missing from the pipeline due to a gzip decompression bug in `scripts/fetch_episodes.py`. The bug affected Flightcast RSS feeds, which serve responses with `Content-Encoding: gzip` that the old parser did not handle. Fix: detect content-encoding and decompress before parsing. All GUIDs in this report have been added to `data/state.json` as processed.
