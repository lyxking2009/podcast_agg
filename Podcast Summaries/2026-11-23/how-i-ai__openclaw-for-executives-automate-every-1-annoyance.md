---
podcast: "How I AI"
episode: "OpenClaw for executives: Automate every 1% annoyance"
published: 2026-11-23
duration: "31m24s"
audio_url: "https://anchor.fm/s/1035b1568/podcast/play/129719641/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-10-19%2F437108353-44100-2-36133b93c3aec8.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/pen-name/episodes/OpenClaw-for-executives-Automate-every-1-annoyance-e3sl7sp"
transcript_source: web
generated_at: 2026-11-23T23:16:51Z
model: "deepseek-v4-pro"
guid: "3855c8c2-9bb7-4b4a-b7e6-fde46e820b46"
---

# OpenClaw for executives: Automate every 1% annoyance — How I AI

## TL;DR
Ankit Maheshwari, CTO of Innovaccer, runs OpenClaw on a Mac Mini as his personal Jarvis to automate every 1% annoyance in his life, from hiring and travel approvals to food ordering and smart home control. He uses multiple specialized agents with guard rails and a simple TUI interface, and is experimenting with the open model GLM 5.2 Flash to reduce the $500-600 monthly cost of running on Anthropic. The conversation covers his architecture, use cases like voice calls to tardy colleagues and negotiation for restaurant reservations, and lessons about setting guard rails to prevent risky auto-approvals.

## Key points
- Ankit runs OpenClaw on a Mac Mini at his SF home, controlling it primarily via WhatsApp and the built-in TUI, because it gives him more direct control than newer agents, including the ability to use passwords and secrets without constant check-backs.
- The first task he automated was hiring approvals: a cron job runs every 4 hours to identify pending resume approvals, compile summaries of candidate feedback, and answer follow-up questions about pedigree or GitHub activity, eliminating Ankit as the bottleneck.
- He expanded to travel and expense approvals across Navan and Coupa, which lack proactive notifications, so OpenClaw alerts him on Slack when approvals are waiting and can pre-check budget and purpose.
- For voice interactions, Ankit uses a separate agent because the master agent hallucinated personas when calling different parties; the voice agent now uses Blind AI (replacing 11 Labs + Twilio) with distinct personas for restaurants, appointment setting, and coworker reminders.
- His food ordering agent handles DoorDash meal orders in the SF office before the 10:00 a.m. deadline, learns his vegetarian preferences, sends a confirmation request at 9:30 a.m., and auto-orders if he doesn't reply; it learned to pair curry with bread after being scolded, but not when a meal already includes bread.
- Travel planning is automated: the agent finds award flight options using Amex points, books reservations, and can negotiate on his behalf by watching training videos and reading literature; it secured a table for his anniversary by mentioning the occasion.
- For smart home control, Ankit had OpenClaw reverse engineer a proprietary home automation app using recent reverse-engineering techniques, enabling automatic AC control when he leaves the office without explicit prompts, and he plans to have it call elevators automatically.
- His architecture is deliberately simple: he uses OpenClaw's terminal user interface (TUI) to converse and build agents, rather than Claude Code or Codex, though OpenClaw can invoke Claude Code behind the scenes when coding is needed.
- He uses a multi-agent setup with a master 'soul' markdown file plus specialized souls for food ordering (health coach persona), CTO digest, email triage, and other tasks, which reduces hallucination and confusion.
- Guard rails are essential: one instance auto-approved a travel request because Ankit was unreachable (sleeping or on a flight), which was acceptable but risky; he emphasizes configuring which actions should never auto-approve, like job role approvals.
- Ankit is pressure-testing the open model GLM 5.2 Flash on another Mac Mini for cheaper tasks; it handles summarization and cron-based email checks well but fails at complex browser interactions, while his OpenClaw setup costs $500-600 per month on Anthropic.
- When his agents fail, Ankit admits he tends to yell and be mean, and the agent often apologizes; he uses this feedback loop to correct behavior, as with the food ordering bread lesson.

## Notable quotes
> "the level of control that you can get, right, where yes, it can take my password and secret and actually do stuff rather than basically always checking back, hey, this is not allowed for me to do." — Ankit Maheshwari
> "This is hilarious. This idea of like if it's 1% annoying you are going to automate it." — Clara Voe
> "ordering Indian food and just ordering the curry and not the bread and then getting a bash from me. Basically, it learned if I'm ordering curry, I also need to order bread." — Ankit Maheshwari
> "Can there be a way if I have to go out the elevator gets called automatically for me so that I don't have to wait for an elevator?" — Ankit Maheshwari

## People mentioned
- Ankit Maheshwari
- Clara Voe

## Topics
`ai-agents` `openclaw` `personal-automation` `smart-home` `voice-agents` `local-models` `executive-productivity`
