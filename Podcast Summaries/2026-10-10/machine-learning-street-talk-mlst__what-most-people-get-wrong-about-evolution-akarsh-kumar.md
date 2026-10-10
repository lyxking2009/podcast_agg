---
podcast: "Machine Learning Street Talk (MLST)"
episode: "What Most People Get Wrong About Evolution | Akarsh Kumar"
published: 2026-10-10
duration: "47m54s"
audio_url: "https://traffic.megaphone.fm/APO3123732451.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/machinelearningstreettalk/episodes/What-Most-People-Get-Wrong-About-Evolution--Akarsh-Kumar-e3q8ds7"
transcript_source: youtube_autocaptions
generated_at: "2026-10-10T22:03:10Z"
model: "deepseek-v4-flash"
guid: "387cf19d-15d2-40f5-9752-7893a16aa830"
---

# What Most People Get Wrong About Evolution | Akarsh Kumar — Machine Learning Street Talk (MLST)

## TL;DR
Akarsh Kumar (MIT PhD student advised by Phillip Isola, collaborator of Sakana AI and of Kenneth Stanley, Jeff Clune and Joel Lehman on the Fractured Entangled Representation paper) argues that AI people wrongly dismiss evolution as naive random search. Tim Scarfe hosts a tour of artificial life: why we study "life as it could be" rather than life as it is, how ASAL uses a foundation model as the critic to search whole spaces of simulations, and how LLMs can act as mutation operators to create an open-ended adversarial arms race of Core War programs (Digital Red Queen). The thread: selection is underrated — retaining partial solutions turns exponential search into linear — and the path a learner takes shapes the representations it builds.

## Key points
- Artificial life studies "life as it could be", not "life as it is": to claim general principles of intelligence or life you must understand the space of all possible intelligences and life forms, not just one instantiation (the human brain, GPT, one car).
- ASAL (Automating the Search for Artificial Life with Foundation Models): instead of hard-coding one simulation, parameterize a *space* of simulations, run them, and ask a foundation model what happened. Candidate worlds include Conway's Game of Life, Lenia (Bert Chan), neural cellular automata, Boids and Particle Life.
- Sweeping all 262,144 Life-like rules, ASAL revealed a large island of non-open-ended rules and a small island where all the interesting/open-ended worlds sit — counterfactual analysis of "in which physics does emergence happen?"
- Persistence and cell-like objects emerge: some patterns exploit the physics to persist, foreshadowing the chain from self-replicating molecules to predator-prey dynamics to "alien animals".
- Statistical intelligence vs regularity-based intelligence: current models are trained in an "alien" way (learn calculus without arithmetic) and are near-perfect statistically, whereas evolution and human learning are path-dependent — and that sequential constraint is arguably what produces regular, robust, generalizable internal representations (the FER hypothesis).
- Curriculum learning must be *serendipitous*, not pre-planned — echoing Stanley & Lehman's "Why Greatness Cannot Be Planned" and Wolfram's computational irreducibility ("no shortcuts").
- Digital Red Queen: using LLMs as mutation operators to evolve Core War "warriors" (a Turing-complete assembly arena) into an open-ended adversarial arms race. The space is deceptive and non-local, so Map-Elites is required; first-round evolved warriors beat 96% of a dataset of ~300 human warriors.
- LLMs are bad at Red Code zero-shot, but excellent inside an evolutionary loop with a verifier — the same lesson as AlphaEvolve. Mutation only needs to succeed ~1% of the time for selection to compound progress (contrast AutoML-Zero's random-mutation program search).
- Later-round warriors generalize better to unseen human warriors, and the behavioral variance across warriors shrinks — the arms race trends toward a single generalist warrior.
- Kumar's research agenda: extract abstractions of natural evolution and artificial life to build better AI systems — an ultra-efficient abstraction of evolution rather than simulating every particle.

## Notable quotes
> "I would just want to clarify evolution is anything but random." — Akarsh Kumar
> "We need to understand the space of all possible intelligences, the space of all possible life forms that can be." — Akarsh Kumar
> "Instead of hard coding a single simulation, you parameterize a space of simulations... Just simulate it, see what happens, plug it into a foundation model, and see what the foundation model thinks about what happened." — Akarsh Kumar
> "There’s 260,000 rules, and we plotted all of them. So, we found that there’s literally like a big island of solutions which are not open-ended, and there’s a small island of solutions where all of the coolest simulations lie." — Akarsh Kumar
> "I really like to call them a statistical intelligence and they’re like basically perfect statistically. But this FER idea... it’s basically like a different paradigm of intelligence." — Akarsh Kumar
> "It’s like Game of Thrones, Turing edition." — Akarsh Kumar (on Core War)
> "The selection mechanism in evolution is really really underrated because if you solve like even one part of a puzzle and you hang on to that and you search for the other ones, you can turn your exponential search problem into a linear search problem." — Akarsh Kumar
> "But all it needs is your mutations to be good like 1% of the time." — Akarsh Kumar

## People mentioned
- Akarsh Kumar — MIT PhD student (advised by Phillip Isola); Sakana AI collaborator
- Tim Scarfe — MLST host
- Phillip Isola — MIT professor (Kumar's advisor)
- Kenneth Stanley — co-author (FER hypothesis; "Why Greatness Cannot Be Planned")
- Jeff Clune — co-author (FER); quoted by Kumar
- Joel Lehman — co-author (FER)
- Bert Chan — creator of Lenia
- John Conway — inventor of the Game of Life
- Craig Reynolds — Boids
- Tom Mohr — Particle Life
- Stephen Wolfram — computational irreducibility
- Mordvintsev et al. — Growing Neural Cellular Automata

## Topics
`artificial-life` `evolutionary-algorithms` `asal` `open-endedness` `core-war` `llm-guided-search` `curriculum-learning` `representation-learning` `foundation-models` `cellular-automata`
