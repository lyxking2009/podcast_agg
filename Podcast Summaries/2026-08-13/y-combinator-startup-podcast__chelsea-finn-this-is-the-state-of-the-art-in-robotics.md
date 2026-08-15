---
podcast: "Y Combinator Startup Podcast"
episode: "Chelsea Finn: This is the State of the Art in Robotics"
published: 2026-08-13
duration: 58m17s
audio_url: "https://anchor.fm/s/8c1524bc/podcast/play/124178146/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-7-13%2F429782123-44100-2-cab930f0a7ef2.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/ycombinator/episodes/Chelsea-Finn-This-is-the-State-of-the-Art-in-Robotics-e3nc492"
transcript_source: web
generated_at: 2026-08-13T23:52:18Z
model: "deepseek-v4-flash"
guid: "d443b078-a6df-481d-bc26-89dafcd0aa33"
---

# Chelsea Finn: This is the State of the Art in Robotics — Y Combinator Startup Podcast

## TL;DR
At Startup School 2026, Physical Intelligence cofounder Chelsea Finn explains what it takes to make robots useful in the real world: not just impressive demos, but reliable, long-horizon autonomy. She walks through a scalable reinforcement-learning recipe — human intervention to prune dead-end trajectories, a general-purpose value function trained on robot experience, and a 2x throughput gain from RL post-training with 90%+ success on an espresso task and 13-hour autonomous runs. The second half covers the shift to a single general-purpose foundation model (pi0.7) that matches or beats fine-tuned specialists out of the box, compositional generalization (interacting with an air fryer barely present in training data; folding shirts on a robot platform never seen in training), memory at multiple timescales, and why she believes robotics is entering its GPT era — moving from specialized models toward general-purpose systems that work across tasks, robots and environments.

## Key points
- Physical Intelligence's goal: let any robot do any task in the real world; robots now wash greasy pans, peel carrots, make grilled cheese, slice zucchini — but the hard problem is reliability over long periods without a human babysitting.
- Real-world AI systems need far fewer mistakes than recommendation-style ML because the robot itself acts on the world; Waymo's 250K+ weekly autonomous rides show trustworthy autonomous ML is possible.
- The espresso benchmark: operating a portafilter precisely, carrying full cups without spilling, timing — with over 90% success required; the team achieved 90%+ success and ran the policy for 13 hours straight.
- Scalable RL recipe: (1) humans intervene early to show recovery from dead-end trajectories instead of wasting robot time; (2) a general-purpose value function trained on robot experience estimates time-to-success and good/bad progress, amortizing the cost across prompts — dramatically reducing attempts needed.
- Results: ~2x throughput from the RL stage alone (pre-train to SFT to RL post-training); the same algorithm works across workflows — Dandelion Chocolate's box construction/labeling/stacking, folding clothes in never-seen homes, latte making.
- Memory: most robot foundation models have no context; naive video context costs ~500K tokens for 10 seconds (4 cameras, 256 tokens/image at 50Hz). Their solution: short-term video memory (~10s) computed efficiently plus compressed text summaries for minutes-to-hours memory — enabling a 10-15 minute autonomous kitchen-cleaning task (wipe counter, dry, throw away towel, put away mustard, dishes, sink).
- pi0.7: a single model trained on diverse data (robot demos including low-quality, policy rollout data, human videos, web data) with detailed prompting (instructions, sub-task instructions, data-quality metadata, optional sub-goal images). Out of the box it matches or exceeds fine-tuned pi0.6 specialists on espresso and box-building, and holds against SFT specialists.
- Compositional generalization: opened an air fryer and cooked a sweet potato despite the dataset having only ~3 incidental air-fryer episodes; folded shirts on the Byarm UR5E industrial robot with zero folding data for that platform — the model transferred skills across embodiments via learned left/right-hand equivariance.
- Ablations: removing the most diverse data collapses held-out performance; metadata prompting lets the model extract value even from low-quality data (performance rises with more low-quality data when prompted, falls without).
- Deployments: YC companies Ultra (laundry folding) and Weave (warehouse packaging) post-train pi models in production; the approach adapts across embodiments — bimanual arms, drones, quadcopters, surgical robots, tractors.
- Q&A highlights: a ChatGPT moment for robotics is unlikely in distribution terms (physical hardware rollout is slower) but "getting to the capabilities of ChatGPT is very much on the horizon in the next few years"; small teams should start with open-source generalist policies (pi0/pi0.5) and fine-tune; robot experience data — not just human video — is the robotics analogue of internet-scale text.

## Notable quotes
> "I think that getting to the capabilities of ChatGPT is very much on the horizon in the next few years." — Chelsea Finn
> "There's no substitute for the robot experience itself. If I watch Roger Federer play tennis, it doesn't mean I can play tennis as well as him." — Chelsea Finn
> "The robot essentially had learned this sort of equivariance between its left hand and its right hand so that it could transfer behaviors from one hand to another, despite the fact that that was never in the data." — Chelsea Finn, on an emergent capability
> "I love seeing robots do anything. I think there's still a long way to push in terms of reliability for robots being able to do tasks for really long periods of time." — Chelsea Finn

## People mentioned
- Chelsea Finn — cofounder, Physical Intelligence (guest; also Stanford professor)
- Y Combinator / Startup School 2026 — venue

## Topics
- Physical Intelligence, pi0.7, robot foundation models, reinforcement learning, long-horizon autonomy, memory for robots, compositional generalization, robotics GPT era, espresso robot, laundry folding, teleoperation, value functions, embodied AI, YC Startup School
