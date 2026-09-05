---
podcast: "The Peterman Pod"
episode: "Creator of TypeScript: 10x Faster Typescript, Why AI Won't Replace SWEs | Anders Hejlsberg"
published: 2026-08-17
duration: 1h05m57s
audio_url: "https://anchor.fm/s/106346b90/podcast/play/124316508/https%3A%2F%2Fd3ctxlq1ktw2n"
episode_url: "https://www.youtube.com/watch?v=cywK3XYYJ2o"
transcript_source: youtube_autocaptions
generated_at: 2026-08-17T15:30:00Z
model: "deepseek-v4-flash"
guid: "00fdc75f-de66-4cd4-84f1-325e8d0ee5ff"
---
# Creator of TypeScript: 10x Faster Typescript, Why AI Won't Replace SWEs | Anders Hejlsberg — The Peterman Pod

## TL;DR
Ryan Peterman interviews Anders Hejlsberg, creator of TypeScript and C#, about TypeScript 7's 10x speedup via a native rewrite in Go. Hejlsberg explains why the compiler was originally written in JavaScript (self-hosting in the ecosystem, running everywhere), why the rewrite was a port rather than a rewrite (preserving semantics and backwards compatibility), and why Go beat Rust (garbage collection plus shared-memory concurrency; the borrow checker can't handle the compiler's circular data structures). He also shares his contrarian take on AI: incumbent languages like JavaScript and TypeScript will flourish because AI is best at languages in its training set, and the best way to use AI on a big migration is to ask it for a deterministic program that does the translation — not for the translation itself.

## Key points
- The original compiler was written in JavaScript deliberately: self-hosting makes the team daily users of their own tooling, and JS runs everywhere — including the browser, which native code couldn't reach before WebAssembly; V8 had also closed the performance gap to within 2-3x of native.
- TypeScript's compiler is unusual: it targets JavaScript (a transpiler), its types are erased and have zero runtime impact, and its type checker exists purely for tooling and developer productivity — with a gradual type system where half-typed code is fine.
- TypeScript 7: the performance problem was JavaScript's 2-3x compute penalty plus its lack of shared-memory concurrency (Web Workers can't share data structures), while Moore's law now delivers more cores, not faster cores. TypeScript 6 ran at only ~50% of TypeScript 1.5's speed per feature, and projects have grown to millions of lines.
- Go was chosen via a structured process: they wanted a port, not a rewrite, and the port required garbage collection, first-class functions, and shared-memory concurrency — Go checked all boxes; Rust's borrow checker forbids the circular data structures (trees with parent pointers, recursive types) that fill the compiler, and hand-rolling GC would have meant solving a pile of new problems.
- LLMs were used surprisingly little in the port (started two years ago, before current model quality), via a tool that syntactically translated TypeScript to Go plus targeted AI help. His advice: ask AI for a program that computes the answer, not the answer itself — a deterministic program you can re-run beats stochastic output you have to eyeball every time.
- On JavaScript's popularity: Brendan Eich's first-class functions were done right, JS is the only true cross-platform language (even Java doesn't run everywhere), and TypeScript's job is to "capture all the badness and park it" — which is why CoffeeScript and Dart (replacements) lost while TypeScript (a superset that fixes rather than replaces) won.
- TypeScript is now the #1 language on GitHub — more code than JavaScript itself, more than Python — with a noticeable knee in adoption coinciding with AI.
- AI will strengthen incumbent languages: models are best at languages with the most code in their training data, so inventing a new language means paying a huge token tax to teach the AI; every AI tool writes TypeScript because types guide models to fewer mistakes and enable static validation before running.
- Flow lost because it was written in OCaml (hard for the community to contribute) and didn't focus on IDE tooling, while TypeScript shipped a deeply integrated language service across editors from day one.
- On building languages: "the world needs another programming language like it needs another hole in the head" — every new language is ~10% new and 90% drudgery; master both the mechanics (parsers, scanners) and the art, and stand on the shoulders of giants.

## Notable quotes
> "If you can self-host in the ecosystem that you want to be a part of, then that is just dramatically better than putting yourself outside the ecosystem and trying to target that ecosystem." — Anders Hejlsberg
> "You don't ask AI for the answer. You ask it for a program that computes the answer." — Anders Hejlsberg
> "Incumbent languages are actually, if anything, going to flourish because of AI." — Anders Hejlsberg
> "The world needs another programming language like it needs another hole in the head." — Anders Hejlsberg
> "We're not really there to optimize your code for runtime. We're there to make you more productive as a developer, or make AI more productive as a developer." — Anders Hejlsberg, on TypeScript's type checker

## People mentioned
- Anders Hejlsberg — creator of TypeScript and C#, Microsoft Technical Fellow
- Ryan Peterman — host, The Peterman Pod (Meta)

## Topics
- TypeScript, compiler design, Go vs Rust, programming languages, native rewrite, AI and software engineering, JavaScript ecosystem, developer tooling
