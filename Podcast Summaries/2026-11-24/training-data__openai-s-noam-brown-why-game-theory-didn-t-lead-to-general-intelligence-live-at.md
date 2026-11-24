---
podcast: "Training Data"
episode: "OpenAI’s Noam Brown: Why Game Theory Didn’t Lead to General Intelligence (Live at No Limit Finals)"
published: 2026-11-24
duration: "30m38s"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/CPUAI4645919503.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-11-24T23:04:29Z
model: "deepseek-v4-pro"
guid: "fa4bc7f0-e514-11f1-8a20-23a2aece5414"
---

# OpenAI’s Noam Brown: Why Game Theory Didn’t Lead to General Intelligence (Live at No Limit Finals) — Training Data

## TL;DR
Noam Brown traces his path from poker AI (2012–2015, MIT Pokerbots wins) to Diplomacy, where game theory failed because it assumes rational players, leading him to pivot to modeling human irrationality and joining OpenAI to work on O1. He identifies reinforcement learning on chain of thought as the key unlock for general reasoning, enabling test-time compute scaling, and discusses current LLM strengths and jaggedness, the centaur period of human+AI collaboration, and rapid progress toward recursive self-improvement. He advises students to jump into AI now and predicts no game is out of reach, though variance still matters in poker.

## Key points
- Noam Brown's research path began with poker AI in 2012: he joined grad school to work on an AI poker bot because he loved game theory and poker, and believed there was a perfect strategy that could make unlimited money. He worked on it through his PhD and won the MIT Pokerbots competition in 2014 and 2015 with an MIT collaborator, earning $10,000, but never used his bot to play online poker.
- The original motivation for poker was to advance beyond perfect-information games like Go and Chess, because techniques for those were narrow. Poker introduced imperfect information, but after building poker bots, Brown found they weren't good for Chess or Go, prompting a more ambitious target.
- In 2019, with GPT-2 emerging, Brown and teammates chose Diplomacy as the hardest game: a natural language negotiation game with cooperation and competition. They thought it would take years and that game theory might help, but the game-theoretic approach failed because it assumes all players are rational and that rationality is common knowledge.
- Brown illustrates the failure with the ultimatum game: game theory optimal play is to offer the other player a penny (or very small amount) because any positive amount is better than zero, but real people reject such low offers. Diplomacy bots trained with game theory started producing similarly exploitative behaviors, and attempts to encode fairness or vengeance became too nuanced.
- The breakthrough for Diplomacy was abandoning game theory and instead training on human data to model human irrationality: build imitation models of people, train the AI with those imitation humans, then deploy against real humans. This was a 'crisis of faith' for Brown, whose expertise was computational game theory, but he concluded game theory was not the path to general intelligence.
- In 2022, Brown pivoted to general-purpose reasoning and joined OpenAI to work on what became O1. At the time, many were extremely bullish on scaling pretraining, but Brown argued that making a model play Go confidently would require about six orders of magnitude more training, which was unrealistic, and that reasoning was essential.
- The key unlock for reasoning in LLMs was reinforcement learning on chain of thought. Simply prompting 'think step by step' was coarse; RL training produced much better chain-of-thought behavior, and importantly, the longer the model thinks before responding, the better it performs. This scales test-time compute in a general-purpose way.
- Brown tracks general LLM poker performance: with reasoning models like Astro (GPT-6 Astro), he says LLMs have become able to make money playing online poker, though they still struggle to autonomously build a full-scale poker bot; Astro can make an excellent river solver from a single prompt but not a complete bot.
- Asked how he would build a poker bot in 60 minutes at this tournament, Brown says a low-fidelity GTO bot could be made using K-means clustering to reduce abstraction size and 30 minutes of self-play, but for the multiplayer format with all competitors, an exploitation-based approach that observes and exploits opponent weaknesses might be better.
- Brown describes the current era as the 'centaur period' where human+AI is most effective. In the tournament, neither a person alone nor an unprompted LLM can produce a winning bot in an hour; success comes from working with the AI hand in hand.
- What models are missing: long-term planning, organization, and noticing when they are going down the wrong direction. Example: when asked to implement CFR, models get overly focused on perfecting numerics even though CFR is robust to small errors, wasting hours on unnecessary optimization.
- On recursive self-improvement (RSI), Brown argues it is not novel—human technological progress is a giant RSI loop, and LLMs have been used to aid in building next-generation LLMs for years, e.g., for data quality checks in 2022–2023. He expects rapid progress but bottlenecks on compute and long-tail tasks requiring humans; jokingly/seriously advises promising students to drop out and join labs now.
- Brown believes any specific game can be solved superhuman with a few months of focused effort; Diplomacy was the hardest he could imagine, and pitches like Civilization or Zelda Wind Waker are likely solvable now. The real challenge is evaluating general-purpose LLMs on games they weren't specifically trained for; currently they aren't superhuman at real-time strategy or Zelda, but that may change in a year.
- For one AI agent in the World Series of Poker within 5 years: if playing GTO, variance is too high to guarantee beating all humans; but if superhuman at reading people (detecting bluffs/cards), it could be extremely strong, though luck factor remains. Brown gives about 20% chance it could win. He also names Doug Polk as the best human poker player he's watched, though likely no longer the best.

## Notable quotes
> "I think we're entering a period, the centaur period, where the combination of human plus AI is going to be like most effective." — Noam Brown
> "the problem with game theory is that it assumes this is the the central premise in game theory that it assumes that all the other players are rational and that it is common knowledge that all players are rational. And people are not actually rational." — Noam Brown
> "The key ingredient was yeah, if you do reinforcement learning to train for this behavior, then you actually get much better chain of thought behavior, you get much better outputs. And then also what we observe at that point is actually the longer the model takes to think before responding, the better it does." — Noam Brown
> "if somebody is uh a promising student, the best thing to do is actually to drop out and join the labs now." — Noam Brown
> "I think that chain of thought is sufficient for scaling test time compute." — Noam Brown

## People mentioned
- Noam Brown
- Alfred
- Doug Polk
- Daniel Kahneman
- Jasper

## Topics
`game-theory` `poker-ai` `diplomacy-ai` `reinforcement-learning` `reasoning-models` `ai-careers` `recursive-self-improvement`
