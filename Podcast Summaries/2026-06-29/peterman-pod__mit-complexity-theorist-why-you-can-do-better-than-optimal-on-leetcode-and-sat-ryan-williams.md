---
title: "MIT Complexity Theorist: Why You Can Do Better Than 'Optimal' On Leetcode & SAT | Ryan Williams"
show: "The Peterman Pod"
author: "Ryan Peterman"
date: 2026-06-29
guid: 968ec1ef-b6d3-467f-a9e5-2300906b8cc7
transcript_source: web
---

## Key Points
- The "optimal" O(n²) solution for Three Sum on Leetcode is not actually optimal — Ryan Williams explains you can achieve approximately O(n² / (log n)^(2/3)) using grouped data structures and preprocessing with finger-search operations.
- Fine-grained complexity is a subfield that asks whether canonical, textbook algorithms are truly optimal, rather than just asking if a problem is in P or NP.
- Problems can be connected through fine-grained reductions: an algorithmic improvement on one problem (e.g., Two Sum) can imply improvements on another (e.g., Subset Sum), even across different traditional complexity classes.
- Williams expresses skepticism about the Strong Exponential Time Hypothesis (SETH), assigning it less than a 50% chance of being true, but argues that trying to refute it has been highly productive for discovering new algorithms.
- Williams assigns only 80% confidence that P ≠ NP — lower than most complexity theorists — because "we really don't understand polynomial time computation as deeply as we think we do."
- His major recent breakthrough shows that any algorithm running in time T can be simulated using only O(√T) space (with exponential time overhead), improving on a decades-old O(T/log T) bound from Hopcroft, Paul, and Valiant.
- His research philosophy centers on finding directions where "you can't lose" — where both proving and disproving a hypothesis yield valuable, publishable results.
- Williams recommends avoiding research inertia by regularly reassessing whether the problem you're working on is still the most valuable use of your time.

## Overview

Ryan Williams is an MIT professor and Gödel Prize winner who works on computational complexity theory — the study of what problems can and cannot be efficiently solved by algorithms. In this episode, he walks through his research area of fine-grained complexity, which examines whether the best-known algorithms for canonical problems are truly optimal. Unlike classical complexity theory, which asks coarse questions like "is this in P?", fine-grained complexity zooms in on specific running times: can you improve on the O(n²) solution for Three Sum, or the O(2^n) solution for SAT?

The conversation opens with a concrete example from Leetcode: the Three Sum problem, which asks you to find all triples in an array that sum to zero. The standard interview solution achieves O(n²) using sorting and two pointers, and most candidates (and interviewers) assume this is optimal. Williams explains that by breaking a sorted list into small groups and using clever data structures involving finger-search trees, the running time can be shaved to roughly O(n² / (log n)^(2/3)) — a mathematically meaningful improvement, even if modest in practice. This sets up the broader theme: our intuitions about algorithmic optimality are often wrong, and proving lower bounds is extraordinarily hard.

Williams also discusses his major new result on time-space tradeoffs: any algorithm running in time T can be simulated using only O(√T) space, improving on a bound from the 1970s. He traces his research philosophy — seeking problems where failure is still informative, using SETH as a productive foil even while doubting it, and staying willing to abandon a research direction when it stops being fruitful. He offers surprisingly candid probabilistic assessments of open problems, including assigning only 80% confidence to P ≠ NP and expressing genuine uncertainty about whether the field's foundational hypotheses are true.

## Implications

Fine-grained complexity challenges the assumption — common in software engineering and interview culture — that well-known algorithmic solutions are definitively optimal; even celebrated "textbook" algorithms may be improvable, and the field is still far from understanding the true limits of efficient computation. Williams's candid 80% confidence in P ≠ NP and his skepticism of SETH signal that the foundations of complexity theory rest on hypotheses that leading researchers do not consider settled, which should temper overconfidence in hardness results used to justify design choices in cryptography and systems. His "you can't lose" research philosophy offers a broadly applicable heuristic for choosing research directions: prioritize problems where the effort is valuable regardless of outcome, rather than betting everything on a single hypothesis being true.

## Notable Quotes

- "We have a variety of problems, canonical problems that we teach to undergrads...we wonder, are these algorithms optimal?"
- "I'm on the record as not believing this hypothesis." (on SETH)
- "We really don't understand polynomial time computation as deeply as we think we do." (on P ≠ NP)
- "I think I'm on the record as not believing this hypothesis." (on SETH, elaborating that trying to disprove it has been the most productive approach)
- "You want to find research directions where you can't lose — where both outcomes teach you something."
- "Constant algorithmic surprises challenge our intuitions about computational limits." (on why he assigns only 80% to P ≠ NP)
- "By breaking a sorted list into small groups and using clever data structures, you can achieve better than quadratic time — and that's a real, meaningful improvement."

## People Mentioned

- Ryan Williams (MIT professor, Gödel Prize winner, complexity theorist, guest)
- Ryan Peterman (host, former Instagram staff engineer)
- Hopcroft, Paul, and Valiant (authors of the original 1970s time-space tradeoff result that Williams improved upon)

## Topics

- Computational complexity theory
- Fine-grained complexity
- P vs NP
- Strong Exponential Time Hypothesis (SETH)
- Algorithm optimization
- Time-space tradeoffs
- Leetcode / coding interviews
- Three Sum problem
- SAT solvers
- Research philosophy and methodology
