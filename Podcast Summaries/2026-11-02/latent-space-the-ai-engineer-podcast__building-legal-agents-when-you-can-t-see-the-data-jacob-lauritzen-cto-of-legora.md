---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Building Legal Agents When You Can't See the Data - Jacob Lauritzen, CTO of Legora"
published: 2026-11-02
duration: "1h7m21s"
audio_url: "https://podcasts.beehiiv.com/3967c1f2-cb97-4eb0-a5c0-21a1d2579408/01a15fdf-0e93-7e1a-880d-d73752a1f915/01a19df0-34a3-7307-96fc-48928908772b/01a19df0-35e0-7c61-90cc-3a37c8983d85.mp3"
episode_url: "https://lspace.beehiiv.com/podcast/s/latent_space_the_ai_engineer_podcast/building_legal_agents_when_you_can_t_see_the_data_-_jacob_lauritzen_cto_of_legora"
transcript_source: rss_vtt
generated_at: 2026-11-09T23:17:26Z
model: "deepseek-v4-pro"
guid: "youtube:bJBEFzlKPoQ"
---

# Building Legal Agents When You Can't See the Data - Jacob Lauritzen, CTO of Legora — Latent Space: The AI Engineer Podcast

## TL;DR
Jacob Lauridsen, CTO of Legora, discusses building AI agents for legal work, covering the product evolution from legal research to Tabular Review and Intake, model routing across frontier and open-weight models, and the challenges of evaluating systems without access to confidential client data. Legora uses a mix of models per task, has an eval team of engineers and lawyers, and emphasizes a citation system to combat hallucination. The conversation also explores how AI will reshape law firm economics, partnerships, and the roles of junior lawyers, with Legora betting on becoming the default operating system for legal work.

## Key points
- Legora started as a ChatGPT wrapper for Swedish legal research using Swedish laws and a search tool, then expanded to drafting, editing Word files, organizing documents, and running complex analyses.
- Tabular Review allows bulk extraction and review of thousands of agreements; it can process up to a million cells in a single day, and agents can spin up Tabular Reviews automatically for due diligence, highlighting red flags and drafting memos.
- Legora uses a model picker in the agent, offering models like Fable, Opus, and Astra, but originally wanted to abstract model choices; the team routes tasks to different models based on cost, latency, and capability, and found that many legal tasks do not require top-tier intelligence.
- The company's eval team consists of three engineers and three lawyers, plus a pool of about 100 legal engineers (Big Law lawyers with CS backgrounds); they create RL environments with 10,000-file data rooms to measure model performance on red-flag detection and report drafting.
- Because client data is confidential, Legora cannot inspect traces or user data; instead, they build synthetic eval sets, occasionally ask clients to anonymize documents, and use online LLM-based metrics to approximate quality.
- Legora's citation system anchors every generated statement to a source with clickable links and bounding boxes, allowing users to drill down through layers, which addresses hallucination but is not always verified by lawyers.
- Infrastructure runs on Azure because early law firms trusted Microsoft (already using Office 365) but not AWS; this has shifted, but data processing agreements keep vendor list tight.
- The company has made acquisitions, including Kira (Sweden) and Wexler (UK fact intelligence for litigation), and uses its own product for M&A due diligence on those deals.
- Legora found that some tasks improved dramatically with model releases; for example, citation accuracy jumped from 70% to 99% after a new model without any engineering effort.
- Open-weight models are now being considered for post-training on bounded tasks like legal research fan-out retrieval, but Legora does not plan to train its own main agent model.
- The forward deployed engineering team started about six months ago with a YC founder in New York; legal engineers configure the product as power users, while forward deployed engineers write production code and integrations.
- Legora's Lists primitive, a flexible Notion database-like feature, enables agents and users to create and manage tasks, and supports triggers and automations, effectively acting as glue for agent orchestration.
- The partnership model in law firms may need to change because partners aren't incentivized to codify knowledge into systems; Legora aims to help firms build durable IP from their workflows.
- Billing is shifting from hourly to project fees and flat rates, especially for lower-end work; some work is done for free to maintain client relationships, while strategic work remains high-value.
- US law firms and corporate clients are more aggressive in custom-building on top of Legora, while European clients prefer off-the-shelf products.
- Legora has APIs but only started six months ago; the team previously resisted to maintain velocity, now explores whether the agent itself can serve as an API.
- The most important tools in the harness are search, analysis, and drafting; long document reasoning and editing across huge contracts remain difficult for current models.
- The company has about 100 engineers and feels too small; it is hiring in New York, Stockholm, and London.

## Notable quotes
> "We had one problem where, like, it wouldn't cite properly, and so we were like, 'Next model release solved.' It went from, like, seventy percent to, like, ninety-nine. It's just like we did nothing and, like, great." — Jacob Lauridsen
> "We can't look at any traces, basically. We can't look at any data 'cause it's all super confidential. And that's, like, extremely frustrating to work with 'cause you, you basically don't know what people are doing." — Jacob Lauridsen
> "If you're doing a, you know, four hundred million dollar M&A transaction, you're gonna want Legora." — Jacob Lauridsen
> "Paralegals, gone. Yeah. Junior lawyers, gone. I don't need them." — Host
> "We're just gonna be twenty engineers for the rest of the company's history, and, like, we're gonna beat everyone who's much bigger. But we're a hundred engineers now, and it feels like we're way too small." — Jacob Lauridsen
> "The partnership model kind of exists because the only type of capital that these companies have is human capital." — Jacob Lauridsen

## People mentioned
- Jacob Lauridsen
- Max
- Dee
- Song
- Jeff Bezos

## Topics
`legal-tech` `ai-agents` `model-routing` `data-privacy` `due-diligence` `partnership-model` `evals`
