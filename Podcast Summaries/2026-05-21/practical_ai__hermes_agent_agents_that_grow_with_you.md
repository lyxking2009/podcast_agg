---
guid: 2883cd96-a811-4539-967a-3705c330d3dc
title: "Hermes Agent: Agents that grow with you"
show: "Practical AI"
date: 2026-05-21
duration: "51:42"
transcript_source: rss_vtt
---

## Key Points
- Hermes Agent was originally built as an internal tool at NousResearch to automate AI model research tasks, then open-sourced and became the number one open source repository on GitHub
- The core differentiator of Hermes Agent is that it gets better the more you use it, through two main emergent systems: a skill system and a hierarchical memory system
- The skill system works by allowing the agent to autonomously recognize when it has learned something useful and create a reusable "skill" for future similar tasks, without the user explicitly telling it to do so
- The model-harness relationship is analogous to brain and body: the model provides intelligence, but the harness enables the model to act in and affect the real world
- A better harness with a weaker model can outperform a better model with a weaker harness, showing that surrounding infrastructure matters as much as the underlying AI
- The open source AI landscape has shifted: Meta's commitment has waned after Llama 4's underperformance, Chinese companies have adopted open source as a growth hack, and NVIDIA has pledged ~$20B to support Western open source models
- NousResearch was founded by a Discord community who cared about keeping AI open, and formalized into a company roughly two years ago
- Hermes Agent is now used internally at NousResearch with MCP-connected agents that have built up organizational knowledge autonomously over time
- Jeffrey advises users to describe desired outcomes and evaluation criteria rather than prescribing step-by-step instructions to get the most from AI agents
- Jeffrey reflects on the broader societal risk of AI reducing human critical thinking, framing NousResearch's mission as "human-centric AI" that makes people better over time

## Overview
Jeffrey Quesnelle, cofounder and CTO of NousResearch, joins Practical AI to discuss the evolution of his organization from an open source Discord community into a company, and the emergence of Hermes Agent — an open source AI agent harness that learns and improves through use. The episode traces how NousResearch went from publishing efficiency research and fine-tuned models (the original Hermes model series) to building an agentic framework centered on recursive self-improvement via persistent skills and memory. Jeffrey explains the architectural philosophy behind Hermes Agent: rather than hard-coding capabilities, the system is designed to get out of the way of the model and let it develop emergent behaviors through self-reflection.

The conversation explores how the model-harness distinction shapes the future of AI products, how open source AI is evolving amid geopolitical pressures and NVIDIA's $20B commitment to Western open source, and how individuals and organizations should rethink workflows to take advantage of agents. Jeffrey also shares honest reflections on the human implications of AI, including the risk of eroding critical thinking across generations.

## Implications
Hermes Agent's emergent skill and memory systems suggest a new design paradigm for AI tooling: rather than building static features, developers can achieve compounding capability by enabling models to self-reflect and learn from their own usage history. Organizations that deploy agents early and let them accumulate organizational knowledge — as NousResearch has done internally — may gain significant leverage over those that treat AI as a one-shot query system. The shift toward outcome-based prompting (describing goals and evaluation criteria rather than steps) represents a fundamental change in how humans need to communicate with AI systems, requiring new skills around explicit articulation of assumptions that humans typically leave unstated.

## Notable Quotes
> "The agent ought to get better the more you use it. That was kind of like the motivating factor here." — Jeffrey Quesnelle

> "The model is your brain and the harness is your body." — Jeffrey Quesnelle

> "A better harness with a worse model can beat a better model with a worse harness." — Jeffrey Quesnelle

> "Think of the agents as humans with infinite patience, but very little creativity. Where would I need that? If there's a place in your workflow that doesn't require creativity, but requires infinite patience, then that's a great place to put it." — Jeffrey Quesnelle

> "Don't tell it how to do something. Describe the outcomes and the conditions for what it is that you're trying to get to." — Jeffrey Quesnelle

> "Explain it to me like I'm an alien, because that's really what the AI is. It's an alien that never grew up on earth." — Jeffrey Quesnelle

> "There's never been a time more where a single person's leverage multiplied can be maximized. AI is a human capability multiplier." — Jeffrey Quesnelle

> "At Noose, we call ourselves being human centric AI, and that AI should make you better today than yesterday and better tomorrow than today." — Jeffrey Quesnelle

## People Mentioned
- Jeffrey Quesnelle — Cofounder and CTO, NousResearch
- Daniel Whitenack — Host, Practical AI; CEO at Prediction Guard
- Chris Benson — Host, Practical AI; Principal AI and Autonomy Research Engineer
- Technium — NousResearch team member who architected and built Hermes Agent using AI tooling despite limited coding background
- Karen — Described as one of the world's best LLM prompt engineers at NousResearch
- Jensen Huang — NVIDIA CEO; committed ~$20B to Western open source model training at GTC keynote
- Mark Zuckerberg — Meta CEO; credited with driving Meta's original open source AI (Llama) commitment

## Topics
- Hermes Agent architecture and design philosophy
- Emergent skill systems in AI agents
- Hierarchical memory systems for persistent agent learning
- Model vs. harness distinction in agentic AI
- Open source AI landscape and geopolitical dynamics
- NousResearch history and company evolution
- Recursive self-improvement in AI systems
- Outcome-based vs. step-by-step prompting
- NVIDIA's commitment to Western open source AI
- Llama model series and Meta's evolving open source posture
- DeepSeek and Chinese open source AI as geopolitical phenomenon
- Multi-tenant and enterprise agentic workflows
- Human implications of AI on critical thinking and society
- MCP (Model Context Protocol) for agent-infrastructure integration
- AI as a human capability multiplier for individuals
