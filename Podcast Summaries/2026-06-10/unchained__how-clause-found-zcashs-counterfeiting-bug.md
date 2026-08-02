---
title: "How Claude Found Zcash's Counterfeiting Bug"
show: Unchained
date: 2026-06-10
duration: "15 min"
transcript_source: web
---

# How Claude Found Zcash's Counterfeiting Bug

## Key Points

- **Critical soundness bug in Orchard shielded pool** zero-knowledge proof circuit, present since May 2022.
- **Risk**: Could have enabled unlimited counterfeit ZEC creation via double-spending within the Orchard pool. The turnstile mechanism protected the total ZEC supply cap (21M).
- **No evidence of mainnet exploitation**; vulnerability fully patched between May 29 and June 3, 2026.
- **ZEC price dropped >30%** on June 5, falling from ~$457 to ~$40 post-disclosure.
- **Discovered by Taylor Hornby** using Anthropic's Opus 4.8 model during an audit funded by Shielded Labs. The AI helped write a proof-of-concept exploit.
- **Five-day response**: Soft fork at block 3,363,426 (June 2) disabled Orchard transactions; NU6.2 hard fork at block 3,364,600 (June 3) activated corrected circuit.
- **Orchard pool background**: Introduced in NU5 (May 2022), built on Halo 2 proving system (no trusted setup). Bug existed for four years before discovery.
- **Post-incident**: Shielded Labs plans new privacy pool with turnstile accounting for all Orchard tokens.

## Overview

A focused episode covering the Zcash Orchard vulnerability discovery. Security researcher Taylor Hornby, during an audit for Shielded Labs, used Anthropic's Claude Opus 4.8 model to identify a soundness bug in the halo2_gadgets crate that had been present since May 2022. The bug could have allowed undetectable counterfeiting of ZEC within the Orchard privacy pool. The episode covers the technical details (missing constraint in the ZK circuit), the emergency response timeline, and the broader implications for AI-assisted security auditing in crypto.

## Implications

- This is a landmark case for AI-assisted vulnerability discovery — a four-year-old bug found in hours with AI assistance.
- The Zcash Foundation found no signs of exploitation, but the privacy properties of Orchard make definitive proof impossible.
- Expect increased investment in AI-powered auditing tools across the crypto ecosystem.

## Notable Quotes

- "The vulnerability could have been exploited to undetectably create an unlimited amount of counterfeit ZEC within Orchard." — Zooko Wilcox
- "There is no way to cryptographically prove whether the vulnerability was exploited before it was remediated." — Zooko Wilcox

## People Mentioned

- Taylor Hornby — Independent security researcher
- Zooko Wilcox — Zcash founder
- Shielded Labs — Nonprofit supporting Zcash ecosystem

## Topics

[Zcash, Orchard, zero-knowledge proofs, soundness bug, Claude Opus 4.8, Anthropic, AI security auditing, halo2_gadgets, counterfeiting vulnerability, emergency fork, NU6.2, turnstile mechanism]
