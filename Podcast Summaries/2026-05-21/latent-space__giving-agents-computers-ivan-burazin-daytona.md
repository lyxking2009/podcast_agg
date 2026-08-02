---
show: "Latent Space: The AI Engineer Podcast"
episode: "Giving Agents Computers — Ivan Burazin, Daytona"
date: 2026-05-21
guid: "substack:post:198688585"
transcript_source: "substack"
backfilled: true
---

# Giving Agents Computers — Ivan Burazin, Daytona

## Key Points

- Daytona pivoted from CodeAnywhere (human browser IDE) to AI agent sandboxes in January 2024, driven by unexpected API demand from an NYE 2023 MVP.
- Bare metal custom scheduler achieves 60ms sandbox startup and 50,000 concurrent sandboxes in ~75 seconds — no Kubernetes or VMs.
- 850,000 daily sandbox runs at peak; 74% month-over-month platform growth with a 25-person team (many together 7+ years).
- RL/eval workloads now constitute ~50% of usage (up from 0% in months), creating extreme spike patterns (0 to 100,000 CPUs) that challenge traditional infrastructure design.
- Mean platform utilization is only 15% due to spikiness; peak reaches 90% — this is a structural feature of agent workloads, not a bug.
- Composable stateful sandboxes: dynamic resource resizing (prevents OOM kills), pause/resume snapshots, and CLI access for richer agent interaction than MCP.
- Windows sandbox support targets the ~$25 trillion US white-collar work market locked in legacy enterprise applications.
- Open source strategy uses AGPL 3.0 licensing; vendor procurement time reduced to 5 days vs. typical 2–3 months.
- ~1,000 Slack Connect customer channels; primary competitive differentiator reported by customers is team responsiveness.
- Burazin critiques token-reselling SaaS premium as misplaced — actual value accrues to consumption-based API pricing models.
- GitHub and CI/CD pipelines are becoming bottlenecks as agent-generated PRs overwhelm review processes built for human throughput.

## Overview

Swyx interviews Ivan Burazin, CEO of Daytona, about why AI agents require fundamentally different computing infrastructure than human developers. Daytona evolved from CodeAnywhere — an early browser-based IDE Burazin founded over a decade ago — and pivoted in January 2024 to building AI agent sandboxes after an MVP built on New Year's Eve 2023 generated immediate demand for API access. The episode is framed around Burazin's long-held thesis that "the end of localhost" is inevitable, and that autonomous agents are its final catalyst.

Daytona runs on bare metal with a custom scheduler (not Kubernetes or VMs), achieving 60-millisecond sandbox startup times and the ability to spin up 50,000 concurrent sandboxes in roughly 75 seconds. The platform serves ~850,000 daily sandbox runs for its largest customer. A striking structural insight is that RL/eval workloads — which create near-zero-to-100,000-CPU spikes — have grown from 0% to ~50% of platform usage in just a few months, driving unique infrastructure challenges around mean utilization (15%) vs. peak (90%).

The business is growing at 74% month-over-month with a 25-person team, many of whom have worked together for 7+ years. Daytona differentiates from disposable-code-box competitors through composable, stateful sandboxes: dynamic resource resizing, pause/resume snapshots, multi-OS support (including Windows for legacy enterprise systems), and CLI access that provides richer agent capability than MCP protocols. Burazin positions the addressable market at ~$10 trillion annually if 40% of knowledge worker tasks become agent-automated.

## Implications

For AI engineers and infrastructure builders, Daytona's trajectory illustrates that agent compute is a distinct category from both human developer tooling and traditional cloud VMs. The workload shape — massive spikes from RL/eval jobs, "follow the sun" background agent patterns, and the need for sub-100ms cold starts — demands custom schedulers and bare metal ownership rather than layering on existing cloud primitives. The rapid rise of RL/eval workloads to 50% of usage in months signals that model improvement pipelines are becoming a first-class cloud workload category. Engineers building agent platforms should plan for 10–100x utilization variance and invest in stateful snapshot infrastructure rather than treating every sandbox as ephemeral. The Windows support angle is underappreciated: a large portion of enterprise automation value is locked behind GUI-based legacy systems, and agents that can operate those environments have a significant moat.

## Notable Quotes

- "Agents don't care about laptops or setups — they need stateful computers with instant startup, dynamic resources, and safety isolation." — Ivan Burazin
- "If 40% of knowledge worker tasks are automated by agents, that represents roughly a $10 trillion addressable market annually." — Ivan Burazin
- "Legacy enterprise applications live in Windows systems representing $25 trillion in US white-collar work annually." — Ivan Burazin
- "The market is adding premium to SaaS vendors reselling tokens — incorrectly. Actual acceleration comes from consumption-based API pricing, not seat licensing." — Ivan Burazin

## People Mentioned

- Ivan Burazin — CEO and co-founder, Daytona (formerly CodeAnywhere)
- Swyx — host, Latent Space podcast

## Topics

- Agent compute infrastructure
- Bare metal sandboxing
- RL/eval workload spikes
- Stateful sandbox snapshots
- Windows/macOS computer use for legacy enterprise
- Custom scheduler design (non-Kubernetes)
- Open source strategy (AGPL 3.0)
- Consumption-based vs. seat-based SaaS pricing
- CI/CD bottlenecks from agent-generated PRs
- Cloud economics and utilization efficiency
- GPU sandboxes
- Founder culture and long-tenured teams
