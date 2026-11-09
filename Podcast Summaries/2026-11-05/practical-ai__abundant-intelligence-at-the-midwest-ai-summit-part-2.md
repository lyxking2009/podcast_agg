---
podcast: "Practical AI"
episode: "Abundant Intelligence at the Midwest AI Summit Part 2"
published: 2026-11-05
duration: "47m53s"
audio_url: "https://pscrb.fm/rss/p/dts.podtrac.com/redirect.mp3/media.transistor.fm/1b301bdc/b7154aa0.mp3"
episode_url: "https://share.transistor.fm/s/1b301bdc"
transcript_source: rss_vtt
generated_at: 2026-11-09T23:20:10Z
model: "deepseek-v4-pro"
guid: "61100a8a-1794-45d2-9fc9-8cde63686074"
---

# Abundant Intelligence at the Midwest AI Summit Part 2 — Practical AI

## TL;DR
This episode features three talks from the Midwest AI Summit. Don Barger of the International Mission Board explains how his organization created an AI charter to define ethical boundaries, leading to tools like Faithbot with 2.5 million users but refusing AI-based pastoral care. Mark Lay of Noblis outlines four key decisions for AI adoption: delegation, accountability, technical debt, and monitoring. Darren Taillie of AeroCore Technologies demonstrates practical AI agents for self-service data access and automated work order verification, reducing errors and saving analyst time.

## Key points
- Don Barger of the International Mission Board (IMB) emphasized creating an AI charter before writing code, defining allowed uses (drafting/editing, summarizing, research, translation, training content, code with human-in-the-loop) and prohibited uses (AI posing as human, final decisions about people solely by AI, monitoring people without knowledge, autonomous high-risk actions, and AI handling pastoral care/counseling/prayer).
- IMB refused to build an AI counselor chatbot despite an urgent internal request, citing their charter; Barger responded that 'something can be worse than nothing if you're doing the wrong things.'
- IMB built Faithbot (faithbot.tools), a faith-grounded chatbot with approximately 2.5 million users in 18 months and no paid advertising, available in 50+ languages, each instance using different LLMs tailored to language and worldview. It was built during a flight from Denver to Narita and explicitly does not pray, write sermons, give medical or financial advice, and flags crisis conversations to connect users to human help or crisis services.
- Barger stressed evaluating AI projects against trust rather than feasibility, and deciding 'what will we never do' early; he found that clear limits actually freed his team to innovate more within boundaries.
- Mark Lay of Noblis presented four critical decisions for enterprise AI. First: what to delegate to AI, using a framework of reward (better/efficient outcomes) versus risk (error likelihood and consequence). Low-risk, low-reward areas like online exploration are safe; high-risk areas need deterministic guardrails.
- Second: accountability cannot be delegated to AI; humans must own data protection, output validation, agent behavior monitoring, and outcome ownership. Noblis spent a year identifying over 1,000 datasets, narrowing to about 120 critical datasets with roughly 30 data stewards.
- Third: AI introduces new technical debt, including vendor platform changes breaking existing apps, irreversibility, and capacity to evolve. Lay compared AI to Excel as a decentralized platform versus centralized Power BI, noting that departments will build their own tools that must be managed.
- Lay adapted Drucker's management principle: AI can produce a lot of output, but its value depends on how organizations address these four decisions.
- Darren Taillie of AeroCore Technologies described their jet engine washing business: patented nucleated foam cleaning, 11 domestic US sites plus international (Japan, Riyadh, Brazil), night shifts from 7PM to 5AM, and 15-30 washes per night. Data challenges included analyst bottlenecks and workaround sprawl from ad-hoc pandas scripts.
- AeroCore built a text-to-SQL agent using Prediction Guard's AgentForge with MCP servers, deployed in Microsoft Teams, enabling self-service data queries. Complex queries that used to take 30 minutes to a week now return in seconds, freeing analysts for strategic work and increasing adoption across leadership.
- A second agent uses a vision model to verify work orders daily at 8:15 AM: it scans photos, extracts aircraft tail numbers, compares them to app inputs, and flags mismatches. It also checks operator notes for abnormal procedures, smoke/fume events, and midnight crossing issues. This reduced errors so much that the monthly reconciliation meeting was canceled, saving 5-6 person-hours.
- AeroCore's future projects include using vision models to automatically gather data from the wash truck and verify valve, temperature, and pressure settings without adding digital sensors to the analog trucks.

## Notable quotes
> "Just because we can do something doesn't mean we should do something." — Don Barger
> "Something can be worse than nothing if you're doing the wrong things." — Don Barger
> "Accountability can't be delegated." — Mark Lay
> "We are an incredibly analog company in a digital world." — Darren Taillie
> "We canceled that meeting this month. There were no incorrect work orders." — Darren Taillie

## People mentioned
- Don Barger
- Mark Lay
- Darren Taillie
- Sam Altman
- Elon Musk
- Chris Benson
- Daniel Whitenack

## Topics
`responsible-ai` `enterprise-ai` `ai-agents` `data-access` `computer-vision` `workflow-automation` `ai-governance`
