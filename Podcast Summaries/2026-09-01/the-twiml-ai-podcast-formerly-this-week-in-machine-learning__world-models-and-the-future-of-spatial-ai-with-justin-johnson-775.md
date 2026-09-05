---
podcast: "The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)"
episode: "World Models and the Future of Spatial AI with Justin Johnson - #775"
published: "2026-09-01"
duration: "1h6m2s"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/MLN6378358457.mp3"
episode_url: "https://twimlai.com/podcast/twimlai/world-models-future-spatial-ai"
transcript_source: web
generated_at: 2026-09-03T22:35:49Z
model: "deepseek-v4-pro"
guid: "dc2b8d24-a61b-11f1-88be-237abfdc47ee"
---
# World Models and the Future of Spatial AI with Justin Johnson - #775 — The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)

## TL;DR
Justin Johnson, co-founder of World Labs and University of Michigan professor, discusses the emerging field of world models—systems that understand space, predict environment changes, and act in the world. He clarifies three meanings of the term: implicit world knowledge, the reinforcement learning POMDP formulation, and generative models that create navigable worlds. Johnson explains World Labs' two approaches: Marble, which generates explicit 3D Gaussian splat worlds from images or text, and RTFM, an implicit real-time frame model. He introduces a taxonomy categorizing world models as renderers, planners, or simulators, and argues that future unified models will combine these capabilities, with transformers remaining central but needing advances in long-context handling and training objectives.

## Key points
- Justin Johnson co-founded World Labs with Fei-Fei Li and is an associate professor of computer science at the University of Michigan.
- There is no consensus definition of 'world model'; the term is used for at least three distinct concepts: implicit world knowledge (models that must internally represent the world to produce correct outputs), the original reinforcement learning POMDP formulation (predicting next state from current state and action), and generative models that produce navigable, self-consistent artificial worlds.
- The world model concept originated in reinforcement learning as part of partially observable Markov decision processes (POMDPs), where an agent takes actions, the world has a hidden state that transitions, and the agent receives observations as low-dimensional projections of that state.
- Johnson distinguishes ground-truth state (a complete description relative to a chosen abstraction) from learned state (a neural vector representation trained to behave like state), analogous to model-based versus model-free reinforcement learning.
- World Labs' Marble product accepts text prompts, images, multiple images, or videos, generates a 360-degree panorama, and lifts it into an explicit 3D Gaussian splat world; users can render it to images/videos or convert to meshes for game engines.
- Marble differs from traditional Gaussian splatting reconstruction because it uses a large pre-trained generative model that has learned from diverse real and fantastical data, rather than simply fitting a point cloud to a fixed set of images.
- World Labs' RTFM (Real-Time Frame Model) takes an implicit approach: it directly generates pixels in real time in response to user navigation inputs, with no explicit 3D representation, relying on model scale and training data for consistency.
- Gaussian splats are a differentiable 3D representation where each point has position, opacity, color, and spherical harmonics for view-dependent color; unlike triangle meshes, they integrate well with neural networks because they are smooth and allow gradient backpropagation.
- Johnson argues that explicit 3D representations like Gaussian splats provide consistency by construction, while implicit pixel-only models can achieve consistency through large data and compute, and the implicit approach is more scalable to unlimited data but requires higher training and inference cost.
- Johnson proposes a taxonomy of world models based on which part of the POMDP loop they output: renderers (output observations, e.g., Genie, RTFM), planners (output actions, e.g., robotics policies), and simulators (output or evolve state); Marble straddles renderer and simulator.
- He predicts the field will develop unified world models that combine rendering, planning, and simulation in one system with shared parameters, surfacing different outputs (actions, states, observations) via different heads as needed.
- On architecture, Johnson believes transformers remain powerful and are unlikely to be replaced soon; the key open problems are loss function choice (diffusion, autoregressive, etc.) and handling very long contexts (hundreds of thousands to tens of millions of tokens) required for world modeling.
- He cautions against hard-coding geometric symmetries or complex inductive biases, as they often break down and limit scalability; simple representations with large-scale training are preferred.
- Johnson recommends trying Marble at marble.worldlabs.ai for hands-on experience, and notes there is no canonical textbook on world models yet.

## Notable quotes
> "There isn't a clear definition of world models that everyone in the field agrees on." — Justin Johnson
> "Gausian splats are kind of consistent by construction, right? Because I've got this explicit 3D representation of the world. So if I look at it, then look away, then look back, like it's it's all there in 3D, so it's going to look the same." — Justin Johnson
> "We're going to have like these giant unified world models that have maybe different input heads, different output heads that know how to input and output different kinds of things." — Justin Johnson

## People mentioned
- Justin Johnson
- Sam Charrington
- Fei-Fei Li

## Topics
`world-models` `spatial-ai` `gaussian-splatting` `reinforcement-learning` `generative-models` `3d-representation`
