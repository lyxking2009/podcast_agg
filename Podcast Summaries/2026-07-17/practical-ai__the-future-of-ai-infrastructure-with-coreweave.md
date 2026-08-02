---
title: "The Future of AI Infrastructure with CoreWeave"
show: "Practical AI"
author: "Practical AI LLC"
date: 2026-07-17
duration: "50 min"
guid: "c555f52e-7f0a-403b-acf4-e3a30957582d"
transcript_source: "rss_vtt"
tags: [ai-infrastructure, coreweave, gpu-cloud, kubernetes, slurm, agentic-ai, ai-training, inference, weights-and-biases, multi-cloud]
---

# The Future of AI Infrastructure with CoreWeave

**Show:** Practical AI  
**Date:** 2026-07-17  
**Duration:** 50 min

## Key Points

- Corey Sanders (SVP of Product at CoreWeave, ex-Microsoft/Azure for 20 years) frames AI infrastructure as split into two streams: training (creating model weights) and inference (running AI applications), which are converging. Training workloads require large, deeply interconnected GPU deployments that must be pre-planned, unlike traditional cloud's "deploy compute as you need it" fungibility model.
- Legacy public cloud assumptions (built over ten years of commoditized general-purpose infrastructure) actively limit thinking about AI-native infrastructure. Sanders describes this as an "innovator's dilemma" — what made you successful before doesn't get you to the next wave, and experienced cloud people must actively question their own baseline assumptions.
- Training job failures span hardware issues (GPU failures/slowdowns, storage loading bottlenecks) and orchestration issues (knowing which job runs where to optimize output). CoreWeave built GPU straggler detection to identify individual underperforming GPUs within jobs spanning thousands of GPUs — a hard observability problem at scale.
- CoreWeave launched ARIA (AI Research and Iteration Agent), which continuously analyzes experiment results (e.g., from Weights & Biases) and recommends next steps, aiming to democratize the trial-and-error research process that currently depends heavily on individual researcher expertise and intuition.
- Sanders predicts that portal/console/dashboard experiences will fade in favor of agent-driven interaction: researchers describe desired model improvements, agents run experiments, report results overnight, and recommend next iterations — while the researcher retains final judgment.
- AI applications are evolving from single frontier-model calls into orchestrated systems of many models/agents of varying size and specialization (e.g., large frontier models for deep analysis, small cheap models for translation or spell-check), each chosen for cost/performance fit to the specific sub-task.
- CoreWeave promotes an "AI loop" workflow: production traces (via the Weave platform) reveal where an application underperforms, ARIA highlights likely fixes (prompt change, model swap, fine-tuning, or RL), changes are evaluated in a sandbox, and validated updates return to production — intended to be fast and low-friction.
- On portability vs. optimization tension: Sanders explicitly rejects the idea that CoreWeave should be a customer's only cloud, favoring a multi-cloud stance built on open-source foundations (Kubernetes, Slurm) so customers can mix providers across the loop's stages ("lock-in with love" — winning on quality, not lock-in).
- CoreWeave built Sunk ("Slurm on Kubernetes"), combining Slurm's job-scheduling strength (familiar to AI researchers) with Kubernetes' orchestration/failure-handling strength. "Sunk Anywhere" lets customers deploy this on other clouds, not just CoreWeave's infrastructure.
- On cost/access to compute: Sanders argues the goal of tools like ARIA and the AI loop is to let organizations do more with less — choosing cheaper models, fine-tuning down model size, and reducing costly trial-and-error — to help companies "one step behind the frontier labs" compete without frontier-lab-scale resources.
- In robotics/embodied AI, CoreWeave added visual (not just line-chart) experiment tracking to Weights & Biases for robotics workflows, and works directly with customers via its "direct to expert" engineering resources — bolstered by the acquisition of Monolith, which brought industrial/manufacturing domain expertise.
- Long-term vision: AI infrastructure adoption will mirror public cloud's trajectory (every enterprise eventually has an AI engineering team exceeding its on-prem team) but compressed to roughly 5-7 years instead of cloud's ~15-20 year arc, with click-based UIs becoming "legacy" in favor of AI-native interaction models.

## Overview

Chris interviews Corey Sanders, SVP of Product at CoreWeave, about how AI-specific infrastructure differs from traditional public cloud. Drawing on 20 years at Microsoft/Azure, Sanders explains that AI workloads (especially large-scale training) demand pre-planned, deeply interconnected hardware deployments rather than the elastic, commoditized "add compute as needed" model that made general-purpose cloud successful. The conversation covers CoreWeave's differentiated capabilities — GPU straggler detection, the ARIA research/iteration agent, the Weights & Biases-based experiment tracking platform, and Sunk (Slurm on Kubernetes) — and how these fit into an "AI loop" of continuous experimentation, evaluation, and deployment. Sanders positions CoreWeave as deliberately multi-cloud and open-source-based rather than seeking lock-in, and closes with a prediction that AI-native, agent-driven interaction will replace today's console/button-click interfaces within roughly five to seven years, following (and compressing) the adoption curve public cloud took over the prior two decades.

## Implications

For AI practitioners and researchers, the episode suggests infrastructure choices increasingly matter as much as model choices — GPU interconnect design, storage caching, and failure/straggler detection materially affect training cost and speed at scale. Organizations without frontier-lab resources may benefit from tools like ARIA that reduce reliance on senior researcher intuition and cut trial-and-error costs, potentially narrowing the gap with better-resourced labs. The framing of AI applications as orchestrated multi-model systems (rather than single frontier-model calls) implies engineering teams should design for per-task model selection and continuous production-trace-driven iteration rather than static prompt/model choices. The emphasis on open-source, portable building blocks (Kubernetes, Slurm) over proprietary lock-in is a competitive signal in the increasingly crowded GPU-cloud market, and the prediction of agent-driven interfaces displacing dashboards within 5-7 years is a forward-looking claim worth tracking against actual product releases.

## Notable Quotes

- "What got us successfully here today does not necessarily get us successfully to the next wave." — Corey Sanders
- "It is deep learning and regurgitation with a ton of amazing magic in what exists... but sort of the ability to now say, well, but maybe what exists is wrong. That's the difference." — Corey Sanders
- "Finding that one slowed down [GPU] and your... output of that is maybe your job is slower, but like which GPU slowed down, right? Like which GPU isn't performing? So that type of detection and observability is actually quite challenging." — Corey Sanders
- "I think this concept of portal experiences and console experiences and sort of standard views, I think they're all going to fade away... I think the experience is going to be an interaction with an agent." — Corey Sanders
- "This concept of doing inferencing, running an AI application, having thirty, fifty different models, all interacting with each other, and then finding the part that needs improvement... that's our future for those types of applications." — Corey Sanders
- "I think, you know, so I talk about this loop and I'm enthusiastic about the loop... but with the strong realization that people are not... gonna use every service that CoreWeave offers for the loop." — Corey Sanders
- "Lock in with love is sort of the joke I use." — Corey Sanders
- "I think within, you know, five to seven years, that's gonna be our world and websites with, you know, buttons that you click are gonna be legacy." — Corey Sanders

## People Mentioned

- **Corey Sanders** — SVP of Product at CoreWeave; guest; formerly 20 years at Microsoft working on Azure
- **Chris** — Practical AI podcast co-host and interviewer
