---
show: "Latent Space: The AI Engineer Podcast"
episode: "Railway: The Agent-Native Cloud — Jake Cooper"
date: 2026-05-20
guid: "substack:post:198575235"
transcript_source: "substack"
backfilled: true
---

# Railway: The Agent-Native Cloud — Jake Cooper

## Key Points

- Railway: 3 million users, 100,000 weekly signups, 35-person team, $124 million raised.
- Owns bare metal data centers (Railway Metal) with ~70% margins and a 3-month payback period vs. renting cloud; approaching near-100% own-metal operations.
- RAM price appreciation caused Railway's servers to appreciate past total capital raised — a structural hardware economics win.
- Burst capacity maintained across five clouds (AWS, GCP, Oracle, and others); workloads are compacted off cloud the moment own-metal space or power is available.
- Core philosophy: activation energy to ship to production should be near zero; "software should be shipped as fast as it is written."
- Agents need: version control and branching, observability (logs/traces/metrics), feature flags for progressive rollouts, orchestration at 1,000x human scale, and safe production forking via copy-on-write environments.
- Content-addressable filesystems in development: lazy-loaded, capable of paging arbitrary snapshots into memory — could eliminate Dockerfiles.
- "The pull request is dying" — feature flags and progressive rollouts will replace PR-based workflows as agent-driven development scales.
- Central Station: internal incident/feedback aggregation system using dynamic clustering for context-based routing to engineering teams.
- Temporal workflow engine acknowledged as powerful but operationally taxing at scale; Railway exploring alternatives.
- Cooper personally spends ~$25K/month on AI coding tools; team budget is $200K–$300K/month — treated as core infrastructure, not discretionary.
- Historical near-death: lost $500K/month during free-tier era on a $20M bank account, forcing rapid business model rethink.

## Overview

Jake Cooper, solo founder of Railway, joins Latent Space to explain how his 35-person company is rebuilding cloud infrastructure from the ground up for an agent-native world. Railway has reached 3 million users with 100,000 weekly signups and has raised $124 million — yet operates with a radically lean team by owning its own bare metal data centers (Railway Metal) that deliver ~70% margins and a 3-month cloud payback period. Cooper's founding thesis, forged through stints at Bloomberg and Uber, is simple: "the activation energy to ship something to production should be near zero."

The episode digs into Railway's infrastructure philosophy: rather than mimicking hyperscaler patterns, Railway focuses on four primitives — network, compute, storage, and orchestration — and builds upward from owned metal. A key economic insight is that RAM price appreciation caused Railway's servers to appreciate in value, ultimately exceeding total capital raised. The company maintains burst capacity across five clouds (AWS, GCP, Oracle, and others) but is rapidly migrating toward near-100% own-metal operations. Cooper also walks through Central Station, an internal system that aggregates customer feedback and incidents into dynamic clusters for context-based routing to engineering teams.

The deeper strategic bet is that agents will consume infrastructure at 1,000x human developer scale, making traditional PR-based software delivery workflows obsolete. Railway is building toward content-addressable, lazy-loaded filesystems that can page arbitrary snapshots into memory — potentially eliminating Dockerfiles entirely — along with progressive feature flag rollouts that replace the pull request as the primary unit of software delivery. Cooper spends ~$25,000/month personally on AI-assisted coding and allocates $200K–$300K/month across his engineering team, treating AI tooling as core infrastructure spend.

## Implications

Railway's episode is a masterclass in infrastructure economics and the organizational design needed to build agent-native cloud. For AI engineers, the key takeaway is that hyperscaler dependency is a strategic liability at agent scale: the per-unit economics of owned metal are dramatically better once workloads are predictable enough to warrant capital expenditure, and the margin reinvested into burst cloud capacity gives flexibility without lock-in. The content-addressable filesystem direction — lazy-loading snapshots rather than building from Dockerfiles — points toward a future where environment reproducibility is a platform primitive rather than a developer responsibility. Feature flags replacing pull requests as the primary deployment unit is the most provocative claim: it implies that the entire code review and merge workflow, designed around human reading speed, will need to be rebuilt for agent commit velocity. Engineers building deployment infrastructure or internal developer platforms should treat this as a near-term design constraint, not a distant hypothesis.

## Notable Quotes

- "If you're writing code by hand, you're doing this wrong." — Jake Cooper, on the capability of current AI coding tools
- "The pull request is dying." — Jake Cooper, on how agent-driven development changes code review workflows
- "Cattle, not pets... if you have a cloning machine." — Jake Cooper, reframing infrastructure management when snapshots enable perfect reproduction
- "Software should be shipped as fast as it is written." — Jake Cooper, core Railway thesis
- "The activation energy to ship something to production should be near zero." — Jake Cooper

## People Mentioned

- Jake Cooper — founder and CEO, Railway
- Swyx — host, Latent Space podcast

## Topics

- Agent-native cloud infrastructure
- Bare metal economics vs. hyperscaler renting
- Content-addressable lazy-loaded filesystems
- Feature flags replacing pull requests
- Temporal workflow engine trade-offs
- Railway Metal (owned data centers)
- Multi-cloud burst strategy
- Central Station incident/feedback aggregation
- Solo founder organizational design
- AI coding tool spend as infrastructure budget
- Copy-on-write production environment forking
- SDLC evolution under agent-driven development
- Hardware economics (RAM appreciation)
