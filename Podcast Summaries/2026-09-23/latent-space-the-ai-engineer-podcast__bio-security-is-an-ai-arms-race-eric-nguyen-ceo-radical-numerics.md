---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "🔬Bio-security is an AI Arms Race - Eric Nguyen (CEO, Radical Numerics)"
published: 2026-09-23
duration: "1h31m58s"
audio_url: "https://api.substack.com/feed/podcast/216723291/b4e6f954d64f5cbcdee95f545365848c.mp3"
episode_url: "https://www.latent.space/p/bio-security-is-an-ai-arms-race-eric"
transcript_source: youtube_autocaptions
generated_at: 2026-09-23T22:09:57Z
model: "deepseek-v4-pro"
guid: "substack:post:216723291"
---

# 🔬Bio-security is an AI Arms Race - Eric Nguyen (CEO, Radical Numerics) — Latent Space: The AI Engineer Podcast

## TL;DR

Eric Nguyen, CEO of Radical Numerics, discusses Omni, a genome language model that applies mid-training and post-training alignment to achieve state-of-the-art results on variant effect prediction and other genomics tasks. He explains the evolution from Hyena DNA and Evo to Omni, focusing on use cases like causal variant identification, chain-of-thought optimization for RNA aptamers, and rare earth mineral extraction. Nguyen also details the company's dual mandate: advancing biological design while building AI-based biosecurity tools for detection, attribution, and countermeasures against engineered pathogens.

## Key points

- Radical Numerics created some of the first generative genomics models, including Hyena DNA and Evo, which can read and write DNA, using long context up to a million base pairs to capture long-range patterns and motifs.
- Evo demonstrated generation of a novel CRISPR-Cas system by co-designing RNA and protein components from a single DNA model, and later Evo2 generated a functional bacteriophage genome from scratch, the first AI-generated genome.
- Omni, the latest model, introduces mid-training and post-training alignment (analogous to instruction tuning and RLHF) to make base genomic models production-ready for tasks like variant effect prediction, causal variant identification, and design.
- Omni achieves state-of-the-art performance on benchmarks like ClinVar and TrChim, outperforming previous models especially in non-coding regions of the human genome, where many disease-causing variants reside.
- The model scores variants using likelihood ratios of mutated vs. wild-type sequences; mid-training adds structured Q&A-style input/output pairs to improve task-specific performance beyond zero-shot methods.
- The team takes extensive measures to prevent data leakage by curating, deduplicating, and aligning training data against benchmark sequences to avoid misleading evaluations.
- Chain of thought in genomics: showing progressively higher-fitness RNA aptamer sequences with scores enabled Omni to generate new aptamers with higher predicted fitness; wet-lab validation is underway.
- Radical Numerics is applying similar progressive optimization to design proteins for rare earth mineral extraction, using surrounding genomic context to generate evolutionary diverse sequences with selective binding affinity.
- Mechanistic interpretability of model activations reveals capture of features like GC content, repeat motifs, and transcription factor binding sites, with goal of mapping disease manifolds.
- Biosecurity is a dual mandate: the company builds tools for detection, attribution, and countermeasures against engineered biological sequences, recognizing an arms race where defense lags far behind offense.
- Current biodefense relies on sequence alignment to known pathogen databases, which fails against novel or obfuscated sequences; AI-based models can detect functionally similar but sequence-divergent threats.
- Threat landscape includes both intentional state actor programs and unintentional releases; lowering expertise barriers and increasing design speed may increase overall risk volume.
- Eric notes the same models good at generating sequences are also good at discriminating pathogenic ones, making the lab uniquely suited for both design and defense.
- Future directions include multimodal models fusing DNA, RNA, protein, epigenomics, and eventually natural language, to build toward a general biological intelligence and virtual cell models.

## Notable quotes

> "A model that is good at generating, turns out is also very good at discriminating or predicting if a sequence is pathogenic or not." — Eric Nguyen
> "The design side is going to get more capable. The defensive side needs to try to get ahead. So I think inherently there is this arms race style dynamic that the defensive side has been far, far lagging. And so what we want to do is bring the defensive side to par essentially." — Eric Nguyen
> "We felt it was important as a lab that a team that was both building the design capabilities is actually also best suited for building the defense capabilities because they're basically the same models." — Eric Nguyen
> "Can you make something far more useful than now? Like that's my bars." — Eric Nguyen

## People mentioned

- Eric Nguyen
- Greg Brockman
- Brandon (host)
- RJ Honake
- Chris Ray
- Michael Pali

## Topics

`generative-genomics` `biosecurity` `ai-safety` `language-models` `synthetic-biology` `drug-discovery`
