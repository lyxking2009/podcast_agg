---
guid: 4945a340-783e-4fba-9782-b4510023ea71
title: "Why Cerebras CEO Andrew Feldman Built The World's Largest Computer Chip"
show: "Odd Lots"
date: 2026-05-21
duration: "51:53"
transcript_source: rss_omny_srt
---

## Key Points
- Cerebras builds wafer-scale chips — roughly the size of a dinner plate — that are 58 times larger than any competitor chip, enabling them to use ultra-fast SRAM instead of slow HBM memory, making inference 15x faster than the fastest GPU and up to 1000x faster on certain workloads.
- The wafer-scale approach took a decade, 500 engineers, and ~$500M to deliver the first chip; every previous attempt in the 75-year history of computing had failed, including a high-profile effort by Gene Amdahl in the 1980s.
- Cerebras IPO'd in May 2026 raising ~$5.5B at a ~$64B valuation; the week of the IPO they highlighted a $20B deal with OpenAI North and an AWS Bedrock partnership to offer ultra-fast inference.
- Cerebras chips avoid the three main AI supply chain bottlenecks: HBM memory (made by Samsung/Hynix/Micron), the CoWoS advanced packaging process at TSMC, and TSMC's most-constrained 3nm node — they use 5nm instead.
- Their current business is constrained not by chip supply but by data center capacity (power and buildings), a problem shared across the entire AI infrastructure industry.
- Feldman argues speed matters equally in both conversational and agentic AI — a company that gets 3x more work done in an agentic loop over time will simply outcompete slower rivals.
- Cerebras operates its own cloud, offering open-source models (including Kimi K2, a 1T-parameter model) at significantly lower cost than closed-source alternatives, with 10-15x speed advantage over comparable providers.
- G42 (Abu Dhabi) was Cerebras' largest customer last year, using the chips to train English-Arabic models and serve inference for UAE enterprises including ADNOC; data centers are located in Santa Clara, Minneapolis, Dallas, and soon Toronto.
- CUDA's moat is largely gone in inference (trivial API migration) and eroding in training — two of the three frontier models (Gemini on TPUs, Claude on Trainium) are now CUDA-free.
- Feldman believes export controls on advanced chips to China make strategic sense, despite foreclosing some markets, and dismisses the argument that keeping China on US-designed hardware is a better strategy.

## Overview
Andrew Feldman, founder and CEO of Cerebras Systems, explains how his company built the world's largest computer chip — a wafer-scale processor the size of a dinner plate — and why that seemingly counterintuitive design choice enables dramatically faster AI inference. The core insight is that using fast on-chip SRAM (instead of the slow off-chip HBM memory that GPUs depend on) eliminates the memory bandwidth bottleneck that causes the perceptible lag when using AI models, and the only way to get enough of that fast memory is to build a much larger chip. Hosts Joe Weisenthal and Tracy Alloway explore the business implications: the $5.5B IPO, the economics of open vs. closed source AI, the CUDA moat question, export controls, and whether inference speed will ultimately command a sustained premium.

## Implications
Cerebras' architecture represents a direct challenge to Nvidia's GPU-centric inference stack, and its ability to sidestep the three main supply constraints (HBM, CoWoS, 3nm) gives it a structural capacity advantage in a market where data center power and buildings are the real bottleneck. The open-source vs. closed-source AI battle is intensifying — if open-source models close the remaining 3-5% quality gap, the economic case for premium closed-source inference weakens significantly, which could reshape valuations across the AI stack. Longer term, the episode raises an unresolved question: as enterprises become more sophisticated about AI cost allocation, the premium placed on speed and model quality will likely fragment by use case, making the current winner-take-all framing of inference economics too simplistic.

## Notable Quotes
> "We're fifteen times faster than the fastest GPU. On some problems we're fifty, one hundred, even one thousand times faster than graphics processing units." — Andrew Feldman

> "How big is the market for slow search? Zero. How big is the market for dial-up internet? Zero. Why is that? Because nobody wants to wait." — Andrew Feldman

> "Two of the three leading frontier models — Gemini, Claude, and GPT — Gemini built by Google on TPUs, trained on TPUs, served on TPUs. No CUDA. Anthropic's models trained on Trainium, no CUDA. So two of the three leading models today use no CUDA. That's a hemorrhaging of share." — Andrew Feldman

> "Far more important than some change in my wealth was that we made more than eight hundred millionaires. And that's something I'm proud of every minute of every day." — Andrew Feldman

> "If you're engaged with the AI, if you're writing code — if your competitor gets three times, five times, ten times as much work done in twenty minutes than you do, you're gonna get smoked." — Andrew Feldman

> "In our business, we measure twice before we cut once. And you have to put that in your soul, and you have to like it. You have to like that mistakes in our business are really expensive." — Andrew Feldman

> "Today's business is constrained by data centers — and that's the grand irony. You invent technology that's been unbuildable for seventy-five years, and what are we all constrained by? Buildings." — Andrew Feldman

## People Mentioned
- Andrew Feldman — Founder and CEO, Cerebras Systems
- Joe Weisenthal — Co-host, Odd Lots (Bloomberg)
- Tracy Alloway — Co-host, Odd Lots (Bloomberg)
- Jensen Huang — CEO, Nvidia (cited as one of the great CEOs of the era)
- Hock Tan — CEO, Broadcom (cited alongside Jensen Huang)
- Lisa Su — CEO, AMD (cited alongside Jensen Huang)
- Gene Amdahl — Computing pioneer who failed at wafer-scale in the mid-1980s at Trilogy
- Ray Wang — Semi Analysis (referenced in hosts' post-interview discussion)
- Donald Trump Jr. — Investor via 1789 Capital (minor investor in Cerebras' G-round)

## Topics
- Wafer-scale chip architecture and the physics of on-chip vs. off-chip memory
- AI inference economics: speed, token cost, and market pricing
- Agentic AI vs. conversational AI and why speed matters in both
- Open-source vs. closed-source AI model competition
- CUDA's eroding moat in AI training and inference
- Cerebras IPO: valuation, revenue concentration, and G42 relationship
- Export controls on advanced semiconductors and China strategy
- US semiconductor manufacturing capacity (CHIPS Act, TSMC Arizona, Samsung Texas)
- Financialization of compute markets and data center infrastructure
- Healthcare and pharma as protected AI use cases due to proprietary data
