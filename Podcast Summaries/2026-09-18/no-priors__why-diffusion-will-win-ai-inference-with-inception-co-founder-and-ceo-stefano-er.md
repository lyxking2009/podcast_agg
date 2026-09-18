---
podcast: "No Priors"
episode: "Why Diffusion Will Win AI Inference with Inception Co-Founder and CEO Stefano Ermon"
published: 2026-09-18
duration: "38m14s"
audio_url: "https://traffic.megaphone.fm/PDP7720707490.mp3"
episode_url: "https://www.youtube.com/watch?v=N1rjtDs8blY"
transcript_source: youtube_autocaptions
generated_at: 2026-09-18T22:31:31Z
model: "deepseek-v4-pro"
guid: "51c4092c-b309-11f1-8603-fb8d30e1277f"
---

# Why Diffusion Will Win AI Inference with Inception Co-Founder and CEO Stefano Ermon — No Priors

## TL;DR

Stefano Ermon — Stanford professor, Inception co-founder and CEO, and one of the fathers of diffusion — explains why diffusion language models can beat autoregressive models on the cost-quality frontier, how masking noise made text diffusion work, and why speed and efficiency will define the next few years of AI.

## Key points

- Ermon began at Stanford in 2014 working on generative models when the field was unfashionable — early papers had to be justified as a way to learn features from unlabelled data to improve supervised learning.
- He frames his early motivation as world models: being able to imagine what happens next is the basis for decision-making and model-predictive control, which requires generative capability.
- The path ran from slow blurry autoregressive image models, through VAEs, to GANs — which worked but were unstable to train and hard to reproduce.
- In 2019, with a PhD student, he developed score-based generative models: train a network to denoise images, then build a generative procedure from those denoisers. That became the basis of modern diffusion.
- Unlike left-to-right pixel generation, diffusion starts from pure noise and gradually refines the object until it is clean — and it now underpins the best models for image, video, some music, and protein generation.
- Moving diffusion from continuous data to discrete text and code required redefining noise as masked tokens and training the model to predict them, which enables parallel rather than sequential token generation.
- Inception's Mercury line is the commercial expression of this — 5-10x faster at inference than autoregressive models while matching quality, which matters most for latency-sensitive uses like voice agents and code completion.
- He notes transformers remain the underlying architecture at Inception; the training objective is orthogonal to architecture, so context scaling is unaffected by switching to diffusion.
- Ermon also co-advised work that became FlashAttention, and DPO started as a rotation project in his group — his argument being that academia's tolerance for contrarian bets is why so many foundational AI ideas originate there.

## Notable quotes

> "We came up with this idea of let's train a neural network to denoise images. And if you can denoise an image then you really are understanding enough about the structure of the image that it should be possible to build a generative procedure." — Stefano Ermon
> "Instead of generating images left to right one pixel at a time you start from pure noise and then you gradually refine the object until you get a clean picture at the end." — Stefano Ermon
> "The training objective... is kind of like completely orthogonal to the architecture. So at Inception we're still using transformers as the underlying neural network." — Stefano Ermon
> "One of the nice things about academia is that it allows you to take these contrarian bets." — Stefano Ermon

## People mentioned

- Stefano Ermon
- Sarah Guo
- Elad Gil

## Topics

- diffusion models
- Inception
- Mercury
- inference cost
- parallel decoding
- score-based models
- FlashAttention
- DPO
- world models
