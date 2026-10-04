---
date: 2026-10-01
show: Practical AI
title: "Open models and the future of Physical AI with NVIDIA"
guid: b836fa1b-9246-43d5-8cbb-cf1446760a6d
transcript_source: rss_transistor_vtt
duration: 47:19
link: https://share.transistor.fm/s/217445d3
model: deepseek-v4-flash (Hermes manual fallback)
---
# Open models and the future of Physical AI with NVIDIA

## Overview

Daniel Whitenack and Chris Benson talk with Ming-Yu Liu, vice president of Cosmos Lab at NVIDIA, about open
models and physical AI. Liu makes the case that open models are the field's engine: APIs solve problems but do not
convey insight, and they cannot serve use cases that need local execution or deep customisation. He explains NVIDIA's
two motives for releasing open models - understanding what hardware to build next, and letting the ecosystem test,
combine and eventually extend models - and describes models as a new kind of library. The conversation then pivots to
physical AI, which Liu defines as AI deployed in physical devices that perturb the state of the world and complete
tasks with material results. He surveys the verticals - automotive, factory automation, robotics, agriculture,
construction - and explains why humanoid robots are the most exciting and most compute-hungry form.

## Key Points

- Open models matter for three distinct reasons: APIs solve problems but do not give the insight needed to build on them; some use cases require local execution without internet connectivity; and commercial users need to customise for their own vertical.
- NVIDIA builds open models for two strategic reasons - to understand what GPU architecture should be built next, and to enable the ecosystem to test and combine models before building their own.
- Liu's framing: “model is a new kind of libraries people can use” - models join computer, libraries and applications as a layer of the stack.
- Physical AI defined as AI deployed in a physical device that perturbs the state of the world and completes tasks with material results - spanning automotive, factory automation, robotics, agriculture and construction.
- Humanoid robots are the most exciting form of physical AI and a direct business opportunity: they will require powerful onboard computers to process visual input, understand instructions and self-correct while completing tasks.
- Policy models are the core primitive - observation plus instruction in, action out - and sensor heterogeneity is the central engineering challenge: one robot has two eyes, another mounts cameras on the gripper; cars carry seven or eleven cameras; some have LiDAR.
- The episode traces the recent turn in the open-model landscape, including NVIDIA stepping into leadership as some Western open-model efforts faltered, and the Hugging Face acquisition.

## Implications

Liu gives the clearest business case for open weights from a hardware vendor's perspective: releasing models is partly
R&D into your own next chip, and partly ecosystem enablement. The physical-AI section is the more actionable part for
practitioners - it explains why policy models cannot be trained once and deployed everywhere, because sensor
configurations differ per robot and per vehicle. That heterogeneity is the structural reason physical AI will need
many specialised models rather than one frontier model, and it is the best argument for NVIDIA's open-model strategy.
Built from the episode's RSS transcript (Transistor VTT).

## Notable Quotes

- Ming-Yu Liu: “API are great. They solve problems, but they don't give you the insight.”
- Ming-Yu Liu: “Model is a new kind of libraries people can use and to build great amazing applications.”
- Ming-Yu Liu: “physical AI are AI deployed in physical device. And physical device perturb the state of the world and complete certain tasks.”
- Ming-Yu Liu (on open models for commercial customers): “Innovation, freedom. I think that is maybe the two words to summarize.”

## People Mentioned

- Ming-Yu Liu - Vice President, Cosmos Lab, NVIDIA
- Daniel Whitenack - co-host, Practical AI; CEO, Prediction Guard
- Chris Benson - co-host, Practical AI

## Topics

open models, physical AI, robotics, humanoid robots, NVIDIA, policy models, autonomous driving, model licensing, sensor fusion
