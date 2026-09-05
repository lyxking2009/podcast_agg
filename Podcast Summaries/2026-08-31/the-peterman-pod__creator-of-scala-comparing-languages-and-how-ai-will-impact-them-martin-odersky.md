---
podcast: "The Peterman Pod"
episode: "Creator of Scala: Comparing Languages And How AI Will Impact Them | Martin Odersky"
published: 2026-08-31
duration: 57m30s
audio_url: "https://anchor.fm/s/106346b90/podcast/play/124414049/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-7-19%2F430103424-44100-2-576b37b6b7294.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/peterman-pod/episodes/Creator-of-Scala-Comparing-Languages-And-How-AI-Will-Impact-Them--Martin-Odersky-e3njal1"
transcript_source: web_podscripts
generated_at: 2026-09-05T22:22:18Z
model: "deepseek-v4-pro"
guid: "83bd3dde-b8d4-4e25-9c71-a86ce0e92c4e"
---

# Creator of Scala: Comparing Languages And How AI Will Impact Them | Martin Odersky — The Peterman Pod

## TL;DR
Martin Odersky, creator of Scala, discusses the benefits of functional programming, comparing Scala to Rust, Zig, Go, and Python, and explains Scala's origins, Twitter adoption, and design lessons. He argues that as AI generates more code, programming languages must emphasize stronger type systems and capability safety to maintain human control, predicting fewer but more highly skilled software engineers in ten years.

## Key points
- Functional programming is programming with values and functions that transform values, avoiding mutable state; this improves predictability and reduces bugs because side effects are minimized and documented.
- Functional programming is directly linked to mathematics, where mutation does not exist, allowing programs to be reasoned about more easily without tracking state changes.
- The learning curve for functional programming is real but short; pure functional programming can become inconvenient, so Odersky recommends using side effects moderately, for roughly 5% of a program.
- Studies on correctness benefits of functional programming are mixed and hard to conduct, but Odersky believes static types and functional style are particularly valuable in large systems.
- Scala uniquely fuses functional and object-oriented programming; object-oriented features handle components, modules, and encapsulation, which pure functional languages often lack.
- The 'power of the dot' in object-oriented languages like Scala provides convenient discovery of methods, unlike purely functional languages where programmers must search for applicable functions.
- Rust is memory safe and close to the metal, suitable for embedded systems, but Odersky argues Rust is overused for higher-level applications where garbage collection is acceptable.
- Scala relies on a garbage collector, which simplifies programming but requires more memory; Rust can run with much smaller memory footprints.
- Go is intentionally small, promoting a uniform style and easier onboarding, but it is limited; its recent addition of generics is a step forward.
- Scala cannot disable its garbage collector in production, though experimental capability tracking may eventually allow safe custom allocators.
- Zig's compile-time inlining is clean and powerful, whereas Rust's macros are clunkier; Scala's inlining is similar to Zig's but prevents type errors after inlining.
- Inlining replaces a function call with its body at compile time, enabling guaranteed optimizations that are not optional as with standard optimizer heuristics.
- Python is ubiquitous with readable syntax, and Scheme is grounded in computer science theory, but Python is far more popular.
- The gap between Scala and Python is closing: Python has added optional types and pattern matching, and many languages are adopting functional features.
- Scala has a strong always-on type system providing guarantees, whereas Python's type system has fewer guarantees and its ecosystem undervalues types.
- Scala 3 syntax resembles Python, and Python excels as a glue language due to efficient bindings to high-performance C++ libraries like NumPy and Pandas.
- Scala's design was influenced by Java, OCaml/StandardML, and Haskell; Odersky's predecessor language Pizza was developed with Phil Wadler.
- Scala's standard library function names mix influences from OCaml and Haskell; the JVM bytecode format allows Scala to interoperate with Java libraries.
- The JVM runs bytecode via an interpreter and JIT compiler, offers high-performance garbage collectors, and enables dynamic class loading crucial for REPLs.
- Compilers are intricate because they must handle complex languages, type inference, efficient code generation, and speed, but their determinism makes debugging easier than distributed systems.
- Odersky wrote a Java compiler in about three months half-time, using a bytecode generation library; compilers start simple and accumulate complexity over time.
- Twitter adopted Scala when its Ruby codebase became unreliable; investors wanted Java, but engineers chose Scala as JVM-compatible and similar to OCaml.
- After Twitter's adoption, many companies followed, especially those coming from dynamic languages like Ruby, PHP, and JavaScript.
- With AI generating code, humans cannot review mountains of code; AI also exploits vulnerabilities, creating a dangerous situation where humans may lose control.
- Odersky predicts programming languages will focus on interfaces and types; types will become stronger and more precise as contracts between humans and AI.
- Current type systems have holes like casts and dirty memory that undermine guarantees; these holes must be closed to establish trust in AI-generated code.
- Capability-based security can give fine-grained permissions to AI agents, ensuring they cannot leak secrets; Scala has experimental capability tracking features.
- Memory safety is table stakes; Rust addresses memory but we need capability safety for read/write permissions and access to secrets; AI can help rewrite legacy C/C++ code.
- In capability systems, types declare captured capabilities, preventing them from escaping scope; for example, a file capability can be confined to a limited operation.
- Memory unsafe languages include C and C++; Rust was a breakthrough as a low-level memory-safe language, but additional capability safety is necessary.
- With AI-generated code, ease of writing matters less; focus shifts to high-level constraints and specifications; prompts should become first-class values in programs for incremental changes.
- Odersky predicts fewer software engineers in ten years, but the profession will demand higher skills in logic and mathematics to guide AI.
- To expand the mind, learn a systems language (C first for hardware, then Rust), and a verification-oriented language like Lean or Roc to understand program correctness.
- Odersky recommends Structure and Interpretation of Computer Programs as a top technical book; his courses at EPFL and Coursera are based on it.
- Odersky chose academia for long-term independence and research freedom, despite industry paying much more; he values working with students and defining his own agenda.
- Scala's fusion of OO and FP was technically successful, but ecosystem challenges arose from culture clashes between Haskell and OO communities, as joked by James Iry.
- Over-abstraction in Scala due to powerful features and monadic libraries from Haskell led to some unreadable code and canceled projects; he advises using fancy abstractions responsibly.
- Odersky wishes Scala had been less dependent on Java's universal methods (toString, equals, hashCode) and had used type classes instead, which are safer though more tedious.
- Scala succeeded as a bridge between slow dynamic languages and cumbersome static languages, offering type inference and platform solidity; many later languages copied its features.
- Odersky advises taking risks, being adventurous, and non-conformist in one's career.

## Notable quotes
> "We are at a moment where it's essentially very dangerous that we lose control as humans." — Martin Odersky (~00:31:53)
> "I believe right now, Rust is actually overused." — Martin Odersky (~00:10:21)
> "If the code is AI generated, then the focus has to go elsewhere. And I think the focus will go to the interfaces and to the types." — Martin Odersky (~00:32:30)
> "I think there will be less. And it will be a higher profession that is essentially has higher standards." — Martin Odersky (~00:43:40)

## People mentioned
- Martin Odersky
- Niklaus Wirth
- Phil Wadler
- Simon Peyton Jones
- James Iry

## Topics
`functional-programming` `scala` `programming-languages` `type-systems` `ai` `memory-safety` `compilers`
