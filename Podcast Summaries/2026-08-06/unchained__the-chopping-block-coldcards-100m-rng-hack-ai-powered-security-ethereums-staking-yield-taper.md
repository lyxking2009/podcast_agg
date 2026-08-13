---
podcast: "Unchained"
episode: "The Chopping Block: ColdCard's $100M RNG Hack, AI-Powered Security & Ethereum's Staking Yield Taper"
published: 2026-08-06
duration: 1h3m7s
audio_url: "https://pdrl.fm/98e0b1/traffic.megaphone.fm/LSHML3153225653.mp3"
episode_url: ""
transcript_source: show_notes
generated_at: 2026-08-07T00:11:28Z
model: "claude-sonnet-5"
guid: "ae1428e6-9127-11f1-8fef-8fd87b013512"
---

# The Chopping Block: ColdCard's $100M RNG Hack, AI-Powered Security & Ethereum's Staking Yield Taper — Unchained

## TL;DR
With no guest this week, hosts Haseeb Qureshi, Tom Schmidt, Tarun Chitra, and Robert Leshner dig into ColdCard's ~$100M RNG exploit — a five-year-old firmware bug that an AI model (Claude Code) found in 8 minutes — and debate whether AI has killed both open-source security and Bitcoin maximalism as a credible security posture. They then torch Ethereum's EIP-8361 staking-yield taper proposal, unpack Leopold Aschenbrenner's ~67% AI hedge-fund blowup, and close on Korean market volatility and the ethics-provision fight inside the CLARITY Act.

## Key points
- ColdCard, Coinkite's Bitcoin-only hardware wallet, was drained of nearly $100M after a five-year-old random-number-generation bug silently fell back to weak software RNG instead of hardware entropy.
- The flaw traces to a single one-word commit that swapped C++ macros, apparently just to get NVK's code to compile — a change that quietly doomed years of generated seed keys.
- Claude Code identified the ColdCard bug in 8 minutes; an open model (GLM 5.2), run fully offline with no internet access, found it in about 20 minutes for roughly $2 in compute.
- Tarun Chitra argues Bitcoin-maximalist developers lack basic security rigor, branding ColdCard's lack of hardening "incredibly delinquent" and calling maxi devs "the RFK of security practices" who "don't do audits."
- Haseeb Qureshi warns that AI models "are much less diverse than humans," meaning security increasingly scales with assets under management — while adversaries like North Korea can find the same flaws for comparatively little compute spend.
- The hosts debate whether AI has effectively ended the viability of open-source security, and whether the hack proves Nic Carter's claim that this is "the death of Bitcoin maximalism," citing posts from holders who lost life savings.
- They dissect Ethereum's EIP-8361 (from Pintail and Justin Drake), which tapers ETH staking yield toward zero above 50% staked; Tarun calls it "a truly horrendous post" and "the proposal reads like shit," arguing that constantly shifting monetary policy undermines Ethereum's case as credible hard money.
- Related tangents cover hard money, post-quantum cryptography concerns, and central bank chaos.
- Leopold Aschenbrenner's Situational Awareness AI hedge fund reportedly fell about 67% while trading 4x levered; Robert Leshner draws comparisons to the Archegos collapse.
- Other segments cover Robinhood's prediction-markets boom versus Hyperliquid's real-world-asset products, a "bloodbath" in Korean markets and the decline of retail volatility, and the CLARITY Act's ethics provisions, which the hosts frame as the linchpin of the whole bill.

## Notable quotes
> "the death of Bitcoin maximalism" — Nic Carter, as cited by the hosts (~01:03)

> "the RFK of security practices" — Tarun Chitra (~05:46)

> "incredibly delinquent" — Tarun Chitra, on ColdCard's lack of hardening (~05:46)

> "are much less diverse than humans" — Haseeb Qureshi, on AI models (~05:46)

> "the proposal reads like shit" — Tarun Chitra, on EIP-8361 (~23:21)

> "a truly horrendous post" — Tarun Chitra, on EIP-8361 (~23:21)

## People mentioned
- Haseeb Qureshi
- Tom Schmidt
- Tarun Chitra
- Robert Leshner
- NVK (Coinkite / ColdCard)
- Nic Carter
- Leopold Aschenbrenner
- Justin Drake
- Pintail

## Topics
`coldcard-hack` `rng-vulnerability` `hardware-wallets` `bitcoin-security` `ai-security-research` `open-source-security` `ethereum-staking` `eip-8361` `bitcoin-maximalism` `clarity-act`
