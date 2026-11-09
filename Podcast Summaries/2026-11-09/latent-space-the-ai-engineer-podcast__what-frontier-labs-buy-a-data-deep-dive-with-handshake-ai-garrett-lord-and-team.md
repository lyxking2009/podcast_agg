---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "What Frontier Labs Buy: A Data Deep Dive with Handshake AI — Garrett Lord & Team"
published: 2026-11-09
duration: ""
audio_url: "https://podcasts.beehiiv.com/3967c1f2-cb97-4eb0-a5c0-21a1d2579408/01a15fdf-0e93-7e1a-880d-d73752a1f915/01a1c0f3-0e12-7b37-bb7f-c0b59fbc3b39/01a1c0f3-1027-7b6d-887c-fbde19347d32.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-11-09T23:09:36Z
model: "deepseek-v4-flash"
guid: " 01a1c0f3-0e12-72e9-b44b-8ca8a4fe611d "
---

# What Frontier Labs Buy: A Data Deep Dive with Handshake AI — Garrett Lord & Team — Latent Space: The AI Engineer Podcast

## TL;DR
Latent Space's first deep dive into an AI data company, and a 164-minute one: Garrett Lord and the Handshake AI team walk swyx and co-hosts through what frontier labs actually buy, using live demos of the RL environments they sell - a bank deal in finance, tissue slides in bio, file-access tasks in coding, and safety simulations in alignment. Handshake went from launch to $1B in gross annualized revenue in about a year.

## Key points
- The business: Handshake's career-network asset - a very large population of PhDs and advanced-degree holders - turned out to be exactly what labs needed for expert data, evals and RL environments. That re-founding took it to roughly $1B in gross annualized revenue in about a year.
- The core thesis is that post-training data is not labeling: labs are buying environments where agents must complete realistic multi-step work and be graded on it, not one-off question-answer pairs.
- Finance demo (Project Falcon): the agent has to find its assignment in a bank's inbox, work out which documents govern the deal, and deliver a workbook. The failure mode shown is instructive - the model can explain a tax rule when asked directly, but fails to recognise that a specific line in the client's email is where the rule applies, carrying the number into Python and subtracting it incorrectly.
- Bio demo (VIALS): Nick Magazine puts nine brain-tissue slides on screen and asks which two show the most advanced vessel necrosis. The model picks the right two and adds a third - and watching the expert point at what is missing from the tissue teaches more than the score does.
- Coding/agent demo: the team inspects which files an agent read, whether it checked the copies, and what it did to the shell history afterwards. A transcript of the final answer would miss most of what is being evaluated - the trace is the product.
- How labs evaluate a dataset: they test candidate data on open models first, look for transfer, and hold out evaluation tasks so vendors cannot overfit to the benchmark.
- Where rubrics fall short: expert judgment captures things a written rubric misses, which is the economic reason a human-expert network still matters at the frontier.
- Alignment segment: CAREBench and the limits of a refusal - safety behaviour has to be trained and evaluated as a trajectory of tool calls, not a single response.
- Market structure takeaway: compute and expert data are becoming the two scarce inputs, and enterprises increasingly want private benchmarks and their own models rather than a public leaderboard score.
- Caveat worth carrying: this is a vendor showcase - the demos are chosen by Handshake, so read the failure modes as the honest part of the episode.

## Notable quotes
> "Handshake went from launch to $1 billion in gross annualized revenue pushing the envelope of specialized data for frontier labs." — episode description
> "What are frontier labs buying at that scale? What evals are models still nowhere close to human level on?" — swyx, episode framing
> "The model can explain the tax treatment when you ask it directly. The mistake comes when it has to recognize that this particular line of code is where the rule applies." — episode show notes, finance demo
> "Watching Nick point to what's missing from the tissue is more instructive than seeing the score." — episode show notes, bio demo
> "A transcript of the agent's final reply would miss most of what Steve Kelling and Nishant Ranka are evaluating." — episode show notes, coding demo

## People mentioned
- Garrett Lord - co-founder and CEO, Handshake
- swyx - Latent Space
- Sahil Bhaiwala - Handshake AI
- Nishant Ranka - Handshake AI
- Nick Magazine - Handshake AI (life sciences)
- Jonas Mueller - Handshake AI (life sciences)
- Collin Schweiker - Handshake AI (finance)
- Steve Kelling - Handshake AI (alignment)

## Topics
- AI training data
- RL environments
- evals
- Handshake AI
- frontier labs
- post-training
- AI benchmarks
- expert data
- alignment
