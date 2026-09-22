---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Jev: System One models for Prod, not God — with Diogo Almeida, CEO, TypeSafe AI"
published: 2026-09-21
duration: "2h20m53s"
audio_url: "https://api.substack.com/feed/podcast/216783460/c2acac966e67afb7e71574f68a44a662.mp3"
episode_url: "https://www.latent.space/p/jev"
transcript_source: rss_substack
generated_at: 2026-09-22T22:03:33Z
model: "deepseek-v4-pro"
guid: "substack:post:216783460"
---

# Jev: System One models for Prod, not God — with Diogo Almeida, CEO, TypeSafe AI — Latent Space: The AI Engineer Podcast

## TL;DR
Swyx interviews Diogo Almeida, CEO of TypeSafe AI, about Jev, a new 'System One' model class optimized for programmable AI consumed by code rather than humans. Diogo argues that RLHF and RLVR are flawed training north stars, causing hallucinations, mode collapse, and jagged intelligence, and introduces RLCD (Reinforcement Learning for Calibrated Decisions) as a better alternative. He explains Jev's API primitives, use cases, and philosophy that AI should become a reliable background utility in software, predicting an economic revolution.

## Key points
- Jev is TypeSafe's first 'System One' large programmable model, named after Jevons Paradox to signal optimization for intelligence per dollar. Diogo describes it as machine-native, designed so that code—not human chat—is the primary consumer, unlike RLHF chat-tuned models and RLVR benchmark-optimized models.
- Diogo identifies three historical RLHF branches: Christiano et al 2017 (teaching a robot backflip), Stiennon et al 2020 (learning to summarize), and Ouyang et al 2022 (InstructGPT). He says RLHF optimizes for human preference, causing mode dropping, sycophancy, hallucinations, and overconfidence because the string-based probability distribution becomes calibrated for coherence rather than epistemic honesty.
- Jev's core training technique is RLCD (Reinforcement Learning for Calibrated Decisions), an unpublished method that optimizes for 'answers with epistemically honest probabilities on System One tasks' rather than human-rated feedback or verifiable rubrics. This targets reliability, composability, and seamless integration into software control flow.
- TypeSafe rejects public benchmarks as gameable and misleading; they maintain internal evals but avoid over-relying on them to prevent self-deception. Diogo argues that true intelligence has a 'je ne sais quoi' that benchmarks can't capture, and developers should evaluate models on their specific workflows.
- Diogo claims that despite solving Millennium Prize problems in math, AI still automates almost none of the world's economically valuable work because models lack reliability and are optimized for chat rather than programmable use. He sees Jev as a plug to make AI usable for real automation.
- Jev's API offers three primitives that map to programming constructs: choice (enum switch statement), noulli (Bernoulli probability for boolean-like decisions), and score (for sorting/thresholding). Diogo says these are new types distinct from int, bool, or float to avoid confusion and enable verifiable decision-making in code.
- He recommends decomposing AI workflows into many small, independent questions with structured JSON state instead of giant system messages. This allows precise specification, easier verification, confidence thresholds, and incremental bug fixing; e.g., for refusals, ask multiple specific questions rather than one 'should I refuse?' query.
- TypeSafe operates as a data lab, not a model lab, using entirely synthetic data and surgically correcting model 'jaggedness.' Diogo says synthetic data lets them target future use cases without overfitting to present usage, and they deliberately avoid training on user data.
- Jev supports up to 256K context length with minimal degradation, best-in-class according to Diogo. TypeSafe plans long-term support (LTS) for some model versions but will iterate quickly, with all models required to stay on the Pareto frontier of intelligence per dollar.
- During launch week, Jev surpassed a trillion tokens per day with continuous machine usage, indicating real production workloads rather than demo traffic. Diogo notes signups don't matter; a single power user's for loop can dwarf all interactive queries.
- Diogo maps early use cases into four families: dark data analysis (large unstructured data hoards), real-time intelligence (e.g., e-commerce, assistants), 'smart software' (composable AI components), and coding agents. He highlights computer use, games, and entity resolution as emerging areas.
- For coding agents, Diogo critiques single-model architectures like Claude Code and Codex, arguing they are trapped by the KV cache and cannot easily decompose tasks or share state. He wrote 'KV Cache Rules Everything Around Me' and plans to publish design patterns for multi-model coding agents.
- Diogo states he would not spend $1 billion on pre-training, preferring to Frankenstein existing models and focus on post-training/data to achieve intelligence per dollar. He believes most neo-labs lack a clear direction and often destroy value.
- Jev has no safety refusals because Diogo views refusal as a type error when AI is buried in software dependencies. He argues safety alignment makes sense for consumer products like ChatGPT but is unacceptable in an API, where developers need predictable behavior.
- Diogo predicts an 'inverse SaaS-pocalypse': existing SaaS will be supercharged as AI becomes reliable enough to automate decisions in the background. He sets a goal of total factor productivity (TFP) growth above 3% within five years, calling it an AI economic revolution.

## Notable quotes

> "So the way I see it is we new-- need a new class of models. We're not attached to naming that class of models. Our-- the most accurate name we've come up with is System 1 models." — Diogo Almeida (~00:03:13)
> "Instead, GANs mode drop. They, like, drop the minority classes and just do the really common ones. And this is why this effect doesn't happen, right?" — Diogo Almeida (~00:07:54)
> "If you're a human being and you're chatting with, like, a bot or whatever, you're cloud coding, and a refusal happens, like, 'I'm sorry, I can't read DNA.py.' that's an annoying time." — Diogo Almeida (~00:12:10)
> "It's like creation is back on the menu. Though it's gonna be a wild-ass world, and buckle up. And I'm so jazzed about that." — Diogo Almeida (~00:31:34)

## People mentioned

- Diogo Almeida
- Swyx
- Yann LeCun
- Sam Altman
- Dario Amodei
- Paul Christiano
- Alec Radford
- Ryan Lowe

## Topics

`system-one-models` `rlcd` `rlhf` `calibration` `programmable-ai` `ai-reliability` `software-engineering`
