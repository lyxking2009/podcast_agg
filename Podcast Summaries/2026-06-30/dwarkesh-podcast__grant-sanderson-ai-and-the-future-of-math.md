---
title: "Grant Sanderson – AI and the future of math"
show: "Dwarkesh Podcast"
author: "Dwarkesh Patel"
date: 2026-06-30
guid: substack:post:204275916
transcript_source: youtube_autocaptions
---

## Key Points

- AI has made its fastest progress in mathematics compared to any other field, and Grant argues this is due to two underappreciated reasons: verifiability *and* grindability — the ability to run massive parallel rollouts in a containerized, deterministic environment. Computer use, while verifiable, lacks grindability due to bot detectors and non-deterministic environments.

- AI systems solved IMO geometry problems in 19 seconds (cold-solved with brute force by 2024), but struggled most on combinatorics problems, which require more creative, context-escaping thinking. AI would have earned IMO gold in 2024 if not for combinatorics problems.

- Grant identifies a key blind spot in current AI: the systems have "superhuman breadth" across all fields but don't spontaneously generate the "lightning bolt" cross-field connections that drive breakthroughs. The famous example is the Montgomery-Dyson lunch conversation linking Riemann zeta function zeros to random matrix theory (eigenvalues of Hermitian matrices from nuclear physics), which opened a new research direction.

- Counterexamples to the unit distance problem conjecture and Erdős problem #1196 (about primitive sets) are cited as early examples of AI making exactly these cross-field lightning-bolt connections, suggesting AI is now beginning to bridge fields in ways it previously couldn't.

- The Galois story illustrates why verification loops for conceptual insights can span a century. Lagrange intuited that symmetries of polynomial roots were the right framework, Galois developed group theory but died in a duel, and it wasn't until Gell-Mann in the 20th century used group theory to predict quarks that the full value became clear. Current RLVR training simply cannot reward this kind of generative insight.

