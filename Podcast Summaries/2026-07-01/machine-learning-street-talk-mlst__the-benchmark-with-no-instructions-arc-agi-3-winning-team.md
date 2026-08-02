---
title: "The Benchmark With No Instructions — ARC-AGI-3 (winning team!)"
show: "Machine Learning Street Talk (MLST)"
author: "Machine Learning Street Talk (MLST)"
date: 2026-07-01
guid: b09afda8-4f1b-4697-8a85-04f8f692e990
transcript_source: youtube-auto-captions (https://www.youtube.com/watch?v=Vg6FBKTlfOw)
---

## Key Points

- ARC-AGI-3 makes the ARC challenge fully interactive and agentic: agents must discover their goal purely through interaction with a novel game environment, receiving no natural-language instructions, no rules, and no stated objectives.
- The benchmark scores on action efficiency (roughly the ratio of human median actions to AI actions, squared), meaning brute-force solutions that pass levels with excess moves are heavily penalized even if they complete games.
- Tufa Labs (led by Benjamin Crouzier, with Dries Smit, Jeroen Cottaar, Stefano Viel, and Michal Tesnar) is the current leaderboard leader; their approach uses large reasoning LLMs (Claude, Qwen) inside a structured coding-agent harness that writes and executes Python programs to explore game mechanics and plan actions.
- Dries Smit won the ARC-AGI-3 preview competition with "StochasticGoose," a brute-force RL agent that searched only actions producing frame changes, but the organizers subsequently hardened the main competition by adding efficiency scoring and harder games, rendering pure brute-force infeasible.
- Frontier models (GPT-5, Claude, Gemini) score under 1% on the no-harness leaderboard, but with a well-designed harness they can reach approximately 35–36% — though even that score is misleading because it reflects action efficiency, and many levels are actually solved but just not efficiently enough.
- The key challenge in ARC-AGI-3 is the explore-vs-exploit tradeoff during goal acquisition: agents must simultaneously probe the game to discover rules and pursue a goal, and LLMs tend to lock onto wrong hypotheses and struggle to escape them.
- LLMs gain an unfair-seeming advantage because game designers are human and encode human priors (maze-like structures, recognizable game patterns, bright colors for interactive objects); permuting colors or rotating frames degrades both human and LLM performance significantly.
- Tufa Labs uses reward shaping across 25+ games (plus procedurally generated environments) with end-to-end RL, training on shorter sequences and generalizing to the long 100k–1M token contexts required for full game play.
- The team finds that goal acquisition is less of a bottleneck than action efficiency and long-context consistency; maintaining coherent knowledge across hundreds of thousands of tokens per game is the main engineering challenge.
- There is active debate about whether the winning solution will be "bitter-lesson" (scale + data) or require genuine architectural innovation; Tufa Labs leans toward the latter being necessary for this competition cycle, though they expect ARC-AGI-3 to be eventually bitter-lessoned in future years.

## Overview

Tim Scarfe (MLST host) travels to Zurich to interview the Tufa Labs team — Benjamin Crouzier (founder), Dries Smit, Jeroen Cottaar, Stefano Viel, and Michal Tesnar — who are currently leading the ARC-AGI-3 Kaggle competition. The conversation opens with a live walkthrough of an ARC-AGI-3 game ("LS20," a maze-like environment presented as a 64x64 pixel grid with 16 possible colors), illustrating the core challenge: the agent receives only a raw observation frame with zero instructions and must figure out who the player is, what the goal is, and how to achieve it efficiently. The team explains how ARC-AGI-3 differs from its predecessors — rather than transducing a static input-output grid, agents must act, observe, and adapt dynamically across multiple sequential levels of increasing difficulty.

The discussion covers both Dries's preview-competition win (StochasticGoose, a brute-force RL approach) and why it failed when the main competition added action efficiency scoring and harder games. The team's current approach uses frontier reasoning models (notably Claude Opus and Qwen) wrapped in a coding-agent harness that generates Python programs, builds minimal world models, conducts search, and writes structured reasoning traces to track hypotheses about game mechanics. They debate whether this constitutes inductive or transductive reasoning, land on it being more inductive (the reasoning chain is in English and could in principle be reused), and discuss how the prior knowledge baked into LLMs about mazes, games, and color conventions gives them an unexpected but significant edge on games that are ostensibly designed to strip away such priors.

The latter half of the episode widens to foundational questions: whether transformers can truly plan (the team argues they can effectively simulate planning by writing and executing code), what Francois Chollet's vision of bottom-up abstraction acquisition looks like versus the top-down fractured-representation approach of LLMs, and whether ARC-AGI-3 can be gamed or truly tests for something meaningful. The team discusses their software engineering practices — requirements-based engineering with coding agents, the dangers of vibing large codebases, and the difficulty of reviewing AI-generated code in a fast-moving research context. The episode closes with reflections on AI safety, the accelerating pace of AI progress, and whether the winning solution in November 2026 will be a compute-intensive bitter-lesson approach or something more architecturally novel.

## Implications

ARC-AGI-3 represents a significant step forward in benchmark design precisely because it forces agents to solve an exploration-first problem with no natural-language scaffolding and with a scoring function that penalizes inefficiency — two properties that together make it extremely resistant to both brute-force and pure-prompt-engineering approaches. The Tufa Labs results suggest that the most promising path forward is hybrid: leverage the rich priors already encoded in large reasoning models (which reflect human game-design conventions even when game designers try to eliminate them), but ground those priors in executable code and structured harnesses that impose domain constraints. This has broader implications for agentic AI systems generally: the gap between "can solve the task with unlimited compute" and "can solve it with human-comparable efficiency" may be the defining frontier for the next generation of benchmarks.

The episode also illustrates a recurring tension in the field: the benchmark is designed by Chollet to reward bottom-up, prior-free general intelligence, but the teams doing best are those most effectively exploiting top-down LLM priors. If this pattern continues and the competition is eventually won by a heavily engineered LLM harness, it raises the question of whether ARC-AGI-3 actually measured what it was designed to measure — or whether the lesson is that "pure" intelligence is inseparable from learned priors, making the distinction Chollet is trying to draw philosophically untenable.

## Notable Quotes

- "ARC-AGI-3 makes ARC interactive and agentic, so the model has to discover the goal rather than transduce a static grid." — Tim Scarfe

- "Often the agents start thinking that reducing the energy bar to the minimum is the goal or that stepping 10 times in a region is the goal, which for a human is kind of clear that that it's not the actual goal." — Michal Tesnar

- "36% might be misleading as a number if you don't look behind it. So what it really measures is action efficiency." — Dries Smit

- "The transformers can't plan but they can do a very good job of pretending — essentially that is in a sense indistinguishable from what might be formal planning in a computer science sense." — Jeroen Cottaar

- "There's some leakage of human priors into the games. If you just remove that prior which shouldn't actually be prior — permuting the colors — you can actually see it becomes harder for humans to play." — Dries Smit

- "We're gradually understanding less and less of our own codebase and we're struggling even with reviewing some of the changes." — Dries Smit (on using coding agents in fast-moving research)

- "The models themselves, they don't seem to work bottom up. They work with an understanding, but it's very fractured and entangled. And that's still useful, but that's a kind of statistical low-level intelligence. It's performance, not competence." — Tim Scarfe

- "I'm maybe a little bit less bitter-lesson-pilled than most AI researchers. I would be willing to make a bet that the winning solution of this competition will actually not be a bitter-lesson solution." — Jeroen Cottaar

## People Mentioned

- Benjamin Crouzier — founder of Tufa Labs, ARC-AGI-3 leaderboard leader
- Dries Smit — Tufa Labs researcher, winner of ARC-AGI-3 preview competition with StochasticGoose, background in electrical engineering and RL
- Jeroen Cottaar — Tufa Labs researcher, background in physics and mathematics, 15 years in AI industry
- Stefano Viel — Tufa Labs researcher, studied computer science and ML, prior RL research at EPFL
- Michal Tesnar — Tufa Labs researcher, masters in data science
- Tim Scarfe — host of Machine Learning Street Talk (MLST), interviewer
- Francois Chollet — creator of the ARC benchmarks (referenced throughout but not present)
- Mike Knoop — co-organizer of ARC Prize (referenced)
- Andrej Karpathy — referenced in context of research challenges and loss minimization

## Topics

ARC-AGI-3, benchmark design, agentic AI, reinforcement learning, reasoning models, LLM priors, action efficiency, goal acquisition, Tufa Labs, inductive vs transductive reasoning, coding agents, bitter lesson, abstraction, general intelligence
