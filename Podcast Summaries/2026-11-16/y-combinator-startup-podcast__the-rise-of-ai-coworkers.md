---
podcast: "Y Combinator Startup Podcast"
episode: "The Rise Of AI Coworkers"
published: 2026-11-16
duration: "1h0m5s"
audio_url: "https://anchor.fm/s/8c1524bc/podcast/play/129571480/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-10-16%2F436903346-44100-2-734c511b058aa.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/ycombinator/episodes/The-Rise-Of-AI-Coworkers-e3sgn6o"
transcript_source: youtube_autocaptions
generated_at: 2026-11-19T23:11:39Z
model: "deepseek-v4-pro"
guid: "ff59a44e-389f-42d8-bc7d-0f859c723669"
---

# The Rise Of AI Coworkers — Y Combinator Startup Podcast

## TL;DR
Lauren Tan and Roman Ugarte discuss Grokbot, a product for creating AI teammates with identities, tools, and always-on proactive behavior. They cover real-world use cases such as marketing bots managing ad budgets, support bots triaging tickets, and small businesses automating bookkeeping. Lauren shares her workflow of orchestrating bots in Slack and pushing 4,000+ PRs per month, while the conversation explores software factories, AI-native companies, and the shift from execution scarcity to goal specification as the new bottleneck.

## Key points
- Grokbot, created by Lauren Tan and Roman Ugarte, enables users to build AI teammates with their own identity, telephone number, credit card, and access to the same tools as humans; it was built internally in seven weeks after seeing AI coding transform engineering, then released publicly.
- The product organizes work around bots as the core primitive—not chats or files—and bots are designed to be always-on, proactive, and able to use tools like a human employee.
- Early internal reception formed a bell curve: both AI-forward and less AI-forward employees found Grokbot intuitive, prompting the team to launch it publicly.
- A marketing use case: the performance marketing team gave their bot access to Google Ads and Facebook Ads with budget and login credentials; it coordinates with a design bot to create and A/B test assets.
- Support and customer success teams use Grokbot to reproduce issues, triage hundreds of customer messages per minute, complete simple tickets end-to-end, and escalate complex ones to the right human DRI.
- Small businesses are among the most enthusiastic adopters because they struggle to hire; one business can set up a bot in hours to handle bookkeeping perfectly.
- Two weeks before this episode, the team launched team bots, enabling multiplayer sharing where a domain expert builds a bot, shares it with the company, and manages it like an AI employee with credentials and access.
- Lauren transports her engineering workflow into Slack: she adds bots like Potato to threads, spawns Cursor cloud agents with VMs to prototype, and receives videos/screenshots; a feature was built end-to-end in a 200-message Slack thread without opening Cursor.
- Lauren now pushes over 4,000 PRs per month (up from 2,000+), and her agents can merge their own PRs because she has encoded guardrails, skills, and static analysis into the codebase.
- Lauren created P-stack (Potato stack), an open-source set of agent skills inspired by Gary’s G-stack; it teaches agents rigorous engineering practices, and she encourages engineers to fork and create their own stacks.
- The smallest effective engineering loop is teaching the agent to run code empirically—taking CPU traces, performance metrics, and acting like a real user—so it can check its own work and iterate until correct.
- Lauren uses an inverted pyramid investment model: most engineering effort goes into codebase architecture, static analysis, lint rules, and compilers, reducing the need for expensive adversarial code review; for example, they built a dependency graph checker to enforce Electron renderer/main thread separation.
- Roman argues that as agents become multiplayer, they need social awareness and decision-making context—currently bots don’t know who to listen to in a Slack thread, like an employee with no EQ.
- Roman says companies will move from execution bandwidth constraints to execution abundance, making problem definition, KPI selection, and objective functions the new bottleneck; teams now spend more time on product vision and alignment than coding.
- Jack Dorsey’s AI-native company model is referenced: eliminate middle management, everyone becomes an IC, one DRI owns the outcome, and a player-coach teaches everyone AI tools; a customer success example uses Grokbot to triage hundreds of requests per minute.
- Lauren’s non-traditional path: she studied accounting and finance, learned to code via Excel VBA, started blogging, and was invited by Tom Dale and Leah Silber to speak at a conference, which led to her career as a top AI engineer.

## Notable quotes
> "Grokbot is meant to be this product where you can create AI teammates. And we mean that seriously. I mean you can kind of give these teammates names and identities and uh functions and skills and tools and all of these things." — Roman Ugarte
> "I sort of see myself as the explorer, you know, like trying to figure out what will what does software engineering look like 6 months through a year from now." — Lauren Tan
> "The smallest loop I find is you know like actually focus on this part. How do you teach your agent to you know run the code empirically uh you know take CPU traces, you know like performance metrics, like run the code and run the application like a real user would." — Lauren Tan
> "I think this next era of AI will be you know these bots really need to learn those things and we need to teach that at the product level at the harness level at the model level um and I think it's going to be it's going to be really interesting next few years." — Roman Ugarte

## People mentioned
- Lauren Tan
- Roman Ugarte
- Jack Dorsey
- Steve Yegge
- Tom Dale
- Leah Silber
- Gary

## Topics
`ai-coworkers` `agent-orchestration` `software-engineering` `startup-culture` `product-management` `artificial-intelligence`
