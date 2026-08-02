---
show: "Latent Space: The AI Engineer Podcast"
episode: "AI-Native Healthcare: 100M Doctor Visits, 10–20 Hours Saved, Prior Auth in Minutes — Janie Lee & Chai Asawa, Abridge"
date: 2026-05-14
guid: "substack:post:197417280"
transcript_source: "substack"
backfilled: true
---

# AI-Native Healthcare: 100M Doctor Visits, 10–20 Hours Saved, Prior Auth in Minutes — Janie Lee & Chai Asawa, Abridge

## Key Points

- Abridge has processed 80M+ patient-clinician conversations across 250+ U.S. health systems; projecting 100M annual visits, 28+ languages, 50+ specialties, $5.3B valuation (June 2025 Series E).
- Three-act product strategy: (1) Save Time — ambient documentation eliminates 10–20 hrs/week of clinician "pajama time"; (2) Save Money — billing compliance, prior auth, denial reduction; (3) Save Lives — clinical decision support and outcome improvement.
- Prior authorization transformed: real-time synthesis of EHR data + payer policy PDFs + clinical guidelines alerts physicians before patient discharge, collapsing 45-day cycles into minutes.
- Constellation-of-models architecture: frontier models (Claude, etc.) plus proprietary models post-trained on 100M+ de-identified medical conversations — the dataset itself is the primary moat.
- EHR treated as "filesystem for agents": agents read, write, and manipulate clinical data; real-time processing uses batched triggers rather than full voice-in/voice-out streaming.
- Multi-layer eval stack: internal clinician review (LFD — "Look at the Data"), LLM judges calibrated against annotated datasets, third-party domain evaluators, specialty-level evals, and progressive rollout matching Waymo's real-world distribution approach.
- De-identification via proprietary one-way anonymization models enables safe training on PHI-containing data while maintaining HIPAA compliance.
- Clinician Scientists role: embedded MDs with technical depth (full-stack to "scrappy prompter") across product and eval teams — raises quality bar across everything the company builds.
- "Air conditioning" design philosophy: AI operates in the background, surfaces high-signal interventions only, deliberately avoiding alert fatigue (90%+ of healthcare alerts are ignored).
- Personalization at three layers: individual doctor style (bullets vs. paragraphs), specialty-specific workflows, and health system-level clinical guidelines.
- Multi-stakeholder enterprise model: CMIOs/CIOs (integration/compliance), CFOs (ROI), clinicians (usability), patients (visit summaries), payers and pharma (real-time clinical trial matching, drug utilization).
- Engineering tooling: Claude Code is the primary development tool across Abridge's engineering team; Cursor used secondarily.

## Overview

Janie Lee (co-founder) and Chaitanya "Chai" Asawa (co-founder, formerly of Glean) of Abridge join a crossover episode with Redpoint's Jacob Effron to explain how Abridge has become the clinical intelligence layer for U.S. healthcare — processing 80M+ patient-clinician conversations across 250+ large health systems, on track for 100M annually, in 28+ languages and 50+ specialties. Founded in 2018 (years before ChatGPT), Abridge spent years in the unglamorous work of building clinical trust and EHR integrations before the generative AI wave arrived; that head start is now reflected in a $5.3B valuation (Series E, June 2025) after $300M in 2025 funding.

The episode is structured around Abridge's "three acts": first saving clinician time (eliminating 10–20 hours/week of after-hours documentation, or "pajama time"); then saving health systems money (billing compliance, fewer denials, prior authorization); and ultimately saving lives through real-time clinical decision support. The prior auth workflow is a standout demonstration: Abridge synthesizes EHR patient history, payer policy PDFs (up to 50 pages), and clinical guidelines in real time, alerting clinicians which authorization criteria are already met and which are missing — before the patient even leaves the room, collapsing 45-day cycles into minutes.

Chai Asawa draws directly on lessons from Glean to articulate Abridge's durability thesis: proprietary training data (100M+ de-identified conversations), deep EHR integration as an "agent filesystem," and a constellation-of-models architecture (frontier models plus proprietary post-trained models) that compounds with scale. The episode also digs into evaluation infrastructure, multi-stakeholder enterprise sales, agentic evolution, and why healthcare's unforgiving accuracy requirements — where "80/20 doesn't work" — are accelerating AI innovation rather than blocking it.

## Implications

Abridge is the clearest current example of how to build a durable AI moat in a regulated, high-stakes vertical: accumulate proprietary domain data at scale before competitors, integrate deeply into mission-critical workflows (EHR), and use that data to post-train models that frontier providers cannot replicate. For AI engineers, the architecture patterns are instructive — constellation-of-models routing, CRDTs for multi-agent conflict resolution, event-driven infrastructure (Kafka, Temporal, WebSockets), and tiered quality-latency-cost optimization at 100M-conversation scale. For investors, Abridge's trajectory demonstrates that the highest-friction regulated markets (healthcare, finance, legal) are not barriers to AI adoption but accelerants: the compliance burden raises the quality floor so high that only well-resourced, deeply integrated incumbents can compete, creating winner-take-most dynamics. The prior auth workflow alone — converting a 45-day administrative burden into real-time AI inference — represents billions of dollars in healthcare system cost savings and sets the template for AI-native administrative automation across the industry.

## Notable Quotes

- "Context is king, but healthcare raises the stakes on safety, evals, and rollout." — Chai Asawa
- "When you think about things that make it hard, it also gives you the moat." — Janie Lee, on healthcare's complexity as a competitive barrier
- "High-stakes domains may drive AI forward" — healthcare's safety/compliance requirements accelerate innovation more than consumer markets.
- "80/20 doesn't work here" — missing edge cases in healthcare cause patient harm, requiring near-perfect accuracy. — Janie Lee
- "Progressive rollout... similar to Waymo, 'the world's most experienced driver'" — building real-world distribution matching through staged deployment. — Chai Asawa

## People Mentioned

- Janie Lee — Co-founder, Abridge
- Chaitanya "Chai" Asawa — Co-founder, Abridge; formerly of Glean
- Jacob Effron — Partner, Redpoint Ventures (guest host/interviewer)

## Topics

- Ambient clinical documentation
- Prior authorization automation
- Clinical decision support
- Constellation-of-models architecture
- EHR integration as agent filesystem
- Healthcare AI evaluation frameworks
- De-identification and HIPAA compliance
- Multi-stakeholder enterprise sales in healthcare
- Agentic healthcare workflows
- Clinician Scientist role
- Alert fatigue and air conditioning UX design
- Personalization in clinical AI
- Quality-latency-cost tradeoffs at scale
- Proprietary medical conversation datasets as moat
- FDA clinical decision support regulation
- Government interoperability mandates
- Lessons from Glean applied to healthcare
- Claude Code in enterprise AI engineering
