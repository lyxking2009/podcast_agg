---
podcast: "Y Combinator Startup Podcast"
episode: "Waymo Co-CEO Dmitri Dolgov: “Move Fast And Ship Safely”"
published: 2026-08-04
duration: 0h49m24s
audio_url: "https://anchor.fm/s/8c1524bc/podcast/play/123764066/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-7-4%2F429211619-44100-2-80c61c5d2a7a4.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/ycombinator/episodes/Waymo-Co-CEO-Dmitri-Dolgov-Move-Fast-And-Ship-Safely-e3mvft2"
transcript_source: youtube_autocaptions
generated_at: 2026-08-06T02:41:20Z
model: "claude-sonnet-5"
guid: "22ad1f48-170f-42b8-81bf-e7b2fc3186c4"
---

# Waymo Co-CEO Dmitri Dolgov: "Move Fast And Ship Safely" — Y Combinator Startup Podcast

## TL;DR
Waymo co-CEO Dmitri Dolgov lays out seven engineering lessons from turning a 2009 research project into a commercial robotaxi service running roughly 500,000 trips per week across 15 US cities. He argues physical AI differs from digital AI in cost-of-error, latency, data, and validation, so teams must 'move fast and ship safely' rather than 'move fast and break things,' investing in multi-sensor redundancy, a structure-augmented end-to-end foundation model, closed-loop simulation, and a safety evaluation framework as the true competitive moat.

## Key points
- Waymo's first fully autonomous demo took about 18 months starting in 2009; turning that demo into a real product took roughly 15 years — a working demo is 'at best 1%' of the total engineering effort required.
- Physical AI faces four gaps digital AI doesn't: cost-of-error, latency (a car needs onboard inference in milliseconds), data (no labeled internet equivalent for the physical world), and validation (needs very high confidence before day one).
- Reliability follows an 'exponential ladder of nines': each additional nine of performance costs about 10x more effort, and at Waymo's scale a one-in-a-million-mile event becomes a daily occurrence.
- Waymo uses multiple complementary sensor modalities — cameras, lidar, and radar — arguing camera-only approaches hit a safety ceiling too early for full autonomy.
- The current core system is the 'Waymo foundation model': a multimodal encoder-decoder with a 'System 1/System 2' design — a fast path for split-second geometric reactions and a slow path for higher-latency semantic reasoning.
- Citing Richard Sutton's 'bitter lesson,' Dolgov says the key nuance is whether added structure 'fights scale' or 'channels scale'; Waymo's 'structure-augmented end-to-end' approach channels it.
- Closed-loop, high-fidelity simulation is essential — Waymo's simulator combines a behavioral world model with a sensing world model (leveraging Google DeepMind's Genie 3 work).
- Dolgov's final lesson: the model itself is 'table stakes'; evaluation and metrics are the actual strategic moat — Waymo's safety data shows the Waymo Driver is roughly 17 times better than human drivers on crashes causing serious injury.

## Notable quotes
> "So the thing you have to do is to move fast and ship safely." — Dmitri Dolgov (~00:03:21)

> "A working demo is 1% at best of the work that you have to do." — Dmitri Dolgov (~00:08:20)

> "So count your nines before you count your demo views." — Dmitri Dolgov (~00:14:44)

> "Your model is really table stakes, but eval and metrics, that's your most important, that's your strategic moat." — Dmitri Dolgov (~00:43:01)

> "The Waymo driver is about 17 times better than human drivers when it comes to crashes that cause serious injury." — Dmitri Dolgov (~00:46:50)

## People mentioned
- Dmitri Dolgov
- Richard Sutton

## Topics
`autonomous-vehicles` `self-driving` `waymo` `physical-ai` `world-models` `robotics` `ai-safety` `simulation` `foundation-models`
