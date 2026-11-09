---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "The Programmable Data Center — Evan Conrad, SF Compute"
published: 2026-10-20
duration: "36m43s"
audio_url: "https://podcasts.beehiiv.com/3967c1f2-cb97-4eb0-a5c0-21a1d2579408/01a15fdf-0e93-7e1a-880d-d73752a1f915/01a15fdf-6a69-7b58-8a9c-954c7b49da6f/01a15fdf-6b96-7008-b50f-08adb32aa676.mp3"
episode_url: "https://www.latent.space/p/sfcompute2"
transcript_source: rss
generated_at: 2026-10-27T22:13:45Z
model: "deepseek-v4-pro"
guid: "substack:post:221372257"
---

# The Programmable Data Center — Evan Conrad, SF Compute — Latent Space: The AI Engineer Podcast

## TL;DR
Evan Conrad returns to Latent Space to detail SF Compute's shift from subleasing a single cluster to building physical settlement infrastructure for compute markets. SF Compute operates the full cloud stack—site visits, cluster architecture, backplane management—without owning hardware, staying a neutral 'Switzerland' to avoid market manipulation; currently only 5% of capacity sits on the order book but the order book has high liquidity, with a $100 capacity purchase averaging $25 resale value. The episode covers the forward curve (with a hump), chip heterogeneity, the arbitrage between $5-6/hr B300 rental and $25-40/hr inference revenue, and why cash-settled derivatives should derive from physical settlement to avoid LIBOR-style manipulation.

## Key points
- SF Compute evolved from subleasing a single cluster (as Junark) into building full cloud operations—what they call 'physical settlement'—to reliably deliver compute; they co-designed the order book with physical constraints such as interruptibility, storage, and network topology rather than standardizing contracts prematurely.
- The compute market remains immature and illiquid overall: only 5% of SF Compute's total capacity is on the order book, but that slice has high liquidity; a key metric is that a customer spending $100 on capacity can on average make back $25 by reselling it to another party.
- Standardizing contracts is still far away; instead SF Compute built a multi-dimensional order book where buyers can specify constraints (time to start, interruptibility, storage type, network path) and match against sellers with those properties; this creates wider spreads but also arbitrage opportunities for users who can use diverse capacity.
- Contracts can span from the next hour to three-to-five years because clusters are financed over that period; pricing differs sharply by interruptibility: batch jobs may tolerate minutes of notice, while live inference customers need months, and the market is effectively for breaking leases.
- The forward curve for compute often shows a 'hump': near-term unsold capacity must drop in price to clear, then prices rise, and longer-dated compute may be cheaper because buyers prefer immediate access; however, expectations of future scarcity can invert parts of the curve.
- A large arbitrage exists: a B300 used for inference can generate about $40/hour at 100% utilization (or $25/hour at 50% utilization), while B300s can be bought at $5-6/hour, indicating one of these prices must converge.
- Different chip generations matter and cannot be reduced to a generalized compute unit because of differences in VRAM, NVLink domain size, and other features; SF Compute lets customers define SKUs like 'any Blackwell at <=$6/hour' while also supporting hyper-specific requirements for premium prices.
- SF Compute insists on physical settlement before cash settlement to avoid manipulation and basis risk; cash-settled derivatives without physical underlyings are side bets, analogous to CDS markets being 12x the underlying bonds or the LIBOR manipulation scandal.
- NVIDIA cannot build SF Compute itself because it would not be independent enough for traders, and lenders would not finance against a non-neutral market; similarly, CoreWeave, Lambda, hyperscalers, Base10, Fireworks, and Modal cannot do it due to demand-side conflicts.
- SF Compute operates as 'supercomputing as a service' but does not own hardware; it performs site visits, supply chain consulting, cluster architecture, and backplane management (PDUs, switches, DPUs, UFM, BMC) to virtualize the data center and make it programmable.
- The company has reached close to several hundred million dollars in volume, is bringing on new clusters, and recently shipped the ability for customers to subdivide their InfiniBand fabric on the fly, enabling platform-as-a-service companies to redraw cluster boundaries programmatically.
- Other ecosystem players include Silicon Data and Compute Change (cash settlement), SemiAnalysis (standards), Jump (entering compute markets), and emerging firms for residual value insurance and deposit lending; SF Compute positions itself as a partner to these rather than a competitor.
- SF Compute's pitch to potential cluster owners: if you want to own GPU capacity for residual value, SF Compute will build and operate the cluster, own the SLA, and if it becomes a bad idea, they can immediately resell the capacity to protect your balance sheet.

## Notable quotes
> "The reason why we don't own the hardware is because if we did, we would be able to manipulate the forwards market. So we have to be Switzerland, we have to be independent." — Evan Conrad
> "We're virtualizing the data center and making it programmable." — Evan Conrad
> "If I have milk and it is about to expire, I will lower the price on it. Um, and so that's basically what we see." — Evan Conrad

## People mentioned
- Evan Conrad
- Oliver

## Topics
`compute-markets` `physical-settlement` `gpu-clusters` `financialization` `programmable-data-center` `ai-infrastructure`
