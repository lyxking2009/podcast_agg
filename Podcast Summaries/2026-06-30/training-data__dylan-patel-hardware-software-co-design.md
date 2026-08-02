---
title: "Why Hardware-Software Co-Design Is AI's Real 100x: Dylan Patel of SemiAnalysis"
show: "Training Data"
author: "Sequoia Capital"
date: 2026-06-30
guid: fb9d2daa-719d-11f1-9e13-9f44da352371
transcript_source: youtube_autocaptions
---

## Key Points

- NVIDIA's CUDA monopoly in machine learning is breaking. PyTorch has become the dominant research framework and its abstraction layer is enabling hardware competitors to build viable alternatives to NVIDIA GPUs for both training and inference.
- The semiconductor supply chain for AI is far more complex than most people appreciate — from chemical inputs to fabs to design IP. Dylan Patel's SemiAnalysis specializes in tracking these supply chain dynamics that determine who wins in AI hardware.
- Hardware-software co-design is the real 100x opportunity in AI. The companies that will win are not those building the fastest chips in isolation, but those optimizing the full stack — chip architecture, compiler, runtime, and model architecture together.
- The compute budget for AI training is shifting dramatically. New model architectures and training techniques (like mixture of experts, sparse attention) require fundamentally different hardware optimizations than the dense transformer workloads that NVIDIA GPUs were designed for.
- Inference is becoming the dominant cost center as AI deployments scale. The economics of inference (latency, throughput, cost per token) are different from training, creating opportunities for specialized hardware that wouldn't make sense for training workloads.
- The job posting data showing TensorFlow dominance over PyTorch in enterprise is misleading — it reflects legacy hiring patterns and HR keyword stuffing, not actual compute budget allocation. The real signal is where the compute dollars flow, which increasingly favors PyTorch-based workloads.

## Overview

Dylan Patel, founder and chief analyst at SemiAnalysis, discusses the evolving AI hardware landscape and why hardware-software co-design is the key to unlocking the next 100x improvement in AI compute efficiency. The conversation covers the breaking of NVIDIA's CUDA monopoly (enabled by PyTorch's abstraction layer), the complexity of the semiconductor supply chain, and the shift from training-dominated to inference-dominated compute budgets. Patel argues that specialized hardware optimized for specific model architectures (rather than general-purpose GPUs) will define the next generation of AI infrastructure, and that companies optimizing the full stack — from silicon to software — will capture the most value.

## Implications

The episode signals that the AI hardware market is entering a new phase where NVIDIA's dominance is no longer guaranteed. The combination of PyTorch abstraction, new model architectures (MoE, sparse), and inference economics creates genuine openings for specialized chip companies. For AI practitioners, this means the hardware landscape will fragment — different workloads will increasingly run on different optimized hardware, rather than everything running on NVIDIA GPUs. This fragmentation could reduce costs but increase complexity in deployment decisions.

## Notable Quotes

- "Software doesn't exist without hardware; every line of code written today is ultimately a request for a specific movement of electrons through a physical gate in Taiwan." — Dylan Patel
- "Every semiconductor company in the world sucks at software — except for Nvidia, which has spent twenty years building a moat that competitors can't simply code their way out of." — Dylan Patel

## People Mentioned

- Dylan Patel — founder and CEO of SemiAnalysis

## Topics

AI hardware, semiconductors, CUDA monopoly, PyTorch, hardware-software co-design, inference optimization, supply chain, NVIDIA competition, AI infrastructure
