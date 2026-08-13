---
podcast: "Y Combinator Startup Podcast"
episode: "Chelsea Finn: This is the State of the Art in Robotics"
published: 2026-08-13
duration: 58m17s
audio_url: "https://anchor.fm/s/8c1524bc/podcast/play/124178146/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-7-13%2F429782123-44100-2-cab930f0a7ef2.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/ycombinator/episodes/Chelsea-Finn-This-is-the-State-of-the-Art-in-Robotics-e3nc492"
transcript_source: youtube_autocaptions
generated_at: 2026-08-13T23:48:59Z
model: "deepseek-v4-pro"
guid: "d443b078-a6df-481d-bc26-89dafcd0aa33"
---

# Chelsea Finn: This is the State of the Art in Robotics — Y Combinator Startup Podcast

## TL;DR
Chelsea Finn, founder of Physical Intelligence, presents the current state of robotics, focusing on achieving long-term autonomy and general-purpose models. She details a reinforcement learning recipe with human interventions and multi-timescale memory that enabled >90% success on espresso making and 2x throughput on box building. Finn also introduces the PIO7 general-purpose model, which matches or outperforms fine-tuned specialists out of the box and exhibits compositional generalization across tasks, objects, and robot platforms.

## Key points
- Physical Intelligence, founded two years ago by Chelsea Finn, aims to develop general-purpose robots that can perform any task in the real world, moving beyond demos to practical deployment.
- Unlike AI systems for recommendations or chat, where humans can ignore mistakes, physical AI must operate autonomously with very high reliability because it directly affects the physical world; Waymo's 250,000 weekly autonomous rides demonstrate this is achievable.
- To achieve high reliability, Finn proposes an RL recipe that includes human interventions to recover from dead-end trajectories and a learned general value function to amortize credit assignment across prompts, reducing required attempts.
- The approach was demonstrated on espresso making: the robot inserted the portafilter, waited for extraction, and transferred a full latte without spilling, running for 13 hours straight with >90% success rate.
- For a Dandelion Chocolate box-building workflow, the same RL algorithm yielded a 2x throughput improvement over pre-training and SFT stages, showing greater reliability and speed.
- Laundry folding RL policy worked in unseen homes on unseen clothing items, acting autonomously for extended periods.
- Finn emphasizes that current robot foundation models lack memory, operating only on current observations; this limits them to short, repetitive tasks.
- They developed a multi-timescale memory system: efficient short-term video memory (~10 seconds) and long-term textual summaries for 10-15 minute autonomy, enabling a kitchen cleaning task with multiple sequential steps (wiping, putting away mustard, dishes, washing).
- The general-purpose model PIO7 was trained on diverse data including robot demonstrations (even low-quality), policy rollout data, human videos, and web data, with detailed prompting of subtask instructions, metadata, and optional subgoal images.
- Ablations show that removing the most diverse data dramatically decreased performance on held-out tasks, while random 20% removal had little effect, proving diversity is crucial.
- Metadata prompting was essential: without it, adding low-quality data (80% to 100%) decreased performance, but with it, performance increased, suggesting it extracts value from low-quality data.
- PIO7, out of the box (no fine-tuning), matched or exceeded the throughput and success rate of fine-tuned RL specialists on espresso and box-building tasks, moving robotics from a BERT-like fine-tuning regime to a GPT-like generalist regime.
- Compositional generalization was observed: the model could operate an air fryer (rare in training data) to cook a sweet potato, and fold clothes on a completely different robot platform (barm robot) despite no folding data for that platform.
- On the barm robot folding task, performance of PIO7 approached human teleoperation levels, demonstrating cross-embodiment transfer.
- The model exhibited emergent left-right hand equivariance: after a mistake left the pin and paper swapped, the robot autonomously switched hands and completed the assembly, a capability never present in training data.
- Physical Intelligence models are being deployed by YC companies Ultra (laundry folding) and Weave (warehouse packaging), and can be adapted to drones, surgical robots, and tractors.
- Finn believes robotics is now in a GPT/DALL-E era, with a single general-purpose model that works out of the box and shows compositional generalization.
- On the ChatGPT moment for robotics: distribution will be slower due to physical hardware, but capabilities are on the horizon in next few years.
- For small teams, Finn recommends starting immediately with a generalist policy (e.g., open-source PI-zero, PI-05) and fine-tuning, except in highly constrained environments like surgical rooms with no internet/GPU.
- On robotics data: the equivalent of internet-scale data is robot experience (teleoperated and autonomous), supplemented by human videos and web data; but robot experience is irreplaceable, analogous to needing to play tennis yourself, not just watch Federer.
- On model output level: models output target joint positions and 3D gripper positions, with a PD controller handling low-level torque; direct torque control could allow variable stiffness but current level is not a bottleneck.
- On imagination: PIO7 can generate subgoal images, which improved performance modestly, but the model performed well even without it; predicting future frames seems useful but not yet critical.
- On speed: teleoperation data is slow, but RL and the RL token release have shown policies faster than human teleop; future work targets faster data or policies.
- On breaking into robotics from software engineering: join as software engineer, or buy a cheap robot, fine-tune open-source models, share results; example of Jenny who did this and got hired at Physical Intelligence.

## Notable quotes
> "I think that actually physical AI and robotics is pretty different from this where if we think about physical AI that are actually operating in the physical world, they have to be directly making decisions that affect the physical world." — Chelsea Finn
> "And with this sort of kind of memory at multiple different time scales, we're able to enable robots to do tasks that can operate for 10 or 15 minutes at a time completely autonomously." — Chelsea Finn
> "The robot essentially had learned this sort of equivariance between his left hand and his right hand so that it could actually transfer uh behaviors from one hand to another. uh despite the fact that that was never in the data." — Chelsea Finn

## People mentioned
- Chelsea Finn
- Roger Federer

## Topics
`robotics` `general-purpose-models` `reinforcement-learning` `physical-intelligence` `autonomy` `compositional-generalization`
