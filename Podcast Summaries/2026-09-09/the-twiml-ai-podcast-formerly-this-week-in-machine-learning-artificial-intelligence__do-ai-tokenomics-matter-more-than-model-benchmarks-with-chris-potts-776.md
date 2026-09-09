---
podcast: "The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)"
episode: "Do AI Tokenomics Matter More Than Model Benchmarks? with Chris Potts - #776"
published: 2026-09-09
duration: 59m29s
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/MLN2214331662.mp3"
episode_url: "https://twimlai.com/podcast/twimlai/do-ai-tokenomics-matter-more-than-model-benchmarks"
transcript_source: youtube_autocaptions
generated_at: 2026-09-09T22:09:42Z
model: "deepseek-v4-pro"
guid: "0d6b3aca-ac83-11f1-826a-6301185ed7a9"
---

# Do AI Tokenomics Matter More Than Model Benchmarks? with Chris Potts - #776 — The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)

## TL;DR

Stanford professor and BigSpin co-founder Chris Potts discusses 'tokenomics'—the economics of AI token usage—arguing that AI progress needs to be measured by value per token, not just benchmarks. He presents a consumer price index for coding agents showing token inflation in Claude Code sessions, explains why inference-time scaling yields diminishing returns, and challenges the bitter lesson narrative. Potts also covers user expertise, DSPy, and architectural innovations needed for more efficient and equitable AI.

## Key points

- Chris Potts argues that AI progress must now be measured by token economics, not just model benchmarks, because reasoning models consume more tokens and providers are beginning to charge true costs plus profit.
- He observes providers are testing real pricing to prepare for IPOs, citing a Copilot user whose projected monthly bill jumped from $500 to $11,000, and likens the shift to ride-share prices rising from $20 to $500.
- Potts and colleagues propose a consumer price index (CPI) for coding agents: define a basket of engineering goods (PRs, code survival, documentation, etc.), measure tokens spent per good, and apply hedonic adjustments for quality.
- Using ~6,000 real Claude Code sessions with Opus 4.6 from February to mid-April, they found token inflation: token purchasing power declined even after adjusting for code survival (lines lasting >4 days).
- The decline partly reflects a shift in token usage: in February most tokens generated code, but by mid-April tokens were split among code, thinking, and user explanation, with explanation value not captured by PR metrics.
- Potts notes inference-time scaling curves flatten: spending more tokens yields small performance gains, which was predictable from scaling laws and underlies current token efficiency concerns.
- He rejects the pure scaling narrative, showing that modern transformers incorporate analysis-driven changes like local positional encodings, wide sparse MLPs, and optimized activations, not just brute-force scaling.
- Despite claims of mysterious models, Potts argues practitioners have deep intuitions (e.g., about positional encodings) that drove progress, comparable to a mechanic's functional understanding.
- His research group shifted to interpretability because it is cheaper and benefits from better models, avoiding direct competition with large labs on tasks like summarization.
- Potts believes current transformer architectures are inefficient because they do not learn modular recursive functions; he advocates pursuing recursive architectures and byte-level models for better capability per parameter.
- Building on Anthropic's AI fluency index, Potts's team found high-fluency experts use augmentative collaboration (iterating, complaining) while novices delegate and accept uncritically, and this behavior causally predicts task success.
- Experts succeed on harder tasks despite more visible failures because they push back, suggesting that teaching users to challenge AI could expand value beyond experts.
- Potts highlights verification as critical: coding is a domain where running code provides cheap verification, enabling non-experts; in law and other domains without cheap verification, users need expertise to evaluate outputs.
- DSPy, created by Omar Khattab under Potts, exemplifies a shift from paper-centric to open-source project contributions; it provides modular, auditable LM pipelines and prompt optimization, relevant even as agents write code.
- LLMs show high variance on subjective tasks and can systematically vary with added requirements; Potts recommends automatic prompt optimization (e.g., JEPA, MIPRO) to enforce consistency.
- Potts points to byte-level/tokenizer-free models (advocated by student Julie Kallini) as more equitable and potentially enabling inference-time scaling, and recursive architectures as a way to overcome fixed-depth limits.
- He is investigating data-centric interpretability and security, noting that small data poisoning attacks can influence model preferences, raising questions about detection and mechanisms, connecting interpretability to safety.
- Potts advocates for open-weights models to maintain ecosystem diversity and avoid homogeneous biases, comparing to diverse red teams; if all models converge, they miss the same bugs.

## Notable quotes

> "The analogy here is like it used to cost me $20 to take a ride share to the airport, Uber or Lift, and now it costs 90, but it's more like 20 to like 500 or something, right?" — Chris Potts
> "We all knew this. We all knew this and we're just seeing it now play out." — Chris Potts
> "You're not bitter lesson enough apparently." — Chris Potts
> "If all the code review agents are biased in the same way, they will miss exactly the same class of bugs and then we're all sunk." — Chris Potts
> "Yes, your token is not buying you what it once did according to everything we can think to measure here." — Chris Potts

## People mentioned

- Chris Potts
- Sam Charrington
- Omar Khattab
- Ed Zitron
- Julie Kallini

## Topics

`ai-tokenomics` `model-architectures` `inference-time-scaling` `ai-fluency` `prompt-engineering` `machine-learning`
