---
podcast: "The Peterman Pod"
episode: "Creator of Lean: Handwritten Math Will Change Dramatically | Leonardo de Moura"
published: 2026-08-10
duration: 01:08:06
audio_url: "https://anchor.fm/s/106346b90/podcast/play/123820587/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-7-5%2F429289053-44100-2-283d434190e17.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/peterman-pod/episodes/Creator-of-Lean-Handwritten-Math-Will-Change-Dramatically--Leonardo-de-Moura-e3n173b"
transcript_source: youtube_autocaptions
generated_at: 2026-08-10T23:00:00Z
model: "deepseek-v4-flash"
guid: "4e8c7a3d-d20e-484e-b9d5-e8f3ce5f7949"
---

# Creator of Lean: Handwritten Math Will Change Dramatically | Leonardo de Moura — The Peterman Pod

## TL;DR
Ryan Peterman interviews Leonardo de Moura, creator of the Lean proof assistant and the Z3 theorem prover, about how Lean works, how formal verification proves the absence of bugs, and why LLMs paired with Lean will fundamentally change how we write software and do math. De Moura discusses shallow vs deep embedding, the zlib-in-Lean experiment (an AI translated and proved a C compression library in a week), Lean's role in mathematical breakthroughs, and his vision for a future where you write a specification and AI synthesizes and proves the code.

## Key points
- Lean is a programming language in which you can also write and machine-check proofs — 'machine checkable proofs, absolute assurance they are correct,' with multiple independent checkers.
- Two ways to verify foreign-language programs: shallow embedding (translate e.g. Rust into Lean and verify the translation) and deep embedding (write the semantics of the language, e.g. C, in Lean so programs become Lean objects you can reason about).
- Concrete example: proving an array index is in bounds — you write the mathematical statement 'i >= 0 and i < 10' and Lean checks it.
- LLM + Lean workflow: you write what you want to be true (a specification), and AI hammers at it until the Lean proof says you're good — the AI proves things automatically, but proofs must be modular to scale.
- The zlib experiment: his colleague Kin Morrison prompted AI to translate the C compression library zlib to Lean, pass the test suite, and prove that compress-then-decompress returns the original data — the whole thing succeeded in one week. 'It's surreal.'
- Lean has assisted mathematical breakthroughs: competition math, verification of frontier math results, and the Liquid Tensor Experiment — 'this is critical because it helps validate the candidate proofs the LLM spits out.'
- Handwritten math will change dramatically: the combination of LLMs (generating candidate proofs) and Lean (validating them) changes the economics of both software verification and mathematics.
- Proof assistants vs programming languages: the difference and why formal verification is a 'second software layer' on top of your program.
- When it's worth formalizing software: complexity management is the big challenge; frameworks help make proofs modular even with AI.
- The Z3 theorem prover: the project he started, its origins, and why it matters for SMT solving.
- The most technically challenging work of his career, and the future of Lean vs its competitors (Coq, Isabelle).

## Overview
Leonardo de Moura — creator of the Lean theorem prover and the Z3 SMT solver — joins Ryan Peterman to explain formal verification and its AI-powered future. The conversation starts with Dijkstra's famous quote that testing can show the presence of bugs but never their absence, and de Moura explains how Lean fills that gap with machine-checkable proofs. He walks through the mechanics: Lean as a programming language that can also state and prove properties, shallow vs deep embedding for verifying foreign-language code, and a concrete array-bounds example. The centerpiece is the emerging LLM + Lean workflow — writing specifications and having AI synthesize code plus proofs, exemplified by the one-week zlib translation-and-verification project by his colleague Kin Morrison. They discuss Lean's role in recent mathematical breakthroughs (competition math, frontier math verification, the Liquid Tensor Experiment), when formalizing software is worth the effort, the Z3 project's origins, the hardest technical work of his career, and how Lean compares with competitors. The episode's throughline: handwritten math and manual proof-writing will change dramatically as LLMs generate candidates and Lean validates them.

## Implications
- For software engineers: AI-generated code can be paired with Lean to prove strong properties (e.g. compression round-trips) — verification is becoming a practical, week-scale activity rather than a decades-long research program.
- For AI developers: 'proof-modularity' is the key to scaling AI-proven code — proofs must be structured so automation can maintain them.
- For mathematicians: Lean + LLMs changes the economics of checking frontier proofs; the Liquid Tensor Experiment shows the model working at scale.
- For the industry: expect formal verification to move from academia into critical software (microkernels like seL4, compilers, compression) as the tooling matures.

## Notable quotes
- "Lean gives you machine checkable proofs. You can check your proofs and get absolute assurance they are correct. You have many checkers, independent checkers." — Leonardo de Moura
- "Another way to view it: if you can express in math what you care about your program, you can verify it using Lean." — Leonardo de Moura
- "Imagine a future where you're writing what you want mathematically precisely, and AI synthesizes the code and proves that the code that was synthesized meets your specification." — Leonardo de Moura
- "She created a very complicated prompt for AI: translate zlib to Lean, ensure it passes the test suite, then prove that if you compress data and you decompress you get the original data back... after one week they succeeded doing the whole thing." — Leonardo de Moura (on Kin Morrison's zlib project)
- "It's surreal. I mean, it's crazy." — Leonardo de Moura (on the zlib-in-Lean result)
- "LLMs paired with the Lean proof assistant have led to breakthroughs in competition math and more recently the verification of frontier math results." — Ryan Peterman (intro)

## People mentioned
- Leonardo de Moura — guest, creator of Lean and Z3 theorem prover (Microsoft Research / now at AWS)
- Ryan Peterman — host, The Peterman Pod
- Kin Morrison — colleague who ran the zlib-to-Lean AI verification project (referenced)
- Kevin Buzzard — mathematician (Liquid Tensor Experiment, referenced)

## Topics
Lean, formal verification, theorem proving, proof assistants, LLMs, AI and math, Z3, SMT solving, software verification, Liquid Tensor Experiment, mathlib, shallow embedding, deep embedding, zlib, handwritten math
