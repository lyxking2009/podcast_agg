---
title: "Building Durable AI Agents"
show: "Practical AI"
date: 2026-07-09
guest: "TBD"
transcript_source: rss_vtt
guid: "8b5b6bd9-8bc8-4b5c-93a8-4d660f14b6dd"
---

# Building Durable AI Agents

## Key Points

- AI agents are evolving from simple prompt-response patterns to complex, multi-step autonomous systems
- Durability in agents requires robust error handling, state management, and graceful degradation
- The distinction between "agents" and "workflows" matters — agents have autonomy and decision-making capability, while workflows follow predetermined paths
- Memory and context management are critical challenges for long-running agents
- Testing AI agents is fundamentally different from testing traditional software — you need to test for behavior, not just correctness
- Production agents need observability: logging, tracing, and monitoring of agent decisions and actions
- The cost of running agents at scale (token spend, compute) requires careful architecture decisions
- Human-in-the-loop patterns remain important for high-stakes decisions

## Overview

This episode dives into the practical engineering challenges of building AI agents that work reliably in production. The hosts discuss the evolution from simple chatbots to autonomous agents, the architectural patterns that make agents durable (error handling, state management, observability), and the unique testing challenges posed by non-deterministic AI systems. Key themes include the cost economics of running agents at scale, the importance of human-in-the-loop patterns, and the emerging best practices for agent memory and context management.

## Implications

- Agent architecture is becoming a distinct engineering discipline with its own patterns and best practices
- Observability and monitoring for AI agents will be as important as APM was for web applications
- The gap between prototype agents and production-ready agents is significant — durability requires serious engineering investment

## Notable Quotes

- "Building an agent that works in a demo is easy. Building one that works reliably for months is hard." — Host

## People Mentioned

- Chris Benson — podcast co-host (Practical AI)
- Daniel Whitenack — podcast co-host (Practical AI)

## Topics

- AI agents, durability, production systems
- error handling, state management, observability
- memory, context management, testing
- cost optimization, human-in-the-loop
