---
podcast: "Bankless"
episode: "Crypto is Ready for Onchain Options | Nick Forster, CEO of Derive"
published: 2026-09-14
duration: "45m39s"
audio_url: "https://pscrb.fm/rss/p/episode.flightcast.com/01M2FJXD36WWSVB61P3AZ9T0X9.mp3"
episode_url: "https://www.bankless.com/podcast/crypto-is-ready-for-onchain-options-nick-forster-ceo-of-derive"
transcript_source: rss_vtt
generated_at: 2026-09-14T22:05:08Z
model: "deepseek-v4-flash"
guid: "flightcast:01M2FJXD36WWSVB61P3AZ9T0X9"
---

# Crypto is Ready for Onchain Options | Nick Forster, CEO of Derive — Bankless

*(Guest: Nick Forster, co-founder and CEO of Derive. Host: David Hoffman. RSS feed errored (Flightcast 167-byte response) — episode and transcript recovered from a direct feed fetch.)*

## TL;DR

David Hoffman interviews Nick Forster, co-founder and CEO of Derive, the largest on-chain options exchange on Ethereum, about why options have lagged perpetual futures in crypto and why that is finally changing. Forster's market-structure answer is that options are always the last vertical to mature: they need anchor flows — institutions earning yield on assets, and structural hedgers like airlines hedging jet fuel — before a competitive two-way marketplace can exist. Perps, by contrast, need only two kinds of participant, which is exactly why they scaled first. Crypto's short-lived asset base made the coincidence of wants impossible to build, but higher-quality tokens with long-term holders plus tokenised RWAs and equities are now supplying it. The turning point he names is the October 10 crash, which demonstrated that perps are path-dependent enough to liquidate a well-managed delta-neutral book — and simultaneously destroyed the basis trade and the pre-TGE yield game, leaving options as the only place to earn yield at institutional scale. He closes with a concrete Derive trade showing a 66x payoff that a perp simply cannot express.

## Key points

- **The core question:** options have not grown in crypto the way you would extrapolate from TradFi. Forster's first answer is structural, not technical.
- **"Options are always the last vertical to mature."** They require anchors: big, slower-moving institutional players earning yield on their assets, plus structural hedgers (airlines hedging jet fuel, farmers hedging a crop before harvest). Those repeat flows take a long time to emerge in a new asset class.
- **Speculation alone is not enough.** Crypto speculation is dominated by perps, and options are good for speculation — but to serve it properly you need sellers writing options and creating a competitive two-way marketplace first.
- **Why many strikes is the point.** The frequent criticism that options present too many choices is, per Forster, the same reason prediction markets are taking off: granularity gives you more precise expression of an opinion, and precision means more money when you are right.
- **Perps scaled because they are simple.** A perpetual needs liquidity providers/lenders and longs/shorts — two participant types. Options require a wider variety of participants doing different things to create a "double coincidence of wants." That explains the growth gap.
- **The asset-quality constraint is lifting.** To date crypto had BTC, ETH and meme coins that last two or three weeks — not enough time for that coincidence of wants to develop. Now higher-quality tokens with longer-term, more sophisticated holder bases are emerging (HYPE produced the most recent breakout options market), and tokenised RWAs, equities and commodities are bringing large, useful assets on chain. Forster calls those two trends very good for options.
- **Technical constraints are largely solved.** Derive started with an AMM architecture in 2021 on an L2. The current model is an off-chain order book with integrated RFQ for price matching and discovery, written in Rust for low latency, with the critical on-chain pieces — self-custodial, portfolio margin, clearing and settlement — in smart contracts. Forster says the systems now compete with centralised exchanges on merit alone.
- **The incumbent's gap:** the most liquid options venue for a long time was Deribit, which built a great business but never switched up its user interface or went after an audience broader than large institutional OTC desks.
- **October 10 was the step change.** Before 10/10 there was very little interest. The crash showed that perps are very path dependent: you can be right on everything, be delta neutral, manage your risk, and still get blown out on a sharp wick. Options lose differently — you have to get timing right or lose the premium — but the bet is locked in.
- **10/10 was also the death knell for crypto's two big yield sources.** First, the basis trade: leverage was wiped and rates reset, destroying funds that had built four-year track records on 10-15% delta-neutral returns. Second, crushed token valuations killed the pre-TGE game of selling tokens, farming TVL and promising a yield in a native token hedged via OTC markets.
- **The result: options became the only place to earn yield.** Forster argues yield generation through selling options has been the only sustainable institutional-scale yield in crypto for the past year — a tailwind and a turning point.
- **The three jobs of options:** speculation, hedging (buying downside insurance), and yield generation (selling options on any asset, not just USDC).
- **The payoff asymmetry, concretely:** a Derive user bought the Ethereum March 2027 5,000/7,000 call spread for $300,000 of premium; if ETH reaches 7,000 by March it pays roughly $20 million — about a 66x return on capital with ETH at roughly $2,500.
- **Why the perp cannot express that trade.** A 66x leveraged perp position would be instantly liquidated on a negative 1.5% drawdown, and with funding at roughly 10% a year, the funding cost on the equivalent position would exceed six times the initial margin. On 10/10 people were only running 1.25x-1.5x leverage and still got liquidated, in part because of one-touch liquidations, shaky exchange liquidity and auto-deleveraging that ignores correlation hedges.
- **Where each instrument wins:** the perp is better suited to long-tail assets on short timeframes; options are better for longer timeframes where you can define your risk up front.

## Notable quotes

> "Options are always the last vertical to mature." — Nick Forster
> "There was a step change on October 10 last year, the 1010 crash... It was the death knell for two big sources of yield in crypto." — Nick Forster
> "It was the only place you could earn yield in crypto, and it has been sustainably really for the last year at institutional scale." — Nick Forster, on selling options
> "The tech is finally good enough... We're now competing with centralized exchanges just on merit alone." — Nick Forster
> "You can do everything right and be delta neutral and manage your risk and yet still get blown out on a scam wick to the downside." — Nick Forster, on the October 10 crash
> "Bankless Nation, I'm here with Nick Forster. He's the co-founder and CEO of Derive. Derive is the largest on-chain options exchange on Ethereum." — David Hoffman

## People mentioned

- Nick Forster — co-founder and CEO of Derive, guest
- David Hoffman — Bankless host
- Derive — the largest on-chain options exchange on Ethereum
- Deribit — the long-dominant centralised crypto options venue, referenced
- Pendle — referenced incidentally in the pre-TGE yield discussion (Forster explicitly notes it was not at fault)

## Topics

`crypto-options` `derivatives` `perpetual-futures` `market-structure` `onchain-trading` `rwa` `yield-generation` `basis-trade` `october-10-crash` `ethereum` `defi`
