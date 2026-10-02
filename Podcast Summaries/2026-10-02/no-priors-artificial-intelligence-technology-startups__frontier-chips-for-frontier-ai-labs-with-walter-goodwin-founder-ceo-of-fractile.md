---
podcast: "No Priors: Artificial Intelligence | Technology | Startups"
episode: "Frontier Chips for Frontier AI Labs, with Walter Goodwin, Founder/CEO of Fractile"
published: 2026-10-02
duration: "35m38s"
audio_url: "https://traffic.megaphone.fm/PDP7579412568.mp3"
episode_url: ""
transcript_source: web
generated_at: "2026-10-02T22:13:50Z"
model: "deepseek-v4-pro"
guid: "44fa0b1c-be0b-11f1-b00d-cb6a27141e9e"
---

# Frontier Chips for Frontier AI Labs, with Walter Goodwin, Founder/CEO of Fractile — No Priors: Artificial Intelligence | Technology | Startups

## TL;DR
Walter Goodwin, founder and CEO of Fractile, discusses how his full-stack AI chip company is building very fast inference chips for the world's largest models by focusing on memory bandwidth and scalable DRAM rather than SRAM. He explains that most AI accelerators, including hyperscaler custom chips, are architecturally similar and built with ASIC partners like Broadcom, while Fractile owns the entire design stack to iterate faster. Goodwin predicts that speed and memory bandwidth will define the next frontier, enabling longer-context agents and helping frontier labs maintain an edge over open-source models.

## Key points
- Fractile was founded in summer 2022 and has about 150 employees; it builds full-stack inference chips optimized for speed, initially using SRAM but pivoting around late 2023/2024 to high-bandwidth DRAM to support growing context lengths and higher-capacity memory while preserving speed advantages.
- Goodwin argues that most AI chips, including Google TPU, Meta MTIA, Microsoft Maya, and OpenAI Jalapeno, rely on the same building blocks—HBM, tensor cores, and TSMC advanced packaging—and are delivered via ASIC houses like Broadcom ($2 trillion), so differentiation is limited; Fractile is one of few teams handling architecture through physical design in-house.
- A core technical bet is on memory bandwidth: Nvidia systems contain 6-9 custom chips, while Fractile aims to build a single chip with roughly 25x more bandwidth per chip than HBM-based chips, enabling multi-trillion-parameter models to run at thousands of tokens per second.
- Goodwin distinguishes between a 'snappier chatbot' (faster horses) and the real opportunity: accelerating long-running agents and long-context attention, which current fast inference chips like Groq and Cerebras cannot handle due to low memory capacity; Fractile seeks to combine DRAM capacity with SRAM-like bandwidth.
- He highlights that flops scaled about a millionfold in 20 years while memory bandwidth scaled only about 40x; increasing bandwidth can reduce the flops needed for a given intelligence, especially for sparser Mixture-of-Experts models (e.g., 1-in-128 or 1-in-256) and less bandwidth-hungry attention mechanisms.
- On chip design AI, Goodwin says front-end design and prototyping could be significantly compressed within a few years, but final sign-off with Cadence/Synopsys and foundry DRC/LVS rules will remain; he suggests dividing a 10-year prediction for intent-to-GDS2 automation by four and subtracting, due to Amdahl's law bottlenecks in physical design.
- Financial and physical constraints remain: chips require 3-5 year amortization windows and 3-5 month fab cycle times, so you cannot ship fundamentally new chips every few weeks; value comes from having a rolling frontier of bets and being ready to ramp the correct one, providing a 3-6 month advantage analogous to frontier model advantage.
- Goodwin expects hyperscalers to continue deploying diverse platforms for supply security and pricing leverage against Nvidia; first-party efforts are architecturally similar and partly gamesmanship, while third-party players like Fractile survive because frontier labs cannot rationally go all-in on proprietary hardware and risk missing a computational breakthrough.
- Speed is crucial for frontier labs because open-source models are catching up; the edge lies in premium intelligence plus fastest deployment to perform the most reasoning in the shortest time, making Fractile's speed-focused chips essential for frontier AI deployments.

## Notable quotes
> "It's the equivalent of the frontier model for the chip space is if you can just find a way to structurally carve out a 3 to 6 months advantage, you will be winning all of those deployments." — Walter Goodwin
> "The snappier chatbot is kind of the faster horses of kind of fast inference." — Walter Goodwin
> "We've scaled flops like a millionfold in the last 20 years. Memory bandwidth has gone up about 40x in the same time frame." — Walter Goodwin
> "I would divide it by four and I probably subtract a bit from that." — Walter Goodwin

## People mentioned
- Walter Goodwin
- Henry Ford

## Topics
`ai-chips` `inference` `memory-bandwidth` `semiconductor` `frontier-models` `chip-design` `market-structure`
