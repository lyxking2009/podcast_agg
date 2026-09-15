---
podcast: "Machine Learning Street Talk (MLST)"
episode: "How Physical AI Learns Across Language, Video and Action — Ming-Yu Liu"
published: 2026-09-15
duration: "25m57s"
audio_url: "https://traffic.megaphone.fm/APO4863104315.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/machinelearningstreettalk/episodes/How-Physical-AI-Learns-Across-Language--Video-and-Action--Ming-Yu-Liu-e3ospub"
transcript_source: web
generated_at: 2026-09-15T22:17:14Z
model: "deepseek-v4-pro"
guid: "5207ae6b-de37-4063-b419-058ae6782077"
---

# How Physical AI Learns Across Language, Video and Action — Ming-Yu Liu — Machine Learning Street Talk (MLST)

## TL;DR

Ming-Yu Liu of NVIDIA discusses Cosmos 3, a multimodal world model that ingests text, video, audio, and actions, and can act as a visual-language model, a video generator, and a closed-loop simulator for physical AI. The architecture combines an autoregressive vision-language model with a bidirectional diffusion generator, and training forward dynamics, inverse dynamics, and policy jointly yields synergy across modalities. Cosmos Dreams enables scalable policy verification by ranking policies in simulation, and edge-sized models like Cosmos H bring world models to robots without data-center dependence.

## Key points

- Cosmos 3 is a single multimodal world model that accepts text, video, audio, and actions, and can perform visual question answering, video generation, and simulation for physical AI; it unifies three jobs including visual language modeling, generation, and closed-loop simulation.
- The model architecture starts from a language model, adds a vision encoder to create a vision-language model, then uses the VLM's weights to initialize a bidirectional diffusion generator that produces video, action, and audio; the autoregressive reason tower provides instruction tokens to the generator, enabling coherent multi-modal generation.
- Ming-Yu Liu defines a world model not as a single agreed-upon entity but as a collection of useful tools; in robotics, the three core modeling tasks are forward dynamics (predict future from state/action), inverse dynamics (infer action from visual transition), and policy (decide action to achieve task).
- Training all three tasks jointly under an information bottleneck forces the model to capture the underlying correlation between observations and actions; results in the paper show synergy, with each task helping the others.
- Different modalities operate at different frequencies (video frame rate, audio Hz, action rate); Cosmos uses a temporal position embedding scheme that normalizes all signals to the same time scale so tokens can identify co-occurring time instances and relative distances.
- There is an asymmetry in training data: abundant egocentric human video but much less robot data; however, shared visual-action patterns between human and robot manipulation allow skill transfer across embodiments, and the model can generalize to unseen embodiments because action-space translation is relatively easier.
- For ambiguous tasks, the model may fail; Liu advocates a system two layer above the model that decomposes tasks, checks completion, and makes choices, using a harness with memory and tools to make system one execution concrete.
- Cosmos Dreams is a set of skills for building closed-loop simulators for any embodiment; it is already good enough for navigation but manipulation remains harder due to occlusions and deformations during contact; Liu expects neuro-simulation to eventually handle complex manipulation for policy testing.
- Using a neuro-simulator for policy development initially focuses on passive verification: given many policy checkpoints, the simulator can rank them by success rate without needing exact real-world correspondence; only ranking preservation is required, greatly improving development velocity by reducing costly real-world rollouts.
- As a teacher, Cosmos provides better data, better environments, and better starting points; the team post-trained a Cosmos model on the DROID dataset (transcript says "joy data set" likely DROID) and achieved state-of-the-art pick-and-place policy results, demonstrating that predictive world models can bootstrap policy learning.
- Cosmos models come in three sizes—Super, Nano, and H (likely Ultra)—with H optimized for edge devices like NVIDIA Jetson Thor Orin and DGX Spark; local inference avoids data-center round trips, which is critical for latency-sensitive robotics tasks, and smaller models are easier to fine-tune.
- All Cosmos models, code, and some training data are open; models are on Hugging Face and code is on GitHub (NVIDIA/Cosmos), with post-training recipes to reproduce results, and the team invites community feedback.

## Notable quotes

> "I think war model is a collection of useful tools, right? We model something because we we are trying to achieve some goal" — Ming-Yu Liu
> "our generator is actually uh diffusion is birectional." — Ming-Yu Liu
> "the cosmos as the teacher. So in Cosmos right so we try to help the ecosystem in three ways better data better environment and better starting points" — Ming-Yu Liu
> "you just need to know if policy A is better than pol CB in neuro simulator and most likely poly A going to be better than policy B in the real world" — Ming-Yu Liu

## People mentioned

- Ming-Yu Liu

## Topics

`world-models` `robotics` `simulation` `vision-language-models` `autonomous-driving` `diffusion-models` `edge-ai`
