---
title: "Why Traditional Benchmarks Fail Modern AI Models with OpenAI Research Scientist Noam Brown"
show: "No Priors"
date: 2026-06-26
transcript_source: web
---

# Why Traditional Benchmarks Fail Modern AI Models with OpenAI Research Scientist Noam Brown

## Key Points
- Traditional AI benchmarks present static scores without accounting for test-time compute — the time/tokens/cost a model spends 'thinking' on a problem
- Modern models like GPT-5.5 can continue improving for weeks or months on complex tasks; benchmarks don't capture this scaling behavior
- Benchmarks should be evaluated with a cost/compute X-axis to show performance as a function of test-time compute
- Responsible Scaling Policies (RSPs) don't adequately account for test-time compute scaling, potentially underestimating latent risk
- OpenAI's internal model disproved the Erdős unit distance conjecture at a very low compute budget — demonstrating untapped latent capabilities
- The AI release cycle (new models every few months) is faster than the time needed to fully explore a model's capabilities via long-running tasks
- Models dramatically accelerate research tasks (10-100x speedup) but lack 'research taste' and can't yet synthesize novel breakthrough algorithms from scratch

## Overview
Noam Brown argues that the AI industry's traditional benchmark grids are broken because they don't control for test-time compute. He proposes evaluating models by cost/compute, discusses how latent capabilities in existing models remain unexplored, and explores the implications for safety policy, recursive self-improvement, and multi-agent coordination.

## Implications
The benchmarking crisis has real safety implications — if dangerous capability evaluations are run at a single compute budget, we may be systematically underestimating model risks. The gap between a model's release and our understanding of its true capabilities creates a knowledge blind spot. Companies that invest in understanding test-time compute scaling will have better models AND safer deployments.

## Notable Quotes
- "The benchmarks are being presented in the wrong way. They're not controlling for the amount of test time compute that is being used on that benchmark question." — Noam Brown
- "With GPT-3, you couldn't scale test time compute... But we're seeing today with the modern models that can think for weeks even before having a performance plateau." — Noam Brown
- "The capability of the model is a function of how much money you put into it, basically." — Noam Brown

## People Mentioned
- Noam Brown — Research Scientist at OpenAI
- Sarah Guo — Co-host, No Priors

## Topics
AI benchmarks, test-time compute, AI safety, responsible scaling, recursive self-improvement, multi-agent systems, GPT-5.5, latent capabilities
