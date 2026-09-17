---
podcast: "The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)"
episode: "From Voice Agents to AI Avatars with Alexander Smola - #777"
published: 2026-09-16
duration: "1h04m58s"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/MLN7382779687.mp3"
episode_url: "https://twimlai.com/podcast/twimlai/voice-agents-ai-avatars"
transcript_source: web
generated_at: 2026-09-17T22:19:42Z
model: "deepseek-v4-pro"
guid: "5a17b890-b20e-11f1-bc6f-87362412e42a"
---

# From Voice Agents to AI Avatars with Alexander Smola - #777 — The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)

## TL;DR

Alexander Smola, co-founder and CEO of Bosan AI and professor at Carnegie Mellon, discusses the current state and future of voice AI and AI avatars. He explains that voice is an intermediate step toward human-like audio-visual agents, but natural interaction remains far off due to latency, interruption handling, and emotional intelligence challenges. Bosan AI focuses on affordable, real-time audio models, leveraging 100 million hours of audio, and has developed benchmarks like Proactbench to measure voice interaction quality.

## Key points

- Voice AI today (e.g., ChatGPT advanced voice) is brittle: it only works in silent rooms, requires careful interruption handling, and breaks in noisy environments; better systems require microphone arrays and noise cancellation, not just a single microphone.
- Human audiovisual perception has a latency of about 150 milliseconds from stimulus to cortex, and humans operate at roughly 6-10 Hz; voice models must be interruptible within ~150 ms to feel natural.
- Audio tokenization creates a tradeoff: high fidelity requires many tokens per second (10+), which limits model size due to compute cost; text is more compressed (3-5 tokens/sec), allowing larger models for reasoning.
- Bosan AI's approach prioritizes affordability: rather than using trillion-parameter models for chitchat, they optimize to beat GPT, Gemini, and Grok on benchmarks at a fraction of the cost, often with thinking/reasoning disabled to reduce latency.
- The company has accumulated about 100 million hours of audio (equivalent to 200 human lifetimes), sourced from the internet and processed in-house using own data center to avoid cloud storage costs; heavy engineering goes into extraction, tagging, normalization, and transcription.
- They did not build an LLM from scratch; they use existing LLMs as a prior and add audio modality, but must perform full pre/mid/post training to prevent catastrophic forgetting of language and reasoning abilities.
- Architecture is not always end-to-end single model; they use hierarchical/multi-stage systems where a main conversational thread fires tool calls and MCP servers in background, with the model deciding when to answer directly or search and when to stall conversation naturally.
- They released benchmarks Proactbench and Ibench to measure voice interaction quality, including interruptibility, responsiveness, and audio-text alignment; these are essential for optimizing user experience beyond task completion.
- Emotional intelligence (EQ) is critical: models must handle backchanneling (e.g., Japanese 'hai' or 'un' cues) without stopping, and scene-dependent interruptibility; the cheerful computer from Hitchhiker's Guide is an example of poor EQ.
- Training on movies is unreliable because social behaviors are often unrealistic (e.g., romantic comedy stalking); instead, LLMs with theory of mind and user simulators (like Nvidia's digital personas) can be used for recursive self-improvement and robustness testing.
- Personalization and global learning are analogous to recommender systems: individual preferences (language, devices, behavior) and cultural norms (e.g., left hand eating in India, showing feet in Thailand) can be learned from interactions to improve agents globally and personally.
- Future timeline: audio will become 'bulletproof' in about a year; avatars/robots with animated faces will appear in 1-1.5 years, but hardware is hard; continuous video generation for avatars requires model modifications because backgrounds are typically boring and compressible.
- The boot screen trick from Apple illustrates UX illusion: progress bar is fake but timed based on previous boot time, creating a pleasant experience without solving impossible technical problem; similar techniques can manage delays in voice agents.
- The company started in 2023 with text, shifted to voice after realizing text interface was awkward; they built TTS model Hick Audio V2 and now have ASR and audio understanding models; they will expand to video inputs.

## Notable quotes

> "I would argue that voice is an intermediate stepping stone. You might think, well, you know, what's next? It's clear that eventually we will have avatars. And so I think what this is converging to is that you'll be talking to an AV agent that looks and feels like a human. And we're still very very far away from making this really natural. So it's quite exciting actually." — Alexander Smola
> "it takes about 150 milliseconds for you know basically a photon hitting your retina to your cortex actually doing something with it and that number is reasonably stable" — Alexander Smola
> "we have in the order of 100 million hours of audio" — Alexander Smola
> "for voice and then also for video you need to really care about how humans feel rather than just doing text only" — Alexander Smola
> "I tend to think that, you know, they've gone through several iterations of it. It does keep getting better, but it's still very infuriating." — Sam Charrington

## People mentioned

- Alexander Smola
- Sam Charrington
- Maja Matarić
- Robert Cialdini

## Topics

`voice-ai` `audio-processing` `human-computer-interaction` `inference-optimization` `ai-avatars` `personalization`
