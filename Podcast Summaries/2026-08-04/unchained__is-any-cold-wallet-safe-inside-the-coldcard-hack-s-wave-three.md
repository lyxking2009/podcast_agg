---
podcast: "Unchained"
episode: "Is Any Cold Wallet Safe? Inside the Coldcard Hack's Wave Three"
published: 2026-08-04
duration: 18m39s
audio_url: "https://pdrl.fm/98e0b1/traffic.megaphone.fm/LSHML4867037427.mp3"
episode_url: ""
transcript_source: youtube_autocaptions
generated_at: 2026-08-06T02:41:20Z
model: "claude-sonnet-5"
guid: "1bed1828-9060-11f1-b8bd-f7b7a2f40246"
---

# Is Any Cold Wallet Safe? Inside the Coldcard Hack's Wave Three — Unchained

## TL;DR
Galaxy Digital's head of research Alex Thorn details the Coldcard hardware wallet exploit: a March 2021 firmware update meant to add a better random number generator was miswired, so key generation silently fell back to a much weaker RNG, letting attackers brute-force seed phrases. Thorn has identified three confirmed attack waves (with a fourth in progress) accounting for roughly 1,600-2,000 BTC (over $100 million) stolen from long-dormant cold storage addresses.

## Key points
- The exploit stems from a March 17, 2021 Coldcard firmware update that added a custom random number generator, but the code was miswired so it would silently fail and fall back to a much weaker RNG.
- Alex Thorn has identified three confirmed attack waves; wave 3 sweeps coins from 293 victims into 293 separate individual vaults, a distinct topology from waves 1-2.
- A possible wave 4 was in progress at the time of recording, which would push the total toward almost 2,000 BTC stolen.
- Affected wallets are Coldcard models generating single-signature (non-multisig) addresses after the March 2021 firmware update; Thorn urges holders to move those coins immediately.
- The average dormancy of drained coins is close to four years, indicating victims were long-term self-custody holders following standard best practices.
- This is fundamentally a cryptography/entropy failure, not a Bitcoin-specific problem — the same flaw could compromise any system relying on that entropy source.
- Thorn draws a parallel to quantum computing risk: a working quantum computer could pose an analogous large-scale threat to key derivation across financial infrastructure.

## Notable quotes
> "It's been a brutal four days. This sort of started early in the morning Thursday UTC time, and is still ongoing." — Alex Thorn (~00:02:20)

> "These people did nothing wrong. In fact, they did everything right." — Alex Thorn (~00:02:58)

> "They miswired it into the firmware, such that it would never be routed through that random number generator, and it would fail silently and it would backfall to this other crappy random number generator." — Alex Thorn (~00:04:31)

> "We're well over a hundred million dollars of self-custodied Bitcoin. I would place it more in the 1,600 BTC range." — Alex Thorn (~00:14:02)

> "You could call it a trial run perhaps if quantum does emerge, it wouldn't look that different." — Alex Thorn (~00:15:52)

## People mentioned
- Alex Thorn
- Austin Campbell
- Ram Ahluwalia
- Chris Perkins

## Topics
`hardware-wallets` `crypto-security` `coldcard` `bitcoin` `entropy-vulnerability` `self-custody` `onchain-forensics` `quantum-computing-risk`
