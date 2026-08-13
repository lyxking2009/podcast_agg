---
title: "Why smarter AI models could drive up compute prices 10x"
show: "Dwarkesh Podcast"
date: "2026-08-03"
guid: "substack:post:209664458"
transcript_source: "web"
---

## Key Points

- This is a solo essay/video from Dwarkesh Patel (time-boxed to two hours of writing), examining the compute economics of frontier AI labs over the coming years, starting from the observation that Anthropic's revenue has roughly 10x'd year over year while its compute has only ~3x'd
- For a lab to 10x revenue while compute only 3x's, some combination of three things must happen: (1) margins increase, (2) the price of compute increases, or (3) labs shift a growing share of compute from training to inference
- Patel argues all three have in fact been happening: Anthropic's inference margins reportedly rose from ~40% in 2025 to possibly >80% now; spot compute prices are up 40%+ since the February trough; and the inference share of compute spend (per Epoch data on OpenAI's 2024 spend) has grown from roughly a quarter toward 50% or more
- Labs would prefer not to lean on option 3, since spending most compute on inference implicitly signals that training bigger/smarter models isn't worth the investment — the opposite of what labs believe and want investors to believe
- For margins alone to explain the growth (option 1), margins would need to reach the mid-90s percent by the end of next year, which Patel calls implausible — leaving rising compute prices as the more likely dominant mechanism
- Evidence cited for compute prices already rising sharply: Google reportedly pays SpaceX ~$900M/month for 110K GPUs (a blend of GB200s/GB300s), about 2x the spot price per hour for that hardware, and spot prices themselves are already 40% above February levels
- Core thesis: as AI models get smarter, they'll better monetize the same amount of compute. Patel estimates that a true human-level software engineer running on an H100-equivalent chip, priced at current software engineer market rates, would justify renting that H100 for over $250k/year — about 15x today's spot prices
- Patel addresses the "lump of labor" objection (that more AI engineers would crash the marginal value of engineering labor) by analogy to economists' general finding that high-skilled immigration doesn't depress wages long-term due to specialization and innovation effects — though he flags uncertainty about whether this holds under an extremely fast, large labor-supply shock from AI
- Implication for market structure: as top models get better at monetizing compute, catching up gets progressively harder for laggards with less revenue to bid for scarce compute; this is theorized via the Alchian–Allen effect — once compute costs ~$20/H100-hour, using a weaker/less-efficient model becomes wasteful, pushing demand and pricing power toward the most efficient frontier models. Popular cheap/low-value AI use cases (e.g., short-form video generation) risk being priced out entirely
- Patel weighs this against the historical failure of scarcity predictions (the Simon–Ehrlich wager, where Paul Ehrlich's bet on rising commodity prices was popularly seen as disproven), but argues compute is a poor analogy to that commodities basket because compute supply is much less elastic — annual 3x compute-capacity growth is a product of Moore's Law (~1.4x), new fab construction (~1.2x, bottlenecked by EUV tool supply through at least 2030), and AI's growing share of leading-edge wafer allocation (~1.8x, which saturates by end of 2027 as AI goes from ~60% to ~86% of N3 capacity) — none of which can easily be pushed much higher
- Patel notes this is a description of a current transitional regime, not a permanent state: eventually compute should become cheap again as automation (robots) turns raw silica and copper into computers at close to input cost — but that's not yet fast enough to offset how much more valuable/useful AI is becoming each year
- He closes by noting the 10x revenue vs. 3x compute gap implies strong economies of scale in the AI model business (a model's training cost is a one-time investment amortized across all users, unlike retraining each new human worker from scratch), which he finds concerning from a power-concentration standpoint even as it's economically logical

## Overview

This is a short, explicitly time-boxed (~2 hour) written/video essay by Dwarkesh Patel rather than an interview. He starts from a puzzle: Anthropic's revenue is roughly 10x'ing year over year while lab compute capacity is only ~3x'ing — a trend that, if it continued, would put Anthropic near $1T in revenue by the end of next year. Patel works through the arithmetic of what would have to be true for that gap to be sustained (rising margins, rising compute prices, and/or a growing inference share of compute), concludes that rising compute prices are doing most of the work, and builds out the supply-side case for why compute is a fundamentally inelastic resource compared to typical commodities. The piece moves from lab-level unit economics to a broader theory of how smarter models change the economics of scarce compute (via the Alchian–Allen effect) and ends with reflections on economies of scale and power concentration in the AI industry.

## Implications

If Patel's framing holds, compute pricing over the next few years will increasingly reflect not the cost of manufacturing chips but the economic value the smartest available model can extract from a given chip — meaning frontier labs with the most capable models could sustain much higher margins and outbid less-capable competitors for scarce compute, entrenching a compounding advantage for leaders. Practitioners and investors should expect: continued upward pressure on GPU rental/spot prices well beyond what Moore's Law or fab capacity growth alone would predict; a shift away from cheap, low-value AI applications (e.g., commodity content generation) as compute gets re-priced toward its highest-value uses; and a market where using an inefficient or weaker model becomes economically irrational once compute costs rise enough to make efficiency the dominant factor (the Alchian–Allen dynamic). It also implies that the "inference revenue funds more training compute" flywheel is under strain, and that watching lab margins, spot vs. contracted compute price spreads (e.g., the Google/SpaceX deal), and wafer-allocation shares to AI are useful leading indicators for how this plays out.

## Notable Quotes

- "Anthropic revenue has 10xed year over year... For this trend to continue, Anthropic would have to make $1T in revenue by the end of next year." — Dwarkesh Patel
- "The point of inference revenue is to convince investors to give you more money to buy more compute to train bigger models." — Dwarkesh Patel
- "I want to emphasize the key conclusion here: as AI models become smarter, they'll better monetize the same amount of compute." — Dwarkesh Patel
- "If a true human-level software engineer that could run on an H100 equivalent, at current market rates for software engineers, that H100 should rent for over $250k a year. That's 15x today's spot prices." — Dwarkesh Patel
- "At $20 per H100-hour, it's going to be extremely costly and stupid to use a weaker, less efficient model, because it's going to burn more tokens running your expensive compute to get the same result." — Dwarkesh Patel
- "I'm guessing the Simon-Ehrlich basket of commodities is not the correct reference class for compute, because compute supply is much less elastic." — Dwarkesh Patel
- "I wish we didn't live in a world with such strong economies of scale of intelligence (because I'm worried about power concentration). But it seems we do." — Dwarkesh Patel

## People Mentioned

- Dwarkesh Patel — author/host, Dwarkesh Podcast
- Paul Ehrlich — economist/biologist referenced via the Simon–Ehrlich wager on commodity price scarcity

## Topics

AI compute economics, GPU pricing, Anthropic, inference vs. training compute, Moore's Law, semiconductor supply, Alchian–Allen effect, AI lab margins, compute scarcity, power concentration
