---
title: "Boris Cherny: Building Claude Code"
show: "Y Combinator Startup Podcast"
date: 2026-07-28
transcript_source: web
guid: "33764cbe-a73b-42e1-97b6-91d55ddb908f"
---

# Boris Cherny: Building Claude Code

**Show:** Y Combinator Startup Podcast
**Date:** 2026-07-28
**Transcript source:** web

## Key Points
- Boris Cherny, creator and Head of Claude Code at Anthropic, joined YC partner Diana Hu at Startup School 2026 one day after Opus 5 shipped to discuss what the new model unlocks for builders.
- Opus 5 scored 30% on ARC-AGI-3, roughly tripling the next-best competitor and up from low single digits for prior models.
- In 'auto mode,' Opus 5 can run autonomously for days, weeks, or even months without stopping or needing external scaffolding.
- Anthropic deleted about 80% of Claude Code's system prompt after Opus 5 shipped and rebuilt it in roughly two weeks; the model got smarter, not dumber, with far less instruction.
- Cherny's 'press delete' philosophy: treat each new model generation as a reason to throw out prior prompts/scaffolding and rebuild empirically rather than accumulate legacy instructions.
- A new 'simple=1' mode lets users strip Claude Code's system prompt entirely to test the raw model's behavior.
- Cherny frames the gap between what a frontier model can actually do and what a product exposes as 'product overhang'; products that suppress that capability are 'hobbling' the model.
- His advice to founders: give the model a task that's slightly too hard, give it tools to verify its own work, and 'let the model cook' rather than over-constraining it.
- Prompt-injection defense now uses a three-layer approach combining alignment training, activation-based classifiers, and an auto-mode classifier; Diana Hu noted they could no longer demonstrate a successful injection in testing.
- As a stress test of autonomy, Claude Code rewrote large parts of the Bun JavaScript runtime (100K+ lines) from Zig to Rust in about 11 days.
- Anthropic now runs roughly 20-30 autonomous daily maintenance workflows across its own codebases and can spawn thousands of parallel agents from a single instruction like 'use a workflow.'
- Cherny: 'Coding is solved for the kind of coding that I do. It's not solved for everyone' — distinguishing well-specified tasks from harder domains like complex or distributed systems.

## Overview
Boris Cherny, the creator and Head of Claude Code at Anthropic, sits down with YC's Diana Hu at Startup School 2026, recorded the day after Anthropic shipped Opus 5. The conversation centers on what the new model changes about building AI products: Opus 5 jumped to 30% on ARC-AGI-3 and can run autonomously in 'auto mode' for days, weeks, or months without stopping. Cherny explains that Anthropic's practice is to 'press delete' rather than layer more instructions onto Claude Code as models improve — the team cut about 80% of Claude Code's system prompt after Opus 5 launched and rebuilt it in about two weeks, finding the leaner prompt made the model perform better. Much of the discussion focuses on 'product overhang': the gap between what a frontier model can actually do and what any given product lets it do. Cherny's advice to founders is to hand models tasks that are slightly too hard, equip them with ways to verify their own output, and 'let the model cook.' He describes autonomy demonstrations including Claude Code rewriting large portions of the Bun JavaScript runtime from Zig to Rust in about 11 days. On safety, Cherny and Hu discuss a three-layer defense against prompt injection, with Hu noting the team could no longer reliably demonstrate a successful injection. Cherny closes on the idea that coding is 'solved' for well-specified work, but not for everyone, and that the right posture toward each new model generation is empirical.

## Implications
The episode captures a broader shift Anthropic is betting on: as models gain raw capability and self-verification ability, the winning product strategy moves from constraining the model with elaborate prompts toward stripping scaffolding away and letting the model exercise more of its own judgment — directly relevant for startups building on frontier LLMs, since scaffolding-heavy products risk becoming liabilities as underlying models absorb capabilities that used to require bespoke engineering. The reported progress on prompt-injection defense and long-horizon autonomous operation signals agentic coding tools are approaching a threshold where they can be trusted with larger, less-supervised chunks of real engineering work.

## Notable Quotes
- It can go for days, weeks, months at a time. It just won't stop. (Boris Cherny)
- Coding is solved for the kind of coding that I do. It's not solved for everyone. (Boris Cherny)
- You have to give it a task that's too hard. You have to give it the tools to verify. (Boris Cherny)
- All you have to say is 'use a workflow.' That's it. (Boris Cherny)
- We just cannot demonstrate prompt injection anymore. (Diana Hu)

## People Mentioned
- Boris Cherny — Creator and Head of Claude Code, Anthropic
- Diana Hu — Partner, Y Combinator

## Topics
Claude Code, Opus 5, ARC-AGI-3 benchmark, autonomous agents / auto mode, prompt injection defense, system prompt design, product overhang and unhobbling AI, Bun runtime rewrite, AI coding automation, empirical prompting methodology
