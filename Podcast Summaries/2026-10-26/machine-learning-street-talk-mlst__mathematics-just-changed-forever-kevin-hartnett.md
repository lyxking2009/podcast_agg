---
podcast: "Machine Learning Street Talk (MLST)"
episode: "Mathematics Just Changed Forever — Kevin Hartnett"
published: 2026-10-26
duration: "1h33m33s"
audio_url: "https://traffic.megaphone.fm/APO6346747125.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/machinelearningstreettalk/episodes/Mathematics-Just-Changed-Forever--Kevin-Hartnett-e3r47eh"
transcript_source: web
generated_at: 2026-10-27T22:22:09Z
model: "deepseek-v4-pro"
guid: "d99b4720-283b-4f07-89c4-791b12acba2d"
---

# Mathematics Just Changed Forever — Kevin Hartnett — Machine Learning Street Talk (MLST)

## TL;DR
Kevin Hartnett discusses his book 'Proof in the Code,' which chronicles the development of the Lean theorem prover and its transformative impact on mathematics. The episode covers Thomas Hales's computer-assisted proof of the Kepler conjecture, the creation of Lean by Leo de Moura, the growth of Mathlib, and the formalization of modern results such as Peter Scholze's work. It also explores the rise of AI in mathematics, including DeepMind's AlphaProof, and debates over whether machine-checked proofs truly confer understanding.

## Key points
- Kevin Hartnett's book begins with Thomas Hales's 1998 proof of the Kepler conjecture, which reduced the problem to 1,762 configurations and required extensive computer checking. The proof was eventually accepted by the Annals of Mathematics with a 99% confidence disclaimer, prompting Hales to later formalize it in a proof assistant (Flyspeck) to achieve absolute certainty.
- Leo de Moura created Z3 at Microsoft, an automated theorem prover that caught bugs and saved Microsoft hundreds of millions of dollars. Dissatisfied with the limitations of automated theorem provers, he started developing Lean, an interactive theorem prover with a small trusted kernel designed for independent checking.
- Jeremy Avigad convinced de Moura to adopt dependent type theory over simpler logical foundations. This choice enabled Lean to handle abstract algebra and ultimately proved essential for Mathlib, which later supported cryptography verification by researchers like Cédric Fournet.
- Mathlib, Lean's mathematics library, began as a crowdsourced effort to formalize basic mathematics. Kevin Buzzard contributed by teaching undergraduates to formalize definitions, and the library grew from 30,000 lines to include most undergraduate mathematics and some graduate-level concepts, with occasional spikes to frontier topics.
- Kevin Buzzard's initial motivation was skepticism about the rigor of modern number theory papers; he wanted to formalize them to check correctness, leading to his evangelism for Lean. He famously formalized the definition of perfectoid spaces, a frontier object created by Peter Scholze, which convinced many mathematicians that Lean could handle advanced math.
- In 2020, Peter Scholze challenged the Lean community to verify a technical proof from his paper with Dustin Clausen. The community, including Johan Commelin, spent six months and discovered a simplification, confirming the proof correct in Lean. This verification marked a turning point, attracting more mathematicians to Lean and accelerating Mathlib's growth during COVID lockdowns.
- DeepMind's AlphaProof, based on reinforcement learning similar to AlphaGo, used Lean to solve International Mathematical Olympiad problems in 2024, achieving a silver medal. It required auto-formalization of problems into Lean, with algebra and number theory easier to translate than combinatorics.
- By 2026, AI systems began tackling open research problems: a pre-release OpenAI model proved Erdős problem 126 and disproved problem 74 in Epoch AI's fixed budget test, with results checked in Lean. OpenAI also published a claimed resolution of the Navier-Stokes Millennium problem with a Lean formalization, though under controversial circumstances.
- Terence Tao's equational theories project used Lean to map implications among algebraic laws, narrowing 22 million questions down to 238, then 138, and eventually being stuck on just four. The project discovered new constructions like magma cohomology, which surprised experts including John Baez, demonstrating the potential of large-scale collaborative mathematics with machine verification.
- Kevin Buzzard's current project translates statements of 50 theorems from the Annals of Mathematics into Lean. AI checking found errors, including from four Fields medalists, where statements were literally false as written due to missing assumptions (e.g., n≥3, non-empty set). This highlights the gap between informal mathematical communication and formal precision.
- Lean's kernel is a small checker that verifies proofs; de Moura emphasized independent checking and minimality to avoid bugs that could validate false statements. Earlier interactive theorem provers like Agda, Coq, and Isabelle had kernel bugs that allowed users to validate false statements.
- Auto-formalization using large language models can translate papers to Lean quickly, but Mathlib maintainers worry about lower quality and lack of proper abstraction, potentially creating a 'spaghetti monster' of code. The community is grappling with the volume of AI-generated Lean code and how to maintain standards.
- The episode discusses the 'formalization gap': Lean certifies the derivation, not that the formal statement matches the intended theorem. Trust shifts to definitions and translation, and definitions cannot be checked for 'rightness'—only for consistency. Mathematicians like Peter Scholze have expressed concern that Lean cannot verify definitions.
- Mathematicians have mixed reactions to AI in mathematics: some grief, some excitement. Kevin Buzzard believes AI will accelerate mathematics and increase understanding, while others worry about losing human insight and the value of ideas over mere correctness. The episode emphasizes that human goals and understanding should remain central.

## Notable quotes
> "What my book is, is ultimately the story of how Lean this interactive theorem prover that De Moura builds to verify the correctness of software programs in a richer way than he could with Z3, gets in a sense, co-opted by mathematicians who see the potential in it, and for the first time, an ITP breaks through into a more mainstream way of working at math." — Kevin Hartnett
> "We have the notion of a small trusted kernel. Uh Lean's massive has zillions of bugs, right? But to trust our results, uh in Lean, only to trust the kernel. That's much smaller, right?" — Leo de Moura
> "I think mathematics is going to be hugely accelerated by this software, because remember, it's already doing interesting things. And the thing you have to remember is it's at its worst, you know, today is the worst it will be, right? Going forward, it's going to be much better." — Kevin Buzzard
> "I think mathematicians found it useful to think about mathematics in this kind of software engineering you know, kind of way of thinking about, okay, you know, what are the different modules? What are the pieces? How do they interact? What are the the interfaces?" — Jeremy Avigad
> "I'm just currently feeling that mathematics remains a human activity. Because if I were to truly check that the computer did the right thing, I would have to unravel the whole proof in my head and check that it really adds up. Lean could verify proofs, but it couldn't check definitions because, logically speaking, definitions aren't right or wrong. They're just what you say they are." — Peter Scholze

## People mentioned
- Kevin Hartnett
- Leo de Moura
- Thomas Hales
- Kevin Buzzard
- Jeremy Avigad
- Peter Scholze
- Terence Tao
- Mario Carneiro
- Johan Commelin
- Patrick Massot
- Alexander Grothendieck
- Andrew Wiles
- John Baez
- Dustin Clausen
- Cédric Fournet

## Topics
`formal-verification` `proof-assistants` `lean-theorem-prover` `mathlib` `ai-in-mathematics` `kepler-conjecture` `mathematical-formalization`
