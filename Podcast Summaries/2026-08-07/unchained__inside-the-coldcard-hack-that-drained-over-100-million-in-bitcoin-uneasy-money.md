---
podcast: "Unchained"
episode: "Inside the Coldcard Hack That Drained Over $100 Million in Bitcoin: Uneasy Money"
published: 2026-08-07
duration: 1h06m55s
audio_url: "https://pdrl.fm/98e0b1/traffic.megaphone.fm/LSHML8969627964.mp3"
episode_url: "https://www.youtube.com/watch?v=V6dfi1wjp5o"
transcript_source: youtube_autocaptions
generated_at: 2026-08-07T22:45:00Z
model: "deepseek-v4-flash"
guid: "0c1a1150-9289-11f1-bf33-e3697544f1ef"
---

# Inside the Coldcard Hack That Drained Over $100 Million in Bitcoin: Uneasy Money — Unchained

## TL;DR
Haseeb Qureshi, Tom Schmidt, Tarun Chitra, and Robert Leshner (The Chopping Block, published on the Unchained feed) dissect the Coldcard exploit that drained ~$100M of Bitcoin: a weak-RNG fallback buried in a firmware change five years ago, found by Claude Code in 8 minutes and by an offline GLM 5.2 for about $2. The panel argues it signals the "death of apathy" for open-source security, that AI rewrites the economics of who audits code, and that minority wallet vendors are becoming impossible to trust — then tears into Ethereum's EIP-8361 staking-yield taper.

## Key points
- The hack: Coldcard (NVK's Bitcoin-only hardware wallet, ~1-2% of the wallet market, beloved by maxis for being open source) generated keys from a weak software RNG after a 2021-era firmware change — a one-word commit altering C++ macro definitions caused a fallback to a broken RNG; ~$100M drained in multiple waves over a weekend.
- AI found it: Claude Code reportedly located the vulnerability in 8 minutes; an open model (GLM 5.2) with no internet access found it in 20 minutes for about $2 of compute. The team's failure to run AI over their own code for five years is called "incredibly delinquent."
- "The death of apathy": open-source projects must now point AI at their own code — white-hat auditing with frontier models is table stakes; victims include long-time savers who "did everything right" (self-custody, hardware wallet) and lost three Bitcoin.
- AI security economics: good Samaritans spend ~$10 of compute per project and, because everyone uses similar models, duplicate identical work that adds no security; attackers spend $1,000-$100,000+ — so open source no longer protects against nefarious third parties; only the vendor spending real money on AI hardening is protected.
- Consolidation: minority "passion project" vendors can't afford the security budget; the panel bets closed-source systems have the bigger catastrophe coming (Fed nearly wired $1B to North Korea), with open source taking the bludgeoning first (tiny DeFi protocols being "rekt" — record incident counts with low total dollars since April).
- EIP-8361 (Pintail, Justin Drake, et al.): taper staking yield toward zero above 50% staked to prevent over-staking/centralization; the community "vomited all over it" — nominal yields matter, Tarun calls it "a truly horrendous post," and constantly changing monetary policy means ETH is never credible hard money; Solana's proposals are better researched.
- The staking complex has collapsed toward stablecoin yields; post-quantum security is called the real priority ("the first real scalable post-quantum chain" gets the hard-money valuation; "Bitcoin people can't say that — they're getting their wallets hacked left and right").
- Later segments (per timestamps): Aschenbrenner's Situational Awareness fund's ~67% blowup at 4x leverage (Archegos comparisons), Robinhood's prediction-markets boom, Korea's retail bloodbath, and the CLARITY Act's ethics provisions.

## Overview
The episode is a classic Chopping Block: bad news first (the Coldcard exploit), then a wide-ranging security-economics debate, then governance drama (EIP-8361). The through-line is AI's transformation of security — both as the tool that found a five-year-old bug in minutes and as the force that makes human-scale open-source review obsolete, since adversaries now out-compute well-meaning reviewers. The second half argues Ethereum's issuance debate is a distraction from post-quantum readiness and notes the parallel collapse of leverage-staking demand into the stablecoin yield complex.

## Implications
- Self-custody users: hardware wallet trust must be re-earned — audit budgets, vendor size, and AI-hardening practices now matter; affected Coldcard users must regenerate keys regardless of firmware updates.
- Open-source maintainers: run frontier models over your own code continuously; assume adversaries are spending thousands of dollars of compute against you.
- Security industry: expect consolidation to trust in well-capitalized vendors and a wave of small-protocol exploits; closed-source institutions (central banks, legacy systems) may be the bigger eventual loss.
- Ethereum governance: issuance taper proposals face fierce backlash; hard-money credibility and post-quantum transition are the live debates.

## Notable quotes
- "It's the death of apathy... any open source project at this point, whether it's a smart contract or anything else, it doesn't hurt to train the security budget on yourself."
- "20 minutes of GLM 5.2 cost about $2. Meaning that this team... did almost anything with respect to using AI to monitor their own code for vulnerabilities — which given where we are in the ecosystem seems incredibly delinquent."
- "Open source in a way is protection against a nefarious developer, but it's not protection against a nefarious third party."
- "Don't trust any code before 2026, period." (Tom Schmidt)
- "I think closed source is going to have the bigger catastrophe... some central bank is going to get hacked." (Tarun Chitra)
- "It's like a truly horrendous post... if you constantly change your policy, no matter what, you're not hard money." (Tarun on EIP-8361)

## People mentioned
- Haseeb Qureshi — Managing Partner, Dragonfly
- Tom Schmidt — General Partner, Dragonfly
- Tarun Chitra — Managing Partner, Robot Ventures / Gauntlet
- Robert Leshner — Founder & CEO, Superstate
- NVK — Coldcard founder (referenced)
- Leopold Aschenbrenner — Situational Awareness (referenced)

## Topics
- Coldcard hack, hardware wallets, RNG vulnerability, AI security, Claude Code, open source, self-custody, Bitcoin, EIP-8361, Ethereum staking, hard money, post-quantum crypto, DeFi exploits, Aschenbrenner, CLARITY Act, prediction markets
