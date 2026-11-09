---
podcast: "No Priors: Artificial Intelligence | Technology | Startups"
episode: "The New Apex Predator: CrowdStrike CEO George Kurtz on Adversaries Armed With AI Agents"
published: 2026-10-22
duration: "45m27s"
audio_url: "https://traffic.megaphone.fm/PDP8573582914.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-10-27T22:14:37Z
model: "deepseek-v4-pro"
guid: "c69c028a-cdc7-11f1-a97a-77799e86d8ea"
---

# The New Apex Predator: CrowdStrike CEO George Kurtz on Adversaries Armed With AI Agents — No Priors: Artificial Intelligence | Technology | Startups

## TL;DR
CrowdStrike CEO George Kurtz argues that AI agents have become 'the new apex predator,' flattening the adversary pyramid so hacktivists now wield nation-state-level capabilities. He details how open-weight models (GLM, Qwen) evade frontier-model guardrails, driving attack speeds to a 29-minute average breakout time with a 27-second record, and explains CrowdStrike's response: internal red/blue models (Red Tempest, Blue Solano) and a digital twin harness that cuts remediation costs from $10 to $0.03. Kurtz also predicts open-weight models will become the 'Linux of AI,' dismisses air-gapping the internet, and urges CEOs to adopt smaller, faster, AI-native teams.

## Key points
- AI agents are the new apex predator in cyber offense: George Kurtz says the agent state has flattened the adversary pyramid, giving hacktivists the same level of capability that nation-states had a few years ago, leading to more vulnerabilities found and faster attacks.
- Attack speed has collapsed from days to seconds: average breakout time is now 29 minutes, with the fastest observed at 27 seconds; attacks are prompt-driven rather than malware-based, enabling unique signatures and rapid iteration.
- Open-weight models dominate offensive AI because frontier models like OpenAI and Anthropic have strong guardrails that refuse cybersecurity tasks; adversaries use GLM, Qwen, and other open-weight models, which can be run locally on systems like DGX boxes or future Mac Studios—Kurtz notes he can create ransomware kits with ease on his home DGX.
- CrowdStrike is building a defensive AI stack called Safe Mind, featuring internally trained red and blue models (Red Tempest and Blue Solano) post-trained on 15 years of CrowdStrike security data, including managed detection and response threat/vulnerability pairings, and leveraging NVIDIA Nemotron models.
- The company uses a digital twin of customer environments to simulate red versus blue model iterations, allowing the blue model to learn and stop the red model; this approach was inspired by Jensen Huang's insistence on digital twins for security.
- The harness around models creates a massive cost advantage: Kurtz claims a frontier cyber model costs about $10 for general remediation, while CrowdStrike's model costs about 3 cents, and the harness improves results from any model at lower cost.
- Shadow AI and agent identity are major enterprise risks: organizations often don't know where AI agents are running, with an average of 90 agents per employee; agents are given identities and access to compute, data, and networking, requiring instrumentation of the entire chain of events from agent birth.
- Open-weight models are gaining enterprise share due to cost: CIOs report frontier token costs got out of control, so they will use frontier models only for 'Formula 1' high-value tasks and open-weight for the rest; open-weight models are considered good enough even if three months behind, and one top-five financial firm said it couldn't absorb new model releases faster than every 10 days.
- Kurtz dismisses internet shutdown doomerism: he calls air-gapping everything 'nonsense,' arguing that agents don't beam themselves between computers—they just do what humans would do faster; no unique new attack class has been created, though there are more vulnerabilities and faster exploitation.
- Even with zero vulnerabilities, breaches will continue because many attackers are logging in rather than breaking in due to identity issues; the long tail of legacy software, like Microsoft's backward compatibility, means the problem won't be solved overnight.
- Advice for CEOs: adopt smaller teams and faster operating cadence, disrupt yourself internally, leverage your data moat, and focus on customer outcomes and ease of deployment; cybersecurity remains a great career field because it is ever-evolving and not solvable.

## Notable quotes
> "The new apex predator is the agent state, not the nation state, it's the agent state." — George Kurtz
> "It's not malware anymore, it's prompts." — George Kurtz
> "The only way to fight AI today is with better AI." — George Kurtz
> "The defender knows their own systems. The defender has the architectural diagram. The defender has the code." — George Kurtz
> "For the first time, security is the gas pedal, not the brake." — George Kurtz

## People mentioned
- George Kurtz
- Jensen Huang

## Topics
`cybersecurity` `ai-agents` `open-source-models` `adversarial-ai` `enterprise-security` `technology-trends`