- Grant distinguishes three types of mathematical progress AIs could achieve: (1) lightning-bolt connections between known fields — already happening, most human-readable; (2) mountain-building of genuinely new theories — would require heavy digestion time, like inter-universal geometry (Mochizuki's abc conjecture attempt); (3) raw long-chain brute-force proofs — least illuminating, causes the "fall of the theorem economy" David Bessis wrote about.

- Lean/formal proof verification is less important than people think for current AI math progress (AlphaGeometry moved from Lean to natural language between 2024-2025 and improved). However, Lean has a distinct future use: allowing AI to extend Mathlib autonomously without human check-in, like AlphaZero playing Go without supervision.

- Grant argues LLMs will likely also become better explainers than most humans, undermining his original theory that human math explainers were safe. The same cognitive capacity that generates novel insights may also produce lucid exposition — as seen historically with Einstein, Shannon, and Feynman.

- On writing, AI is limited partly because good writing requires accurate mental models of the reader's mind state — mentalizing — and autoregressive token prediction is architecturally misaligned with holding a model of reader comprehension across a whole piece. An analogy: Botox recipients become worse at reading emotions, apparently because mimicking facial expressions subconsciously helps recognition.

- Grant's advice to prospective math students: think explicitly about where the money comes from and what value you add. Teaching and curation roles are among the most stable post-AGI jobs because they are relational and social, not merely informational.

## Overview

Dwarkesh Patel interviews Grant Sanderson (creator of 3Blue1Brown, currently producing an AI-and-math documentary series) about the remarkable pace of AI progress in mathematics and what it means for the field and for human understanding. The conversation opens by revisiting a prediction Grant made three years earlier — that IMO gold wouldn't mean AGI — and finding it validated. The discussion then explores *why* math is uniquely amenable to AI improvement, landing on two core properties: verifiability (you can confirm whether a proof is correct) and grindability (you can containerize a math problem and run millions of parallel rollouts). Both conditions hold for math and code, but not for most real-world domains.

The bulk of the episode dissects the nature of mathematical breakthroughs and where AI stands on the spectrum from routine theorem-proving to genuinely creative insight. Grant uses the Galois/group-theory story at length to illustrate that the most valuable mathematical contributions — generative concepts like groups, definitions that spawn entire new fields — have century-long verification loops and cannot be captured by current reinforcement learning from verifiable reward. He and Dwarkesh discuss what the next benchmarks should be: not just solving Millennium Prize problems, but coming up with interesting new conjectures and definitions. They observe that the most recent AI results (unit distance conjecture disproof, Erdős problem 1196) look like "lightning bolts" connecting existing expertise in two fields — precisely the style where LLMs, with their superhuman cross-domain breadth, should excel.

The conversation closes by addressing practical implications: what math students should think about given AI, how to use LLMs effectively for learning (treat them like a supercharged Google pointing to human-curated resources), why writing remains harder for AI than math or code, and whether faster AI math progress will translate into economic value. Grant is optimistic about incremental practical leakage (PDE solvers helping aerospace, CFD simulations improving), while acknowledging that heavily abstract areas like algebraic number theory may not immediately benefit the economy.

## Implications

The episode frames mathematics as the leading indicator for AI progress broadly: whatever rate-limiting factors separate current AI from solving a Millennium Prize problem are the same factors limiting AI in white-collar work generally. This makes math both an unusually clean test bed and a warning about what's coming. The key bottleneck is not verifiability per se but grindability — the ability to cheaply simulate many rollouts — which explains why computer use (verifiable but not grindable) has lagged behind math and code.

For human roles, Grant's analysis suggests that curation and relational teaching are structurally stable, while pure theorem-proving is not. The "fall of the theorem economy" is real: if AI handles proofs and even exposition, the premium shifts to deciding what is worth proving and building the right new conceptual frameworks. This mirrors broader economic predictions where judgment, taste, and social trust outlast technical execution. Grant also notes a potential boon: in a world where AI has seen further into mathematics than any human, there will be enormous demand for people who can explain what the AIs discovered — making math communication potentially *more* valuable, not less.

## Notable Quotes

- "AI has been making the fastest progress in mathematics out of any other field. Whatever is happening here will tell us about what will happen to the rest of the world as AI gets better and better." — Dwarkesh Patel

- "The dirty secret with the IMO is that you really can train for a lot of them." — Grant Sanderson

- "You've pointed this out. It's bizarre to have something with this superhuman breadth that knows all the fields so well, and yet isn't finding those lightning bolts that connect them." — Grant Sanderson

- "Good mathematicians prove theorems, great mathematicians come up with conjectures, and the greatest mathematicians come up with definitions." — Grant Sanderson (quoting a mathematician from a Polylog video)

- "Everyone knows the idea of an unsolved research problem. I want to propose the idea of an unsolved expository problem — sure, we've proven it, but we don't really know why it's true." — Grant Sanderson (quoting Timothy Chow)

- "The verification loop on whether group theory is an interesting concept was a hundred years long. It involves cryptography coming around and physics making progress, and the ideas in group theory being relevant to understanding symmetries in physics." — Grant Sanderson

- "It's not just verifiability — it has to be grindable." — Grant Sanderson

- "Teaching is one of the most stable post-AGI jobs that there is, because it's so relational. Even if LLMs are good explainers, the thing that a teacher is doing is such a social, coaching, mentor-type thing." — Grant Sanderson

## People Mentioned

- Grant Sanderson (Creator of 3Blue1Brown, math educator and YouTuber)
- Dwarkesh Patel (Host, Dwarkesh Podcast)
- Hugh Montgomery (Number theorist; connected Riemann zeta zeros to random matrices)
- Freeman Dyson (Physicist; identified the link to random Hermitian matrix eigenvalues)
- Évariste Galois (19th-century mathematician who developed group theory before dying in a duel)
- Niels Henrik Abel (Norwegian mathematician who proved quintics have no general formula)
- Joseph-Louis Lagrange (18th-century mathematician who first recognized symmetries were key to polynomial roots)
- Joseph Liouville (Mathematician who rescued and published Galois's notes)
- Murray Gell-Mann (Physicist who used group theory to predict quarks)
- Timothy Chow (Mathematician; coined "unsolved expository problem" for forcing/continuum hypothesis)
- Terence Tao (Mathematician; mentioned as failing the "troll problem" IMO question)
- Alex Kontorovich (Mathematician; discussed frustration with AI-generated papers with potential errors)
- David Bessis (Mathematician; wrote "The Fall of the Theorem Economy")
- Robert Langlands (Mathematician; developed the Langlands program connecting disparate fields)
- Andrei Mochizuki (Mathematician; attempted proof of abc conjecture via inter-universal geometry)
- Steven Strogatz (Mathematician; author of *Nonlinear Dynamics and Chaos*, mentioned as exemplary textbook)
- Andrej Karpathy (AI researcher; cited on "sucking supervision through a straw" and auto-research)
- Eric Jang (AI researcher; built Go bot with interesting agent exploration observations)
- Andy Matuschak (Researcher; worked on teaching LLMs to write spaced-repetition prompts)
- Dario Amodei (Anthropic CEO; Dwarkesh had earlier asked him about cross-field connections)

## Topics

- AI progress in mathematics
- Verifiability and grindability as drivers of AI improvement
- International Math Olympiad (IMO) and AI performance
- Cross-field "lightning bolt" connections in mathematics
- Galois theory and group theory history
- Langlands program
- Lean and formal proof verification
- Mathlib and autonomous AI math exploration
- Math education and the role of human teachers post-AGI
- LLMs for learning and their limitations
- Why AI struggles at writing vs. math/code
- Theory of mind and mentalizing in AI
- Millennium Prize problems and AI
- The "fall of the theorem economy"
- Career advice for mathematics students in the AI era
