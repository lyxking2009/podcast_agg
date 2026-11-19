---
podcast: "Practical AI"
episode: "Implementing Federated Learning to enhance Agentic AI"
published: 2026-11-13
duration: "43m33s"
audio_url: "https://pscrb.fm/rss/p/dts.podtrac.com/redirect.mp3/media.transistor.fm/58a69653/1bd19fd9.mp3"
episode_url: "https://share.transistor.fm/s/58a69653"
transcript_source: rss
generated_at: 2026-11-13T23:09:12Z
model: "deepseek-v4-pro"
guid: "831710d1-ed18-46f7-b479-539ba9a751d3"
---

# Implementing Federated Learning to enhance Agentic AI — Practical AI


## TL;DR
Lorenzo Sani, a research scientist at Flower Labs and PhD candidate at Cambridge, explains how federated learning keeps private data on devices while training models, and how it applies to agentic AI systems that produce complex trajectories. He discusses privacy guarantees, open vs closed models, and practical uses from enterprise workflows to personal home automation, emphasizing that controlling trajectories is essential for improving agent performance.

## Key points
- Federated learning moves the training computation to where the data is collected (e.g., phones, laptops, edge devices), so raw private data never leaves its source, helping satisfy privacy regulations and enabling training on otherwise inaccessible data.
- Privacy protection in federated learning has two components: the guarantee that data stays on-device, and the risk of the model memorizing private information; techniques like differential privacy provide statistical guarantees but are case-specific and sometimes difficult to apply.
- Agentic AI systems generate trajectories that contain user prompts, intermediate tool calls, retrieved external data, and environment interactions; these trajectories often include highly personal information and are valuable for improving agent performance, making federated learning a natural fit for distributed training on them.
- Trajectories can become tree-like: starting from a prompt, an agent may branch into sub-agents or retrieve data, then consolidate back to a final answer; each step accumulates more private context about the user, increasing privacy risk over the course of the trajectory.
- For privacy-heavy applications, open-source models are preferable because they can be inspected, probed, and fixed if privacy leaks occur, whereas closed-source models are harder to verify and control.
- Lorenzo's personal projects illustrate practical agentic AI: a Halloween costume-judging app using locally hosted vision-language agents that scored costumes and resisted prompt injection attempts, plus plans for house management reminders, recording evening research discussions, and a daily research paper audio briefing.
- Enterprises increasingly need to collect and analyze agent trajectories to identify failure modes and improve models, rather than relying solely on external model providers that do not disclose full reasoning traces or intermediate steps.
- In sensitive applications like robotic surgery, collaborative or federated learning can pool diverse experiences from many hospitals while keeping patient data private, accelerating improvements for models that must handle high variability (e.g., different anatomies, veins, and surgical conditions).
- Federated learning for LLM pre-training was initially challenging but is now well-developed with many papers showing deeper technical results; early work from DeepMind and the research group at Cambridge helped establish the field.
- Lorenzo envisions a future where models are small enough for everyone to run on their own devices, enabling personal automation and problem-solving driven by constraints and annoyances, as a massive enabler for individuals.

## Notable quotes
> "Federated Learning tries to move the training pipeline, so all the computations that you need to train your machine learning model to where the data is collected." — Lorenzo Sani
> "The first ingredient for protecting your privacy is the fact that your data never moves from where it's collected." — Lorenzo Sani
> "But with open source models, it's much easier because the amount of techniques that you can use is just many, many more. I would say that if I had to develop a privacy heavy application, I would prefer to use an open source model as a starting point." — Lorenzo Sani
> "My strong hope is that the work I am doing right now and the work I did in the past few years is gonna serve as a massive enabler for people. Like, as models are reduced in size, as time goes, as compute gets better, I hope that everybody will be able to run the same pipelines I've developed and I'm developing on their own laptop, on their own infrastructure, in their own houses, and to enable them to do whatever they want." — Lorenzo Sani

## People mentioned
- Lorenzo Sani
- Chris Benson
- Daniel Whitenack
- Nick Lane

## Topics
`federated-learning` `agentic-ai` `privacy` `open-source-models` `edge-computing` `trajectory-analysis`
