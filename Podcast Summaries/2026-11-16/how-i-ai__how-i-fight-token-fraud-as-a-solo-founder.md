---
podcast: "How I AI"
episode: "How I fight token fraud as a solo founder"
published: 2026-11-16
duration: "39m46s"
audio_url: "https://anchor.fm/s/1035b1568/podcast/play/129333644/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-10-12%2F436600883-44100-2-1ab8e49c524fd.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/pen-name/episodes/How-I-fight-token-fraud-as-a-solo-founder-e3s9euc"
transcript_source: description
generated_at: 2026-11-19T23:10:03Z
model: "deepseek-v4-pro"
guid: "c2f0c4bc-e38e-48a7-a353-4a2fd5c34cf2"
---

# How I fight token fraud as a solo founder — How I AI

> ⚠️ Based on episode description only — full transcript was unavailable.

## TL;DR
Claire Vo, solo founder of ChatPRD, details how token fraud cost her tens of thousands of dollars and consumed most of her engineering time over the past year. She identifies six abuse patterns—including free trial farming, promo code reselling, and a custom MCP running as a 24/7 compute worker—and explains the business and technical controls she deployed, such as Stripe Radar and Vercel AI Gateway, ultimately killing free trials. Her core warning: if you sell AI, you sell compute.

## Key points
- Token fraud at ChatPRD cost tens of thousands of dollars and consumed almost all of Claire Vo's engineering energy over the past year.
- Fraudsters employed at least six abuse patterns: farming free trials, reselling promo codes, wrapping the product in relays, and installing a custom MCP that ran as a 24/7 compute worker, among others.
- Claire categorized abusers into four distinct types (specific categories not detailed in this description).
- She implemented business controls to slow fraud, including turning off free trials for good.
- Technical defenses include Stripe Radar, Vercel AI Gateway, Vercel Bot ID, Vercel Firewall, Clerk, and Codex, used to block and monitor abuse.
- A 'Jev loop' using Jev (TypeSafe AI) classifies fraud in real time, enabling automated detection and response.
- AI support desks can become an attack vector, as fraudsters may exploit them to gain access or information.
- The episode's central principle: 'if you sell AI, you sell compute,' meaning AI SaaS is uniquely exposed because costs scale directly with token usage.
- Claire accidentally funded Roblox and Fortnite maps, indicating fraudsters used stolen compute for unrelated high-cost tasks.

## People mentioned
- Claire Vo
- Gareth Wham
- Simon Willison

## Topics
`token-fraud` `ai-saas` `security` `fraud-prevention` `bootstrapping` `compute-cost` `solo-founder`
