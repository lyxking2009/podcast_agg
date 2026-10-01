---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Why Dwarkesh is Wrong about Computer Use + How OpenAI shipped its Jev competitor in 1 Week"
published: 2026-09-30
duration: "39m12s"
audio_url: "https://api.substack.com/feed/podcast/218243619/bab92a493c836ce907351acda5d8c2ec.mp3"
episode_url: "https://www.latent.space/p/devday-2026"
transcript_source: rss
generated_at: "2026-10-01T23:34:57Z"
model: "deepseek-v4-pro"
guid: "substack:post:218243619"
---

# Why Dwarkesh is Wrong about Computer Use + How OpenAI shipped its Jev competitor in 1 Week — Latent Space: The AI Engineer Podcast

## TL;DR
OpenAI DevDay interviews with Ari Weinstein and Nikunj Handa cover the new Computer Use stack (Dots, GPT-6.1 Sol, Agents API, app shots) and platform updates (async tool calling, WebSockets, Decisions API). Ari argues Computer Use has transformed in the past year, now matching or exceeding average human speed and debugging skills, with next frontier superhuman performance. Nikunj details how the Decisions API shipped in about a week as a Jev-inspired fast classification layer on Luna, plus Responses API caching and compaction improvements.

## Key points
- OpenAI DevDay announcements: Dots personal assistant with its own Linux cloud computer, GPT-6.1 Sol model (1/5 cost of Astra generally, 1/7 for Computer Use), Agents API with Computer Use, app shots, and native Mac Computer Use.
- Dots enable delegation of full desktop and web tasks; each Dot has a persistent Linux VM in the cloud. Examples include meal prep ordering (2 hours reduced to 15 minutes) and YouTube automation where APIs are lacking.
- Computer Use has improved dramatically in the past year: models now debug, retry, and introspect; the harness writes JavaScript code that executes multiple actions at once; and it uses accessibility, DOM, and Playwright to reduce scrolling overhead.
- Measurement approach: multiple benchmark permutations across harness configurations; consistent gains come from both harness and model improvements; GPT-6.1 is even more cost-effective for Computer Use than its general cost reduction vs Astra.
- App shots (double-command in Codex/ChatGPT) pull raw accessibility representation and full context, not just a screenshot; this token-efficient data lets the LLM see entire pages and act on metadata like links and calendar details.
- Future of Computer Use: aiming for superhuman speed vs expert humans; current bottlenecks include model, inference, harness, and representation; waiting for website loads is a non-trivial time sink; event-driven triggering is preferred when possible.
- Safety practices: ask user consent before consequential actions like payments; restrict access to only needed websites/apps; build trust through reliability and safety checks.
- Computer Use enables agents to test the software they build, completing the software development lifecycle (build → test → deliver). It is also used for customer service, bill payment, DNS configuration, and other high-stakes tasks.
- New GPT-6 API capabilities: async function calling (pause/resume tool calls), mid-turn steering (inject messages while reasoning), WebSockets for bidirectional communication, and UltraFast for extremely low latency.
- Decisions API: OpenAI's answer to Jev, built entirely on top of existing Luna weights (not a new model); uses structured outputs, optimized inference stack, and parallel batch; includes vision from Luna; priced same as Luna; shipped in about one week.
- Decision model use cases: fast classification (support tickets), Computer Use action selection, GPT Live tool calling for snappier interactions; limitations vs full Astra-level intelligence but good enough for many tasks.
- Responses API performance focus: reduced latency (TTFT/DVD), caching improvements with 30-minute guarantee, preview of 12-hour caching, lower cache read costs (25% cheaper with new model), and pre-warming API to prepopulate cache.
- Compaction for long agent threads: Agents API has built-in compaction; Responses API offers server-side compaction and /compact manual endpoint; open-source Codex harness uses /compact; new file-based compaction techniques are being developed.
- Platform strategy: OpenAI exploring higher-level primitives (memory walls, storage concepts) beyond low-level APIs, analogous to AWS building EC2/S3; actively seeks developer feedback to shape roadmap.

## Notable quotes
> "I think the biggest delta that I see is before they could, like, reliably start tasks, but then they would run into problems, and now they’re really good at debugging. They’re really good at trying again, introspecting what is and isn’t working." — Ari Weinstein (~00:06:30)
> "I think what’s really crazy that I think, You know, the team’s accomplished over the past couple of months is that now Computer Use is, like, faster at accomplishing tasks than, like, the average human probably in most cases." — Ari Weinstein (~00:12:32)
> "Props to Jev for, like, inspiring this whole thing. obviously a bunch of people at OpenAI get nerd sniped by that, and they’re like, “How can we, like, make this work? We’re not gonna, like- train a new model.”" — Nikunj Handa (~00:23:50)
> "So this is, like, really just Luna. And, on top of that, what you’re doing is you’re constraining. So, like, structured output’s a big part of it. you’re really optimizing the inference stack to, like, get very fast on TTFD." — Nikunj Handa (~00:27:40)

## People mentioned
- Ari Weinstein
- Nikunj Handa
- Swyx
- Vibhu
- Sam Altman
- Dwarkesh Patel
- Diogo

## Topics
`computer-use` `ai-agents` `openai-api` `decision-models` `low-latency-inference` `caching`
