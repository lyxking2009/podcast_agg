---
title: "The Coolest Diffusion Research Isn't in LLMs — Evan Feinberg & Sergey Edunov, Genesis Molecular AI"
show: "Latent Space: The AI Engineer Podcast"
author: "Latent.Space"
date: 2026-07-01
guid: web_search_fallback:latent-space-the-ai-engineer-podcast:2026-07-01
transcript_source: web_search_fallback
---

## Key Points

- Sergey Edunov, who led Llama training at Meta, left to join Genesis Molecular AI after recognizing that the most innovative diffusion research is happening in 3D structure prediction for drug discovery, not in LLMs.
- Genesis's PEARL (Place Every Atom at the Right Location) model achieved superior zero-shot performance on the OpenBind benchmark — 802 previously unseen protein-ligand complexes — without any fine-tuning.
- The standard 2-angstrom RMSD threshold used in academic benchmarks is inadequate; speakers argue 1 angstrom is necessary for accurate interaction modeling, since hydrogen bonds operate within just 0.6 angstroms.
- The drug-like molecule search space contains roughly 10^60 candidates; multi-parameter optimization makes the problem harder because strong binders frequently conflict with required absorption and pharmacokinetic properties.
- PEARL excels at modeling protein flexibility during ligand binding (induced fit), a notoriously difficult aspect of structure prediction that prior models struggle with.
- Genesis's internal agentic system, SAPPHIRE, operates like a virtual chemist: it analyzes molecular poses, forms hypotheses, retrieves literature, and iterates autonomously in continuous loops with automated laboratory partners.
- The episode argues that while LLM architectural research has largely converged on transformer variants, structure prediction remains a hotbed of genuine architectural innovation.

## Overview

Host Brandon Anderson speaks with Evan Feinberg (Genesis Molecular AI) and Sergey Edunov (formerly Meta, now Genesis) for nearly two hours about how diffusion models are transforming small molecule drug discovery. Edunov describes his transition from leading Llama training at Meta to joining Genesis after recognizing that the most architecturally interesting diffusion work is happening in 3D molecular structure prediction rather than in language modeling. The conversation centers on Genesis's PEARL model, which predicts how small molecule ligands bind to target proteins, and covers both the technical underpinnings and the benchmark context.

The hosts walk through why the canonical 2-angstrom RMSD threshold is misleading: hydrogen bonds, the critical interactions that govern binding affinity, operate at sub-angstrom distances, meaning a model that scores well on conventional benchmarks may still be producing physically wrong predictions. PEARL, evaluated on the OpenBind benchmark against 802 never-before-seen complexes — including the antiviral target EV-A71 — outperformed competing models in zero-shot setting, particularly on cases requiring accurate modeling of protein flexibility (the induced-fit problem).

The conversation then shifts to the broader drug discovery pipeline. The vast chemical space (~10^60 molecules) combined with multi-parameter optimization (potency, selectivity, ADMET properties) creates a combinatorial challenge no brute-force approach can address. Genesis's answer is SAPPHIRE, an agentic system that mirrors a medicinal chemist's workflow: generate hypotheses about binding poses, propose modifications, search the literature, and commission synthesis and testing through automated lab partners — enabling a continuous, nearly hands-off discovery loop.

Anderson shares a personal anecdote: he nearly joined Genesis early on but signed with competitor Atomwise instead. He expresses genuine enthusiasm for recent data suggesting protein-ligand binding accuracy may be approaching a "solved" threshold within the near term, while acknowledging that closed-source progress at firms like Isomorphic Labs and Deep Origin makes direct comparison difficult.

## Implications

The episode makes a strong case that diffusion model research has bifurcated: LLM-adjacent diffusion (image, video, text) has largely commoditized architecturally, while 3D molecular diffusion remains a frontier with room for fundamental innovations in how continuous 3D geometry and physical priors are incorporated. This has talent implications — researchers seeking architectural novelty may increasingly migrate from LLM labs to bio/chem AI companies.

PEARL's OpenBind result and the SAPPHIRE agentic pipeline together suggest that drug discovery timelines could compress significantly if structure prediction crosses the 1-angstrom accuracy threshold. The combination of high-fidelity structure prediction, multi-parameter optimization, and autonomous lab integration could reduce the hit-to-lead phase from years to months. However, the dominance of closed-source firms makes independent reproducibility difficult, and the episode does not deeply probe regulatory or safety questions around AI-accelerated drug development.

## Notable Quotes

- "The most innovative diffusion research happening in our field is in 3D structure prediction right now." — Evan Feinberg
- "It's like finding a needle in a haystack, where everything except your needle is very, very dangerous." — on the drug-like molecule search space
- "If your model is sitting at 1.8, 1.9 angstrom RMSD, that's slop." — on the inadequacy of current benchmarks

## People Mentioned

- Evan Feinberg — co-founder/researcher at Genesis Molecular AI, guest
- Sergey Edunov — former Meta AI (led Llama training), now at Genesis Molecular AI, guest
- Brandon Anderson — host, Latent Space; previously considered joining Genesis, signed with Atomwise instead

## Topics

diffusion models, drug discovery, small molecule design, structure prediction, protein-ligand binding, PEARL model, OpenBind benchmark, RMSD threshold, induced fit, SAPPHIRE agentic system, Genesis Molecular AI, molecular AI, chemical space, multi-parameter optimization, ADMET, agentic drug discovery, Llama, Meta AI
