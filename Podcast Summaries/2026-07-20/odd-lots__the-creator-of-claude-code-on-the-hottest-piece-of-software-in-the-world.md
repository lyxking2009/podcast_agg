---
title: "The Creator of Claude Code on The Hottest Piece of Software in the World"
show: "Odd Lots"
author: "Bloomberg"
date: 2026-07-20
duration: "67 min"
guid: "ef32f81a-cd8d-4184-8eb6-b48a015709b4"
transcript_source: "rss_omny_srt"
tags: [claude-code, anthropic, ai-coding, ai-agents, software-engineering, ai-safety, prompt-injection, enterprise-ai, boris-cherny]
---

# The Creator of Claude Code on The Hottest Piece of Software in the World

**Show:** Odd Lots
**Date:** 2026-07-20
**Duration:** 67 min

## Key Points

- Boris Cherny, creator and head of Claude Code at Anthropic, explains that Claude Code emerged from Anthropic's core safety mission — models are software agents that act on the world through code, so building a coding product was both a way to teach the world about AI capabilities and to learn about model safety in real-world use, not merely a commercial afterthought.
- Cherny attributes almost all of Claude Code's explosive 2026 growth to model improvements rather than harness/UI design — he traces clear "inflection points" in usage tied to Opus 4, Opus 4.5, Opus 4.6, and now Fable, each release driving a step-change in adoption.
- Claude Code is built on the same public Anthropic API available to all customers ("dogfooding") — there is no secret internal API — which Cherny says is deliberate, since using their own product the same way customers do forces continuous improvement.
- On safety: Anthropic uses alignment training, mechanistic-interpretability "neuroprobes" that can detect prompt-injection at the model's neuron level, and a sandbox (open source) restricting file/web access as layered defenses. In an external red-team competition, researchers prompt-injected every model tested except Claude/Claude Code.
- Cherny says at Anthropic itself, 100% of his own code and all of Claude Code, Cowork, and an increasing share of infrastructure/research code is now written by Claude Code — company-wide average is roughly 90%. The remaining human-written code is mostly trivial config-file edits, and he expects even that to shrink.
- He describes an emerging shift in engineering roles as coding itself commoditizes: people are segmenting into "prototypers," "builders," "maintainers," "growers/scalers," and "sweepers" (polishing/finishing work) rather than the traditional engineering/design/product/data-science split.
- New interaction surfaces are replacing the terminal as the primary interface: Cherny now uses Claude mostly through Slack (via "Tag," an agent that proactively joins conversations) and previously mobile, treating Claude more like a coworker than a tool.
- On enterprise adoption, Cherny describes an "adoption ladder" — from individual IDE use, to one-engineer-one-Claude-session, up to hundreds/thousands of concurrent Claude sessions per engineer — accompanied by spend controls, effort-level settings, and default-safe sandboxing so companies don't need to configure safety manually.
- Rollout of the most capable/dangerous models (e.g., "Mythos," a more hacking-capable variant with major cyber-offense capability) is deliberately staged and given first to vetted "good guys" via "Glass Wing," while the everyday model (Fable) is available to all customers on the same terms, largely via pay-per-token pricing rather than rate-limited subscriptions.
- Cherny pushes back on competitor narratives (referencing Microsoft's CEO and an Alex Karp/Palantir interview) suggesting reliance on Anthropic risks "the fox in the henhouse" — he argues Anthropic cannot see customer conversation data, and that continued frontier model progress makes staying on the frontier (vs. hosting your own smaller open-source model) the better bet for most businesses.
- Claude Code is now handling large-scale legacy migrations: banks use it for COBOL modernization, and an internal Anthropic example had one engineer migrate the Bun JavaScript engine from Zig to Rust in about 11 days (at roughly $150K in inference credits) — work that previously would have taken a team a year.
- Cherny argues programming languages matter less over time as models improve — even raw assembly could eventually be generated well — and predicts a possible "Cambrian explosion" of new, more niche languages rather than convergence on one dominant language, since LLMs remove much of the cost of language proliferation.
- On the SaaS-moat debate, Cherny invokes the "Seven Powers" framework: switching costs erode as Claude Code makes vendor migration trivial, but most durable businesses stack multiple moats (network effects, scale economies, cornered resources) simultaneously, so they remain largely intact even as switching-cost advantages weaken.

## Overview

Bloomberg's Joe Weisenthal and Tracy Alloway interview Boris Cherny, the creator and head of Claude Code at Anthropic, about the product's origins, its explosive growth in 2026, and the broader implications for software engineering, enterprise adoption, and AI safety. Cherny frames Claude Code less as a standalone business bet than as a natural extension of Anthropic's safety mission — since AI models interact with the world through code, building a great coding agent both teaches people what AI can do and generates safety-relevant data from real usage. The conversation covers the mechanics of prompt-injection defense (alignment, mechanistic interpretability "neuroprobes," and sandboxing), the staged rollout of more powerful/dangerous models like "Mythos" versus the broadly available "Fable," and Cherny's own admission that virtually all Anthropic code — including his own — is now written by Claude Code rather than humans. They also discuss shifting engineering roles, the move away from the terminal toward chat-based interfaces like Slack's "Tag," large-scale enterprise use cases (COBOL migrations, a Zig-to-Rust rewrite of the Bun engine in 11 days), and Cherny's responses to rival narratives that using Anthropic's hosted models risks excessive vendor lock-in or competitive exposure.

## Implications

For investors and technologists, the episode offers a detailed insider account of why 2026 became "the year of Claude Code" and what that implies for the broader SaaS and enterprise software landscape: switching-cost moats are eroding as AI-assisted migration becomes trivial, but companies with multiple stacked moats (network effects, scale, cornered resources) remain structurally protected. The staged rollout of higher-risk models (Mythos vs. Fable) previews how frontier labs may increasingly gate capability by risk profile rather than by customer tier, which has competitive and regulatory implications. The claim that ~90% of Anthropic's own code is now AI-written — including a full production language migration completed in 11 days by one engineer — is a concrete data point for how quickly engineering headcount needs and role definitions could shift at other companies adopting similar tooling. The discussion of privacy/trust concerns (the "fox in the henhouse" argument from Microsoft/Palantir) also highlights a live debate enterprises are having about whether to build on frontier hosted models versus self-hosted open-source alternatives.

## Notable Quotes

- "Programming is this kind of weird discipline... my grandfather actually programmed in the Soviet Union... on punch cards." — Boris Cherny, on the history of programming eventually leading to Claude Code
- "When I look across Anthropic, for me personally, one hundred percent of my code has been written by Claude Code since November last year... across Anthropic, I think the average is something like ninety percent Claude Code." — Boris Cherny
- "It's funny because I use... six months ago I could have given you a big list [of AI writing tics in code]. Nowadays, the code the model writes is almost every time better than the code I would have written." — Boris Cherny
- "If you just gave everyone Mythos access on day one, everyone would just kind of be hacking... it was just really important to give it to the good guys first." — Boris Cherny, on staged rollout via "Glass Wing"
- "Jared on the Bun team... migrated the entire code base from one language to another language, from Zig to Rust, and it took about eleven days for one person... in the past this would have taken like a few engineers, like a year." — Boris Cherny
- "I think they're largely irrelevant today [programming languages]... increasingly with LLMs, I think it matters less because the LLM doesn't really care." — Boris Cherny
- "The way to approach it is give this icon designer a thousand Clauds and let them be the greatest icon designer in the world." — Boris Cherny, on enterprise role friction

## People Mentioned

- **Boris Cherny** — Creator and head of Claude Code at Anthropic; guest
- **Joe Weisenthal** — co-host, Odd Lots
- **Tracy Alloway** — co-host, Odd Lots
- **Dario Amodei** — Anthropic co-founder, referenced as an author of the original scaling laws paper
- **Jared** (Bun team engineer at Anthropic) — migrated the Bun JS engine from Zig to Rust using Claude Code in ~11 days
- **Satya Nadella** (Microsoft CEO, referenced indirectly) — cited via a viral post suggesting reliance on Anthropic is risky for customers
- **Alex Karp** (Palantir CEO, referenced) — cited via a viral CNBC interview making a similar "fox in the henhouse" argument

## Topics

Claude Code, Anthropic, AI coding agents, AI safety and alignment, prompt injection, mechanistic interpretability, enterprise AI adoption, SaaS competitive moats, COBOL/legacy code migration, model rollout strategy (Mythos/Fable/Glass Wing), programming language relevance, future of software engineering roles
