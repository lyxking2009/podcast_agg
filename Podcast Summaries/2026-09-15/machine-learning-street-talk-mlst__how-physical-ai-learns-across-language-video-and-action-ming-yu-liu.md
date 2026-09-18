---
podcast: "Machine Learning Street Talk (MLST)"
episode: "How Physical AI Learns Across Language, Video and Action — Ming-Yu Liu"
published: 2026-09-15
duration: 25m57s
audio_url: "https://traffic.megaphone.fm/APO4863104315.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/machinelearningstreettalk/episodes/How-Physical-AI-Learns-Across-Language--Video-and-Action--Ming-Yu-Liu-e3ospub"
transcript_source: youtube_autocaptions
generated_at: 2026-09-15T22:17:48Z
model: "deepseek-v4-flash"
guid: "5207ae6b-de37-4063-b419-058ae6782077"
---

# How Physical AI Learns Across Language, Video and Action — Ming-Yu Liu — Machine Learning Street Talk (MLST)

## TL;DR
Ming-Yu Liu, who leads research on NVIDIA's Cosmos Lab, explains how Cosmos 3 collapses three separate robotics modelling problems — forward dynamics, inverse dynamics and policy — into a single omni-modal world model that takes text, video, audio and action as first-class inputs. The conversation walks through the architecture (language model + vision encoder → vision-language model → bidirectional diffusion generator with an autoregressive ‘reason tower’), why closing the simulation loop lets you replace a 100-car test fleet with compute, and why manipulation is far harder than navigation because interaction brings occlusion and deformation. Liu resists a crisp definition of ‘world model’, comparing the debate to the definitional fog around AGI, and frames a world model as a collection of useful tools aimed at a goal.

## Key points
- Cosmos 3 is presented as a single model handling three jobs at once: describing video (VLM behaviour), generating video (simulation), and acting (robot policy) — action is treated as a ‘first class citizen’, meaning a model's next observation can be caused by the action it just took.
- Architecture path: start from a language model (trained from scratch or an open model), attach a vision encoder to build a vision-language model, then transfer weights to initialise a generator. The VLM half is autoregressive; the generator half is bidirectional diffusion, producing coherent chunks of video, action and audio.
- A ‘reason tower’ (the retained VLM component) lets the generator follow instructions — it understands what you want generated. Training is two-stage: pre-training, then mid-training where actioning is added.
- Multi-rate modalities are a core engineering problem: video frame rates, audio hertz and action frequencies all differ. NVIDIA uses a temporal position embedding scheme that normalises every signal onto a common scale, so tokens know which other tokens share the same time instance and their relative temporal distance.
- Liu argues forward dynamics (given a start state and action, predict the future), inverse dynamics (given a visual transition, infer the action taken) and policy (what should the robot do) reinforce each other when trained jointly under an information bottleneck — limited capacity forces a shared representation.
- Closing the loop in simulation changes the economics of testing: instead of employing 100 drivers and 100 cars across real intersections, you can launch simulated variants on compute. ‘You are not limited by the size of your fleets.’
- Navigation is comparatively easy; manipulation is hard because interaction introduces occlusion and deformation of the objects being handled. Liu expects neural simulation to eventually handle complex manipulation, at which point robot policies can be tested against the world simulator.
- Liu declines to give a single definition of a world model, noting that AGI has the same problem — no common agreement, everyone has their own definition. He frames a world model as a collection of useful tools built to achieve a goal, such as predicting the future or explaining why something happened.
- This episode is a paid partnership with NVIDIA — a disclosure worth noting when weighing the enthusiasm for Cosmos 3.

## Notable quotes
> "You are not limited by the size of your fleets. Anytime you need to do a lot of testing, if you have the computer, just launch it." — Ming-Yu Liu
> "We now have action as a first class citizen, which means the model might actually take an action and then its next observation could be caused by the thing that it did before." — Ming-Yu Liu
> "A world model is going to be the same as AGI — people have their own definition. I think a world model is a collection of useful tools; we model something because we are trying to achieve some goal." — Ming-Yu Liu
> "Navigation, you don't want your physical device to touch any other things. But manipulation tasks require all kind of interactions. When there's an interaction there's occlusion, there's potential deformation when things are manipulated. So it's more challenging." — Ming-Yu Liu
> "Different signals have different frequency — even for video we have a different frame rate, and audio has different hertz, and action also different. So we have a temporal position embedding scheme where we normalize all the signals into the same scale." — Ming-Yu Liu

## People mentioned
- Ming-Yu Liu — Vice President, Cosmos Lab, NVIDIA; leads world-model research
- Tim Scarfe / Keith Duggar — MLST hosts (show context)

## Topics
world models, physical AI, Cosmos 3, NVIDIA, robotics, forward dynamics, inverse dynamics, policy learning, diffusion models, vision-language models, simulation, manipulation
