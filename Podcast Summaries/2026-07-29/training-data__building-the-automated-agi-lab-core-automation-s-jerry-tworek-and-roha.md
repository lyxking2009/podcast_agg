---
title: "Building the Automated AGI Lab: Core Automation's Jerry Tworek and Rohan Anil"
show: "Training Data"
date: 2026-07-29
transcript_source: web
guid: "feef20f0-8a05-11f1-beda-77fe6808cfdf"
---

# Building the Automated AGI Lab: Core Automation's Jerry Tworek and Rohan Anil

**Show:** Training Data
**Date:** 2026-07-29
**Transcript source:** web

## Key Points
- Jerry Tworek (former OpenAI VP of Research, led o1/o3 reasoning models) and Rohan Anil (former Gemini pre-training lead with prior stints at Google Brain and Anthropic) co-founded Core Automation and argue the transformer architecture itself, not data or compute, is now the primary bottleneck to AGI.
- Transformers cannot perform true continual learning: in-context learning degrades after roughly 20 minutes of use, while fine-tuning induces catastrophic forgetting, leaving models unable to durably absorb new experience.
- Transformers have limited effective computational depth (roughly 100 layers), and chain-of-thought reasoning is a workaround that adds inference-time tokens rather than genuinely deepening computation.
- Rohan argues that treating pre-training and reinforcement learning as separate phases wastes potential; unifying optimization across both could yield roughly an order-of-magnitude improvement in capability.
- Jerry contends reinforcement learning is not the only route to learning from experience — approaches like deeper reflection or self-critique loops may ultimately prove more effective.
- A major structural bottleneck in the field is the 'kernel bottleneck': novel architectural ideas go unexploited because turning them into efficient, hardware-optimized kernels requires rare expertise, creating a five-to-six-year lag between a research idea and industry adoption.
- Core Automation's strategy is to automate kernel generation and the broader research loop itself, aiming to collapse that lag and rapidly test transformer-replacement architectures at production scale.
- Jerry defines AGI operationally as 'a model that can improve itself without a human in the loop in any way,' distinguishing it from current systems that still require continuous human guidance.
- Core Automation's flagship project, codenamed Ceres, is a continual-learning model aiming for roughly a 100-fold reduction in training-data requirements and the ability to update its weights during live production deployment, with catastrophic forgetting as the central technical challenge.
- The company frames itself as combining research, kernel engineering, and production into a single automated pipeline, positioning itself as 'the most automated AI lab in the world.'

## Overview
In this Training Data episode, Sequoia's Sonya Huang sits down with Jerry Tworek and Rohan Anil, co-founders of the new AI lab Core Automation, to make the case that the transformer architecture has hit fundamental limits. Tworek, who led OpenAI's o1 and o3 reasoning model efforts, and Anil, who previously led Gemini pre-training, argue transformers are structurally incapable of continual learning: they forget catastrophically when fine-tuned and their in-context learning ability decays after roughly 20 minutes. They also point to a hard ceiling on computational depth that chain-of-thought prompting merely papers over. The conversation turns to why better architectures haven't already displaced transformers, identifying a 'kernel bottleneck': even promising alternative architectures stall because writing efficient, hardware-tuned kernels requires scarce engineering talent, creating a multi-year lag between a research paper and a production-ready model. Core Automation's bet is to automate that kernel-engineering step, along with much of the research loop, so new architectural ideas can be tested and deployed far faster. Rohan also argues that today's rigid separation of pre-training and RL leaves significant performance on the table. Jerry closes by offering a concrete definition of AGI and ties this back to Core Automation's flagship effort, Ceres, a continual-learning model aiming to cut training-data requirements roughly 100-fold while allowing weights to update during live deployment.

## Implications
The episode articulates an increasingly vocal thesis among senior AI researchers that scaling transformers on more data and compute is running into diminishing returns, and that the next major capability jump will come from architectural innovation around continual, test-time learning. If Core Automation's premise is right, labs that remain committed purely to transformer scaling could find themselves structurally disadvantaged. The emphasis on the 'kernel bottleneck' also suggests that systems engineering — automating the translation from novel architecture to efficient, production-ready implementation — may become as strategically important as the research ideas themselves, rewarding labs that can compress the research-to-deployment cycle.

## Notable Quotes
- The bottleneck to better models and smarter systems is the architecture itself. (Jerry Tworek)
- A theoretically optimal architecture is not useful unless it comes into practice. (Rohan Anil)
- Transformers derive usefulness from things present in training; when absent, they suffer. (Jerry Tworek)
- AGI is a model that can improve itself without a human in the loop in any way. (Jerry Tworek)

## People Mentioned
- Jerry Tworek — co-founder of Core Automation, former VP of Research at OpenAI, led development of the o1 and o3 reasoning models
- Rohan Anil — co-founder of Core Automation, former Gemini pre-training lead, prior researcher at Google Brain and Anthropic
- Sonya Huang — Sequoia Capital partner and Training Data podcast co-host
- Pat Grady — Sequoia Capital partner and Training Data podcast co-host
- Andrej Karpathy — advisor to Core Automation
- Jeff Dean — advisor to Core Automation

## Topics
transformer architecture limitations, continual learning, test-time learning, catastrophic forgetting, reinforcement learning vs. pre-training, chain-of-thought reasoning limits, kernel engineering bottleneck, AGI definition, AI lab automation, Core Automation and project Ceres
