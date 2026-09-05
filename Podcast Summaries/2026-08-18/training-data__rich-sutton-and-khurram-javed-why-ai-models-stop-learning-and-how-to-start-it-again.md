---
podcast: "Training Data"
episode: "Rich Sutton and Khurram Javed: Why AI Models Stop Learning, and How to Start It Again"
published: "2026-08-18"
duration: "53m43s"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/CPUAI4645036827.mp3"
episode_url: ""
transcript_source: web
guid: "f22093e6-9ac6-11f1-aa57-23eadad59cc7"
generated_at: 2026-08-18T22:11:52Z
model: "deepseek-v4-flash"
---
# Rich Sutton and Khurram Javed: Why AI Models Stop Learning, and How to Start It Again — Training Data

## TL;DR
On Sequoia's Training Data podcast, Rich Sutton — co-inventor of reinforcement learning, author of The Bitter Lesson — and his former student Khurram Javed, co-founder of Oak Lab, argue that the current LLM paradigm is a dead end: deployed models have frozen weights and never learn from experience, the internet is finite, and synthetic data is a poor substitute for real experience. They lay out Oak Lab's technical program — step-size optimization, generate-and-test, and Continual Backprop — for training models from scratch that learn how to learn, grounded in the Big World Hypothesis, with the stated ambition of a self-maintaining trillion-parameter mind running on 20 watts. (Summarized from BigGo Finance's structured summary of the episode, which includes verbatim quotes.)

## Key points
- **The Bitter Lesson, restated:** "Don't be distracted by human knowledge... Instead, focus on learning methods that will scale with computation, like search and like learning"; in practice, researchers with an affection for existing human knowledge ended up fighting learning itself — "in practice, they have been enemies."
- **Continual learning is the natural default:** "Before there was all this AI craziness, you wouldn't have to say continual learning... All learning is continual." The industry's obsession with static pre-trained models is the "weird" position.
- **LLMs: amazing breakthrough, category error:** Sutton credits LLMs as a positive example of the Bitter Lesson (scaling with computation: "you could just drink in the internet and scale so much") but a negative one because "the weights never change" once deployed; language is only "20% or a quarter of intelligence."
- **The finite internet and the synthetic-data dead end:** the world is "massively bigger than everything we stored on the internet," and the scaling paradigm is running out of human data; synthetic data is dismissed on three grounds — a synthetic world is a small world (a program, not reality), it scales with human expertise not compute (someone must decide what to generate), and simulation only worked for self-driving cars because large teams iterate against real-world feedback. Javed's sharpest version: "If all the engineers at OpenAI, Anthropic, or all the big labs went on vacation, who would generate the synthetic data?"
- **The Big World Hypothesis:** the world is infinitely complex, containing many agents each as complex as any single mind, so no agent can ever be optimal or complete; learning must be continuous because the world is not static — "what's important to me is what's going on in your mind right now"; no single system will ever learn everything; there will always be many minds.
- **The algorithmic gap:** continual deep learning is an algorithmic problem, not infrastructure or privacy; naive weight updates on a single stream trigger catastrophic forgetting, while batch updating from thousands of users (Cursor, Tab) dilutes any individual's learning signal.
- **Two cures:** (1) step-size optimization — every weight gets its own step size, most moving slowly so new examples don't destroy them; (2) generate-and-test in feature space — the network must propose new features without gradients (gradient descent consumes its initial randomness over time), injecting fresh random units that backpropagation then tests; this is the basis of Continual Backprop, published in Nature.
- **Train from scratch or not at all:** these algorithms cannot be retrofitted onto pre-trained models — the model must learn how to learn while learning content; big labs are "so locked into a product that it is not possible for them to pursue a path where things get worse for a while" — "it is almost impossible that things will not get worse before they get better."
- **The missing capability:** "The big challenge that we don't see in our field yet is the ability to learn a model and then plan with a model" using self-discovered abstractions; AlphaGo/AlphaZero work because game rules are known in advance, math proofs because operators are given; elite athletes develop "weird niche terminologies" for movements — forming new abstractions and planning with them is "totally missing in our current systems."
- **Oak Lab's bet:** a trillion-parameter model on 20 watts (two orders of magnitude of Moore's law over 5-10 years; today's ~2,000W equivalent is already achievable with the right algorithms); a deliberately small team ("maybe a handful or two or three" researchers) that grows slowly, betting the current paradigm is a local minimum.
- **Closing prediction:** when genuine continual learning with abstraction formation arrives, the LLMs "might be at risk" — though Sutton credits them with a good run and a genuine scientific breakthrough.

## Notable quotes
> "Don't be distracted by human knowledge as AI traditionally has been many times. Instead, focus on learning methods that will scale with computation, like search and like learning." — Rich Sutton
> "There's no reason why there has to be a conflict between prior knowledge and then learning knowledge... But in practice, they have been enemies." — Rich Sutton
> "Before there was all this AI craziness, you wouldn't have to say continual learning, because it wouldn't make any sense to talk about learning that wasn't continual. All learning is continual." — Rich Sutton
> "If all the engineers at OpenAI, Anthropic, or all the big labs went on vacation, who would generate the synthetic data?" — Khurram Javed
> "First you need to do what we call step size optimization. And it means every weight in your network has to have a separate step size. So some will move fast, some will move slow." — Rich Sutton
> "The big challenge that we don't see in our field yet is the ability to learn a model and then plan with a model." — Rich Sutton
> "Our biggest ambition is to have a mind that is self-consistent and can keep training itself and make it coherent." — Rich Sutton
> "It is almost impossible that things will not get worse before they get better." — Khurram Javed, on why big labs cannot pivot
> "The weights never change." — Rich Sutton, on deployed LLMs

## People mentioned
- Rich Sutton — co-inventor of reinforcement learning; author of The Bitter Lesson; Alberta Plan
- Khurram Javed — co-founder, Oak Lab; former Sutton student
- Oak Lab — venture building continual-learning models

## Topics
- Continual learning, catastrophic forgetting, The Bitter Lesson, reinforcement learning, Big World Hypothesis, synthetic data limits, Continual Backprop, step-size optimization, generate-and-test, Oak Lab, LLM limitations
