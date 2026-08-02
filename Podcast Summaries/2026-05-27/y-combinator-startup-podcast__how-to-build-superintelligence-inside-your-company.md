---
guid: 6ce77659-6fdc-4ec0-a32e-0d287714af91
title: "How To Build Superintelligence Inside Your Company"
show: "Y Combinator Startup Podcast"
date: 2026-05-27
duration_sec: 2789
transcript_source: youtube_captions (B246K_G7mHU)
episode_url: https://podcasters.spotify.com/pod/show/ycombinator/episodes/How-To-Build-Superintelligence-Inside-Your-Company-e3jur1o
---

# How To Build Superintelligence Inside Your Company

## Key Points
- YC general partner Pete Koomen (creator of Optimizely) built YC's internal AI agent infrastructure from scratch over the past year, now comprising 350+ tools used across the organization
- The single biggest unlock was giving agents read-only SQL access to YC's production database — because all YC data lives on one Postgres instance, agents could suddenly answer arbitrary organizational questions in natural language
- YC's internal harness is a multiplayer/organizational layer built on top of solo tools like Claude Code; the core primitives are an agent loop, a shared tool registry, and a model router
- Self-improving skill loops run nightly: an agent reads all previous day's conversation transcripts, identifies gaps, and improves the skill prompts autonomously — YC calls this a "dream cycle"
- The two-sentence pitch skill exemplifies organizational superintelligence: a human-authored skill was improved by feeding it real office hours transcripts, and the resulting skill now outperforms any individual partner at writing YC-style two-sentence descriptions
- Default transparency matters: all agent conversations at YC are broadcast to an internal Slack channel, which simultaneously teaches other employees how to use agents, acts as a social control on privacy, and builds organizational trust
- The "horseless carriage" critique: most AI-as-a-feature products hide prompt context from users and encode developer assumptions, perpetuating the old paradigm of deterministic software wrapping AI rather than agents wrapping deterministic tools
- Chat is vindicated as the right interface because it most closely mirrors human thought expression; context-constraining UIs sacrifice the flexibility that makes agents powerful
- Building "organizational superintelligence" requires egalitarianism and default trust — both properties most organizations lack; startups have a one-time window to leapfrog incumbents before costs normalize
- The AI centralization vs. decentralization fork: the "1984 scenario" (five AI kings controlling all compute and prompts) vs. the "personal computer moment" (individual ownership of agents, models, and prompts)

## Overview
Gary Tan hosts Pete Koomen on the Lightcone for a detailed account of how YC transformed itself into an AI-native organization. The origin story is grounded: Koomen and a few engineers started building internal tooling for the YC finance team about a year ago. The original insight was that the painful human-in-the-loop cycle — finance team explains workflows to engineers, engineers code deterministic software, repeat — could be replaced by giving non-technical users agent tools that encode workflows in English rather than Ruby. The "magic moment" arrived when Jared gave agents read-only SQL access to YC's single Postgres database, suddenly enabling arbitrary organizational queries in natural language — a direct application of Jevons' paradox (removing friction dramatically increases the number of questions asked, not just the ease of answering them).

The episode then traces the evolution from 20 initial tools to 350+, the introduction of self-improving "dream cycle" agents, and the two-sentence pitch skill as a case study in organizational knowledge codification. The broader framework: building superintelligence inside a company is not technically complex — it is a compositional process of encoding every organizational function as a skill, letting agents run those skills, and feeding transcripts back into the prompts to improve them automatically. The barriers are cultural (default secrecy, command-and-control structures) not technical.

The final third of the episode covers the "horseless carriage" essay Koomen wrote critiquing AI-as-feature products, the case for chat as the permanent interface, Gary's experience building Gary's List 2.0 (a 500K-line Rails app now being rebuilt as minimal TypeScript + Claude Code), and the civilizational choice between centralized AI control and personal/open AI ownership.

## Implications
Organizations willing to invest $100K–$1M/year in tokens today, build a unified internal data layer, and default to transparency can effectively "live in 2028" — gaining a multi-year operational lead over incumbents before these costs normalize to a few hundred dollars. The window for this leapfrog is narrow. Startups have a structural advantage because they lack the siloed data and permission culture that blocks large enterprises from giving agents the unrestricted context access that makes them powerful.

The multiplayer agent problem is explicitly unsolved: all popular harnesses (Claude Code, Codex, OpenClaw, Hermes) are single-user. YC's internal infrastructure represents one of the few working examples of a team-level and org-level agent deployment. The primitives YC discovered — unified data context, shared tool registry, dream cycle self-improvement, transparent conversation broadcasting — are likely to become standard infrastructure for AI-native companies within 2–3 years.

## Notable Quotes
> "Part of the key thing is not to just use AI as a copilot. This is the thing where you use it as the building layer for everything. And you need to start recording all the artifacts." — Pete Koomen

> "It's like a shared organizational brain. It's like the closest thing to us being able to connect our brains." — Gary Tan

> "If you frame this as a way for everyone in an organization to get better at what they do using the collective skill and instinct of the people they work with, it's incredibly powerful." — Pete Koomen

> "Removing the amount of back-and-forth between different teams to get a thing done is an instance of Jevons' paradox — if I have to knock on the data science team's door and wait for their backlog, I'm going to ask far fewer questions." — Pete Koomen

> "This is how superintelligence happens inside organizations. You compose everything that you do — any given thing any given person can do — combine that in aggregate with this process, and you have a super organization. It's not more complicated than that." — Gary Tan

> "You have to be relatively egalitarian and you also have to be trust by default. Neither of those things are most organizations in the world." — Gary Tan

> "There's a one-time time warp where you can leapfrog every incumbent, all Fortune 500s, all startups that exist by doing this. What you spend $100K on now will cost $10K in a year and a couple hundred bucks the year after — and everyone will do it. So you can live in 2028 right now." — Gary Tan

> "Most software in the future is going to be very small and add the smallest amount of code ahead of time you need in order to let the model shine." — Pete Koomen

## People Mentioned
- Pete Koomen — YC general partner; creator of Optimizely; architect of YC's internal agent infrastructure
- Gary Tan — YC president; host of the Lightcone podcast; built Gary's List as a personal AI-native publishing project
- Jared — YC software engineer; built the foundational SQL query tool that was the first major agent unlock
- Tom (YC partner) — wrote the two-sentence description skill that was later improved via transcript feedback
- Andrej Karpathy — referenced for "auto-research" loops and knowledge LLM wiki concept (GBrain inspiration)
- Jack Dorsey — CEO of Block; mentioned for turning Block into a mini-AGI around payments
- Boris (Claude Code) — referenced for Claude Code's obsession with simplicity

## Topics
AI agents, internal tooling, organizational AI, agent harnesses, tool registry, dream cycles, skill loops, YC, Optimizely, Claude Code, multiplayer agents, agentic AI, horseless carriage, chat interface, just-in-time software, AI decentralization, personal AI, organizational learning, knowledge management, two-sentence pitch, Postgres, SQL agents, startup infrastructure
