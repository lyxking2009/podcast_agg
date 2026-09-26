---
podcast: "Y Combinator Startup Podcast"
episode: "Robot-Use Agents: Why General-Purpose Models May Win in Robotics"
published: 2026-09-26
duration: "29m49s"
audio_url: "https://anchor.fm/s/8c1524bc/podcast/play/126339684/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-8-26%2F432708848-44100-2-ba9049a9c4a07.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/ycombinator/episodes/Robot-Use-Agents-Why-General-Purpose-Models-May-Win-in-Robotics-e3pe354"
transcript_source: web
generated_at: "2026-09-26T22:03:12Z"
model: "deepseek-v4-pro"
guid: "7973c2f5-3986-449b-8aea-5d1290abddc0"
---

# Robot-Use Agents: Why General-Purpose Models May Win in Robotics — Y Combinator Startup Podcast

## TL;DR
In this episode, YC hosts discuss the emergence of 'robot-use agents' with founders from Waddle Labs and Roboccurve. They trace the progression from RT2 and code-as-policies to modern LLMs like Astra controlling robots directly via tool calls, and argue that general-purpose models trained on diverse data (code, computer use, egocentric video) will surpass specialized robotics models. Guests highlight latency improvements (~2x/month) and predict general-purpose robots within two years, while emphasizing the need for consolidating LLM reasoning into fast, reusable skills.

## Key points
- Early research like RT2 (2022) fine-tuned pre-trained language models on web text and images to output end-effector poses, demonstrating that leveraging pre-trained knowledge improves robotics control over training models from scratch, but the fine-tuning approach required robot-specific data.
- Code-as-policies (2022) showed that coding agents could write Python functions to control robots one-shot without additional robot data, because they were pre-trained on large code corpora; this shifted the bottleneck from action data to code generation.
- Voyager's Minecraft agent pioneered on-the-fly tool creation, compressing experiences into reusable code tools, which inspired using LLMs to write 'code policies' for robots, effectively distilling task knowledge into compact programs.
- Modern frontier models like Astra can control robots directly via tool calls or code generation, bypassing fine-tuning; this is likened to the 'chain-of-thought moment' where models allocate more compute for complex tasks, but latency remains a bottleneck.
- Waddle Labs' harness approach packages learned skills into reusable programs, consolidating in-context learning into deterministic code graphs with variable points handled by vision-language models, enabling faster and more reliable execution than having the LLM in the loop every step.
- Frontier LLM latency is improving at ~2x per month; guests estimate this could enable real-time robot control by the end of the year, making LLM-driven robotics economically viable.
- Computer-use data (e.g., GUI interactions, CAD manipulation) teaches spatial reasoning that transfers to robotics; feeding diverse data types (code, computer use, egocentric video) into a single model may produce the most capable robot-use agents, as suggested by the Platonic representation hypothesis.
- In-context learning is sample-efficient but saturates quickly (around 20-40 examples) and degrades beyond the context window; scalable robot learning will require hierarchical compression, such as distilling experiences into skills or updating model weights during 'sleep' phases.
- There is consensus among frontier labs and robotics companies that general-purpose robots (performing tasks a competent teenager can do with hands) will arrive within two years, though society is unprepared for this shift.

## Notable quotes
> "One of the big surprises the last few years has been the generalizability of coding agents across different domains." — Host (~00:00:00)
> "We measure everything, any robot, any model." — Jay (~00:01:11)
> "If the trends continue, we could get real-time control by end of the year." — Guest (~00:17:50)
> "There's some consensus within the Frontier Labs and also in the Robotics Foundation models companies that we will have general-purpose robots within the next two years or even earlier." — Jay (~00:26:37)
> "I think what Astra does incredibly well is its vision capabilities." — Guest (~00:23:32)

## People mentioned
- Philip Isola
- Francois
- Hamei
- Vincent
- Jay

## Topics
`robot-use-agents` `large-language-models` `robotics` `computer-use-data` `real-time-control`
