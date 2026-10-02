---
date: 2026-10-02
show: Latent Space: The AI Engineer Podcast
title: "Academia is for Ambition — Alex Zhang, MIT"
guid: substack:post:218414422
transcript_source: web
duration: 1:41:27
link: https://www.latent.space/p/rlm
model: deepseek-v4-flash (Hermes manual fallback)
---
# Academia is for Ambition — Alex Zhang, MIT

## Overview

swyx and Vibhu interview Alex Zhang, MIT PhD and first author on Recursive Language Models (RLMs), in the slot Latent Space reserves each year for an emerging PhD superstar (2024 was Shunyu Yao, 2025 was Jack Morris). The conversation starts in GPU Mode and KernelBench - AI-written GPU kernels and where human expertise still beats brute-force search - then moves to research taste and why academics should take bets industry labs won't. The technical core is RLMs: treating a model's own prompt as an object in an external environment, offloading context, calling programmatic subagents, and the harness as a compositional generaliser. That leads to Prime Agent and persistent subagents, OpenAI's ~10,000-agent experiment (about 130B output tokens, roughly $40M-equivalent), Kimi's swarm approach, capability overhang, speculative programmatic tool calling, and whether English, code or a new 'Neuralese' constrains how models reason. Full public transcript on latent.space.

## Key Points

- The RLM idea: much like the 2025 shift from language models to reasoning models, Zhang argues 2026 will be about recursive language models - letting a model treat its own prompt as an object in an external environment rather than a fixed context window.
- An RLM-based harness was the first system to approximately solve ARC-AGI-3, ahead of OpenAI's Astra.
- AI-written GPU kernels still leave substantial room for human expertise - one expert insight can replace enormous amounts of brute-force token search.
- Research taste is the differentiator: SWE-bench, ReAct and Quiet-STaR all looked trivial or pointless when released, and the value of such work is that it tells a story about what the field should look like rather than shipping a product.
- Zhang's advice to PhD students is to work on problems industry labs are not looking at - if a paper looks obviously useful to an industry lab today, the comparative advantage is gone.
- Harnesses matter because they are compositional generalisers: Claude Code, Codex and Pi are structurally far more similar than they look, and harness design can improve generalisation across tasks and domains.
- RLMs decompose into context offloading, code execution, recursive subagents and shared memory.
- Prime Agent is a self-improving RLM harness for coding and long-running autonomous tasks, designed to be token-efficient via programmatic tool calling, context-as-a-variable, multi-agent messaging and a self-modifiable harness state.
- Persistent agent-to-agent communication is the frontier: the model you query in future may actually be an entire swarm or scaffold hidden behind a simple interface.
- OpenAI's experiment ran roughly 10,000 agents across ~130B output tokens at roughly $40M-equivalent cost to solve a problem - and a large share of the swarm's work is likely wasted search, with convergence still hard.
- Capability overhang: current frontier models may already be substantially more capable than the primitive systems wrapping them expose.
- Speculative programmatic tool calling - overlapping tool execution with generation - is one practical route to closing that gap.
- The closing question is representational: whether English, code, or a new 'Neuralese' is the right substrate for model reasoning.

## Implications

The most useful claim here is not about any single technique but about where the headroom is: Zhang's argument is that a large fraction of current frontier capability is being left unharvested because models are wrapped in primitive systems, and that the fix is architectural rather than a bigger model. That has an immediate practical reading for anyone building agents - harness design is a first-class engineering surface, and programmatic tool calling plus persistent subagents are the levers. It also has a sober counterweight: the OpenAI swarm experiment's ~$40M-equivalent cost against largely wasted search suggests that multi-agent scale is currently an expensive way to buy search, not a free lunch.

## Notable Quotes

- Alex Zhang: 'I think the most successful research from grad students or like in academia comes when people care about problems that maybe most people in industry are not looking at.'
- Alex Zhang: 'I think when you get a reaction like that, it's almost like a good sign in the sense that it's clear that people aren't thinking about what the purpose of this is.'
- Alex Zhang: 'When SWE-bench came out, Ofir loves to tell this story. Like nobody cared. Everybody was like, this is an impossible task. Like why would we ever even consider this as a benchmark?'
- Alex Zhang: 'You tend to see that a lot of ideas... the value of the paper comes from, it tells a bit of a story as to what you want the field to look like.'
- Alex Zhang: 'I have yet to see an example in the wild of, like, we bootstrap the ability to solve a very difficult class of problems without any examples.'
- Alex Zhang: 'As a PhD student, I think you're in such a unique position where you can work on literally whatever you want for the most part. If you're not taking advantage of that...'

## People Mentioned

- Alex Zhang - MIT PhD; first author, Recursive Language Models; GPU Mode; KernelBench
- swyx (Shawn Wang) - co-host, Latent Space
- Vibhu Sapra - co-host, Latent Space
- Mark Saroufim - GPU Mode; credited with bringing Zhang into the community
- Ofir Press - SWE-bench team, Princeton (cited)

## Topics

recursive language models, RLMs, GPU kernels, KernelBench, GPU Mode, research taste, harnesses, multi-agent swarms, Prime Agent, OpenAI, capability overhang, programmatic tool calling, Neuralese, AI for science
