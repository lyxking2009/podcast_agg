---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Runway’s WorldPrompt and the Engineering of Real-Time Worlds"
published: 2026-09-24
duration: "1h36m21s"
audio_url: "https://api.substack.com/feed/podcast/217289983/369e64e1c34f479bca75098a1d53f814.mp3"
episode_url: "https://www.latent.space/p/runway"
transcript_source: web_search_fallback
generated_at: "2026-09-25T22:07:41Z"
model: "deepseek-v4-flash (Hermes manual fallback)"
guid: "substack:post:217289983"
---

# Runway’s WorldPrompt and the Engineering of Real-Time Worlds — Latent Space: The AI Engineer Podcast

## TL;DR

How a video model becomes a real-time runtime: Runway’s GWM Worlds 2 streams interactive worlds as continuous 720p video at 24fps with 48kHz audio, built by fine-tuning an audio-video model to the new WorldPrompt format and post-training it to generate autoregressively, then using distillation to hit real time — with error accumulation and long-term memory as the open problems.

## Key points

- WorldPrompt is Runway’s proposed input format for specifying a generated world and the actions within it: you can fix some aspects of a simulated environment — including the first frame — and then create a series of timestamped events, which can even be prompted in real time.
- Performance envelope for GWM Worlds 2: real-time interactive worlds streamed as continuous 720p video at 24 frames per second with audio at 48,000 Hz.
- The engineering recipe, in order: take the foundational audio-video generation model and fine-tune it to the WorldPrompt format so the model can follow it, post-train it to generate autoregressively, then make it real-time through distillation.
- The transition from bidirectional diffusion (‘generates an entire video at once’) to autoregressive generation is what allows the model to ‘generate one frame or a few frames at a time’.
- Two forms of distillation are discussed for hitting real-time: distilling a larger model into a smaller one, or reducing diffusion steps.
- The headline technical risk is error accumulation — because generated frames are fed back in to generate the next frames, small errors compound over time.
- Infinite generation raises a second class of problems: deciding what context to keep and what to discard, and optimising so the system is ‘not blowing up our GPU memory’.
- Long-term memory is an unsolved research problem — ‘the model does not have perfect memory’.
- Agent-adjacent use case: using world models alongside reasoning models — reasoning for scene planning, then handing off to the diffusion head that generates the pixels.
- Historical arc covered in the episode: Runway’s creative-tools thesis, the bet on ~1,000 A100s, starting from video-to-video and depth-to-video (Gen-1) because stronger conditioning is an easier problem, then Gen-2 as the first text-to-video model in market — and the early realisation that text-to-video alone was not the answer because users wanted more control.
- Source note: the feed item was recovered via browser-UA direct fetch after a Flightcast 167-byte error; content summarized from the public Latent Space post (latent.space/p/runway), which carries the structured summary and transcript excerpts.

## Notable quotes

> > ‘GWM Worlds 2 offers real-time interactive worlds streamed in continuous 720p video at 24 frames per second (fps) and audio at 48,000 Hz.’ — Latent Space write-up
> > ‘And after that, we work on making it real-time through distillation methods.’ — Kahlow, Runway
> > ‘The biggest challenge with autoregressive models is error accumulation... if there are any small errors, they accumulate over time.’ — Runway engineer
> > ‘There’s all these challenges around what context to keep, what to discard that’s not important. And so there’s all these optimizations we have to think about, so we’re not blowing up our GPU memory.’ — Sindi, Runway
> > ‘The model does not have perfect memory. That’s still an open research problem.’ — Kahlow, Runway
> > ‘You’re maybe using some reasoning [for] planning of the scene, and then you’re passing it into the diffusion head that’s actually generating the pixels.’ — Anastasis Germanidis, Runway
> > ‘People wanted a lot more control than that, and so we invested in control building on top of those models very quickly.’ — Anastasis Germanidis

## People mentioned

- Alessio Fanelli — co-host, Latent Space
- swyx (Shawn Wang) — co-host, Latent Space
- Anastasis Germanidis — co-founder/CTO, Runway (guest)
- Kahlow — Runway (cited on distillation and memory)
- Sindi — Runway (cited on inference optimisation)

## Topics

`world-models` `video-generation` `runway` `real-time-inference` `diffusion` `distillation` `ai-agents` `gpu`
