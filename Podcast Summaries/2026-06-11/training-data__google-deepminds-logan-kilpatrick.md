---
title: "Google DeepMind's Logan Kilpatrick: Why the Model Eats the Harness"
show: "Training Data"
author: "Sequoia Capital"
date: 2026-06-11
duration: "51:09"
transcript_source: web
guid: c8ac5d90-6517-11f1-83d7-d3b49f07aa8f
---

# Google DeepMind's Logan Kilpatrick: Why the Model Eats the Harness

## Key Points
- Google's Gemini 3.5 Flash is ~3x faster than other large models at significantly lower cost, benchmarking at ~280 tokens/second. It drives agentic/coding workflows and will be deployed across Gemini App, AI Mode in Search, Antigravity IDE, and Gemini Spark.
- The core strategic philosophy: "The model eats the scaffolding" — as models improve, they absorb functionality that was previously handled by external harness/tooling code. DeepMind ships a full agent harness (Antigravity), not just bare models.
- Google leads with cost-adjusted efficiency, not absolute capability. No "Ultra" tier was announced at I/O — a deliberate strategic choice. Internal data shows latency and cost matter more than marginal quality improvements for 2B+ user products.
- The model lineup: Pro (frontier quality), Flash (sweet spot — smart, fast, cost-effective), Flashlite (maximum speed/cost efficiency), Deep Think (runtime test-time compute scaling as a "fourth dimension").
- Gemini Omni Flash is Google's "nano banana moment for video" — native video generation/editing with personal avatar insertion and scene composition from multiple images. Part of Google's push toward "all modalities in, all modalities out."
- Gemini Spark (coming 1-2 weeks after I/O) brings more agentic functionality into the Gemini app, built on 3.5 Flash.
- "Harness diversity" is an explicit training objective — models must generalize beyond Google's own harness. Kilpatrick proposes a "Harness Bench" benchmark to test whether models work across different harnesses.

## Overview
Logan Kilpatrick (Member of Technical Staff, Google DeepMind) and Tulsee Doshi (Sr Director & Head of Product for Gemini models) join Nathan Labenz for a first-ever in-person episode recorded days before Google I/O 2026. They discuss headline launches including Gemini 3.5 Flash, the Omni video generation model, and Gemini Spark, while exploring DeepMind's strategic philosophy of "the model eats the scaffolding" — the idea that each generation of models absorbs more functionality from external tooling. The conversation covers model psychology, AI welfare, recursive self-improvement, and why Google deliberately prioritizes efficiency and cost over absolute capability for its 2B+ user products.

## Implications
- The "model eats the scaffolding" thesis suggests that heavy investment in agent harness/tooling infrastructure may have a short shelf life — models will progressively internalize these capabilities.
- Google's efficiency-first strategy (Flash over Ultra) is a deliberate bet that serving 2B+ users at low latency matters more than frontier benchmarks — a contrarian position vs. competitors pushing absolute capability.
- The proposed "Harness Bench" benchmark would be a significant addition to model evaluation — testing cross-harness generalization rather than performance on a single harness.
- The convergence of video generation (Omni), agentic products (Spark), and voice interfaces (Gemini Mic) positions Google as the most vertically integrated AI platform — all modalities in, all modalities out.
- Antigravity IDE's "slash teamwork" (sub-agents completing tasks in parallel) represents the next evolution of coding assistants from single-agent to multi-agent orchestration.

## Notable Quotes
- **Logan Kilpatrick:** "The model eats the scaffolding. That's my favorite way of thinking about this. Just as at every crank of the model flywheel, the model eats a bunch of scaffolding."
- **Tulsee Doshi:** "We've seen so much value from the flash and the flashlight dimensions because we also see an extremely large number of users... latency really matters, cost matters. Because actually you find that users aren't willing to wait — even when we tweak the model and hurt latency, we actually see that play out in our live experiments on search and the app, even if the model is hugely better from a quality perspective."
- **Tulsee Doshi:** "We want any of our enterprise customers or a developer who's building their own use case to be able to leverage Gemini effectively. And so it is important then from a model standpoint that we're training in such a way that we actually support a range of different approaches to tooling."
- **Logan Kilpatrick:** On harness diversity as a training objective: "Proposed benchmark idea: 'Harness Bench' — testing whether models work across different harnesses, a 'jagged intelligence' test."

## People Mentioned
- **Logan Kilpatrick** — Member of Technical Staff, Google DeepMind (leads AI Studio and Gemini API)
- **Tulsee Doshi** — Sr Director & Head of Product for Gemini models, Google DeepMind
- **Nathan Labenz** — Host, The Cognitive Revolution / Training Data

## Topics
- Gemini 3.5 Flash, Gemini Omni, Gemini Spark, Antigravity IDE, agent harness, model efficiency, cost-adjusted performance, video generation, Google I/O 2026, AI agents, multimodal AI, harness diversity, Deep Think, test-time compute, model psychology, AI welfare, recursive self-improvement
