---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "[One Million AI Engineer Special] How Stanford Teaches AI-Native Engineering — Mihail Eric, Monaco"
published: 2026-11-08
duration: "2h0m44s"
audio_url: "https://podcasts.beehiiv.com/3967c1f2-cb97-4eb0-a5c0-21a1d2579408/01a15fdf-0e93-7e1a-880d-d73752a1f915/01a1bcf7-771f-7c67-ba1d-94f856e787b0/01a1bcf7-78db-7028-b115-7c753e3b3b76.mp3"
episode_url: "https://lspace.beehiiv.com/podcast/s/latent_space_the_ai_engineer_podcast/one_million_ai_engineer_special_how_stanford_teaches_ai-native_engineering_mihail_eric_monaco"
transcript_source: rss_vtt
generated_at: 2026-11-09T23:23:17Z
model: "deepseek-v4-pro"
guid: "01a1bcf7-771f-7da1-affb-a7ead0a1434c"
---

# [One Million AI Engineer Special] How Stanford Teaches AI-Native Engineering — Mihail Eric, Monaco — Latent Space: The AI Engineer Podcast

## TL;DR
Mihail Eric, Stanford adjunct and Head of AI at Monaco, walks through the second iteration of his CS 146 / Modern Software Developer course, which now teaches students to build a Claude Code clone in ~200 lines, then covers context engineering, skills, agent-ready codebases, code review, security, background agents, and software factories. He describes Monaco's in-house agentic sales platform and developer tools like Monocoder and Hal, plus their use of Devin for code review and Parallel/Exa for enrichment. The episode also offers career advice for AI-native engineers, emphasizing open source contributions, benchmark building, and moving beyond single-agent coding toward software factories.

## Key points
- Mihail Eric is teaching CS 146 / Modern Software Developer at Stanford for the second time, first taught in fall 2025, after spending 9-10 months developing the curriculum and getting it approved by the Stanford committee; the idea began in November 2024 when coding agent knowledge was mostly scattered blog posts.
- This year's course shifted from a broad survey of AI tools across the SDLC to solidifying principles like careful context engineering and design/architecture for complex tasks; it is not for beginner programmers and assumes prerequisites like algorithms, aiming to produce practitioner-level AI-native engineers.
- Week 1 covers internals of coding agent harnesses: students build a working Claude Code clone in about 200 lines, then use MITMProxy to intercept actual Claude Code requests, examining system prompt, plan prompt, and compaction prompt; he showed Opus 55 prompts and discussed how the system prompt shrank as capabilities moved into model weights.
- Week 2 focuses on context engineering beyond vanilla prompting; introduces the Repit framework (research, proposal, implementation, testing) similar to Dex's RPI, and emphasizes multi-agent context management, fan-out for subtasks, compaction, and avoiding over-constraining as models improve.
- Week 3 covers skills and CLI/web skills, extending from MCP single-tool calls to procedures encoded as Skill MD files; highlights Matt Peacock's Grill Me and Jeffrey Blitz's Explain Diff, which quizzes users on diffs to force understanding rather than turning their brain off.
- Week 4 covers customizing agents via agent config files, AgentsDB, Claude MD, hooks, and sub-agents; Swyx shares skill-writing advice, including a skill cutter to trim verbose skill outputs and the observation that even slop in skills is in agent language, while Eugene Yan prunes unused skills regularly.
- Week 5 on agent-ready codebases emphasizes verifiability, docs, and surface boundaries needed for agents to ingest repos effectively; Factory readiness reports are highlighted as a key tool, and agent readiness is framed as a precursor to multi-agent systems and background agents.
- Week 6 on agentic code review identifies code review as a new bottleneck because agents can generate 30+ PRs per day; the course discusses auto-approving low-risk changes based on design and verifiability. Monaco converged on Devin for code review after evaluating vendors like CodeRabbit, Graphite, and Ellipsis; Cloudflare, Amplitude, and Augment have notable posts on the topic.
- Week 7 on security covers new attack vectors from AI-generated code, supply chain/dependency management, pinning dependencies, and not arbitrarily running NPM install; high-risk areas are data storage, billing, auth, and IAM, and while building everything in-house is theoretically safest, the episode acknowledges the nuance of open source and model-introduced bugs.
- Week 8 on background agents focuses on trigger/event-based agents for CI/CD, SRE bots, and examples like Ramp Inspect, Spotify, and Stripe Minions; at Monaco, they built Monocoder on Coder platform for isolated agent scaling and Hal, a Slack bot serving AISRE, customer success, and QA roles with different personas.
- Week 9 on AI-native team infrastructure discusses LLM gateways and MCP portals for governance, security, auditing, and cost optimization; Swyx argues gateways and MCP portals are the same layer problem and may converge, and the platform engineer role is increasingly central, owning cloud, DevEx, and SecOps.
- Week 10 on software factories frames the factory as a machine that builds the machine, with humans remaining at intent and review choke points; Warp runs its open-source terminal through a factory as an OSS partner, OpenAI built an internal tool with zero manually written code, and Steve Yegge's levels of psychosis suggest progressing from single-agent to multi-agent and orchestrator without producing slop.
- At Monaco, Mihail leads AI for a Series B all-in-one sales platform targeting early-stage startups; they built their own agentic harness for TAM/ICP scoring, pipeline evolution, and meeting notes, use Parallel and Exa for enrichment, and maintain a world database of companies/contacts; demand signals include job transitions detected via LinkedIn changes.
- The course OSS partners include Factory, Cognition, OpenAI, Warp, and others, providing credits and mentoring while students contribute to complex agentic repos; final projects are seeded with YC requests for startups, and prior examples include an EE student translating memory-address manuals and a design student whose photo-to-app demo led to a Vercel job on Eve.
- Advice for students includes building benchmarks/evals and writing rigorous engineering blog posts, citing SweetBench as an undergrad project that opened doors; ambition should go beyond trivial demos, and graduates should target roles that blend product and engineering, such as manager of agents or platform engineer.

## Notable quotes
> "most students are cooked, some of them are gonna be super valuable." — Swyx
> "You buy to learn and you build to keep." — Mihail Eric
> "There's no situation in which turning your brain off is a good idea." — Swyx
> "But for most people that are using code agents today, we're all experiencing something of a new bottleneck in code review." — Mihail Eric
> "There is still a core bit of like foundational software engineering that is important to have some of these core values of, of sort of how software systems are built and the trade-offs of those systems." — Mihail Eric

## People mentioned
- Mihail Eric
- Swyx
- Chip Hieatt
- Boris
- Silas
- Walden
- Eno
- Matt Peacock
- Jeffrey Blitz
- Dex
- Steve Yegge
- John Tracy
- Yunja
- Rajesh
- Amjad Masad
- Elad Gil
- Wilson
- Sam Blan

## Topics
`ai-native-engineering` `coding-agents` `stanford` `software-factory` `code-review` `security` `background-agents`
