---
podcast: "The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)"
episode: "Canva Is Tackling AI Design’s Most Frustrating Problem with Stefano Corazza- #784"
published: 2026-11-10
duration: "3450"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/MLN8451107730.mp3"
episode_url: "https://twimlai.com/podcast/twimlai/canva-tackling-ai-designs-most-frustrating-problem"
transcript_source: show_notes
generated_at: 2026-11-13T23:11:05Z
model: "deepseek-v4-flash"
guid: "40e82f5e-dd1e-11f1-a32e-b33c3d6e1b8c"
---

# Canva Is Tackling AI Design’s Most Frustrating Problem with Stefano Corazza- #784 — The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)

## TL;DR
Stefano Corazza, head of AI research at Canva, explains why Canva trains custom design models instead of renting frontier ones — and how Craft splits layout planning from pixel generation while Brush uses design tools in a feedback loop.

## Key points
- The core strategic choice: train from scratch versus adapting open-weights models. Canva does both, and proprietary design data is what makes specialisation pay.
- Craft separates layout planning from pixel generation — structure first, rendering second — which makes the output editable rather than a flat image.
- Brush closes the loop by using design tools and visual feedback to create and revise compositions iteratively.
- Evaluating design quality is the hard problem: a single generation is not a fair test, so Canva uses vision-language models as judges alongside human evaluation.
- Simplified agent harnesses beat elaborate ones; faster inference matters because it buys more iteration cycles, which is where quality actually comes from.
- Owning the model creates room to optimise serving costs — a direct argument that model ownership is an economic decision, not just a research one.
- APIs and MCP are expanding access, and the design surface changes when software companies build tools for agents as well as for people.

## Overview
A technical episode in which Corazza walks through Canva’s model stack: why custom models, how Craft’s two-stage architecture works, how evaluation is done, and what changes when agents become first-class users of design software.

## Implications
For builders: the differentiator in applied AI is increasingly data and evaluation infrastructure, not base-model access. If your product’s quality depends on tight iteration loops, inference latency is a product feature.

## Notable quotes
> “Why faster inference enables more iterations, and how model ownership creates opportunities to optimize serving costs.” — episode summary
> “Evaluating design quality beyond a single generation, including the use of vision language models as judges alongside human evaluation.” — episode summary

## People mentioned
- Stefano Corazza — head of AI research, Canva
- Daniel Whitenack — TWIML co-host (not on this episode)

## Topics
- Canva
- design AI
- custom models
- open weights
- layout generation
- evaluation
- MCP
- inference cost

## Source note
Summary from the official episode description and chapter list (TWIML #784); show notes at twimlai.com/go/784. No transcript published; quotes are episode paraphrase.
