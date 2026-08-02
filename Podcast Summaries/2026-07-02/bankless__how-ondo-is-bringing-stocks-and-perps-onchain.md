---
title: "How Ondo Is Bringing Stocks and Perps Onchain | Ian De Bode"
show: "Bankless"
author: "Bankless"
date: 2026-07-02
guid: web_search_fallback:bankless:2026-07-02
transcript_source: web_search_fallback
---

## Key Points

- Ondo launched a tokenized SpaceX stock on Ethereum within 5 minutes of the NASDAQ IPO on June 12, 2026, demonstrating the speed advantage of on-chain infrastructure
- 80% of Ondo's tokenized stock trading volume flows through third-party integrations (MetaMask, Binance Wallet, Uniswap, CowSwap) rather than Ondo's own platform
- Ondo uses a Request-for-Quote (RFQ) system integrated with DeFi protocols rather than traditional AMM liquidity pools; market makers pay a 5 basis point spread to cover volatility risk
- Settlement completes in approximately 1 second on Solana and up to 12 seconds on Ethereum
- CircleToken leads trading volume and is particularly popular among Chinese trading communities; Micron, NVIDIA, and Tesla also see strong demand
- Ondo Perps executes off-chain using secure enclave technology (acquired via Enclave Markets) while maintaining non-custodial on-chain collateral
- Tokenized stocks as collateral enable 100% capital efficiency on Ondo Perps because market makers can hedge synthetically on the same platform — eliminating the need to hold offsetting TradFi positions
- Ondo targets $1.5–2 billion in tokenized stock TVL by end of 2026 and plans general availability for Ondo Perps within weeks
- The episode closes with a tribute to Nathan Allman, Ondo's founder who recently passed away

## Overview

Host David Bankless speaks with Ian De Bode of Ondo about the company's mission to bring public equities and perpetual futures on-chain. De Bode explains that Ondo's tokenized stock platform is designed not as a standalone exchange but as open infrastructure: the company partners with DeFi wallets and DEXs so that users can access tokenized equities directly from their existing crypto interfaces. The RFQ model is key — rather than seeding AMM pools with liquidity (which creates inventory and volatility risk for market makers), Ondo routes each trade through competitive quotes from multiple market makers who are charged a 5 bps spread to compensate for the risk of crypto-market-hours volatility against US equity closing prices.

De Bode details the June 12 SpaceX IPO as a proof-of-concept moment: Ondo listed a tokenized SpaceX share on Ethereum within five minutes of the NASDAQ open, before any traditional brokerage had settled the first trade. The platform currently supports Solana (roughly 1-second settlement) and Ethereum (up to 12 seconds). Settlement speed is presented as a key differentiator versus traditional T+1 equity settlement.

The conversation then moves to Ondo Perps, which uses secure enclave technology — inherited from the Enclave Markets acquisition — to run an off-chain matching engine whose code cannot be unilaterally altered by Ondo or any single party. Users deposit tokenized stocks as collateral, which unlocks superior capital efficiency: a market maker hedging a synthetic long on Ondo Perps can use a tokenized short stock on the same platform, rather than maintaining separate positions in TradFi and crypto. De Bode argues this architecture gives Ondo a structural cost advantage over competing perp platforms. The episode closes with De Bode speaking about Ondo founder Nathan Allman, who passed away recently, describing him as prescient about real-world asset tokenization and committed to the mission with unusual humility.

## Implications

Ondo's infrastructure-first approach — powering wallets and DEXs rather than competing for end users directly — positions it as a settlement layer for tokenized equities rather than a retail brokerage. If this model scales, it could disintermediate traditional equity custodians for offshore investors who currently lack access to US stocks. The 5 bps spread charged to market makers is thin enough to attract volume while covering volatility risk, suggesting the economics may be viable at scale.

The Ondo Perps launch raises the prospect of a fully on-chain derivatives market for equities with institutional-grade capital efficiency. If the off-chain enclave model gains regulatory acceptance, it could attract market makers currently operating in TradFi derivatives, dramatically expanding the total addressable market for DeFi. Near-term, the biggest risk remains regulatory uncertainty around tokenized equities for non-US investors, which De Bode acknowledged is a primary constraint on geographic expansion.

## Notable Quotes

- "Every single week there is a large crypto exchange launching their own version of tokenized equities." — Ian De Bode, on competitive pressure
- "We sit on the backing of the stocks, not Hyperliquid," — Ian De Bode, explaining Ondo's unique liquidation capability
- "The real way to really do this is to tap into that tradified liquidity that already exists." — Ian De Bode, on market maker integration strategy
- "I don't see why a perp wouldn't be able to grow into the same size as the options market" over a 10-year horizon. — Ian De Bode

## People Mentioned

- Ian De Bode — Guest; Ondo
- David Bankless — Host; Bankless
- Nathan Allman — Ondo founder (recently deceased); credited with the vision for tokenized real-world assets

## Topics

tokenized equities, real-world assets, DeFi, perpetual futures, Ondo Finance, on-chain markets, Solana, Ethereum, RFQ, secure enclaves, SpaceX IPO, capital efficiency, market makers, settlement infrastructure
