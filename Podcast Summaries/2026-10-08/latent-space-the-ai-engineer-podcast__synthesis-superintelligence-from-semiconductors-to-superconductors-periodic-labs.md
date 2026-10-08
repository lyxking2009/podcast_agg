---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Synthesis Superintelligence: from Semiconductors to Superconductors — Periodic Labs’ Liam Fedus and Ekin Dogus Cubuk"
published: 2026-10-08
duration: "1h24m0s"
audio_url: "https://api.substack.com/feed/podcast/219146438/4317811e52984f32f5f67455cb91c1e9.mp3"
episode_url: "https://www.latent.space/p/periodic"
transcript_source: web
generated_at: "2026-10-08T22:07:41Z"
model: "deepseek-v4-pro"
guid: "substack:post:219146438"
---

# Synthesis Superintelligence: from Semiconductors to Superconductors — Periodic Labs’ Liam Fedus and Ekin Dogus Cubuk — Latent Space: The AI Engineer Podcast

## TL;DR
Periodic Labs, founded by Liam Fedus and Ekin Doğuş Çubuk, aims to create 'synthesis superintelligence' by combining AI, simulation, and high-throughput physical experiments for materials discovery. They argue that intelligence must be grounded in reality, so their lab integrates solid-state chemistry, physics, robotics, and machine learning to iteratively design, synthesize, and characterize materials. Their approach focuses on overcoming noisy experimental data, sample inefficiency, and the lack of perfect simulators, with the ultimate goal of accelerating discovery of materials like room-temperature superconductors.

## Key points
- Periodic's core thesis: intelligence alone is insufficient; new knowledge requires iterative experimentation in the physical world. They build AI systems, simulations, and high-throughput labs to close the loop between conjecture and reality.
- Unlike big AI labs, Periodic's reinforcement learning environments are derived directly from physical lab data, requiring handling of noise, uncertainty, and sample inefficiency. Data is precious because physical experiments are slow and not arbitrarily scalable like digital simulations.
- The lab integrates diverse expertise (solid-state chemists, physicists, experimentalists, theorists, hardware engineers, LLM experts) similar to Bell Labs, with senior researchers doing hands-on work.
- Key challenge: experimental data is noisy and incomplete. Examples include furnace degradation, non-uniform temperature, and vibrations affecting optical instruments. Periodic adds telemetry and uses AI to detect anomalies, such as a cyclic permutation error from a misloaded machine.
- The materials discovery loop has three stages: predicting stable materials and properties, synthesizing them, and characterizing the result. They create RL environments for phase identification from X-ray diffraction, rewarding correct phases and penalizing spurious ones.
- To train on the process of science, Periodic constructs RL environments by timestamping experimental history, allowing models to learn from decisions made over time. This avoids faking answers from pre-trained knowledge and builds reasoning strategies that generalize to novel systems.
- Known physics is insufficient for many materials problems: high-temperature superconductivity remains unexplained, and DFT has limitations (strong correlation, microstructure). Experiments are essential because simulations alone cannot capture all complexities.
- Periodic uses a mix of open-source and closed models, leveraging proprietary experimental data for compute efficiency. They contribute to open source (TorchSim, JAX-MD, etc.) and run an academic grant program.
- Scaling strategy includes building new labs, custom hardware to reduce noise and bottlenecks, and forward-deployed engineering for semiconductor partners. They aim to monetize by accelerating outcomes, analogous to how software copilots evolved into autonomous agents.
- Future vision: 'synthesis superintelligence' to unlock materials like room-temperature superconductors, better magnets, and batteries, possibly approaching the Landauer limit for energy-efficient computing.

## Notable quotes
> "You can’t just think your way to a solution. The universe is so complicated that in order to actually push the frontier of knowledge and to make progress, you need to create these conjectures and then actually see whether or not it holds." — Liam Fedus (~00:00:29)
> "We feel like simulations will never be enough by themselves, but in the loop of simulations, AI, and experiments, I think we can make progress much faster than before." — Ekin Doğuş Çubuk (~00:35:41)
> "No one’s going to zero shot the room-temperature superconductor." — Liam Fedus (~01:00:51)
> "It’s not that hard to mix powders to get to try stuff. But if you can’t characterize and analyze it and then decide what the next step should be intelligently, you don’t really benefit much from mixing powders randomly." — Ekin Doğuş Çubuk (~01:03:22)
> "We kind of feel like our best contribution to solid-state physics and science could be if we made these tools and topics profitable, similar to how ChatGPT made CS and LLM majors way more popular in colleges before and after." — Ekin Doğuş Çubuk (~01:18:03)

## People mentioned
- Liam Fedus
- Ekin Doğuş Çubuk
- Joe Checkelsky
- Daniel Chica
- Dima Bahdanau
- Ray Nakano
- Alex Müller
- Harold Hwang

## Topics
`materials-science` `ai-for-science` `reinforcement-learning` `high-throughput-experimentation` `superconductors` `synthesis-superintelligence` `lab-automation`
