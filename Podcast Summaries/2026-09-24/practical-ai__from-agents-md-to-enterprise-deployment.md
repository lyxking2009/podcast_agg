---
podcast: "Practical AI"
episode: "From AGENTS.md to Enterprise Deployment"
published: 2026-09-24
duration: "48m52s"
audio_url: "https://pscrb.fm/rss/p/dts.podtrac.com/redirect.mp3/media.transistor.fm/74934e48/4c629c07.mp3"
episode_url: "https://share.transistor.fm/s/74934e48"
transcript_source: rss_vtt
generated_at: "2026-09-24T23:30:00Z"
model: "deepseek-v4-flash (Hermes manual fallback)"
guid: "2dbddf03-71ae-48d0-b902-a03f383621ac"
---

# From AGENTS.md to Enterprise Deployment — Practical AI

## TL;DR

Nick from Broadcom Tanzu/VMware joins Daniel Whitenack and Chris Benson to argue that enterprise agents are just apps, and should be shipped with the same platform discipline. The core argument is that cloud-native plumbing (buildpacks, a certified path to production, MCP gateways) already solves most of what people think is new about agents, while the genuinely new problem is state: agent harnesses were built around local files and MD-file memory, which evaporates in ephemeral cloud infrastructure. Enterprises also want the intelligence co-located with apps and data, which is a networking and reliability argument, not a philosophical one.

## Key points

- Defining an enterprise environment: assume no or limited internet access, plus heavy regulation (PCI, SOX, HIPAA, FIPS and more) where a mistake has dramatic consequences. Nick says some customers run true air gaps and physically carry media into the data centre. He spent fourteen years inside a large enterprise before five years at VMware/Broadcom, and says vendors routinely assume internet access and are wrong.
- The pre-agent answer was platform-as-a-service: Cloud Foundry (originating around 2011 out of VMware, predating Kubernetes and Docker) — hence Tanzu Platform — where a developer pushes code and the platform builds the container, handles ingress, certificates, load balancers, health monitoring and sandboxing between apps. A few constructs — push, bind, scale — are the whole path to production.
- Why that matters: enterprises do not want a hundred snowflakes. They want one auditable, repeatable, already-certified pattern so developers spend their time on business logic instead of infrastructure, and do not have to re-certify for every sub-team. Once the platform is certified against their standards, adoption is effectively unlocked.
- The core claim is that agents and apps should be treated the same way. Nick's analogy: just as you could never leave an app running on a laptop, 'I've got all these agents running on my laptop — if I shut my lid, everything stops.' Enterprises want agents running 24/7/365.
- Where the app analogy breaks: 12-factor apps cleanly decouple storage and state and can scale to a thousand instances. Traditional agent harnesses were built assuming file-system access and MD files as memory. In a cloud world where instances are ephemeral, that memory goes off into the ether — persisting agent state is the single biggest challenge in moving the deployment methodology to agents.
- The latency argument for co-location: enterprises want agents and LLMs close to the apps, data and users that consume them. If the model sits 30 network hops from the microservices calling it, physics adds latency that becomes problematic at scale. Nick says this surprised him — customers explicitly want to collate the intelligence with the app and the data.
- Reliability: SaaS providers often do not meet the uptime expectations of traditional enterprises. Nick illustrates the stakes from his first job out of college, maintaining a grocery retailer's Unix/C warehouse inventory system that was older than he was — if it went down, within five minutes trucks were backing up on the interstate.
- The Tanzu Agent Build Pack extends the buildpack concept to agents: instead of machine-readable compiled code you push a human-readable AGENTS.md describing what the agent does, and it starts up in about a minute and scales like anything else on the platform.
- Air-gapped customers who bought GPUs a few years ago can run local models on-platform and tie the agent to them; others register an approved LLM from an existing cloud or vendor contract. Along the way MCP (Model Context Protocol) provides the tool surface.
- MCP servers are treated as first-class deployables: at Broadcom the only approved way for developers to use MCP is to deploy servers on Tanzu Platform, where they are secured and maintained, with an MCP gateway service registering and protecting them and extending secure access to the agent's runtime.
- The result is an out-of-the-box agent with a chat experience and secure access to a battle-tested runtime and an approved LLM — enterprise-grade plumbing rather than a novel agent framework.

## Notable quotes

> > "Where you could potentially do something that has dramatic consequences. And you have a highly controlled, highly regulated environment." — Nick, on what makes enterprise enterprise
> > "I've got all these agents running on my laptop. If I shut my lid, everything stops. I don't really want that to happen." — Nick
> > "If that thing went down, within five minutes the warehouse was backing up trucks on the interstate." — Nick, on enterprise scale
> > "We wanna get the LLMs and the agents closer to us and our actual apps using them and the people using them as well." — Nick
> > "It's just kind of like an unlock for these enterprises because it's so easy and so simple to use, and they don't have to recertify everything." — Nick, on a certified platform path

## People mentioned

- Nick — guest; Broadcom Tanzu/VMware, co-host of Cloud Foundry Weekly (surname not given in episode)
- Daniel Whitenack — Practical AI co-host, CEO at Prediction Guard
- Chris Benson — Practical AI co-host, principal AI and autonomy research engineer

## Topics

`enterprise-ai` `agents` `agents-md` `mcp` `platform-engineering` `cloud-foundry` `air-gapped-systems` `compliance`
