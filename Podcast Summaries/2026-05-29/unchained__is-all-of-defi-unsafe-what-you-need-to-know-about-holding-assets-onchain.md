---
guid: 08993a3e-5ba1-11f1-a42b-7be2c14bf56c
title: "Is All of DeFi Unsafe? What You Need to Know About Holding Assets Onchain"
show: "Unchained"
date: 2026-05-29
transcript_source: web
episode_url: https://www.youtube.com/watch?v=yzglmrFxerw
---

# Is All of DeFi Unsafe? What You Need to Know About Holding Assets Onchain

## Key Points
- Manuel Aráoz, OpenZeppelin co-founder, declared 'I now consider all of DeFi unsafe,' advising friends and family to exit blue-chip protocols including Aave, MakerDAO, and Compound, citing AI coding agents that are 'superhuman at finding vulnerabilities.'
- The core asymmetry Aráoz identified: defenders must patch every possible vulnerability while attackers need only discover one exploit to drain funds — a structural disadvantage that worsens as AI tools improve.
- DefiLlama data shows more than $1.1 billion lost to DeFi exploits in the prior 12 months, with April 2026 alone recording nearly $630 million in losses across at least 27 separate incidents — the worst month since the Bybit hack in early 2025.
- The two largest April 2026 exploits — the Drift Protocol hack (~$285 million via a six-month social engineering campaign) and the Kelp DAO bridge attack (~$293 million) — were both attributed to North Korean state-backed hacking groups.
- Isaac Patka (Security Alliance) pushed back on Aráoz's framing by distinguishing code-level smart contract bugs from operational security failures such as key compromises and phishing, arguing that a significant portion of recent losses are not caused by smart contract vulnerabilities per se.
- Mike Silagadze (Ether.Fi CEO) offered a practitioner's view, noting that Ether.Fi proactively hardened its weETH cross-chain bridge across all 20 supported chains in response to the bridge vulnerability wave, illustrating that real-time defensive response is possible.
- The guests agreed that AI tools lower the barrier for attackers but argued the same tools can be deployed defensively for continuous monitoring and automated vulnerability scanning — a position that OpenZeppelin's current leadership has also taken.
- OpenZeppelin's CEO Demian Brener distanced the firm from Aráoz's remarks, stating that Aráoz's views 'do not represent OpenZeppelin's current position' and that the company remains committed to securing onchain finance through 'continuous, AI-augmented security.'
- DeFi total value locked declined approximately 14% — from $172 billion to $148 billion — in the weeks following the April exploit wave, with Aave alone seeing TVL drop from $26.4 billion to $14.6 billion.
- The episode concluded that the answer is not a blanket exit from DeFi but rather a differentiated, risk-aware approach: favoring protocols with layered security (continuous monitoring, bug bounties, formal verification, insurance), and avoiding single-point-of-failure dependencies like poorly secured cross-chain bridges.

## Overview
This 50-minute episode of Unchained, hosted by Laura Shin, was sparked by a viral declaration from Manuel Aráoz, co-founder of the smart contract security firm OpenZeppelin, who posted on X that he now considers "all" of DeFi unsafe. Aráoz argued that AI-powered coding agents have become superhuman at identifying smart contract vulnerabilities, and that the security asymmetry — where defenders must fix every bug while attackers need find only one — has become untenable. He went so far as to advise friends and family to exit even blue-chip DeFi protocols like Aave, MakerDAO, and Compound. This bombshell from one of the most respected figures in blockchain security triggered widespread debate across the industry.

To examine the claim from two different vantage points, Laura Shin brought in Isaac Patka, Certifications Lead at the Security Alliance (SEAL), and Mike Silagadze, CEO of Ether.Fi. The guests parsed what Aráoz got right and what he got wrong, engaging with the empirical record of DeFi exploits and the structural features of onchain systems that make them uniquely vulnerable. They also discussed what concrete changes — at the protocol, operational, and industry levels — are necessary for DeFi to remain a viable and trustworthy financial system in an era of increasingly capable AI-driven attack tools.

The episode arrives amid a stark backdrop: DefiLlama data shows more than $1.1 billion lost to DeFi exploits over the prior 12 months, with April 2026 alone accounting for nearly $630 million across at least 27 reported incidents. The two largest single events were the Drift Protocol exploit on Solana (approximately $285 million, traced to a six-month social engineering campaign) and the Kelp DAO cross-chain bridge attack (approximately $293 million), both of which have been attributed to North Korean state-backed hacking groups. The severity of these incidents caused DeFi total value locked to fall roughly 14%, from $172 billion to $148 billion, in the weeks following.

