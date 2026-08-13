---
title: "Stacie Mintz – Turning Qualitative Fundamentals into Quantitative Factors (S7E33)"
show: "Flirting with Models"
date: "2026-08-03"
guid: "9f77c183-6628-4b52-92cb-35891275bc02"
transcript_source: "web"
---

## Key Points

- **Origins of PGIM's quant equity strategy**: The strategy launched in 1996 out of a challenge from PGIM's largest multi-asset client, whose active managers lacked consistent performance. At the time, academics were connecting behavioral biases (overconfidence, loss aversion, anchoring) to well-known market anomalies like the low-P/E and momentum effects. PGIM's team researched where these biases were most relevant and published "Behavioral Bias, Valuation and Active Management" in 1999. That core philosophy has persisted for three decades even as the strategy itself evolved.

- **Building an in-house risk model (1999)**: PGIM abandoned off-the-shelf (Barra-style) risk models after discovering the risk model was undoing their alpha insights — e.g., treating a deliberate value tilt as pure risk and unwinding it, leaving the portfolio imbalanced. Moving the risk model in-house gave PGIM control to tailor which risks to target, easier customization for clients, and let their risk model act more as a diversification engine than a "predict which stock is safe" model.

- **Surviving the 2007 Quant Quake**: When a quant hedge fund's unwind snowballed into a broader deleveraging event that exposed crowding across quant strategies, PGIM's differentiated, diversification-focused in-house risk model meant it fared better than peers relying on standardized off-the-shelf models, reinforcing the earlier decision to build in-house.

- **Post-GFC push to be a "different quant"**: The GFC, combined with the rise of cheap smart-beta ETFs, pushed PGIM to differentiate through alternative data and richer quality metrics beyond simple ROE. This led to their annual "Research Shark Tank" every December, where researchers of all seniority levels pitch and stress-test new ideas as a team.

- **Fundamental quant philosophy**: Every factor must have a solid theoretical rationale that's understandable and explainable — not just a signal that backtests well. Because model "space" is limited and any new factor takes weight from others, PGIM curates a small set of factors they can fully explain, aiming for models that are resilient and intuitive rather than maximal.

- **The financing factor**: An example fundamental-quant factor added after the GFC — companies that grow via organic cash flow tend to outperform peers that consistently tap external capital markets, because abundant external financing can tempt management into funding lower-quality, diminishing-return projects, while internally financed firms must be more selective.

- **Factor taxonomy — growth, linkages, quality, valuation**: PGIM organizes its model into four groups, applied dynamically based on a company's lifecycle stage. Faster-growing, younger companies are weighted more toward growth factors; mature, stable companies are weighted more toward valuation factors, since their return case rests on discounted existing earnings rather than future growth.

- **Linkages — an unusual factor category**: Linkages captures how shocks propagate between connected companies (e.g., customer-supplier chains, shared board members, industry ties) rather than evaluating a company in isolation. Effects can cascade multiple layers deep (suppliers of suppliers), giving an informational edge on how a shock to one company will ripple through others.

- **Excluding traditional momentum**: PGIM deliberately excludes price momentum because it conflates company-specific signal with noise (industry-wide moves, speculation). Instead they built "information momentum" — isolating price reactions specifically around identifiable company events — which is roughly as effective as price momentum over time but carries about half the crash/downside risk.

- **Signal orthogonality is scarce**: Despite 30 years of research, PGIM's regional models contain only about 25–40 meaningful factor "concepts," not hundreds, because each must carry a meaningful, economically grounded weight that can be attributed and understood — contrasted with models that might carry 400 signals of uncertain individual contribution.

- **Where alpha comes from today**: Mintz argues raw data access is no longer the main edge — data is now abundant (if expensive) and PGIM's scale within a large organization like PGIM helps there. The bigger edge now lies in disciplined data engineering decisions (normalization, time-weighting, handling missing data) and in dynamically weighting factors at the individual stock level based on company type.

- **Systematic growth/value weighting**: The weighting of growth vs. value factors per stock is fully systematic, not discretionary — determined by where a company's growth rate falls relative to the investment universe. This lets the model behave like a "growth manager" for high-growth names and a "value manager" for mature ones, aggregating into a balanced portfolio built from very different types of stocks.

- **Resilience to emergent shocks (COVID, meme stocks, AI)**: PGIM builds resilience through diversified alpha sources and adaptive weighting rather than trying to predict shocks. During market stress, portfolio-manager discretion is deliberately limited — the model should be trusted rather than overridden by human bias — though there are exceptions requiring experienced PM judgment, such as resisting the temptation to buy Russian equities purely because they looked statistically "cheap" after Russia's invasion of Ukraine.

