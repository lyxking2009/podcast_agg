---
podcast: "Bankless"
episode: "The Complete Zcash Roadmap | Sean Bowe and Dev Ojha"
published: 2026-11-02
duration: "1h38m59s"
audio_url: "https://pscrb.fm/rss/p/episode.flightcast.com/01M6DX5NMF2SHCSAT661GF2H78.mp3"
episode_url: ""
transcript_source: youtube_autocaptions
generated_at: 2026-11-09T23:17:06Z
model: "deepseek-v4-pro"
guid: "flightcast:01M6DX5NMF2SHCSAT661GF2H78"
---

# The Complete Zcash Roadmap | Sean Bowe and Dev Ojha — Bankless

## TL;DR
Zcash co-founder Sean Bowe and Dev Ojha lay out the complete Zcash roadmap: the Tachyon upgrade uses recursive SNARKs to aggregate proofs and eliminate nullifier set growth, enabling massive scale and statelessness. They detail quantum resistance pillars—quantum recoverability (now live), post-quantum privacy via Tachyon, and post-quantum soundness—along with formal verification efforts after the Orchard counterfeiting bug. The discussion also covers scalable proof-of-work to reach 100 MB/s throughput, the network sustainability module collecting 60% of transaction fees for future miners, and the philosophy of maximal privacy without backdoors.

## Key points
- Sean Bowe, Zcash co-founder and lead developer at Tachyon, has worked on Zcash since 2015, originally motivated by Bitcoin's privacy flaws and the emergence of new cryptography papers enabling private blockchains.
- Dev Ojha, co-founder of Osmosis and long-time ZK researcher, was drawn to Zcash because it is the first crypto he held and because it tackles privacy for MEV, scaling private transactions, and quantum resistance.
- Privacy is urgent in 2026 because AI and LLMs enable anyone, including teenagers, to de-anonymize transparent ledger users; public databases will quantify and track every individual's blockchain activity, making privacy essential.
- Zcash's central philosophy is ledger indistinguishability—a cryptographic property ensuring shielded transactions look like random data, preventing set intersection attacks that break weaker privacy systems—and a refusal to include backdoors or compromise privacy for regulators.
- The Zcash roadmap is organized into three broad categories: cryptography (making proofs more secure and efficient), consensus protocol (networking and scaling), and wallet UX; alternatively, high-level goals are scale, hardening, and economic sustainability.
- Tachyon is a scaling upgrade that addresses two cryptography bottlenecks: large zero-knowledge proofs and the perpetually growing nullifier set (state bloat). It uses recursive SNARKs to aggregate many proofs into one per block and compress state, making full nodes stateless for historical data.
- With Tachyon, a full node verifying a million transactions only needs to verify a single zero-knowledge proof, and state bloat is eliminated; this yields a 5x scale improvement on day one and enables discussions of massive TPS.
- The remaining scaling challenge is scalable proof-of-work: Zcash currently processes about 60 KB/s, but to reach 50k TPS with Tachyon's 2 KB transactions, the network needs 100 MB/s throughput, requiring erasure coding, improved gossip methods, and handling proof-of-work block time variance (1% of blocks may be 100x faster than average).
- Wallet scaling is uniquely hard for private chains because a wallet cannot ask a server for its balance; Zcash's solution uses private information retrieval (PIR), based on fully homomorphic encryption and optimized by GPU hardware, allowing a server to answer queries without learning which transactions belong to the user. Zcash recently used PIR in a coin holder vote, a first for any cryptocurrency project.
- To avoid centralization and tracking by RPC providers, Zcash plans a private subscription service: users prove they have an active subscription and are not exceeding request limits using the same double-spend prevention technology as private payments, without revealing identity.
- Zcash's quantum roadmap has three pillars: quantum recoverability (already live), post-quantum privacy (fully enabled by Tachyon's decoupling of wallet cryptography from the protocol), and post-quantum soundness (replacing elliptic curve proofs with post-quantum recursive STARKs).
- Quantum recoverability lets users move shielded funds into a pool that can be recovered if quantum computers appear, providing a safety buffer; the Tachyon pool will add post-quantum privacy using ML-KEM key exchange, and the community plans to ship post-quantum transparent transactions early next year with a demo of freezing elliptic curves by mid-next year.
- Zcash responded to the Orchard counterfeiting bug by formally verifying the entire shielded pool and fuzzing the formal verification itself: Zcash Security injected known classes of zero-knowledge bugs and confirmed the formal proofs caught them, achieving a higher plane of assurance than traditional audits.
- Multi-assumption security, advocated by Dev Ojha, would require an attacker to break two independent cryptographic assumptions (e.g., two different hash functions) rather than one, raising security from 128-bit to 140-160 bits; while not yet consensus, it is a long-term hardening goal once scale is achieved.
- Zcash deliberately limits expressivity to avoid feature bloat and attack surface; the two types of expression deemed worth exploring later are programmable custody methods (e.g., passkeys with alternative curves) and trustless light client bridges between chains, but only after scaling payments.
- The Network Sustainability Module (NSM), beginning with the next upgrade on November 5, collects 60% of transaction fees into a fund to redistribute to miners starting four years later, addressing the long-term security budget without changing the 21 million hard cap.
- Zcash governance is informal but moving toward coin holder voting: there is a ZIP process for specifying changes, but real governance involves multiple development orgs reaching consensus; recent large coin holder votes on retroactive grants and the next upgrade signal that coin holder opposition would be decisive.
- Both guests agree that a high ZEC price is necessary for the end goal: to sustain tens of thousands of transactions per second, you need millions of users and corresponding market cap, so price is a requirement rather than a primary motivation.

## Notable quotes
> "Every time Zcash made a technical decision, the rest of the industry copied it. This was namely Ethereum." — Dev Ojha
> "When you use a transparent ledger now, you should be actually really nervous." — Sean Bowe
> "There's a difference between something being like 99.9% private and 100% private." — Dev Ojha
> "We don't want to bend the knee on the privacy stuff. It would compromise our values." — Sean Bowe
> "Do you have better anonymity in a system that has 1,000 payments per second or one payment a week? Obviously, 1,000 payments per second." — Dev Ojha
> "I don't know who the whales are because it's private." — Sean Bowe
> "Zcash holders are the lifeblood of the project." — Sean Bowe

## People mentioned
- Sean Bowe
- Dev Ojha
- Zooko Wilcox
- Justin Drake
- Ian Miers

## Topics
`privacy` `zero-knowledge-proofs` `zcash` `quantum-resistance` `blockchain-scaling` `cryptography` `governance`
