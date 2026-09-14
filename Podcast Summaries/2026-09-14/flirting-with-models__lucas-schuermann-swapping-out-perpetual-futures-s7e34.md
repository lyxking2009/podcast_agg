---
podcast: "Flirting with Models"
episode: "Lucas Schuermann – Swapping Out Perpetual Futures (S7E34)"
published: 2026-09-14
duration: "1h14m01s"
audio_url: "https://episodes.captivate.fm/episode/4a276c86-30f8-41ad-afce-5c3cafab293c.mp3"
episode_url: "https://www.flirtingwithmodels.com"
transcript_source: show_notes
generated_at: 2026-09-14T22:04:50Z
model: "deepseek-v4-flash"
guid: "4a276c86-30f8-41ad-afce-5c3cafab293c"
---

# Lucas Schuermann – Swapping Out Perpetual Futures (S7E34) — Flirting with Models

*(Guest: Lucas Schuermann, co-founder of Variational. Host: Corey Hoffstein. Source: the episode's full structured show notes — Variational is a follow-up to Corey's earlier conversation with co-founder Edward Yu.)*

## TL;DR

Corey Hoffstein's return visit to Variational, this time with co-founder Lucas Schuermann. The core subject is market structure: Variational's retail venue Omni does not run a central limit order book. Every trade is quoted by a single internal liquidity provider — the OLP — through a request-for-quote model that segregates flow. Because the OLP knows who it is trading with, it can treat retail flow as non-toxic: netting offsetting positions against each other, warehousing what it chooses, and hedging only the residual externally. That is what let Variational list a long tail of crypto-native assets at competitive spreads even where on-platform open interest was thin. The latter half of the episode is the more interesting structural claim: as demand for on-chain real-world-asset exposure grows, Variational has both widened where hedges can go — a global dealer network rather than rebuilt order-book depth — and started questioning whether the perpetual future is the right instrument at all. Perp funding is set by where the contract trades versus its index, which makes it volatile and hard to forecast; Variational's answer is a swap with a price-return leg plus an explicit financing leg priced off short rates, giving a far more predictable cost of carry for holding levered exposure long term. There is currently over a billion dollars of dealer capacity behind it.

## Key points

- **The business:** Variational seeks to bring the trillion-dollar OTC derivatives market on chain, with a retail-facing platform called Omni.
- **Not a central limit order book.** On a CLOB a crowd of competing market makers quotes against unknown flow. On Omni, every trade is quoted by a single internal liquidity provider (the OLP) through a request-for-quote model that *segregates* flow.
- **Why segregation matters:** because the OLP knows who it is trading with, it can price retail flow as non-toxic. It nets offsetting positions against each other, warehouses what it chooses to keep, and hedges only the residual risk externally.
- **The consequence is breadth.** The RFQ/OLP model let Variational list a long tail of crypto-native assets at competitive spreads even where on-platform open interest remained thin.
- **First pressure from real-world-asset demand — hedging.** Rather than trying to rebuild forty years of traditional market depth on a crypto order book, Variational built a global network of dealer relationships to tap into that depth directly.
- **Second pressure — is the perp even the right instrument?** Perp funding is driven by where the contract trades relative to its index, which makes it volatile and hard to forecast.
- **Variational's answer: a swap.** A price-return leg plus an explicit financing leg, priced off short rates — a far more predictable cost of carry for anyone who wants to hold levered exposure for the long haul.
- **Scale of the hedge capacity:** there is currently over a billion dollars of dealer capacity behind the swap product.
- **Continuity of thesis:** Corey's prior Variational episode was with co-founder Edward Yu a little over a year earlier; the mission has not changed but the structure has evolved substantially.

## People mentioned

- Lucas Schuermann — co-founder of Variational, guest
- Corey Hoffstein — host, CIO of Newfound Research
- Edward Yu — Variational co-founder, guest on Corey's earlier Variational episode
- Variational — on-chain OTC derivatives venue; operator of the Omni retail platform and the OLP liquidity provider

## Topics

`market-structure` `perpetual-futures` `rfq` `liquidity-provision` `funding-rates` `otc-derivatives` `on-chain-derivatives` `real-world-assets` `basis-trade` `quantitative-investing`
