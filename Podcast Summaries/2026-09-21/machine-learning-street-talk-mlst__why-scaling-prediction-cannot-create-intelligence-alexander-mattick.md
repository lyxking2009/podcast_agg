---
podcast: "Machine Learning Street Talk (MLST)"
episode: "Why Scaling Prediction Cannot Create Intelligence - Alexander Mattick"
published: 2026-09-21
duration: "2h14m20s"
audio_url: "https://traffic.megaphone.fm/APO3817128671.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/machinelearningstreettalk/episodes/Why-Scaling-Prediction-Cannot-Create-Intelligence---Alexander-Mattick-e3p69rq"
transcript_source: youtube_autocaptions
generated_at: 2026-09-21T22:08:59Z
model: "deepseek-v4-pro"
guid: "5aad98f5-7294-4fb4-bd28-f091ec21d3bd"
---

# Why Scaling Prediction Cannot Create Intelligence - Alexander Mattick — Machine Learning Street Talk (MLST)

## TL;DR

Alexander Mattick provides a technical deep dive into inference methods, tracing the evolution from variational inference and energy-based models to normalizing flows, diffusion, and flow matching. He argues that energy-based models are too general and not economical for generation, while flow matching improves training and sampling. Mattick also critiques Rich Sutton's 'reward is enough' thesis, advocates for constrained reinforcement learning to encode prior knowledge and safety, and warns that current 'world models' are mostly branding without the reliability needed for real-world deployment.

## Key points

- Historical inference methods have fundamental limits: variational inference with normalized densities fails on multimodal data because it splits mass between modes, while rejection sampling and MCMC are slow in high dimensions due to accept/reject steps and difficulty escaping modes.
- Energy-based models parameterize unnormalized energy functions where lower energy means higher probability; training avoids normalizing constants, but sampling requires expensive MCMC, and they only provide ratios, not absolute likelihoods, making them impractical for generative modeling.
- GFlow Nets, proposed by Yoshua Bengio, amortize sampling over graph-structured distributions by balancing inflow and outflow at each node, but they assume a specific structural decomposition and are not a general-purpose inference solution.
- Normalizing flows start from a base density and apply invertible transformations while tracking Jacobians; continuous normalizing flows replace discrete transforms with a vector field and gradient flow, making invertibility automatic; flow matching fixes a reference path such as a straight line and turns training into supervised regression without solving full ODEs.
- Diffusion models learn to reverse a noising process over an infinite number of time steps; they are a specific flow matching variant that minimizes KL divergence to a normal distribution, adding a synthetic time dimension during training but enabling cheap sampling via ODE solvers.
- For language, autoregressive token prediction is natural because of discrete categorical structure; continuous domains like images, audio, and robotics benefit from diffusion or flow matching to decompose densities in latent space.
- Theories of deep learning split into parametric views, such as spline theory explaining batch norm, and functional views, such as neural tangent kernels and mean field theory, which explain overparameterization and generalize better across architectures at scale.
- The manifold hypothesis is neither clearly true nor a useful framing according to Mattick; interpretability methods like circuits analysis and PCA geometry rely on strong simplifying assumptions and are often not robust, so they cannot settle whether learned manifolds exist.
- Rich Sutton's 'reward is enough' is technically true because any objective can be written as a reward, but it is impractical: it assumes a suitable reward function can be found for every task and ignores the high cost of information and the need for constraints to avoid relearning everything from scratch.
- Constrained reinforcement learning better captures real-world requirements: OpenAI Five used reward penalties for time in the wrong lane and negative reward for killing enemy heroes to avoid gold correlation, which are implicit constraints; explicitly modeling constraints improves composability and communication with domain experts.
- Constrained RL formulations include CMDP expected cumulative cost below a threshold, CVaR, and percentile risk; algorithms like CPO, projection, Lagrangian, and local approximations each have issues such as oscillation or hallucinated constraints due to estimating constraints from data.
- Encoding hard constraints in neural network architectures works for action masking but state constraints are much harder; some constraints are NP-hard to enforce in a static architecture, so external structure or sampling may be required.
- World models are an overloaded branding term encompassing video diffusion, JEPA-style latent prediction, sensor prediction, and model-based RL; Mattick defines world models as learning dynamics outside the data collection loop, while model-based RL includes active data gathering.
- Yann LeCun's claim that world models make RL unnecessary is false: even a perfect world model of chess does not tell you the best move, because planning or inference over the model is still required to select optimal actions.
- World models enable cheap rollouts for RL, but real-world deployment needs minimal per-state failure probability, not just high average performance; current demos rarely show failure rates, and consequences in physical environments are much higher than in software.

## Notable quotes

> "Effectively, information can be very expensive and in fact I would say most information is very very expensive. If you completely say well constraints don't matter. Uh I I don't need any prior information. I don't want to encode anything in there. We still have the problem of now needing to actually relearn everything from scratch." — Alexander Mattick
> "I think that energy based models are not really useful or at least not useful anymore." — Alexander Mattick
> "I think that world models and so both Jeppa and world models are ultimately branding." — Alexander Mattick
> "I can give you a perfect world model of chess and you still cannot solve this like you will still not be like Magnus Carlson because there's a difference between being able to predict the future and being able to actually make inferences for what is the correct action to take." — Alexander Mattick
> "I think they are set up to look very nice in demos and to actually do some really impressive stuff. But how do you know like how many times did it fail to do something?" — Alexander Mattick

## People mentioned

- Alexander Mattick
- Yann LeCun
- Richard Sutton
- Chris Watkins
- Yoshua Bengio
- Randall Balestriero
- John Jumper
- Kenneth Stanley
- Yannic Kilcher

## Topics

`energy-based-models` `diffusion-models` `flow-matching` `reinforcement-learning` `constrained-reinforcement-learning` `world-models` `theory-of-deep-learning`