- **COVID's effect on backtesting standards**: COVID compressed years of factor dynamics into weeks, changing how PGIM evaluates backtests — a strategy that performed strongly straight through 2020 is now treated as a red flag rather than a strength, since it suggests inconsistency with a fundamentally driven philosophy. PGIM has also had to accept shorter historical datasets (5–7 years vs. the traditional 20-year standard) for newer factors, often starting them at lower weights until confidence builds.

- **LLMs as "bazookas"**: Mintz describes large language models as tools of enormous power with real potential to "blow up what already works." PGIM's discipline is to start from a research concept and escalate tool complexity only as needed — not to apply LLMs broadly by default. Risks (hallucination, black-box behavior, look-ahead bias from model memorization) are managed by tightly controlling test data, using multiple models to cross-validate outputs, testing sensitivity to question phrasing, and always tying LLM-derived insights back to hard fundamentals like sales or earnings.

- **Turning qualitative signals into systematic factors**: Using NLP and LLMs on abundant text data (earnings transcripts, news), the hard part isn't data availability but reliably extracting comparable, "apples to apples" information across companies before converting it into a score. Board composition is one example — mapping how board members overlap across companies (a linkages-style network) shows that well-connected boards correlate with more resilience, particularly for struggling companies.

## Overview

Corey Hoffstein interviews Stacie Mintz, Managing Director and Head of Quantitative Equity at PGIM Quantitative Solutions, who has spent over three decades at the firm and witnessed every major inflection point in its quant equity effort. The conversation traces PGIM's quant equity strategy from its 1996 origins as a client-driven challenge to move beyond indexing, through the 1999 decision to build risk models in-house, surviving the August 2007 Quant Quake, and the post-GFC push to become a differentiated "fundamental quant" shop. Mintz explains PGIM's factor taxonomy (growth, linkages, quality, valuation), the deliberate exclusion of traditional price momentum in favor of a custom "information momentum" measure, and the rationale behind unconventional factors like financing behavior and board-network connectivity. The back half of the discussion turns to research philosophy under uncertainty: how COVID reshaped backtesting standards, how PGIM builds resilience into models for emergent shocks like COVID, meme-stock mania, and AI, and how the firm is cautiously integrating large language models into its research process — treating them as powerful but risky tools that must be tightly disciplined rather than broadly deployed.

## Implications

Mintz's account offers a template for quant shops navigating a crowded factor landscape: differentiation increasingly comes not from raw data access (now commoditized) but from disciplined feature engineering, explainable factor construction, and dynamic (company-specific) weighting schemes. Her framing of backtests — where "too good" performance through a shock period like COVID is a red flag rather than a virtue — is a useful heuristic for investors evaluating any systematic strategy's claimed track record. Her treatment of LLMs as "bazookas" needing tight process discipline (starting from concept rather than tool, cross-validating across models, tying outputs back to fundamentals) is a pragmatic risk-management framework relevant to any research team adopting generative AI. More broadly, the emphasis on limiting discretionary overrides during market shocks, while still preserving room for rare, experience-driven interventions, speaks to a broader debate in systematic investing about the right balance between model trust and human judgment.

## Notable Quotes

- "We had built an alpha model we had a lot of confidence in. We realized that your portfolio is really the combination of obviously your alpha model and your risk model." — Stacie Mintz
- "We find that our measure, we call it information momentum, is as effective over time as price momentum, but it has a lot less of the crash risk, so about half the downside." — Stacie Mintz
- "Having a meaningful weight being tied to an economic underpinning — for us, the number is more in the forties as opposed to the four hundreds." — Stacie Mintz
- "The scariest thing is a back test that does well every year back in time because that can't possibly be." — Stacie Mintz
- "Don't go in there and kinda throw the baby off with the bathwater or do something with your human biases to mess this up." — Stacie Mintz
- "Our models were like, wow, Russia's really cheap." — Stacie Mintz
- "You've called large language models bazookas, tools of enormous capability with a real chance of blowing up what already works." — Corey Hoffstein
- "What we're trying to do is find companies that are just good companies that anybody would want to invest in." — Stacie Mintz

## People Mentioned

- Stacie Mintz — Managing Director and Head of Quantitative Equity, PGIM Quantitative Solutions (guest)
- Corey Hoffstein — CEO and CIO of Newfound Research, co-founder of Return Stacked Portfolio Solutions (host)

## Topics

quantitative equity, fundamental quant, risk models, factor investing, momentum, behavioral finance, factor taxonomy, large language models, backtesting, board composition, quant quake of 2007, PGIM
