---
title: "OpenAI Eng & Dev Tools Founder: How Software Engineering Is Changing | Charlie Marsh"
show: "The Peterman Pod"
date: 2026-06-22
transcript_source: youtube_autocaptions
duration: 01:22:56
---

# OpenAI Eng & Dev Tools Founder: How Software Engineering Is Changing | Charlie Marsh

**Show:** The Peterman Pod
**Date:** 2026-06-22
**Transcript Source:** youtube_autocaptions

## Key Points

- Charlie Marsh founded Astral (the Python devtool startup behind Ruff, uv, and ty) which was acquired by OpenAI
- The cost of putting up a plausible PR has gone to zero with AI agents, but the cost of reviewing has remained the same — a fundamental shift in software engineering
- Python tooling was historically slow because it was all written in Python — Astral's key insight was building Python devtools in Rust for 10-100x performance improvements
- Ruff started as a linter because it was simpler than a type checker — the blog post 'Python tooling could be much, much faster' was the initial hypothesis that proved out
- The nine-day gap between writing about MyPy performance and publishing the 'could be much faster' post shows how quickly the proof of concept came together
- Early career software engineers face a uniquely challenging time — the profession is fundamentally changing with AI agents writing code
- The web ecosystem (ESBuild, SWC, Bun, Deno) proved that native tooling for interpreted languages was viable — Astral applied the same pattern to Python
- Building a type checker is much harder than building a linter — Astral now has a type checker (ty) but it took significant additional work beyond Ruff

## Overview

Ryan Peterman interviews Charlie Marsh, founder of Astral and now at OpenAI after the acquisition. They trace Charlie's journey from a computational biology company where he saw Python tooling limitations, through building Ruff (a Rust-based Python linter), uv (package manager), and ty (type checker). The conversation explores how AI agents are changing software engineering — the cost of writing code is approaching zero while code review remains expensive — and what this means for the profession's future.

## Implications

For software engineers: The profession is bifurcating — code generation is commoditizing while code review, architecture, and system design become more valuable. Early career engineers face particular challenges as traditional learning paths (writing code to understand code) are disrupted. For tool builders: The pattern of building native-speed tooling for interpreted languages (proven by ESBuild/SWC/Bun in JS, then Ruff/uv in Python) continues to expand. OpenAI's acquisition of Astral signals that AI labs see developer tooling as strategic infrastructure.

## Notable Quotes

- "The cost of putting up a plausible PR has gone to zero while the cost to review has remained the same" — Charlie Marsh
- "I do think it would be really hard to be an early career software engineer right now" — Charlie Marsh
- "It's such an interesting time to be building software because things are just changing so fast" — Charlie Marsh
- "Python tooling could be much, much faster — for me it was really like this is a hypothesis, could Python tooling be much faster, and then I built a prototype and the prototype was like yes I think it could be" — Charlie Marsh
- "A linter weirdly ended up being the perfect form factor because it has a pretty simple core but then you have tons of rules — it's extensible in a lot of different ways but people can get value very quickly" — Charlie Marsh

## People Mentioned

- Charlie Marsh — founder of Astral (Ruff, uv, ty), now at OpenAI
- Ryan Peterman — host of The Peterman Pod
- Andrew Kelley — referenced for data-oriented design talk

## Topics

- Python tooling
- Rust
- Ruff
- uv
- OpenAI acquisition
- AI agents
- software engineering
- code review
- developer tools
