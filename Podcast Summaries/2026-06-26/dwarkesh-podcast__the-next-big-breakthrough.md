---
title: "The next big breakthrough will be AIs learning on the job"
show: "Dwarkesh Podcast"
date: 2026-06-26
transcript_source: web
---

# The next big breakthrough will be AIs learning on the job

## Key Points
- Labs are betting on RLVR (Reinforcement Learning from Verifiable Rewards) across thousands of environments as the path to AGI, but this approach has fundamental limitations
- The core problem: many real-world skills (building a business, winning court cases) can't be recreated in deterministic simulators for training — the data is idiosyncratic, sparse, and unverifiable
- 30-50% of a lab's compute goes to inference, which currently doesn't improve the model — deployment reveals the most valuable data about real-world mistakes
- On-Policy Self-Distillation (OPSD) is a promising technique for continual learning that trains the base model to match a teacher with accumulated session experience
- 'Dreaming' — using compute to build and train against internal simulations — could be a fourth scaling axis, like EfficientZero playing many simulated games per real step
- Short-horizon RL training may not generalize to long-horizon performance, as Dario Amodei has noted about context length scaling

## Overview
Dwarkesh argues the AI paradigm must shift from one-time pre-deployment training to continuous learning from real-world deployment. Labs are currently wasting the most valuable data — what happens when models are actually used in organizations. The article critiques the RLVR optimism, noting that verifiable and grindable domains are limited, and proposes OPSD and 'dreaming' as techniques for continual learning that updates model weights rather than just expanding context windows.

## Implications
The AI industry's current training paradigm may be fundamentally incomplete. Labs that crack continual learning from deployment data will have a massive competitive advantage. The 30-50% of compute spent on inference represents an enormous untapped resource for model improvement. This suggests the 'AGI via scaling RLVR' thesis may be too narrow.

## Notable Quotes
- "But this doesn't work with computer use — at least not trivially. You can't have a thousand agents go try the same checkout flow on Amazon.com. Because Andy Jassy will find and detect your bots and shut your ass down."
- "What is the RL environment to make an AI as good at politics as Lyndon Johnson, or as good at building a space launch business as Elon Musk?"
- "What's actually happening in the organizations I'm being used at? And what kind of mistakes do I tend to make in the real world?"

## People Mentioned
- Dwarkesh Patel — Podcaster and AI researcher
- Dario Amodei — CEO of Anthropic (referenced)

## Topics
AGI, reinforcement learning, continual learning, RLVR, test-time compute, self-distillation, dreaming, sample efficiency, inference compute
