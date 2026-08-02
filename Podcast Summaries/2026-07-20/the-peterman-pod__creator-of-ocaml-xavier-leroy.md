---
title: "Creator of OCaml: Functional Programming, Formal Verification, Programming Languages | Xavier Leroy"
show: "The Peterman Pod"
author: "Ryan Peterman"
date: 2026-07-20
duration: "84 min"
guid: "42eefb1c-e1fb-4b0a-925f-684ebcd36963"
transcript_source: "youtube_autocaptions"
tags: [ocaml, xavier-leroy, formal-verification, functional-programming, programming-languages, compcert, type-inference, rust, llm-generated-code, theorem-provers]
---

# Creator of OCaml: Functional Programming, Formal Verification, Programming Languages | Xavier Leroy

**Show:** The Peterman Pod
**Date:** 2026-07-20
**Duration:** 84 min

## Key Points

- Xavier Leroy, creator of OCaml, explains what sets it apart: a "fine" functional language with pattern matching, higher-order functions, and recently added user-defined effect handlers, but also a decent systems language with a predictable cost model, a performant compiler, and a low-latency garbage collector — originally designed for theorem proving/DSLs, but adopted by systems users like the Ensemble network-protocol project at Cornell in the late '90s and later Jane Street for trading infrastructure (via PhD student Yaron Minsky).
- Comparing Rust and OCaml, Leroy identifies automatic (garbage-collected) vs. manual memory management as the fundamental dividing line. Rust is "the finest language I know for manual memory management," giving more control but more programmer responsibility; manual memory management is not always faster than GC, since patterns like defensive copying in C++ can be costlier than garbage collection.
- Leroy is candid about disliking JavaScript, mainly its extreme dynamism — nearly everything (including method invocation semantics) can be redefined at runtime, which he considers a security and reliability weakness rather than a strength, despite JavaScript containing "a decent functional language" inside (a Lisp-derived core, per creator Brendan Eich).
- On functional programming's reputation for difficulty: Leroy argues it isn't fundamentally harder, especially with a math background, and notes Jane Street uses OCaml partly as a hiring filter to attract more diverse/educated applicants. He also claims "Python is 50% functional language" via comprehensions.
- He explains Hindley-Milner-style type inference (crediting Robin Milner) as constraint collection and solving, analogous to Sudoku, and how parametric polymorphism falls out "for free" when constraints remain unconstrained — a discovery he calls "beautiful." Trade-offs include confusing error messages that may not point to the true source of a type error, and tension between subtyping and full type inference.
- On formal verification: Leroy cites Dijkstra ("testing can only show the presence of bugs, not their complete absence") and describes proof assistants (Lean, Coq, Isabelle) as machine-checked alternatives to pencil-and-paper proofs. He spent years using Coq to prove correctness of CompCert, a verified C compiler, showing the generated assembly is faithful to the source C semantics — a proof too large (thousands of pages) for any human to trust unmechanized. He also cites the seL4 microkernel (~8,000 lines of verified C) as a landmark achievement.
- OCaml added multicore support only in 2022 despite multi-core hardware being standard for years, due to (1) engineering difficulty rewriting the garbage collector/allocator for concurrent-safe operation (done largely by OCaml Labs at Cambridge), and (2) a language-design detour: Leroy personally preferred message-passing concurrency (à la Erlang) over shared-memory concurrency, and designing a sound memory model for shared-memory access (avoiding Java's five problematic memory-model iterations) took significant time.
- On LLM-generated code, Leroy is skeptical: "every new line of code is a liability," and the idea that humans will thoroughly review AI output is unrealistic given the sheer volume produced. He describes OCaml and CompCert repositories receiving many AI-generated bug reports/PRs where only "maybe one good issue among 10." He anticipates possible "no to AI slop" movements in open source and sees promise in having AI generate machine-checkable proofs (e.g., in Lean) of its own code's correctness, though specification quality (avoiding unsatisfiable or trivial preconditions) remains an unsolved problem.
- Speculating on the future of programming languages given LLM-generated code, Leroy notes OCaml code quality from LLMs is already "quite decent," possibly because static typing helps models avoid errors; he expects most C/C++-to-Rust rewrites (for safety) to dominate near-term migrations, though some generated Rust ends up wrapped entirely in `unsafe` blocks, undermining the safety benefit.
- He argues most programming-language innovation has shifted from academia to industry over recent decades — citing Java's popularization of garbage collection and bytecode verification, Swift's popularization of algebraic data types/pattern matching, and Rust as a "spectacular" case of an academic-language feature set (ADTs, GC, borrow-based safety) taken to production scale by a major company.
- Leroy names two book recommendations: "Programming Pearls" (Jon Bentley) for its timeless demonstration of expert programmer judgment, and "How to Design Programs" (Felleisen/Findler/Flatt/Krishnamurthi) from the Scheme/pedagogy community. His career advice: he specialized in programming-language research too early and encourages a broader computer-science background (including seemingly impractical theory like computability) before specializing.

## Overview

Ryan Peterman interviews Xavier Leroy, creator of the OCaml programming language and long-time INRIA researcher, in a wide-ranging conversation spanning language design, formal verification, and the impact of LLM-generated code. Leroy walks through what makes OCaml distinctive as a "fine functional language" with systems-programming chops, contrasts it with Rust (automatic vs. manual memory management) and JavaScript (which he's frank about disliking for its runtime dynamism), and explains Hindley-Milner type inference in concrete terms. A large portion of the conversation covers formal verification and proof assistants (Lean, Coq, Isabelle), grounded in Leroy's own multi-year project proving the correctness of the CompCert verified C compiler, plus the seL4 verified microkernel. He recounts the technical and language-design challenges behind adding multicore support to OCaml only in 2022, and closes with a candid, skeptical take on LLM-generated code — arguing that the assumption of adequate human review is unrealistic — while seeing potential in AI systems that can produce machine-checkable proofs of their own correctness. He ends with book recommendations and reflections on having specialized too narrowly early in his career.

## Implications

For software engineers and language designers, the episode is a rare deep, technical conversation from one of the field's most credentialed practitioners (OCaml, CompCert) on how memory-management models, type systems, and formal methods trade off against each other — useful context for anyone evaluating Rust, OCaml, or verified-systems approaches for production use. Leroy's skepticism about LLM-generated code — "every new line of code is a liability" and human review capacity cannot scale to AI output volume — is a notable counterpoint to more optimistic industry narratives (including, ironically, the adjacent Odd Lots episode this same day featuring Anthropic's Boris Cherny), and his proposed path forward (AI-generated code paired with AI-generated, machine-checked correctness proofs) offers a concrete technical direction rather than just a review-process fix. His observation that PL innovation has shifted from academia to industry (Java, Swift, Rust) is a useful framing for understanding where new "safer" languages might emerge as AI-driven code rewrites make large-scale language migrations newly economical.

## Notable Quotes

- "Testing can only show the presence of bugs, but not their complete absence." — Xavier Leroy, quoting Dijkstra
- "I'm not a big fan of JavaScript... it is the ultimate dynamic language, in my opinion, while OCaml is very static." — Xavier Leroy
- "Rust is... the finest language for manual memory management... but manual memory management is not always faster." — Xavier Leroy
- "Every new line of code is a liability... I don't think it's a good way to split the work between machines and humans." — Xavier Leroy, on LLM-generated code
- "Maybe this will be the decade of formal verification of software. We've been waiting for that for 50 years, so maybe it will finally take off." — Xavier Leroy
- "If you took LLM-generated code and turned it up [10 years from now], what languages would become more popular? What languages might fade away?" — Ryan Peterman
- "Industry hasn't decided on a particular programming language. Industry is still interested in new programming languages... I find it extremely encouraging." — Xavier Leroy

## People Mentioned

- **Xavier Leroy** — creator of OCaml, INRIA researcher, author of the CompCert verified compiler; guest
- **Ryan Peterman** — host, The Peterman Pod
- **Yaron Minsky** — former PhD student on the Ensemble project; later built Jane Street's OCaml-based trading infrastructure
- **Robin Milner** — British computer scientist, pioneer of the ML language family and Hindley-Milner type inference
- **Brendan Eich** — creator of JavaScript, referenced regarding its Lisp heritage
- **Rob Pike** — designer of Go, quoted on hiring "Googlers, not researchers" as a rationale for language simplicity
- **Edsger Dijkstra** — quoted on the limits of testing vs. formal verification
- **Jon Bentley** — author of "Programming Pearls," recommended book

## Topics

OCaml, functional programming, formal verification, proof assistants (Lean/Coq/Isabelle), CompCert verified compiler, seL4 microkernel, type inference and polymorphism, Rust vs. garbage collection, JavaScript language design critique, OCaml multicore support, LLM-generated code and AI slop, programming language innovation (academia vs. industry)
