---
podcast: "Practical AI"
episode: "How to get discovered in AI search"
published: 2026-09-17
duration: "55m14s"
audio_url: "https://pscrb.fm/rss/p/dts.podtrac.com/redirect.mp3/media.transistor.fm/66185604/d94b5158.mp3"
episode_url: "https://share.transistor.fm/s/66185604"
transcript_source: rss
generated_at: 2026-09-17T22:12:43Z
model: "deepseek-v4-pro"
guid: "feacf9a3-586b-4106-8da3-aab51f1c2765"
---

# How to get discovered in AI search — Practical AI

## TL;DR

In this episode, Daniel Whitenack and Chris Benson interview Liam Dunne and Ben Moore, cofounders of Discover Labs, about AI search and answer engine optimization (AEO). They explain that traditional SEO is not dead but has expanded; businesses must now optimize for both human visitors and AI agents. Key insights include the importance of statistical rigor in measuring AI visibility, the role of Reddit and consensus in influencing model outputs, and the need for relevancy, consensus, and consistency across brand messaging. They also discuss the future of agent accessibility and the increasing influence of model weights as training cycles shorten.

## Key points

- AI search is an umbrella term; AEO and GEO are synonymous, with AEO favored because venture-funded companies have invested in that terminology. Traditional SEO is not dead but has expanded into organic search, requiring the same on-page, off-page, and technical work with shifted tactical priorities.
- Buyer behavior has shifted: users consume information inside LLMs, causing a drop in organic traffic as the LLM becomes the new website visitor, leading to zero-click research and making measurement of brand visibility inside these systems a critical challenge.
- Marketers must now optimize for two types of visitors: human users and AI agents. This requires considering how agents access and understand website information, potentially leading to separate agent-optimized versions of site content.
- To measure AI search performance, one must statistically bound noise on metrics like citation rate, mention rate, and share of voice. The SEO industry generally lacks statistical rigor, often misinterpreting random fluctuations as significant changes due to non-deterministic LLM outputs.
- Behind the scenes, an LLM response involves multiple stages: prompt ingestion, reasoning/forward pass with sources of randomness (GPU floating point error and temperature), retrieval engine query fan-outs, consensus algorithms (similar to DeepMind's agree style) to avoid hallucinations, and final response generation. Optimizing visibility requires addressing all four pillars: model weights, reasoning, retrieval, and response.
- Research on ChatGPT's retrieval engine found that about one-third of retrieval slots were allocated to Reddit threads, yet most of these Reddit sources were not cited in the final output. They served to ground the model's reasoning; thus Reddit remains influential even if not directly cited.
- Reddit training data is also used in reinforcement learning from human feedback (RLHF), appearing in model weights especially in ChatGPT and Gemini. Therefore, a Reddit strategy should focus on providing relevant content rather than chasing upvotes, as engagement metrics like upvotes did not correlate with citation likelihood, whereas content relevance did.
- Conversational prompts create a long tail distribution of queries with specific entities and constraints, which are less competitive than traditional head keywords. This allows smaller brands to compete against incumbents like HubSpot by creating highly relevant content for niche queries that map to their ideal customer profile and differentiators.
- Authority and trust are determined by consensus: the LLM looks for agreement across multiple sources. If contradictory or negative information exists, it can sway the model. Marketers can exploit 'blank space' by publishing accurate but previously unavailable data (e.g., pricing) to become the source of truth.
- A practical strategic framework is relevancy, consensus, consistency. Brands must define clear messaging and ensure consistent representation across websites, social channels, YouTube, customer advocacy platforms like G2, and third-party sites so that the entity is uniformly described in the training data and retrieval context.
- The next frontier is agent accessibility: moving from read-only discovery to read-write environments where AI agents not only retrieve information but take actions such as booking demos or installing software. Google's Web OCP program hints at this shift, requiring websites to be optimized for agent action completion.
- Training cycles are shortening (top layers retrained weekly/biweekly, backbones every few months), making model weights more influenceable. Getting brand information into training data is orders of magnitude more valuable than appearing in a single context window, but both are needed. Fingerprinting and AI-content detection will make it harder to game, increasing value of genuine human-generated content.

## Notable quotes

> "You won't hear me say that SEO is dead. Not an opinion I hold. I just think the space has grown." — Liam Dunne
> "The SEO industry has like an incredible ability to point at numbers going up and claim that it was them, and then has an also incredible ability to point at a number that's going down and claim its Google core update." — Ben Moore
> "I think it's relevancy, consensus, consistency." — Liam Dunne
> "If there's blank space, you can almost publish, you can almost get anything cited." — Ben Moore

## People mentioned

- Daniel Whitenack
- Chris Benson
- Liam Dunne
- Ben Moore

## Topics

`ai-search` `answer-engine-optimization` `seo` `generative-engines` `marketing-strategy` `reddit` `agent-accessibility`
