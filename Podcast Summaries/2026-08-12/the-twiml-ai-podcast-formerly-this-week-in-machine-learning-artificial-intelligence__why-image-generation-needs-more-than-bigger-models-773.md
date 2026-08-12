---
podcast: "The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)"
episode: "Why Image Generation Needs More Than Bigger Models - #773"
published: 2026-08-12
duration: 56m47s
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/MLN4037379631.mp3"
episode_url: "https://twimlai.com/podcast/twimlai/why-image-generation-needs-more-than-bigger-models"
transcript_source: youtube_autocaptions
generated_at: 2026-08-12T22:21:24Z
model: "deepseek-v4-pro"
guid: "9ca7e710-9676-11f1-aec7-03563e574aa9"
---

# Why Image Generation Needs More Than Bigger Models - #773 — The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)

## TL;DR
Text-to-image models produce realistic images but struggle with controllability, identity diversity, and high-resolution efficiency. In this episode, Qualcomm's Fati Periqi discusses CVPR papers that address these gaps using reinforcement learning with explicit diversity and composition objectives (Disco, R2-CAM), a patch-based latent diffusion approach for on-device 4K/16MP generation (Pixel Rush), and inversion-based inpainting (Inver Field). The overarching message is that progress requires better training objectives and task decomposition, not just larger models.

## Key points
- Fati Periqi, VP of Technology at Qualcomm, argues that text-to-image models still face major challenges: they often generate nearly identical faces for multiple people, ignore requested identity or count, and hit quality, speed, and memory limits at higher resolutions. These problems are not solved by simply scaling model size; they require better training objectives and task decomposition.
- The Disco paper fine-tunes existing T2I models with reinforcement learning (group relative policy optimization, GRPO) using reward functions that explicitly encourage intra-image diversity (distinct faces within one image), inter-image diversity (different faces across runs of the same prompt), and correct person count, while preserving image quality. It achieves 98-99% unique face accuracy compared to a 10-20% gap for base models, and introduces the 'Diverse Humans' benchmark.
- Disco's approach shows that adding identity diversity as an explicit optimization objective is data- and compute-efficient: it does not require large new datasets or new architectures, but leverages curriculum learning (starting with simple scenes and gradually increasing complexity) to stabilize RL fine-tuning.
- The R2-CAM paper (referred to as RT-CAM in discussion) decomposes image generation into two components: an 'architect' that plans the composition (e.g., positions and number of faces) and an 'artist' that renders the final photorealistic image. Both are optimized with GRPO using reward terms for composition, identity matching, count accuracy, and pose. This separation mirrors how human artists plan before painting, allowing controlled multi-person scenes with specified identities.
- Fati Periqi suggests that instead of a single model solving all attributes, a more scalable approach is agentic orchestration: use specialized models for different attributes (e.g., facial diversity, text rendering) and a router that selects the right model based on the prompt. Qualcomm is actively working on such pipelines, with 'one size doesn't fit everyone' for top-quality generation.
- The Pixel Rush paper targets on-device high-resolution image generation (up to 16MP) by generating a base 1K image, upsampling it to the target resolution, then encoding it into the model's latent space and dividing the latent into 16 patches. Each patch is refined with diffusion using guided noise injection, with extra noise near patch boundaries to avoid seams. This approach achieves roughly 35x speedup (e.g., from 10 minutes to ~20 seconds) compared to generating a large latent space directly, while staying within smartphone memory limits.
- Pixel Rush operates entirely in latent space rather than pixel space for patch refinement, because latent space is smaller and allows noise-induced flexibility to add meaningful texture. The method does not require retraining or changing the base T2I model, making it compatible with any existing model.
- The Inver Field paper addresses image inpainting (object removal/editing) by inverting the input image into a specific noise (using the reverse diffusion process) in about 60 ms, then injecting new random noise only within the mask region. This preserves the background, harmonizes boundaries, and eliminates artifacts compared to starting from pure random noise, without modifying the original model.
- Qualcomm also presented video generation papers at CVPR: one optimizes the open-source Wan model to run 5x faster on memory-limited devices, another introduces hybrid recurrent attention, and a third performs 'attention surgery' to reduce compute. These efforts aim to make video generation accessible on PCs, laptops, and phones.

## Notable quotes
> "Maybe we are also asking a single model to solve too many difficult problems at once." — Fati Periqi
> "Instead of asking one model to do everything what if we separated for instance planning from rendering similar to what human artists might do" — Fati Periqi
> "Existing training objectives focus heavily on the realism and matching the user prompt, but they don't explicitly encourage diversity." — Fati Periqi
> "The next frontier in image generation is closing this gap between plausible images and precise results." — Sam Sharington

## People mentioned
- Fati Periqi
- Sam Sharington

## Topics
`text-to-image-generation` `controllability` `reinforcement-learning` `on-device-ai` `diffusion-models`
