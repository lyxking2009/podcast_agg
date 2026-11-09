---
podcast: "No Priors: Artificial Intelligence | Technology | Startups"
episode: "Democratizing Security for the Agentic Web with Cloudflare CTO Dane Knecht"
published: 2026-10-23
duration: "41m23s"
audio_url: "https://traffic.megaphone.fm/PDP2314551616.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-10-27T22:14:15Z
model: "deepseek-v4-pro"
guid: "1b967280-ce7b-11f1-8a29-072c10b8d7bc"
---

# Democratizing Security for the Agentic Web with Cloudflare CTO Dane Knecht — No Priors: Artificial Intelligence | Technology | Startups

## TL;DR
Cloudflare CTO Dane Knecht describes an 'AI security apocalypse' where vulnerabilities like Log4j-level events now surface weekly, driven not just by nation states but by high school 'agent kitties' using AI agents. He argues organizations will spend 50% of engineering time on security in the next 18 months, requiring more red teams and automated patching, while Cloudflare democratizes protections. Knecht also outlines Cloudflare's new agentic web infrastructure—monetization gateway, wallets, stablecoin, and OS—to support a machine-readable internet and new micropayment economics.

## Key points
- AI accelerates vulnerability discovery: previously once-a-year Log4j/Shellshock-type incidents now happen weekly; Dane Knecht expects over 50% of engineering time to be spent on security in the next 18 months, a short-term transition because AI will eventually survey most code and find bugs.
- A critical TLS/webpki key was generated with seed number 4 and is being reverse engineered; OpenAI recently found seeds for NIST curves from 2000, illustrating AI's ability to break cryptographic assumptions faster than human red teams.
- Attackers are no longer just nation states: high school kids and 'agent kitties' use AI agents against targets, motivated by bug bounties, effectively democratizing offensive capabilities previously held by state actors.
- Cloudflare is expanding from one red team to four or five by the end of next year, focusing on discovery; Dane believes engineers will automate patching when volume pressure makes manual processes unsustainable.
- To improve software security, reduce attack surface area and simplify code; formal verification and safer languages will help later, but current AI models create verbose complexity, so human judgment and Cloudflare's internal codex principles remain essential.
- The web is in a 'messy middle' where agents parse human-designed pages inefficiently; Cloudflare is building parallel economic rails: a monetization gateway, agent wallets, a stablecoin, and project Signals with OpenAI to help agents discover content and pay site owners via micropayments.
- Cloudflare OS, originally an internal platform for secure employee AI use, is now a reference architecture; it is used more by go-to-market and G&A teams than engineering, enabling non-technical staff to build hundreds of internal tools without security overhead.
- Cloudflare's May restructuring to an AI-first organization included layoffs and process redesign; a product manager email workflow that once involved three teams now uses a single tool where the PM describes a cohort, AI writes and sends the email after manager approval.
- Cloudflare hired 1,000 interns for its 5,000-person company after observing that AI-native interns drive faster AI adoption among managers; curiosity is now a key hiring trait, and not using AI in personal life is considered a red flag.
- Token budgeting is still manual but should become intelligent routing; Cloudflare uses tiered models—frontier models for planning, open-source models for execution, frontier models for review—and sets alert thresholds at $300/day email, $500 to manager, $1000 to VP.
- Dane argues that AI security models must eventually be open because bad actors will get access anyway, and closed models exclude the mass population that needs defensive capabilities.
- For smaller organizations, Dane recommends putting sites and infrastructure behind Cloudflare, which provides security protections for major vulnerabilities to free customers as a public good, democratizing security like it democratized CDN performance.

## Notable quotes
> "AI has become the tool that's causing this kind of security apocalypse is also the tool that can help us to solve it faster as well." — Dane Knecht
> "It went from how to get everyone to using AI. Now all of a sudden everyone's racing to figure out what AI security means." — Dane Knecht
> "we're going to call them agent kitties now." — Host

## People mentioned
- Dane Knecht
- Greg Brockman
- Tim Berners-Lee

## Topics
`ai-security` `agentic-web` `cloudflare` `zero-trust` `internet-infrastructure` `micropayments`
