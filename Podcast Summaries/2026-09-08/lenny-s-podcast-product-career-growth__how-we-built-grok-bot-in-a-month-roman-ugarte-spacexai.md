---
podcast: "Lenny's Podcast: Product | Career | Growth"
episode: "How we built Grok Bot in a month | Roman Ugarte (SpaceXAI)"
published: 2026-09-08
duration: 1h22m43s
audio_url: "https://pscrb.fm/rss/p/api.substack.com/feed/podcast/214070217/3483f2b4b11da80edfd1766d944779f8.mp3"
episode_url: "https://www.lennysnewsletter.com/p/how-we-built-grok-bot-in-a-month"
transcript_source: web
generated_at: 2026-09-08T22:30:42Z
model: "deepseek-v4-pro"
guid: "substack:post:214070217"
---

# How we built Grok Bot in a month | Roman Ugarte (SpaceXAI) — Lenny's Podcast: Product | Career | Growth

## TL;DR

Roman Ugarte, product lead for Grok Bot, recounts how a small SpaceXAI team built the product from scratch in about a month, followed by three weeks of internal beta and manual onboarding of 200–300 users before public launch. He attributes success to building it as a separate product from Cursor, giving each bot a cloud-based computer, and aggressively unshipping complexity. The vision is a team of AI teammates for work and personal life, with patterns like 'chief of staff' bots emerging organically and a focus on just working.

## Key points

- Grok Bot began as a blank-page, from-scratch project with a small, isolated team working for about a month; first line of code to internal prototype took roughly one month, followed by three weeks of internal beta and then public launch roughly three weeks prior to recording. This speed was enabled by small team, focused decision-making, and avoiding long-horizon planning.
- The team decided to build Grok Bot as a separate product rather than integrating into Cursor, despite Cursor being used for non-coding tasks; reasoning: existing brand and UI were intimidating to nontechnical users, and a new product allows controlling every pixel and maintaining a consistent vision for knowledge work.
- Two early, non-obvious decisions were critical: first, all bots run in the cloud, not locally, so they act as persistent colleagues with their own state accessible from any device; second, each bot has its own computer (browser) to perform tasks without requiring APIs/MCPs, similar to how humans interact.
- During the two-week manual onboarding period for 200–300 early users, the team learned key usage patterns without biasing users: one emergent pattern was a 'chief of staff' bot that fans out tasks to other specialized bots, which later influenced product direction.
- The team aggressively unshipped experimental features and developer-oriented visibility tools to simplify the product; they focus on 'Grokbot can now' rather than 'Grokbot now has', and features like automations are defined in natural language instead of UI, with 99% of automations built that way.
- Computer use infrastructure improvements were a major focus: issues like fine mouse control on Salesforce dashboards were solved iteratively, unlocking workflows for sales and other nontechnical teams; user feedback directly drove engineering prioritization.
- Recruiting team use case: bots are used as always-on sourcing tools to scan conference PDFs, identify new names, enrich data, and request intros via Slack; this aligns with company philosophy of proactive recruiting, not just resume sorting.
- Grok Bot's product philosophy treats AI as a teammate/colleague: decisions are evaluated by asking 'what would you want from a human teammate?' leading to choices like hiding internal mechanics, progressive updates, and avoiding micromanagement.
- The next shift in AI will be from reactive to proactive agents: users are experimenting with bots that monitor streams (Slack, email, X mentions) and page them only for urgent matters, acting as an always-on chief of staff that preserves focus.
- Go-to-market strategy mirrors coding tools: early adopters use it for personal projects, then demand it at work; the company is prioritizing business and team deployments, considering how bots integrate into company systems and memory.
- On moats: Roman argues moats are discovered, not planned; Cursor's success came from obsessively building useful products today, pulling future capabilities forward, and repeatedly deleting now-obsolete scaffolding, leading to distribution and data advantages.
- Cultural values include 'delete the product' (removing features when models no longer need them) and 'do the thing' (empower individuals to act without asking permission); these sustain startup speed even at scale.
- For new users, Roman recommends giving Grok Bot access to tools like Slack/email and asking it to suggest tasks it can take off your plate; for power users, create a shared store/digest where bots write outputs for easier review.
- A notable power-user pattern is running Grok Bot inside Grok Bot: e.g., a QA tester bot uses its own Grok Bot instance to test workflows and compare regressions across builds, demonstrating the flexibility of a bot with its own computer.

## Notable quotes

> "The ultimate vision of Grockbot is incredibly simple, which is you should have a team of AI bots that help you with your job and help you with your life." — Roman Ugarte
> "Once you start breaking out of this is AI chat with a set of connections instead to this is a colleague with a computer. It just raises the ceiling I think of of what you would think to give to AI." — Roman Ugarte
> "I think for me what made me so excited to work on Grockbot and contribute to it is it was the first time for non-coding tasks that I felt like I could truly delegate work to AI and not have to think about it and I would come back and it's done." — Roman Ugarte
> "It was two early decisions that at the time definitely did not feel obvious, but in hindsight I think are critical to what makes Grockbot work for people." — Roman Ugarte

## People mentioned

- Roman Ugarte
- Lenny Rachitsky
- Adam Ward
- Claire Vo
- Shub
- Steven Pressfield

## Topics

`ai-agents` `product-development` `startup-culture` `ai-assistants` `grok-bot` `cursor`
