---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Building Visual Intelligence — Andreas Blattmann & Dustin Podell, Black Forest Labs"
published: 2026-11-16
duration: "1h38m23s"
audio_url: "https://podcasts.beehiiv.com/3967c1f2-cb97-4eb0-a5c0-21a1d2579408/01a15fdf-0e93-7e1a-880d-d73752a1f915/01a1e632-c02d-7e1d-84cb-b3d9453f63eb/01a1e632-c136-7f19-ba30-c3bc167811bd.mp3"
episode_url: "https://latent.space/p/bfl"
transcript_source: rss
generated_at: 2026-11-19T23:11:49Z
model: "deepseek-v4-pro"
guid: "01a1e632-c02d-713d-a840-cae7f745d45e"
---

# Building Visual Intelligence — Andreas Blattmann & Dustin Podell, Black Forest Labs — Latent Space: The AI Engineer Podcast

## TL;DR
Andreas Blattmann and Dustin Podell of Black Forest Labs recount their path from inventing latent diffusion and releasing Stable Diffusion to founding BFL, which has raised $450 million and surpassed $100 million in revenue. They detail technical breakthroughs such as SDXL's multi-aspect training, SD3's rectified flow and diffusion transformers, and Flux Context's efficient image editing trained on a tiny dataset, plus the multimodal Flux.3 model for images, video, audio, and robotics actions. Looking ahead, they predict 2027 will be 'the year of robotics' with single-shot action learning, driven by their visually intelligent model architecture.

## Key points
- Andreas Blattmann co-invented latent diffusion and has worked on diffusion models for seven years; he met Robin Rombach and Patrick Esser at Heidelberg's Computer Vision group, and together they developed the latent diffusion algorithm that became the basis for Stable Diffusion.
- Dustin Podell came from a software development background, not formal ML research; he became interested in generative AI after seeing Midjourney V1 and quit his job the day Stable Diffusion was released in summer 2022, later joining Stability AI as an engineer and working his way up to research lead.
- Black Forest Labs was founded by the core Stable Diffusion team (13-14 people), choosing Freiburg, Germany near the Black Forest as its base; the company has raised $450 million in total funding and surpassed $100 million in revenue, with the $100M figure announced in December 2024.
- Latent diffusion compresses images into a lower-dimensional latent space before applying denoising, dramatically improving efficiency and enabling models like Stable Diffusion to run on consumer GPUs, which was pivotal for the open-source AI movement.
- SDXL introduced large-scale multi-aspect training using bucket-style batching to handle multiple image resolutions, along with multiple training stages including pre-training and post-training, which improved aesthetics, prompt adherence, and usability compared to SD1.x.
- SDXL's refiner was actually a separate full diffusion model from an unreleased architecture, trained to add high-frequency details after the base model generated structure; this early work exploited the insight that structural features are learned near the noise while fine texture is learned near the clean image.
- Stable Video Diffusion applied latent diffusion to video and showed that data quality filtering is crucial; a key experiment demonstrated that a video model pre-trained only on 2D videos could be fine-tuned for just 1,000 steps on 3D data to produce an explicit 3D model, revealing emergent 3D understanding.
- SD3 transitioned from classic diffusion to rectified flow matching (a type of flow matching) and from convolutional U-Nets to diffusion transformers; flow matching uses an ordinary differential equation to map between arbitrary probability distributions, simplifying training and enabling faster sampling without loss.
- SD3 investigated the noise-to-data trajectory and found that not all timesteps contribute equally to learning; oversampling the most informative regions during training improves efficiency, building on the refiner concept from SDXL.
- Improved text rendering in SD3 was attributed primarily to better modeling and better data (captions that accurately described visible text), rather than any specific architectural change, reinforcing that data quality is often more impactful than algorithm tweaks.
- After leaving Stability due to frustrations about capturing value from their models, the BFL founding team trained Flux.1 from scratch during a remote offsite in France; the model launched three months later and was widely praised, also powering xAI's image generation for a time.
- Flux Context, championed by Dustin Podell, enables image editing by conditioning on an image and text prompt without masks; an early prototype trained for only 4-6 hours on a small photography dataset showed emerging generality like adding logos to shirts, demonstrating how pre-trained relationships can be evoked with little training.
- Flux Context was initially limited to single-image editing; Flux.2 expanded this to support up to 10 reference images, enabling multi-reference reasoning, and BFL released the open Context Bench eval set to benchmark such models, built from internal evaluation prompts.
- Evaluations for generative models are challenging; BFL combines automated metrics with extensive human evaluation and encourages researchers to actively 'play with' the models to discover strengths and weaknesses, noting that viral use cases often emerge unpredictably (e.g., image restoration of old family photos).
- Flux.3 is a single multimodal model trained on a large corpus of images, videos, and audio, from which BFL derived Flux.3 Image, Video, Video Edit, Action, and a 4K video upscaler; this demonstrates that general pre-training compounds across applications, and the upscaler's temporal consistency avoids flickering.
- Flux Action for robotics extends the video model to predict robot actions alongside video: conditioned on camera frames and an instruction, it predicts a chunk of 9-21 frames with joint states (6 DoF per joint), enabling real-world control without a separate planner; current 7B models run at around 10 FPS.
- Because Flux.3 is pre-trained on broad natural video, fine-tuning for specific robot tasks is highly data-efficient; one industry application required only 4 hours of demonstration data, eight times less than other models like Octo, and BFL aims to achieve single-shot learning where a robot observes an action once and can replicate it, predicting 2027 as the 'year of robotics' with first manufacturing deployments.
- BFL frames visual intelligence as System 1 (fast, intuitive) intelligence from Daniel Kahneman's framework, distinct from language models' System 2 reasoning; they believe learning from natural signals (pixels, waveforms) without explicit 3D structure is sufficient for world understanding, and the debate between autoregressive and diffusion approaches remains open.

## Notable quotes
> "Stable Diffusion came out, and basically that day I turned in my resignation" — Dustin Podell
> "video modeling is actually the central means to understanding how our world basically works" — Andreas Blattmann
> "You should be showing a robot an action one time, and then it should be able to do this, and this would be truly a superhuman intelligence" — Andreas Blattmann
> "It was maybe like six hours of training, four hours of training. It was, it was a very, very short training run, and all of a sudden, like, I saw what felt like a clear generality coming out of it" — Dustin Podell

## People mentioned
- Andreas Blattmann
- Dustin Podell
- Robin Rombach
- Patrick Esser
- Anj Mittal
- Imad Mostaque
- Greg Rutkowski
- Scott Hawley
- Lukas Beyer
- Tim Dawhorn
- Daniel Kahneman
- Gary Marcus
- Axel

## Topics
`diffusion-models` `image-generation` `video-generation` `robotics` `world-models` `generative-ai` `black-forest-labs`
