---
podcast: "Dwarkesh Podcast"
episode: "What will computer chips look like in 2045?"
published: 2026-10-22
duration: "1h9m0s"
audio_url: "https://api.substack.com/feed/podcast/221632772/30c38515d3ea0554c60350beead67f42.mp3"
episode_url: "https://www.dwarkesh.com/p/intel-tour"
transcript_source: web
generated_at: 2026-10-27T22:11:06Z
model: "deepseek-v4-pro"
guid: "substack:post:221632772"
---

# What will computer chips look like in 2045? — Dwarkesh Podcast

## TL;DR
In this episode, Dwarkesh Patel tours Intel's fabs and interviews process development leaders Chris Auth, Kevin Fischer, Jack Kavalieros, Tyler Osborn, and CTO Pushkar Ranade about how advanced chips are made and what's next. They discuss Intel's 14A node, High-NA EUV lithography, backside power, gate-all-around transistors, CFETs, ruthenium interconnects, and hybrid bonding, while addressing supply chain bottlenecks and the economics of competing with TSMC. The conversation highlights that AI's insatiable demand for compute faces hard limits from tool availability and fab capacity, and Intel is betting on architectural innovations and foundry services to capture that demand.

## Key points
- Intel's upcoming 14A process has been in development for over three years and involves roughly 2,000 process steps. The current phase focuses on finalizing tool sets and fine-tuning steps to achieve repeatability at thousands of wafers per week. Yield improvement relies on inline metrology to detect defects and end-of-line electrical testing, followed by cross-section analysis to locate defect sources.
- Intel's 10nm node (target 2016, actual manufacturing 2018) taught the company to limit innovation per node to maintain schedule. Aggressive hyperscaling without ready tools, especially EUV, caused delays. Intel now uses phase-gate maturity metrics (yield, defect, performance criteria) to decide which technologies progress from pathfinding to development.
- High-NA EUV lithography (NA 0.55) is being used by Intel for 14A because its backside power architecture reduces front-side pitch requirements, allowing single-pass patterning. TSMC, lacking backside power, would still need double patterning, making High-NA uneconomical for them. Intel offers High-NA as a customer option in the PDK.
- Backside power delivery moves thick power lines below the transistors, freeing space on the front side for signal lines and reducing pitch constraints. This architectural change is a major differentiator for Intel, enabling more efficient use of High-NA EUV and improving signal routing.
- Transistor evolution progressed from planar (1960) to FinFET (demonstrated 1989, Intel production 2011) to gate-all-around (demonstrated 1988, Samsung production 2022). Gate-all-around wraps channel ribbons completely with the gate, requiring sacrificial placeholder layers and new materials like high-k metal gate. The gap between lab demonstration and production is due to integration challenges and tool development.
- R&D shifted from simple Dennard scaling in the 80s/90s to a golden age of innovation, introducing strained silicon, high-K metal gate, FinFETs, gate-all-around, and backside power. The number of materials used has increased 10X, and atomic layer deposition now allows atomic-scale control. Engineers now intentionally disrupt process flows rather than preserving them.
- Intel's transition to a foundry required building PDKs from scratch. The 18A node initially lacked a formal PDK, causing customer difficulties. Intel now has maturity metrics for PDK versions (0.5, 0.9, 1.0) and incorporates customer feedback. 14A will launch with high-quality PDKs from the start.
- CFET (complementary FET) stacks n-type and p-type transistors vertically, potentially doubling transistor density. Challenges include high aspect ratios for etches and fills, and thermal management. Research labs test simplified versions (one ribbon on one ribbon) using short-loop vehicles to faster evaluate integration before scaling to full production.
- Ruthenium is being explored to replace copper for very fine interconnects (around 20nm pitch). Copper requires a resistive liner, but ruthenium can be etched directly, eliminating the liner. At small dimensions, ruthenium's lower effective resistance wins, making it a candidate for future nodes.
- Hybrid bonding enables direct copper-to-copper connections between chips, eliminating solder bumps. Pitch has shrunk from 130-150 microns 15 years ago to 25 microns in production today, with research reaching 0.45 microns. This technology improves bandwidth and power efficiency, critical for AI accelerators and memory stacking.
- Supply chain bottlenecks limit foundry capacity expansion. ASML produces fewer than 100 EUV machines per year, and all equipment suppliers face shortages. Intel's Pushkar Ranade estimates leading-edge wafer output could maybe 2X, not 10X, due to these constraints. AI's compute demand has been 3X year-over-year, creating a mismatch.
- Foundry economics depend on extreme economies of scale: about 80% of cost is fixed. Even small increases in fab utilization or yield have nonlinear revenue impact. Only three advanced foundries remain (TSMC, Intel, Samsung) because consolidation is driven by these economics.
- Intel's CTO Pushkar Ranade suggests the industry is moving from horizontal to pseudo-vertical integration as hyperscalers like Elon Musk consider building fabs. Intel and Musk are exploring a partnership on 'Terafab' to accelerate fab construction, though tools remain a bottleneck.
- The cost of building a fab doubles every 4-5 years, and a single High-NA EUV scanner costs around $400 million. This risk-averse industry avoids radical changes, but clean-slate approaches may be needed to rethink cleanrooms and chip design. Intel may increase tool purchases to lock down supply for future AI demand.

## Notable quotes
> "Ten nanometer was a good example of where we had very aggressive targets. We were doing what was called hyperscaling at the time, so we were scaling much faster than what had been historically done. In order to do that, we came up with all sorts of innovative ideas on how we could achieve that without new tools, because the tools weren’t ready. That put a lot of burden on the process and a lot of burden on the engineers." — Chris Auth
> "One thing you learn in this industry is that you will never have a complete data set. If you want to wait for a complete data set, you’ve waited too long. It’s already too late." — Kevin Fischer
> "It is a limiter. We have to make the most of the capacity that we already have installed. We’ll try to increase the output from our existing fleet of tools. But EUV or ASML is not the only bottleneck." — Pushkar Ranade
> "Ruthenium is very different in that regard because it’s a material that we can actually etch. As such, by etching it, we don’t need that liner. Now you scale those very fine dimensions, and if you can trade off that resistive liner with no liner, ruthenium starts to win." — Jack Kavalieros

## People mentioned
- Chris Auth
- Kevin Fischer
- Jack Kavalieros
- Tyler Osborn
- Pushkar Ranade
- Elon Musk

## Topics
`semiconductor-manufacturing` `ai-hardware` `advanced-packaging` `process-technology` `foundry-economics` `moores-law`
