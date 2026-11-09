---
podcast: "Bankless"
episode: "The Race for Private Ethereum | Zac Williamson"
published: 2026-11-03
duration: "46m15s"
audio_url: "https://pscrb.fm/rss/p/episode.flightcast.com/01M6G8SSB7AK4B3BPNCHA6N7CW.mp3"
episode_url: ""
transcript_source: rss_vtt
generated_at: 2026-11-09T23:18:53Z
model: "deepseek-v4-pro"
guid: "flightcast:01M6G8SSB7AK4B3BPNCHA6N7CW"
---

# The Race for Private Ethereum | Zac Williamson — Bankless

## TL;DR
Zac Williamson, co-founder of Aztec, argues that Aztec is the only privacy-preserving blockchain outside Zcash without backdoors, viewing keys, or centralized actors, enabling genuine private smart contracts and composability. He explains how encrypted on-chain data can solve compliance for tokenized real-world assets, preserving DeFi composability, and discusses Aztec's ZK Passport, AI-driven security threats, and upcoming protocol upgrades like Aztec Tiara and faster finality.

## Key points
- Aztec positions itself as the only privacy-preserving blockchain outside of Zcash that lacks backdoors, centralized actors, or third-party auditors, enabling genuine private smart contracts with composability and no surveillance. The protocol layer has no viewing keys, but applications built on Aztec can add viewing keys at the application layer if they choose.
- Other private projects typically include viewing keys or trusted authorities for two reasons: technological difficulty (relying on third-party actors to handle zero-knowledge proofs reduces network load) and integration pressure from entities like Circle that require compliance monitoring via viewing keys. This is the path of least resistance.
- Aztec launched as a Stage 2 rollup on Ethereum, fully decentralized from day one. This is critical because if a privacy network is not fully decentralized, its centralized actors would have a responsibility to surveil and monitor the network, undermining privacy guarantees.
- Privacy will win not because individuals actively pay for it (a tragedy of the commons), but because it enables products and applications that cannot exist without it. Most TradFi tokenized assets require compliance checks on holder identity, which currently forces off-chain data custodians and sacrifices composability.
- Off-chain mega-apps like Robinhood, Hyperliquid, or Polymarket are vertically integrated and sacrifice composability because compliance credentials are not on-chain. Aztec solves this by allowing encrypted data to be stored on-chain, enabling applications to query compliance checks without revealing sensitive information or needing relationships with data custodians.
- ZK Passport is an Aztec product that performs KYC checks using zero-knowledge proofs. The user's passport data never leaves their device; only a mathematical proof (e.g., 'this person is a citizen of country X' or 'not on a sanctions list') is generated and can be used as a reusable credential inside Aztec smart contracts.
- AI-driven exploits are dramatically lowering the cost to find vulnerabilities, threatening the trusted operator model of the internet. Blockchains already live in adversarial environments and are hardening much faster. Aztec has formally verified large parts of its protocol and plans to formally verify everything, viewing this as urgent security work.
- Aztec's token powers the network, used for gas, staking for validators, and governance. Privacy adds overhead through prover entities, but the economics are being optimized. The Aztec Tiara upgrade halves proving time, brings faster transaction confirmations (soft finality of 6-12 seconds), and includes more formal verification and AI audits.
- Near-term roadmap includes formally verifying the entire protocol, enabling faster withdrawals, reducing hard finality from 40 minutes to 5 minutes (with an instant withdrawal option via tipping), improving prover speed, and achieving real-time proving to produce blocks faster.
- Aztec is initially focusing on Web3 users because they understand blockchain and are easier to onboard, while institutions will be targeted later after the network is hardened and can meet traditional expectations. Users can try zk.money, P-Inference for private AI, the Nolo wallet, and Nix DeFi.

## Notable quotes
> "The thing about Aztec is that outside of Zcash, we're the only privacy-preserving blockchain that doesn't have backdoors in them, that doesn't have centralized actors, doesn't have third-party auditors that can control you or censor you." — Zach Williamson
> "We're the only ones where you can have genuine private smart contracts, where you can have private data, private information, identities, credentials, whatever it is, and it just works out of the box. It's composable and there's no third party snooping on you and looking at your transactions." — Zach Williamson
> "If you're a privacy network, then if you're not fully decentralized, then those centralized actors have a responsibility to surveil and monitor your network." — Zach Williamson
> "Privacy is going to win because it enables products and applications that cannot exist without it." — Zach Williamson
> "You put the data on chain and you don't worry about it because it's encrypted and it's private." — Zach Williamson

## People mentioned
- Zac Williamson
- David Hoffman
- Andreas Antonopoulos

## Topics
`privacy` `zero-knowledge-proofs` `ethereum` `defi` `digital-identity` `ai-security` `blockchain`
