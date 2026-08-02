---
title: "OpenAI Codex lead on the new shape of product work | Andrew Ambrosino"
show: "Lenny's Podcast: Product | Career | Growth"
author: "Lenny Rachitsky"
pub_date: 2026-06-28
duration_sec: 4196
guid: "substack:post:202769820"
episode_url: "https://www.lennysnewsletter.com/p/openai-codex-lead-on-the-new-shape"
transcript_source: "web"
---

## Key Points

- **Implementation is no longer the constraint — taste is.** AI models have inverted the cost structure of software development. At OpenAI, multiple teams independently build competing, shippable implementations of the same feature; leadership's role has shifted to evaluating and curating among already-working options.
- **Taste has three layers:** aesthetic (visual/interaction details), systemic (how features relate to existing abstractions and user mental models), and curatorial (deciding what to build and how to frame solutions). Current models struggle with the systemic and curatorial layers.
- **Codex grew 6x since February 2026** to over 5 million weekly active users. Nearly 100% of OpenAI employees — not just engineers — use it weekly.
- **"Baby Codex"** — a dramatically simplified parallel codebase — lets the team explore interaction patterns rapidly without polluting production code, replacing formal design process phases.
- **Zone defense:** Designers write code, PMs own infrastructure, role boundaries blur. But Ambrosino warns against "vibe coding" culture eroding discipline-specific best practices — maintaining "command over the discipline" remains critical.
- **Roadmap planning splits into two horizons:** short-term (weeks, concrete, derived from dogfooding pain points) and long-term (6–12 months, deliberately vague — "any amount of precision you add to a nine-month plan right now is false precision"). The team maintains a **"baking features" list** of items incubated until model improvements unlock them.
- **Self-extending architecture:** Codex can write its own extensions to external software. Example: OpenAI's videographer needed Premiere Pro editing — Codex wrote its own Adobe Premiere Pro plugin, installed it, and used it to control the editing software.
- **Code review is the new bottleneck.** Code generation has accelerated so much that verification and review are now the binding constraint, not generation.
- **Harvesting user workflows into features:** Users build ad-hoc systems (memory mechanisms, spam filters, daily briefs); the team's value lies in pattern recognition and curation — identifying which personal hacks should become built-in capabilities.

## Overview

Andrew Ambrosino leads the Codex desktop app team at OpenAI. The episode's central thesis is what Ambrosino calls "the great inversion": for five decades, coding was the primary constraint in building software. AI models have reversed this — implementation is now cheap, and **taste** (aesthetic, systemic, and curatorial judgment) is the scarce resource.

The Codex desktop app launched in February 2026 and has grown 6x to over 5 million weekly active users, organically attracting non-engineers from marketing, finance, and legal despite originally being designed as a developer-hostile tool. The team chose a graphical interface over terminal-first design to support parallel agent management and multimodal capabilities.

Ambrosino describes how formal design processes (double-diamond methodology) are largely obsolete because they existed to manage implementation cost. Those functions still happen, but in different media — production code, simplified exploratory versions, or A/B tests — without the stage labels. The team uses "baby Codex," a simplified parallel codebase, for rapid exploration.

Planning is restructured around two horizons: concrete short-term roadmaps derived from actual dogfooding pain, and deliberately vague long-term plans. A "baking features" list holds items retested whenever new models arrive. Automations (scheduled tasks: hourly merge conflict resolution, daily code-change digests, automated bug detection) and Skills (bundled tool-integration instructions) are two core workflow features.

## Implications

- **PM and designer roles are transforming faster than organizations realize.** Teams that don't develop cross-disciplinary fluency while maintaining domain standards risk both stagnation and quality collapse.
- **Enterprise software incumbents face structural pressure.** Traditional advantages — complex codebases, steep learning curves, entrenched workflows — are weakening. Salesforce down ~43% YTD, Intuit down ~60% as investors contemplate AI agent cannibalization.
- **The "baking features" list is a replicable planning pattern** for any product team whose key capabilities depend on external AI model improvements.
- **Self-extending AI apps** represent a qualitative shift in what software can do: the app doesn't just use existing APIs — it generates its own interfaces to external software on demand.

## Notable Quotes

- "Implementation is actually not the expensive part anymore. It's, dare I say, taste." — Andrew Ambrosino
- "Taste is not about pixel-perfect mockups" — it's about understanding which solutions deserve investment and why.
- "Any amount of precision you add to a nine-month plan right now is false precision."
- On the new O3 Codex model: they had to "slow it down ever so slightly so you can see the words come in a little smoother."
- On zone defense: maintaining "command over the discipline" while expanding capabilities — abandoning discipline-specific best practices for "vibe coding" culture is a genuine risk.

## People Mentioned

- **Andrew Ambrosino** — Product/Engineering Lead, OpenAI Codex Desktop App
- **Lenny Rachitsky** — Host, Lenny's Podcast; product advisor
- **Thibault Sottiaux** — Co-lead, OpenAI Codex (noted on verification bottleneck)
- **John Collison** — President, Stripe (referenced on cross-disciplinary fluency as competitive advantage)
- **Brent** — OpenAI's in-house videographer (used as example of self-extending Codex workflow)
- **Paul Graham** — Referenced on taste and design

## Topics

- AI-native product development
- OpenAI Codex
- Product management transformation
- Software development workflow
- Taste and design judgment
- Planning under uncertainty
- Agentic software / self-extending apps
- Code review bottleneck
- Enterprise software disruption
- Cross-disciplinary teams