The guests engaged critically with Aráoz's framing. Isaac Patka, drawing on his operational security and incident-response background at SEAL, distinguished between code-level vulnerabilities and operational security failures — noting that a significant share of major losses stem from key compromises, phishing, and social engineering rather than smart contract bugs alone. Mike Silagadze, whose Ether.Fi platform had itself hardened its weETH cross-chain bridge infrastructure in response to the wave of bridge attacks, offered a practitioner's perspective on what layered security at a live protocol looks like in practice. Both acknowledged that AI tools do meaningfully lower the bar for attackers but argued that the same tools can enhance the defensive side, and that a blanket "exit everything" recommendation does not reflect the heterogeneous risk landscape across DeFi protocols.

## Implications
The episode captures a genuine inflection point for decentralized finance: for the first time, a founder of one of the industry's most foundational security firms has publicly declared that the current security model is inadequate for the AI era. The significance is not merely rhetorical — the data on exploit volume and dollar losses in 2026 gives concrete weight to the concern. If AI coding agents can systematically identify vulnerabilities in public smart contract code faster than human auditors can address them, the traditional audit-at-deployment model faces an existential challenge. The responses from Isaac Patka and Mike Silagadze point toward a direction the industry is likely to move: continuous, automated, AI-augmented monitoring rather than point-in-time audits; sharper distinctions between code risks and operational security risks; and layered defensive frameworks that treat security as an ongoing process rather than a pre-launch checkbox.

For users holding assets onchain, the episode's practical takeaway is risk stratification rather than wholesale exit. Protocols with active bug bounty programs, formal verification, real-time monitoring infrastructure, and strong operational security practices (multisig key management, hardware security modules, phishing-resistant team practices) represent meaningfully lower risk than those relying solely on historical audits. The episode also highlights that cross-chain bridges remain the highest-risk attack surface in the ecosystem — a lesson reinforced by multiple nine-figure exploits in 2026 — and that users and protocols alike should minimize unnecessary bridge exposure until the infrastructure matures.

## Notable Quotes
> I now consider all of DeFi unsafe. Coding agents are superhuman at finding vulnerabilities, and smart contract security is too asymmetric: defenders need to fix every bug while attackers need just one exploit to steal funds. -- Manuel Aráoz

> Aráoz's views do not represent OpenZeppelin's current position. -- Demian Brener, OpenZeppelin CEO

> A defender must fix every potential bug, while an attacker only needs to find one critical flaw to drain all funds. -- Manuel Aráoz (paraphrased in coverage)

> Fewer than 10% of recent DeFi failures stem from code vulnerabilities — losses are driven by parameter misconfigurations, collateral issues, and operational security lapses. -- Marc Zeller, Aave Chan Initiative founder

> no DeFi protocol, including blue-chip platforms like Aave and Compound, can currently be considered safe -- Manuel Aráoz

## People Mentioned
- Manuel Aráoz -- Co-founder and former CTO of OpenZeppelin, co-founder of Decentraland; issued the viral 'all of DeFi is unsafe' warning
- Isaac Patka -- Certifications Lead at the Security Alliance (SEAL), co-founder of Shield3; episode guest representing security/incident-response perspective
- Mike Silagadze -- CEO of Ether.Fi, a non-custodial ETH liquid restaking protocol; episode guest representing practitioner/protocol-operator perspective
- Laura Shin -- Host of Unchained podcast, former senior editor at Forbes, author of The Cryptopians
- Demian Brener -- CEO of OpenZeppelin; publicly distanced the firm from Aráoz's remarks
- Marc Zeller -- Founder of Aave Chan Initiative; strongly disputed Aráoz's framing, calling the characterization of the threat 'moronic'
- Stani Kulechov -- Founder of Aave; among the DeFi leaders who responded to Aráoz's warning
- Hayden Adams -- Founder of Uniswap; responded to Aráoz's warning in the broader industry debate

## Topics
DeFi security, AI-powered exploit discovery, smart contract vulnerabilities, onchain asset safety, cross-chain bridge risks, operational security, Security Alliance, Ether.Fi, OpenZeppelin, DeFi hacks and exploits
