---
podcast: "The TWIML AI Podcast"
episode: "Why Jev Is Changing How We Build With AI with Diogo Almeida - #779"
published: 2026-10-06
duration: 1h30m09s
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/MLN2040611678.mp3"
episode_url: "https://twimlai.com/podcast/twimlai/jev-changing-how-we-build-ai"
transcript_source: youtube_autocaptions
generated_at: 2026-10-07T00:40:07Z
model: "deepseek-v4-flash"
guid: "edcb2910-c1cb-11f1-b902-a7ba02688fa0"
---

# Why Jev Is Changing How We Build With AI with Diogo Almeida - #779 — The TWIML AI Podcast

## TL;DR
Diogo Almeida — co-founder and CEO of TypeSafe AI, and a former OpenAI researcher who worked on InstructGPT and the RLHF pipeline behind ChatGPT — joins Sam Charrington to defend Jev against the charge that it is “just a classifier.” His argument: the gap between AI that is superhumanly good (chat, deep research, coding agents) and AI that is unusable at easy work (data entry, underwriting, customer service) comes down to “you get what you optimize for.” String LLMs are optimized for strings consumed by humans; Jev is optimized for typed, calibrated decisions consumed by software. Speed and cost are side effects — reliability, he insists, is what developers are actually paying for.

## Key points
- The elevator pitch is a question: where is all the automation? Almeida says AI is unbelievably smart yet unbelievably useless at the things you would most expect it to do — agents cannot do basic data entry or insurance underwriting, and even Anthropic's customer-support agent is great at throwing FAQs at you but cannot do something as easy as changing a credit card.
- His explanation of the divide: modern AI is one tool — a token-predicting transformer — being jammed into every problem. “You get what you optimize for,” and almost all string LLMs are optimized for strings meant to be consumed by humans or other LLMs. Work meant to be consumed by a computer (specific fields, forms, discrete decisions — accounting) is a fundamentally different optimization target, and that is the largest single explainer of the capability gap.
- Jaggedness is not intrinsic to LLMs; it is a product of optimization pressure. Models are super-reliable in the directions you push them: RLHF makes certain failure modes never occur (giving an unearned refund), practitioners largely choose their model's menu of desirable traits, and the asymmetry of what is easy to punish shapes behaviour at inference time.
- The “bitterest lesson” — his modification of Sutton's Bitter Lesson: compute matters more than algorithms, but data matters more than compute, and more important than either is the right task. RLHF (instruction following) had to create its data from scratch because none of the shape existed on the internet; first-principles task selection is where the gigantic jumps (ChatGPT, o1, Jev) come from.
- On the “Jev is just a classifier” critique: “Classifiers are an interface. Creating classifiers is not an ML innovation... they are literally the shape of usefulness.” Meta is powered by classifiers, Google probably mostly so. He says being called a zero-shot general classifier for anything would be the greatest compliment he has been given.
- What makes Jev different is not the interface, the speed, or the cost — it is the intelligence behind them. Speed and cost are “bad things” to sell (ideal speed is instant, ideal cost is zero); an embedding model plus logistic regression buys you exactly the intelligence of an embedding model plus logistic regression, which most copycats miss.
- TypeSafe's RLCD (Reinforcement Learning for Calibrated Decisions) is the differentiator and took far longer than expected. Almeida admits he predicted the project would take a week, and had written an internal OpenAI document titled roughly “we could have had AGI last year” estimating an RLHF-scale effort would suffice. It did not: LLMs are far better suited to human-pleasing than to calibrated, robust decision-making.
- OpenAI's decision-model announcement and the wave of open-source lookalikes replicate the interface, cost and speed — not the intelligence. He worries that if developers get burned by bad versions they will write off the entire subgenre.
- Stated goal: make AI “so reliable that it's boring like SQL” — predictable enough that you can write queries without running them against eval sets, giving software engineers superpowers. This is why they ship later than they could and why shipping decisions are still debates internally.
- On self-driving he distinguishes an engineering win from an AI win: Waymo as the gold standard, built from decomposed, well-understood, separately-verifiable components with programmed policy behaviour that generalizes out of distribution. His cynical loop for the LLM world is “make a demo, raise a Series A, promise reliability, never make it reliable, pivot to human-in-the-loop” — which he concedes is the rational move given how unreliable the models are.
- He does not know anyone who runs an LLM as a real separate dependency, because you cannot get abstraction: it breaks arbitrarily and the abstraction has to leak for you to see why, which is fine for a first-party app and nonsensical for a platform developers build on.
- Context: Almeida last appeared on the podcast just under 10 years ago (October 2016). Since then: Google, a retirement doing competitive gaming and improv, then ~4.5 years at OpenAI during which he worked on many of its greatest hits — always obsessed with the same question that led to TypeSafe and Jev.
- His framing of ML's place: ML is one part of a larger system and should be scoped to the part of the system it can genuinely do well; higher reliability comes from zooming in, decomposing, and being able to debug which component broke — very hard to do in a one-model-rules-them-all setting.

## Notable quotes
> “The elevator pitch I use for Jev is just, where is all the automation? I think AI is just so unbelievably smart, yet so unbelievably useless at the kinds of things you'd really expect it to be useful for.” — Diogo Almeida
> “You get what you optimize for... basically all the string LLMs have been optimized for strings, and strings are meant to be consumed by humans or other LLMs. But if you want something like accounting, that's meant to be consumed by a computer — specific fields and forms and discrete decisions.” — Diogo Almeida
> “Classifiers are an interface. Creating classifiers is not an ML innovation... They are literally the shape of usefulness.” — Diogo Almeida
> “If someone told us that we were a zero-shot general classifier for anything, that would be the greatest compliment I've ever been given.” — Diogo Almeida
> “I want to be a paragon of making AI so reliable that it's boring like SQL.” — Diogo Almeida
> “Reliability is what people are paying for. Reliability is what people want. And if you want to put intelligence to applications, you need the intelligence.” — Diogo Almeida
> “More important than data is you need the right task. This is the most important thing in all the ML that we do, and the right task is kind of the north star.” — Diogo Almeida
> “Demos will always be significantly ahead of the curve. OpenAI has been showing customer service demos since 2020... and customer service is still not solved.” — Diogo Almeida
> “ML is meant to be one part of a larger system. ML should be scoped to the part of the system that it can really do well.” — Diogo Almeida

## People mentioned
- Diogo Almeida — co-founder and CEO, TypeSafe AI; formerly OpenAI (InstructGPT, RLHF)
- Sam Charrington — host, The TWIML AI Podcast
- Paul Christiano — RLHF pioneer (referenced)
- Richard Sutton — author of “The Bitter Lesson” (referenced)

## Topics
`jev` `typesafe-ai` `machine-native-intelligence` `rlcd` `rlhf` `calibrated-decisions` `structured-outputs` `reliability` `automation` `bitter-lesson` `classifiers` `agents`
