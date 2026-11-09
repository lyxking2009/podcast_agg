---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Why AI Compute Comes in Rectangles — Erik Bernhardsson, Modal CEO"
published: 2026-10-22
duration: "1h25m32s"
audio_url: "https://podcasts.beehiiv.com/3967c1f2-cb97-4eb0-a5c0-21a1d2579408/01a15fdf-0e93-7e1a-880d-d73752a1f915/01a16579-8537-79c0-86b7-c9fb7a723041/01a16579-85f9-7405-b65b-7eeb25ff90b0.mp3"
episode_url: "https://www.latent.space/p/modal-d"
transcript_source: rss
generated_at: 2026-10-27T22:12:55Z
model: "deepseek-v4-pro"
guid: "substack:post:221647996"
---

# Why AI Compute Comes in Rectangles — Erik Bernhardsson, Modal CEO — Latent Space: The AI Engineer Podcast

## TL;DR
Erik Bernhardsson, CEO of Modal, discusses building AI infrastructure that supports inference, sandboxes, and batch jobs, with customers like Suno and Runway. He details Modal's journey from self-funded startup to significant revenue growth, driven by stable diffusion and later agentic workloads. Key topics include GPU capacity procurement across 25 clouds, the evolution of serverless functions and sandboxes (now 1/3 of revenue), the importance of developer experience and ephemeral compute, and strategic bets on training and forward-deployed engineering.

## Key points
- Modal provides AI infrastructure including a compute platform and storage platform, enabling customers like Decagon, Suno, Runway, and Co-Cognition to build AI applications, mainly for inference, sandboxes, training, and batch jobs.
- Erik Bernhardsson started Modal after leaving Better.com around 2020/2021, drawing on his experience building Spotify's recommendation system (Discover Weekly) and recognizing that AI infrastructure 'sucks.' He self-funded early development and only raised a seed in early 2022 after about a year.
- Modal's revenue grew from about $100/month in November 2022 to $2,000/month the next month, then $30,000/month by January 2023, largely due to stable diffusion customers like Mage, and later diversified with Suno.
- Modal grew roughly 50x in revenue and usage since appearing on the podcast two years prior.
- To secure GPU capacity, Modal originally relied on hyperscaler spot and on-demand instances but now works with almost 25 clouds, requiring three-year deals with upfront payments. They have a procurement team of four and an engineering team of about ten managing bare metal capacity.
- Erik believes hardware prices normally go down but current GPU prices are rising due to scarcity; he worries more about a future price collapse than about current high prices, as Modal has locked in reservations.
- Modal's primary products are Modal Functions (serverless functions) and Modal Sandboxes. Functions account for about two-thirds of revenue, Sandboxes about one-third.
- Modal Sandboxes launched in 2023, initially underperformed, but exploded with the rise of LLMs that generate code, enabling 'vibe coding' companies like Lovable and background agents at Ramp. Sandbox revenue has grown 30% month over month.
- Erik envisions sandboxes becoming far more ephemeral: today median sandbox in Modal runs about 8 seconds, and future direction includes fast snapshotting, forking, live migration, and spin-up in ~100ms to enable massive parallelism like running each CI test in its own sandbox.
- Modal uses forward-deployed engineers (FDEs), growing from 4 at start of year to 24, to help customers with performance tuning, cost optimization, and migration, with the goal of turning those engagements into product features.
- Modal is exploring training products, particularly for smaller and mid-sized training runs, post-training reinforcement learning, and computational biotech, where customers can train on 64 GPUs rather than hyperscale clusters.
- Regarding the Hugging Face incident where an app on Modal was exploited, Erik clarifies it was an app vulnerability, not a Modal hack, but they are now using AI agents internally to probe for vulnerabilities and may tighten default security restrictions.
- Erik remains a cloud maximalist and does not plan on-prem; BYOC (bring your own cloud) is possible but at a very high price, and multi-tenancy provides agility and cost benefits.
- Team at Modal grew 4x in last 12 months to about 200 employees, expected to reach 250 by year-end and possibly 700 next year, with most hires being systems engineers, sales, and research.
- Erik notes that GPU capacity is often sold in 'rectangles' (3-year commitments), but demand is bursty and variable; Modal's value is aggregating demand across thousands of customers to provide on-demand granular capacity.

## Notable quotes
> "The easiest way to put it is we build AI infrastructure." — Erik Bernhardsson
> "Every new sort of era of, like, you know, technology kind of runs up against limitations of, of tools." — Erik Bernhardsson
> "I think the median sandbox in Modal today is, like, eight seconds long or something like that." — Erik Bernhardsson
> "We're in the business of building blocks." — Erik Bernhardsson

## People mentioned
- Erik Bernhardsson
- Akshat Bubna
- Zach Frankel
- Vishal Garg

## Topics
`ai-infrastructure` `gpu-capacity` `serverless-compute` `agent-sandboxes` `neo-clouds` `developer-experience` `llm-inference`
