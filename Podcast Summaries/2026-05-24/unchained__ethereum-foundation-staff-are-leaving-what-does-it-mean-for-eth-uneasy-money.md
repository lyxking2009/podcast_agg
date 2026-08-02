---
title: "Ethereum Foundation Staff Are Leaving. What Does It Mean for ETH? - Uneasy Money"
show: "Unchained"
author: "Laura Shin"
pub_date: "2026-05-24"
duration_seconds: 4320
guid: "0cfe5894-5709-11f1-a029-7b83aa810d2c"
episode_url: ""
transcript_source: "whisper_transcription"
---

## Key Points

- **EF missionary exodus**: Multiple senior EF protocol researchers have departed — Trent Van Epps, Josh Stark, Barnabe Monnot, Tim Beiko, and Carl Beek — with more rumored to be leaving.
- **Kain's theory ("crushed hope")**: Tomasz Stanczak's appointment as co-executive director was seen as a ray of hope that the EF would become more commercially aware and founder-friendly. When Vitalik reasserted the old ways and Tomasz departed, the missionaries who had believed in change capitulated.
- **Vitalik's unchallenged authority**: Everyone is "so scared of Vitalik" — information leaks only through back-channel DMs. The EF culture is described as cult-like: opaque, confusing, saying one thing and doing another.
- **Structural critique**: The EF historically struggles with organizational structure, accountability, and goals. The dual executive director model was quintessentially EF: "just pick a fucking guy."
- **Missionaries, not mercenaries**: EF staff are definitionally ideologically motivated — they stayed despite being able to earn $2M+ at Solana or elsewhere. Losing them is "dire" for a mission-based organization.
- **AI empowers exit**: AI tools now allow these world-class researchers to go solo — building what would have taken a large team solo, which may accelerate departures.
- **Vitalik's challenge**: What empirical evidence would cause Vitalik to change how the EF is run? If no evidence could, he should at least be transparent about it rather than raising and crushing hope.
- **Three DeFi exploits in four days**: Echo Protocol (~$76M nominal, ~$10M actual), plus two others totaling ~$21M. Thorchain bridge exploit was technically sophisticated — attacker became a threshold signer and obtained the private key through the TSS ceremony.
- **Thorchain attack via AI**: Taylor Monahan reconstructed the Thorchain exploit in real time with AI assistance (including Banteg and a "clanker bot" AI agent), which ultimately wrote a full working exploit demo — key generation, TSS ceremony, and CRT proof — without being explicitly asked.

## Overview

*Uneasy Money* is a weekly show hosted by Kain Warwick (Infinex/Synthetix) with co-hosts Taylor Monahan (security expert) and Luca Netz (CEO, Pudgy Penguins). This episode covers three main topics: the SpaceX pre-IPO perp debut on Hyperliquid, the Ethereum Foundation staff exodus, and three DeFi exploits.

**EF Exodus (main topic):** Kain advances what he calls the "crushed hope" theory. The EF, long characterized by research-first culture and commercial indifference (he jokes about Aya Miyaguchi's philosophy of "preventing winning from being possible"), got a brief infusion of optimism when Tomasz Stanczak — a person with actual company-running experience — was brought in as co-executive director. Kain observed renewed outreach to founders, guest post initiatives, and a general energy shift. But Tomasz has now departed, and in Kain's telling, Vitalik "blew the guys' brains out in front of you" — reasserting the old ways. The missionaries who had believed in change are now leaving.

The discussion notes the paradox: those leaving (Trent Van Epps, Josh Stark, etc.) are archetypal EF missionaries — ideologically aligned, Malady PFP wearers, the "Vitalik vision types." If even they are leaving, the exodus can't be explained by commercial mercenaries finding greener pastures. Something structural has broken. Kain's challenge to Vitalik: state explicitly what evidence would change your mind about how the EF should be run. If none exists, at least be transparent rather than repeatedly raising and crushing hope.

Taylor adds a second thread: AI tools may be enabling former EF researchers to go solo in ways previously impossible. These are among the smartest people in crypto; with modern coding agents and infinite inference, they can accomplish what previously required a large team.

**DeFi Exploits:** Taylor walks through three exploits in four days. The Thorchain bridge attack is the standout — an attacker became a threshold signature scheme (TSS) validator, participated in the key generation ceremony, and somehow obtained the private key (which theoretically should never exist given TSS cryptography). Taylor reconstructed the attack in real time during incident response using an AI agent that ultimately produced a complete working exploit demo — including CRT proof and TSS key generation — unprompted. The incident illustrates both AI's power as a security analysis tool and its potential for dual-use harm.

## Implications

- The EF missionary exodus may signal a structural inflection point — if ideologically aligned researchers are leaving, the organization may be drifting away from founder/ecosystem engagement.
- Vitalik's reassertion of the old EF culture is a bearish signal for Ethereum's commercial ecosystem development; the community deserves transparency about the organizational philosophy.
- AI tools are creating a new class of capable solo builders among world-class researchers — ex-EF talent may produce significant competing or complementary projects.
- The Thorchain exploit highlights that TSS/MPC bridge designs remain fragile; even with correct cryptographic design, implementation and ceremony participation can be compromised.
- AI-assisted incident response is now real: Taylor's account demonstrates the speed advantage defenders gain — and the corresponding risk if attackers use the same tools.

## Notable Quotes

> "It's like you're living in a communist utopia, care quotes, and then you get invaded by these capitalist dirt bags and life becomes so much better... and then the communist emperor comes back and takes the country over again and takes away your Coca-Cola... I'm back to digging potatoes out of the ground. Like fuck this, I'm out." — Kain Warwick

> "If you've been in the EF, you are a missionary, not a mercenary... definitionally you weren't there because you're getting paid." — Kain Warwick

> "What would it take to invalidate Vitalik's position here? Like, what would it take for Vitalik to go, maybe my way is not working?" — Kain Warwick

> "You're losing the missionaries. That is dire." — Kain Warwick

> "Everyone's so fucking scared of Vitalik, right?" — Kain Warwick

> "The place you want to be in the world right now is the place that gives you infinite inference." — Taylor Monahan

## People Mentioned

- **Kain Warwick** — Co-host; Founder of Infinex and Synthetix
- **Taylor Monahan** — Co-host; Security expert
- **Luca Netz** — Co-host; CEO, Pudgy Penguins
- **Vitalik Buterin** — Ethereum co-founder; referenced as reasserting EF's original culture
- **Tomasz Stanczak** — Former EF co-executive director; departure central to the exodus narrative
- **Aya Miyaguchi** — Referenced as EF executive director
- **Trent Van Epps** — Former EF protocol researcher; departed
- **Josh Stark** — Former EF; departed
- **Barnabe Monnot** — Former EF protocol researcher; departed
- **Tim Beiko** — Former EF; departed
- **Carl Beek** — Former EF; departed
- **Banteg** — Participated in Thorchain incident response alongside Taylor
- **Elon Musk** — Referenced as SpaceX founder (pre-IPO perp context)

## Topics

- Ethereum Foundation staff exodus
- EF organizational culture (missionary vs. mercenary)
- Vitalik Buterin's leadership and organizational philosophy
- Tomasz Stanczak's tenure as EF co-executive director
- SpaceX pre-IPO synthetic perpetual futures (Hyperliquid / Trade.xyz)
- AI tools enabling solo researchers/builders
- DeFi security exploits (Echo Protocol, Thorchain bridge)
- Threshold signature scheme (TSS) cryptography and attack vectors
- AI-assisted security incident response
- Hyperliquid token ($HYPE) analysis
