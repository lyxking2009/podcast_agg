---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Why AlphaFold Didn't Solve Protein Folding — Pushmeet Kohli, Google DeepMind & Sal Candido, Biohub"
published: 2026-10-10
duration: "31m56s"
audio_url: "https://api.substack.com/feed/podcast/219666029/8fefb52482ab31b6be94745a76cff81f.mp3"
episode_url: "https://www.latent.space/p/biohub-deepmind"
transcript_source: web_substack
generated_at: "2026-10-10T22:05:33Z"
model: "deepseek-v4-pro"
guid: "substack:post:219666029"
---

# Why AlphaFold Didn't Solve Protein Folding — Pushmeet Kohli, Google DeepMind & Sal Candido, Biohub — Latent Space: The AI Engineer Podcast

## TL;DR
This Latent Space panel with Google DeepMind's Pushmeet Kohli and Biohub's Sal Candido, moderated by Brandon Anderson, reframes the bitter lesson around data: scaling laws are not automatic, and the right data plus flexible problem-first approaches drive breakthroughs like AlphaFold. They argue that protein structure prediction is far from solved—AlphaFold 2 replicated PDB structures but proteins are dynamic, disordered, and context-dependent—and that trust, calibration, and broader biological models are needed for true clinical acceleration. Both emphasize open community efforts such as Biohub's BBI to define missing data and push toward 10x improvements in drug discovery.

## Key points
- Sal Candido argues that scaling laws are not universal: much of the work is finding situations where more compute and data yield better results. The right data must contain the information and statistics needed; a model can only extract information present in its training data and generalize beyond it.
- Training protein language models on low-quality metagenomic sequences—data that likely includes incomplete or non-real proteins—improves performance on designing real proteins, demonstrating that 'back room' data can be more valuable than pristine datasets.
- Sal cautions against simply scaling easily generated data; instead, Biohub's BBI emphasizes open community collaboration to identify the data actually needed to solve biological problems and find scaling laws.
- Pushmeet Kohli interprets Rich Sutton's bitter lesson as a warning against religious commitment to one solution (modeling vs data); problem-first flexibility is essential. In ML, optimizing only models on fixed datasets is broken; data, modeling, and expertise must all be considered.
- AlphaFold's development illustrates the tradeoff: DeepMind could not significantly augment the PDB, so they focused modeling effort on existing curated structural data, leveraging decades of global investment. In cell genomics, applying similar modeling to cell-by-gene data revealed the data was not yet sufficient for a virtual cell, prompting data generation.
- Kohli advises: define the problem first, understand constraints (data generation limits, compute budget), fail fast, and select approaches feasible for long-term impact; build multidisciplinary expertise.
- AlphaFold 2's handcrafted features were informed by biophysics—e.g., residues influence each other—giving the model an unfair advantage and making it more data-efficient. But curating good data is also an art; not just big data, but good data with the necessary coverage.
- Sal adds that inductive bias helps at smaller data scales but incorrect bias can hold back models as data scales, with a tipping point; scaling itself involves craft, including bespoke architectures beyond standard transformers and algorithmic work for efficient training and inference.
- Despite headlines, protein structure prediction is not solved: AlphaFold 2 replicated single PDB structures, but proteins are dynamic, disordered, and context-dependent; the true ground-state distribution is unknown, and function, dynamics, and design remain open problems requiring continued funding.
- Kohli's magic-wand request is to work directly on cryo-EM micrographs rather than PDB structures, because micrographs capture dynamics and distributional information that curated structures may lose; scaling models on this source remains an open challenge.
- Sal uses a bicycle analogy: current models are like modeling a spoke; the field needs to move to wheels and whole bicycles to understand real biological context. People want to use a bicycle model to design a pickup truck part, so models must incorporate specific biological contexts.
- On design vs understanding, Sal notes generative black boxes predate AI and can be useful, but models like protein language models contain latent information about structure, function, and motions; interpretability can extract world-model insights from evolutionary compression.
- Kohli distinguishes behavioral trust from internal interpretability: AlphaFold 2's GDT score was ~90 on CASP at the time, but even a higher score wouldn't matter if pLDDT confidence were uncalibrated; users need behavioral trust. AlphaFold 2 also generalized to disordered protein prediction after launch. Interpretability is relative: humans may not understand the model, but a future LLM given activations might.
- On clinical AI, Kohli says AI is already used throughout drug discovery, but dramatic acceleration requires better biological models; efforts like BBI are crucial for tackling hard biology in coming years. Sal says a fully AI-created drug timeline is unpredictable, but rapid progress is likely; he advocates 10x approaches and Biohub's mission to cure all disease.

## Notable quotes
> "You can only really pull information from the data you have and use that to generalize beyond it." — Sal Candido (~00:01:50)
> "If you approach a problem with that mindset, you might not succeed. The problem comes first, and you should be flexible in your solution space." — Pushmeet Kohli (~00:05:00)
> "I think of the models we're building now as someone who's trying to understand how a bicycle works, but is modeling a spoke on it." — Sal Candido (~00:19:15)
> "Proteins aren't blocks, and they don't act as blocks. I say proteins are the building blocks all the time, but I don't actually believe it." — Pushmeet Kohli (~00:15:35)
> "Calibration of the uncertainty measure was extremely important." — Pushmeet Kohli (~00:24:00)

## People mentioned
- Pushmeet Kohli
- Sal Candido
- Brandon Anderson
- Rich Sutton
- John Jumper
- Richard Feynman

## Topics
`protein-folding` `scaling-laws` `data-curation` `drug-discovery` `computational-biology` `model-interpretability` `ai-for-science`
