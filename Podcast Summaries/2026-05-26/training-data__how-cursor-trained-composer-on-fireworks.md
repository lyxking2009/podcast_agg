---
guid: c85a76ee-3dd8-11f1-8934-37efe54dd294
title: "How Cursor Trained Composer on Fireworks: Distributed Infrastructure for High-Performance RL"
show: "Training Data"
date: 2026-05-26
duration_sec: 2733
transcript_source: web
episode_url: 
---

# How Cursor Trained Composer on Fireworks: Distributed Infrastructure for High-Performance RL

## Key Points
- Cursor and Fireworks collaborated to build Composer 2, a specialized foundation model for agentic software engineering, using Kimi K2.5 (a 1.04T parameter MoE model with 32B active parameters) as the base
- Rather than starting from pre-training, they took a top-down approach: mid-training and large-scale RL on top of an open-source base, then specializing the model around real Cursor usage patterns
- Fireworks solved the distributed RL infrastructure problem by discovering that ~98% of model weights remain bit-equivalent between consecutive RL checkpoints, enabling delta-compressed updates of ~20 GiB instead of the full 1 TB
- The Composer 2 RL training run was distributed across 3–4 geographically dispersed clusters spanning the US and Europe, demonstrating that frontier RL does not require massive co-located hardware
- Composer 2 achieved 61.3% on CursorBench (37% relative improvement over Composer 1.5), 73.7% on SWE-bench Multilingual, and 61.7% on Terminal-Bench
- The team implemented a self-summarization technique in RL training, allowing the model to compress 100,000+ token trajectories into ~1,000-token summaries, enabling training on tasks far longer than the model's context window
- Real-time RL updates from production usage allow checkpoint refreshes as frequently as every five hours, creating a continuous improvement loop from actual developer interactions
- CursorBench is designed around real Cursor engineering tasks, with a median of 181 lines changed per task (vs. 7–10 for SWE-bench) and shorter, more ambiguous prompts reflecting realistic developer requests

## Overview

Cursor's Federico Cassano and Fireworks' Dmytro Dzhulgakov joined Sonya Huang on the Training Data podcast to explain how they jointly built Composer 2 — a specialized foundation model purpose-built for agentic software engineering inside Cursor. The episode unpacks a core architectural insight: models have finite weight capacity, and concentrating that capacity entirely on software engineering tasks makes the model simultaneously better at the task and far more inference-efficient. Instead of the conventional bottom-up approach of starting from scratch with pre-training, the team adopted an unconventional top-down strategy — mid-training and reinforcement learning on top of Kimi K2.5, an open-source 1.04 trillion parameter Mixture-of-Experts base, to get a capable model into users' hands quickly, then iteratively specializing it using real Cursor usage data.

The training pipeline involves two phases: continued pre-training on a code-heavy data mix to deepen the base model's coding knowledge, followed by large-scale RL in authentic Cursor environments using the same tools and problem distributions that reflect actual developer requests. A critical finding from the RL phase is that it improves both average performance and best-of-K sampling simultaneously — evidence that the model is learning genuinely new solution paths rather than simply concentrating probability on already-known ones. The training infrastructure uses Ray and PyTorch with four decoupled services (training, environments, inference, evaluations), custom CUDA kernels optimized for NVIDIA Blackwell GPUs, and Anyrun — an internal platform managing hundreds of thousands of sandboxed coding environments using Firecracker VMs.

The self-summarization capability represents a particularly novel contribution: rather than relying on external compaction methods, the team incorporated summarization as a trained behavior within the RL process itself. Each training rollout can involve multiple generations chained together by model-generated summaries, with the summaries themselves becoming part of what gets rewarded. This allows training signal to flow from trajectories far longer than the model's context window, producing self-summaries of ~1,000 tokens that outperform 5,000-token baseline summaries while using one-fifth the tokens. A real-time RL loop further closes the gap between training and production: Cursor serves model checkpoints live, collects user interaction signals as rewards, and can update the deployed model checkpoint as frequently as every five hours.

## Implications

The Cursor-Fireworks collaboration demonstrates that frontier-class RL training no longer requires monolithic co-located GPU clusters. Fireworks' key discovery — that over 98% of model weights remain unchanged between consecutive RL checkpoints — transforms the economics of distributed training by reducing cross-region weight transfer from ~1 TB to ~20 GiB per update. This means operators can aggregate geographically stranded GPU capacity across different regions and cloud providers into a single coherent training run, dramatically lowering the infrastructure barrier to frontier RL. The Composer 2 run spanning 3–4 global clusters is a proof point that specialized AI companies can compete on model quality without hyperscaler-scale co-located infrastructure.

More broadly, the episode illustrates the power of domain specialization as a training strategy. By focusing all model capacity on a single application domain and training with real user data via real-time RL, Cursor achieved frontier coding performance at a fraction of the inference cost of general-purpose frontier models. This points toward a broader trend: the most competitive AI products will likely be those that continuously close the loop between production deployment and model training, using actual user interactions as the richest possible source of reward signal rather than relying solely on synthetic benchmarks or academic datasets.

## Notable Quotes

> "Models have finite capacity in their weights, and allocating all those bits to the singular task of software engineering in Cursor frees the model to be both better at the task and far more efficient at inference." — Federico Cassano

> "You do not need to move the full 1 TB on every update. More than 98% of weights in bf16 format remain bit-equivalent between consecutive checkpoints." — Dmytro Dzhulgakov (Fireworks blog)

> "The Composer 2 RL run was distributed across 3 (sometimes 4) different clusters around the world." — Federico Cassano

> "RL training enhances both average and best-of-K performance — the model is learning new solution paths rather than just concentrating on known ones." — Composer 2 Technical Report

> "Self-summary consistently reduces the error from compaction by 50% while using one-fifth the tokens." — Federico Cassano & Sasha Rush (Cursor blog)

> "We serve model checkpoints to production, observe user responses, and aggregate those responses as reward signals — enabling checkpoint updates as frequently as every five hours." — Cursor real-time RL blog post

> "Rather than start from pre-training and work up, they took an unconventional top-down approach — mid-training and RL on top of an open-source base to get a useful model into users' hands fast, then specializing the model around real Cursor usage." — Training Data podcast description

## People Mentioned

- Federico Cassano (Cursor, research lead on Composer 2)
- Dmytro Dzhulgakov / "Dima" (Fireworks AI)
- Sonya Huang (host, Sequoia Capital)
- Sasha Rush (co-author of self-summarization work with Federico Cassano)

## Topics

Reinforcement learning at scale, distributed training infrastructure, weight delta compression, asynchronous RL, domain-specialized foundation models, Mixture-of-Experts architecture, agentic coding, self-summarization, real-time RL from production data, CursorBench evaluation, SWE-bench, Kimi K2.5, Fireworks AI inference platform, NVIDIA Blackwell GPUs, Firecracker VMs, reward hacking, KL divergence estimation
