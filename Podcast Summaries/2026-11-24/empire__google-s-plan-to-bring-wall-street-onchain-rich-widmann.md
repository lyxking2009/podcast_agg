---
podcast: "Empire"
episode: "Google’s Plan to Bring Wall Street Onchain | Rich Widmann"
published: 2026-11-24
duration: "1h11m7s"
audio_url: "https://chrt.fm/track/713ACD/traffic.megaphone.fm/BWG1210144624.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-11-24T23:04:19Z
model: "deepseek-v4-pro"
guid: "a34549b8-e370-11f1-939b-ff49e9f6f462"
---

# Google’s Plan to Bring Wall Street Onchain | Rich Widmann — Empire

## TL;DR
Rich Widmann of Google's digital assets team explains Google's enterprise-focused crypto strategy, which started with indexing Bitcoin and Ethereum data and running validator nodes, then evolved into building GCL, a blockchain for regulated financial institutions, and Google MPC, a custody infrastructure product. He details how GCL targets tokenized deposits, integrates with core banking providers like Jack Henry and CME, and discusses quantum computing threats, agentic payments, and why Google avoided retail crypto.

## Key points
- Google began its digital asset work in 2018 by indexing Bitcoin and Ethereum into public datasets, which became the most popular datasets in its free public data program, even before a formal team existed.
- The formal digital assets team was stood up within Google Cloud around 2020-2021, focusing on B2B infrastructure such as nodes, staking, and data rather than retail crypto products like Bitcoin in Google Pay.
- Google learned from running nodes and staking on major L1s that many blockchains had scaling limitations, often requiring high-end hardware or specific conditions, and that network effects—not necessarily the best technology—dominated L1 competition.
- This led to the development of GCL, a blockchain built from scratch to rival top L1s while playing nicely with business logic and tech stacks of regulated financial institutions, using Python-based smart contracts (a variant called G-copy).
- GCL was announced at the end of 2024 and is now in GA, with early adopters including CME for 24/7 clearing and settlement, and Jack Henry, a core banking provider, enabling regional banks to access blockchain capabilities through their existing core infrastructure.
- GCL focuses on tokenized deposits rather than stablecoins for banks, because deposits at OCC-chartered banks hold special regulatory status, and tokenized deposits offer a bank-friendly evolution without disintermediation or deposit flight.
- Google's business model for GCL includes transaction fees and API call charges, similar to Google Cloud services, and GCL is designed to be cloud-agnostic, though hosted primarily on Google Cloud.
- Google also launched an MPC (multi-party computation) custody product in 2025, built on Google's existing KMS, TEEs, and an open-source DKLS23 library, offering key sharding without stitching together third-party components, aiming to be infrastructure for custodians rather than a direct custodian competitor.
- Rich Widmann argues that permissioned chains like GCL, Canton, and Tempo reinstantiate enterprise trust and compliance between counterparties, while permissionless chains remain essential for peer-to-peer digital cash and bearer instruments.
- He sees the 'everything app' trend (exchanges adding equities, brokerages adding crypto) driven by blockchain's shared ledger enabling fungibility between tokenized dollars, stocks, bets, and other assets, making cross-asset trading pairs possible.
- Agentic commerce is a major future use case: programmable money on blockchains can handle payments for AI agents, and blockchains uniquely allow data and value to move together, creating audit trails for agent transactions and reducing fraud disputes.
- Google's quantum team published findings that elliptic curve cryptography (securing Bitcoin and Ethereum) could be broken by the end of the decade, earlier than previously thought; this influenced Google's MPC product design to incorporate post-quantum cryptography.
- The major challenge for quantum migration is collective action: agreeing on a new algorithm and migrating wallets, especially lost or Satoshi's wallets, with potential incentivization regimes to recover locked coins before malicious actors.
- Rich Widmann emphasizes that Google's approach has been deliberately quiet and long-term, with organizational stamina framed as a 5-10 year strategy, avoiding short-term bets that killed many other corporate digital asset initiatives.

## Notable quotes
> "I don't think the market understands all the cool things Google is doing in digital assets." — Host
> "I think the other assumption was because we weren't screaming from the hilltops about everything that we were doing, we weren't doing anything." — Rich Widmann
> "This is less about disintermediating my bank. This is less about deposit flight. This is less about find, you know, transformation of my business and it's more about evolution." — Rich Widmann
> "His view on the market was like basically it's you know he views crypto as like oil in which case like the way you win is oil just continues to go up in price." — Rich Widmann

## People mentioned
- Rich Widmann
- Steve Lee
- James Tromans
- Satoshi Nakamoto

## Topics
`digital-assets` `google-cloud` `blockchain-infrastructure` `tokenized-deposits` `custody-technology` `quantum-resistance` `agentic-payments`
