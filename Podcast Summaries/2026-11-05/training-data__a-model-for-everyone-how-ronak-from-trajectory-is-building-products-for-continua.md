---
podcast: "Training Data"
episode: "A Model For Everyone: How Ronak From Trajectory Is Building Products For Continual Learning"
published: 2026-11-05
duration: "32m0s"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/CPUAI8366427179.mp3"
episode_url: ""
transcript_source: youtube_autocaptions
generated_at: 2026-11-09T23:20:11Z
model: "deepseek-v4-pro"
guid: "eed8f8c6-d88e-11f1-8b73-637e085f73d5"
---

# A Model For Everyone: How Ronak From Trajectory Is Building Products For Continual Learning — Training Data

## TL;DR
Ronak, a former Windsurf/DeepMind researcher, co-founded Trajectory to bring continual learning to every company. Trajectory's platform distills real-world usage data into training signals, and with Nvidia and Harvey, it trained a legal model (Nemotron 3 Super) that matches frontier performance at lower cost. The team also open-sourced continuous LoRA and self-distillation policy optimization to enable always-improving AI agents.

## Key points
- Ronak previously worked at Codeium/Windsurf, where he helped build coding agents and an in-house model (referred to as SWE-1) that used user signals to surpass frontier models; Windsurf was acquired by Google DeepMind for $2 billion, with Cognition buying the remainder the following Monday.
- The Windsurf acquisition was surprising: the team expected OpenAI to acquire them, but instead met with Demis Hassabis and Sergey Brin via call and became Google employees the next day; Ronak gave up his acquisition payout to start Trajectory.
- Trajectory's thesis is that coding was just the first domain; legal, healthcare, and finance are next, and static models that don't learn from corrections waste valuable user signals.
- The platform distills all product data—agent traces, user corrections, and edits—into a unified "trajectory" format, then generates evals, judges, and training environments to close the loop.
- In partnership with Nvidia and legal AI company Harvey, Trajectory trained Nemotron 3 Super on Harvey's expert legal workflows, improving metrics like issue spotting, citation, and completeness while being drastically cheaper and faster than frontier models.
- Trajectory's engagement speed improved from three months for the first customer to under one month for Harvey, and a new customer was onboarded within a week.
- The company has 11 employees and positions itself as both a product company and research lab, hiring from OpenAI, Meta, Super Intelligence, Amazon AGI, Stripe, and Figma.
- Co-founders Michael (ex-DeepMind robotics, Gemini 1.5 robotics release) and Arjun (ex-Apple Vision Pro core interaction models) met Ronak in freshman year at Stanford.
- Trajectory released research on self-distillation policy optimization (SDPO), which addresses RL's limitation of compressing entire trajectories into a single reward by using privileged information as a hint for the teacher model, enabling faster convergence and robust off-policy training.
- The team open-sourced continuous LoRA in collaboration with Sky RL (Berkeley) and Anyscale, allowing concurrent training runs with separate sampling and training pools; this cut wall-clock time roughly in half for two jobs and scaled to eight concurrent runs without performance degradation.
- Next product phases include giving customers observability and control over their models, then expanding from AI-native companies (Clay, Harvey, Rogo, Decagon, Mor) to tech incumbents like Airtable and Notion, and eventually Fortune 500s.
- Trajectory plans to eventually optimize not just the model but also the harness, skills, and memory layer together, as those combined components constitute the full continual learning solution.

## Notable quotes
> "It's very clear now that we're getting that explosion for every single other domain, right? like legal, healthcare, finance, they're all going to be supercharged in the way that coding was for the last few years." — Guest
> "The model that you used yesterday, it's going to be the same model making the same mistakes tomorrow. All of those corrections you gave it, the edits like in any product is all just being put to waste." — Guest
> "RL it's still taking all of this kind of useful information from the real world like I mentioned all the corrections and everything and putting it into just one number which is really broken." — Guest
> "We're able to cut the wall clock time in half. As this goes up to four jobs, eight jobs, we're able to run concurrency really well." — Guest

## People mentioned
- Ronak
- Varun
- Douglas
- Demis Hassabis
- Sergey Brin
- Sam Altman
- Michael
- Arjun

## Topics
`continual-learning` `ai-training` `reinforcement-learning` `legal-ai` `open-source` `startups`
