---
date: 2026-09-30
show: Machine Learning Street Talk (MLST)
title: "Who Checks a Proof No Human Can Read? - Leo de Moura"
guid: ad0b2172-04c4-4760-b439-6895a5f4a809
transcript_source: web
duration: 1:14:19
link: https://podcasters.spotify.com/pod/show/machinelearningstreettalk/episodes/Who-Checks-a-Proof-No-Human-Can-Read---Leo-de-Moura-e3pjhg5
model: deepseek-v4-flash (Hermes manual fallback)
---
# Who Checks a Proof No Human Can Read? - Leo de Moura

## Overview

Tim Scarfe talks with Leonardo de Moura, creator of Lean and co-creator of Z3, about what happens when formal
verification leaves the lab. De Moura explains how Lean escaped its original audience - dependent types plus Mathlib
made it useful to working mathematicians - why the trusted kernel is kept small and protected, and how independent
checkers provide safety through transparency. The centrepiece is his account of the Collatz incident, in which a
purported proof was accepted by both Lean's official kernel and the independent Nanoda checker, apparently by
exploiting a different bug in each. The conversation extends to reward hacking, spec drift, AlphaProof, and what AI
agents still lack.

## Key Points

- Lean's design keeps a small trusted kernel with independent external checkers (Lean4Lean, Nanoda), so verification is auditable rather than authoritative by fiat.
- The Collatz incident: a purported proof was accepted by both Lean's official kernel and Nanoda, apparently by exploiting a different bug in each kernel - a case study in why multiple independent checkers matter.
- Governance tension: cathedral versus bazaar for Lean's core, the Slack purge, Brandolini's law, and the role of the Lean FRO in maintaining the language's core.
- Kim Morrison used Claude on the zlib formalisation; the discussion weighs whether specs can be written for complex systems and whether AI makes proof re-doing cheap once specs change.
- De Moura's blunt claim: AI agents have “breadcrumbs, not learning” - competence without comprehension - which is precisely why certificates and verified guardrails still matter.
- The Fermat's Last Theorem formalisation and Terence Tao's involvement are cited as evidence the Mathlib ecosystem has become real infrastructure (the Mathlib Initiative).
- Reward hacking and safety-by-transparency: more kernels and independent checkers make cheating harder, not easier.

## Implications

This is one of the more important episodes of the year for anyone working on AI-assisted formal methods. The Collatz
incident is the practical lesson: a green checkmark is only as trustworthy as the kernel that produced it, and a
single kernel - even a small one - is a single point of failure. The “certificates still matter” argument is also a
useful counterweight to the view that stronger models make verification unnecessary. This record is built from the
episode's structured description and full chapter list rather than a verbatim transcript, so the chapter titles are
listed as topics and not presented as quotes.

## Notable Quotes

- (structured show notes / chapter list only, no verbatim transcript)

## People Mentioned

- Leonardo de Moura - creator of Lean, co-creator of Z3
- Tim Scarfe - host, MLST
- Kim Morrison - Lean formalisation of zlib
- Terence Tao - mathematician
- Kevin Buzzard - mathematician, Lean proponent

## Topics

formal verification, Lean 4, Mathlib, theorem proving, AI safety, reward hacking, Collatz conjecture, dependent types, AlphaProof
