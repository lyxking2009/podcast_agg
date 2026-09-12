---
podcast: "Y Combinator Startup Podcast"
episode: "Open Models Change The Economics of AI"
published: 2026-09-12
duration: "57m15s"
audio_url: "https://anchor.fm/s/8c1524bc/podcast/play/125638035/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-8-12%2F431757948-44100-2-a60c62d3496b1.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/ycombinator/episodes/Open-Models-Change-The-Economics-of-AI-e3ooluj"
transcript_source: web
generated_at: 2026-09-12T22:11:22Z
model: "deepseek-v4-pro"
guid: "8a800373-10ea-4204-8d82-b7fad9603c9b"
---

# Open Models Change The Economics of AI — Y Combinator Startup Podcast

## TL;DR

Jeffrey Morgan, CEO of Ollama, discusses the rapid enterprise adoption of open AI models driven by cost savings and control. Ollama has 9M developers and 178k GitHub stars, used by 85% of Fortune 500; token usage on its cloud is dominated by Chinese-origin models. AT&T has moved 40% of tokens to open models. Morgan predicts 80-90% of tokens will be open but only 10-20% of budget, with hybrid local/cloud execution and accelerating open model releases.

## Key points

- Open models are being adopted by enterprises primarily to lower cost; however, the north star is control and customization. AT&T has already shifted 40% of its token consumption to open models as reported by The Information, using US and European models while evaluating Chinese models. Ollama's cloud sees token usage predominantly from Chinese-origin models accessed by businesses worldwide, especially US and Germany.
- Ollama has 9 million developers, 178,000 GitHub stars, and is used by 85% of the Fortune 500. Per-developer weekly token usage on Ollama cloud exploded in 2025: first from coding agents like Kimmy, GLM, and MiniMax, then in April from OpenClaw and the Hermes agent project, allowing non-developers in finance, support, marketing, and sales to automate tasks. Cloud token consumption grew 150x since the start of the year.
- Open model release cadence is accelerating; DeepSeek Flash had three iterations in one summer. The gap between open and closed frontier models has narrowed to under 3 months. For example, Qwen 3 8B now matches Opus 4.6 in coding benchmarks and can run on an entry-level MacBook.
- Security and safety are the main blockers for open model adoption in enterprises. The GLM53 model shows strong cybersecurity capabilities, creating opportunities for startups in security and governance. Open-weight models are used for penetration testing where closed models refuse; Hugging Face even used open-weight models to detect a hack from a frontier provider.
- Model developers contact Ollama pre-release to coordinate day-zero launches. Successful launches require integrating with inference engines, matching harnesses, ensuring hardware support, and benchmarking. Ollama packages model, harness, and hardware into a common runtime, acting like an operating system for open models.
- Between the model and application, layers like knowledge, coordination, and execution (sandboxing) are emerging as separate opportunities. Open source leads to best-of-breed products; Morgan predicts these layers will unbundle into standalone companies, similar to how cloud infrastructure evolved.
- Morgan predicts a supermajority (80-90%) of enterprise tokens will be served by open models, but open models will account for only 10-20% of AI spend due to lower per-token cost. Frontier closed models will be reserved for the hardest tasks, with routing between open and closed models.
- Local models excel at 20-40B parameters on Apple Silicon and Nvidia hardware; they are used for document processing and easier tasks, while cloud models handle coding agents. A hybrid execution model uses a router to decide when to call cloud models, reducing costs further.
- Nvidia DGX Spark offers 128GB unified memory and can be stacked to run 400B models. Nvidia GB300 workstations and Apple's MLX stack make local inference competitive. Morgan expects coding loops to return to local hardware as speeds improve.
- For cost-sensitive startups, use ultra-low-cost flash models like DeepSeek Flash for high-volume tasks; orchestrate multiple flash models to solve harder problems. This enables unlimited-token experiences and new applications without worrying about cost.
- Jeffrey Morgan and Michael Chiang, previously from Docker, pivoted multiple times before launching Ollama in July 2023 after Llama 2. They raised Series A from Benchmark pre-ChatGPT. YC (2021) provided community and reduced founder loneliness.

## Notable quotes

> "Cost is by far the largest pain point that open models can jump in and solve, but every business has a vision of getting better control over AI and customizing it for their business. That's really their north star." — Jeffrey Morgan
> "The super majority of tokens... will be open models within a business. Call it 80, 90%... maybe you'll only pay 10 to 20% of the cost towards open models, but... most of your tokens will be going through the open models." — Jeffrey Morgan

## People mentioned

- Jeffrey Morgan
- Michael Chiang
- Jensen Huang
- Gary Tan
- Jared Friedman
- Peter Fenton
- Elon Musk

## Topics

`open-source-ai` `enterprise-ai` `coding-agents` `ai-infrastructure` `ai-safety` `geopolitics-ai`
