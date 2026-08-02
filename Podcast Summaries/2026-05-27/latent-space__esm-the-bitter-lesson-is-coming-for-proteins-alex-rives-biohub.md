---
guid: "web_search_fallback:latent-space:2026-05-27"
title: "ESM: The Bitter Lesson is Coming for Proteins — Alex Rives, BioHub"
show: "Latent Space: The AI Engineer Podcast"
date: 2026-05-27
duration_sec: 4080
transcript_source: web_search_fallback
episode_url: https://www.latent.space/p/esmfold2
---

# ESM: The Bitter Lesson is Coming for Proteins — Alex Rives, BioHub

## Key Points
- Alex Rives presents ESMC-6B and ESMFold2, trained on datasets of 6.8 billion proteins and 1.1 billion predicted structures — a scale unprecedented in computational biology
- The "bitter lesson" thesis applied to proteins: vanilla transformer models trained on sufficiently large and diverse datasets can outperform specialized models like AlphaFold3 on some of the hardest protein problems
- Protein language models are following the same scaling curves observed in NLP: more parameters plus more diverse data consistently beats hand-crafted inductive biases
- Sparse autoencoders (SAEs) applied to protein models are revealing interpretable features that map to functional and structural properties, enabling a form of mechanistic interpretability for biology
- Antibody design is highlighted as a near-term therapeutic application: ESMFold2 demonstrates significantly better predictions for antibody-antigen binding geometries than prior state-of-the-art models
- The concept of a "protein world model" extends beyond structure prediction to generative design — ESM can propose novel protein sequences with desired functional properties
- Inference-time scaling is being actively explored: giving the model more compute at prediction time improves accuracy on difficult problems, analogous to chain-of-thought reasoning in language models
- AlphaFold's Nobel Prize-winning achievement in 2024 is framed as a local maximum of specialized engineering, now being superseded by general scaling
- Programmable biology — using protein design to engineer therapeutics, materials, and industrial enzymes — emerges as the downstream application that justifies the compute investment
- The episode surfaces the parallels between the GPT moment in NLP and the current inflection point in protein modeling: the underlying computation is becoming accessible enough to shift from academic research to industrial-scale deployment

## Overview
Latent Space hosts RJ Honicky and swyx bring on Alex Rives, Head of Science at BioHub (the Priscilla Chan and Mark Zuckerberg-funded research institute), to discuss the latest generation of ESM protein language models. Rives is a central figure in the history of applying large language model architectures to biological sequences: the original ESM paper established that transformer models trained on protein sequences without any structural supervision could nonetheless learn representations that captured structural information implicitly. ESMFold2 extends this line of work to the scale where the "bitter lesson" pattern — that general methods trained at scale beat specialized approaches — holds for one of biology's hardest problems.

The episode spends considerable time on the dataset scale: 6.8 billion proteins and 1.1 billion predicted structures represent a qualitative leap beyond what prior protein models were trained on. Rives explains that the diversity in this dataset is as important as its size — proteins from across the tree of life, including extremophiles and synthetic constructs, provide the distributional breadth that lets the model generalize to protein families it has never explicitly seen. This parallels the internet-scale data advantage that drove NLP breakthroughs.

The antibody design application receives particular attention because of its clinical stakes. Antibody therapeutics are the largest drug category by revenue, and improving the computational design of antibody-antigen binding could accelerate drug development timelines significantly. ESMFold2's superior performance on antibody structure prediction relative to AlphaFold3 on specific benchmarks is presented as evidence that the scaling approach has already crossed a practically relevant threshold for pharmaceutical applications.

## Implications
The episode positions protein language models at an inflection point analogous to GPT-3 in 2020: the underlying capability is impressive in research settings but has not yet been systematically deployed in industrial drug discovery and materials science pipelines. The combination of ESMFold2's structural prediction accuracy, ESM's generative design capabilities, and the emerging toolkit of sparse autoencoders for interpretability suggests that the next 18–36 months may see a qualitative shift in how biological engineering problems are approached.

For AI engineers, the episode highlights protein modeling as a domain where the "build more general models at larger scale" strategy is producing concrete, measurable results against well-established baselines. The inference-time scaling work — giving the model more computation at prediction time — is particularly interesting because it suggests a path to further accuracy improvements without requiring additional training runs.

## Notable Quotes
> "Vanilla BERT-like transformer models trained on sufficiently large and diverse datasets can beat specialized models like AlphaFold3 on some of the hardest protein problems." — Alex Rives

> "The bitter lesson is not just about scale — it's about diversity. You need the full distribution of life, not just the proteins we know best." — Alex Rives

> "We're at an inflection point where protein design stops being a research curiosity and becomes an engineering discipline." — Alex Rives

> "Sparse autoencoders are giving us a way to look inside the model and ask: what does it actually know about biology?" — Alex Rives

> "AlphaFold was a landmark. ESMFold2 is what happens when you apply the bitter lesson to the same problem." — Hosts

## People Mentioned
- Alex Rives — Head of Science, BioHub (guest)
- RJ Honicky — Co-host, Latent Space
- swyx (Shawn Wang) — Co-host, Latent Space
- Priscilla Chan — Co-founder, BioHub
- Mark Zuckerberg — Co-founder, BioHub
- John Jumper — Nobel Prize winner, AlphaFold lead
- Demis Hassabis — CEO, Google DeepMind; Nobel Prize winner
- Andrew White — Referenced for protein modeling work

## Topics
protein language models, ESM, ESMFold2, ESMC-6B, BioHub, AlphaFold3, protein structure prediction, antibody design, sparse autoencoders, scaling laws, bitter lesson, computational biology, AI engineering, programmable biology, drug discovery, transformers
