---
podcast: "Machine Learning Street Talk (MLST)"
episode: "Speech Recognition Is Not a Solved Problem — Pavan Muddireddy"
published: 2026-09-14
duration: "1h42m22s"
audio_url: "https://traffic.megaphone.fm/APO7397293133.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/machinelearningstreettalk/episodes/Speech-Recognition-Is-Not-a-Solved-Problem--Pavan-Muddireddy-e3orlk9"
transcript_source: web
generated_at: 2026-09-14T22:04:50Z
model: "deepseek-v4-flash"
guid: "05a9d7a4-9b0e-49ec-99c1-cb90da7a28fa"
---

# Speech Recognition Is Not a Solved Problem — Pavan Muddireddy — Machine Learning Street Talk (MLST)

*(Guest: Pavankumar Reddy Muddireddy, who leads audio research at Mistral AI, with Tim Scarfe. Episode produced in partnership with Mistral AI. Source: the podcasters.spotify.com episode page — full description, topic map and reference list.)*

## TL;DR

A deep technical tour of Mistral's Voxtral family with the person who leads audio research there, Pavankumar Reddy Muddireddy. Tim Scarfe's organising argument — and the episode's title — is that deployed speech recognition is *not* solved: what customers actually run is a cascade of specialised models, not one end-to-end system, and quality falls off sharply outside the top few languages. Along the way Muddireddy walks through Voxtral's architecture (a 3B Ministral trunk consuming continuous audio embeddings as direct token input rather than through cross-attention, so emotion, timing and speaker identity survive into the decoder), the dual-stream real-time decoder targeting delays down to 160ms, and the TTS stack's move from discrete codec tokens to continuous latents via the SoundStream → EnCodec → Mimi lineage with FSQ and flow matching. The most interesting material is on failure modes: autoregressive in-transcript diarisation makes streaming speaker attribution fragile, and because the architecture commits to its own predictions, a single out-of-distribution mistake compounds into loops or skipped segments — which is precisely what DPO is there to correct. The closing theme is that voice works *beside* a screen rather than instead of one, because absorbing information and deciding in one serial stream is real cognitive debt.

## Key points

- **Architecture — no intermediate transcript.** Voxtral Chat feeds a 3B Ministral text trunk with continuous embeddings from an audio encoder, passed to the decoder as direct token input rather than through cross-attention as in Whisper. That lets the model answer questions about emotion, timing and who spoke when without an intermediate transcript losing those signals.
- **Real-time variant.** The real-time model is a dual-stream decoder that consumes audio and emits text at once, targeting latency down to 160ms, with slower streams running in parallel for anything that can wait for more context.
- **TTS predicts continuous latents, not discrete codec tokens.** Muddireddy traces the lineage from SoundStream through EnCodec to Mimi's split of semantic and acoustic codebooks, and places FSQ and flow matching in that story.
- **On priors:** why a mel spectrogram rather than a raw waveform, what noise augmentation actually buys, and when acoustic overfitting becomes somebody's fine-tuning problem.
- **Failure mode 1 — streaming diarisation is fragile.** Diarisation is emitted autoregressively inside the transcript rather than by a separate head. With less context you get late speaker changes and invented extra speakers.
- **Failure mode 2 — compounding errors.** The architecture commits to what it has already predicted, so a single out-of-distribution mistake compounds into looping or skipped segments. DPO is what corrects this: the negative supervision that pre-training and SFT cannot provide.
- **The central claim: customers run scaffolding, not solved ASR.** Teams running voice agents across millions of sessions describe a scaffolded stack, with a sharp quality drop outside the top few languages.
- **Why cascades survive.** Cascades persist because each component stays separately adaptable, observable and constrainable — an argument against the single end-to-end model where deployments actually live.
- **Voice alone is cognitive debt.** Absorbing information and deciding in one serial stream is harder than glancing at a menu. Voice becomes ubiquitous *beside* a screen, not instead of one.
- **Topic map (from the episode page):** cold open; why Mistral moved into audio; inside Voxtral (trunk, encoder, dual streams); speech that works in real time; how a voice becomes tokens; flow matching, FSQ and the new codec; when speech models lose the speaker; correcting hallucinations with preferences; controlling synthetic speech; why cascades still win; speech in the wild; audio models as interfaces; why voice still needs a screen.
- **References cited** include Mistral 7B, Voxtral, Whisper, Voxtral Realtime, Kyutai's Delayed Streams Modeling, Voxtral TTS, SoundStream, Flow Matching for Generative Modeling, EnCodec, Moshi/Mimi, FSQ, DPO, and Mozilla Common Voice.

## People mentioned

- Pavankumar Reddy Muddireddy — leads audio research at Mistral AI, guest
- Tim Scarfe — MLST host
- Mistral AI — producer partner and developer of the Voxtral model family
- Kyutai — authors of Delayed Streams Modeling, referenced
- Mozilla — publisher of the Common Voice dataset, referenced

## Topics

`speech-recognition` `voxtral` `mistral-ai` `tts` `diarisation` `flow-matching` `fsq` `dpo` `audio-codecs` `voice-agents` `cascades`
