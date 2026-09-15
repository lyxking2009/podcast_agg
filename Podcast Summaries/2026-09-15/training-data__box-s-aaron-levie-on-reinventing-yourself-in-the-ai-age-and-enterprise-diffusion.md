---
podcast: "Training Data"
episode: "Box's Aaron Levie: On Reinventing Yourself in the AI Age and Enterprise Diffusion"
published: 2026-09-15
duration: "1h05m21s"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/CPUAI7737498941.mp3"
episode_url: "https://www.youtube.com/watch?v=NE4CLThMPGU"
transcript_source: web
generated_at: 2026-09-15T22:17:12Z
model: "deepseek-v4-pro"
guid: "d2e9aa74-b080-11f1-8007-f31e2b6b4e53"
---

# Box's Aaron Levie: On Reinventing Yourself in the AI Age and Enterprise Diffusion — Training Data

## TL;DR

Aaron Levy, founder and CEO of Box, argues that the application layer—not just model providers—is where massive value will accrue in AI, as enterprises need deep workflow integration and data access that generic models can't provide. He details Box's pivot into AI agents that read and act on hundreds of billions of files, enabling use cases like contract analysis and automated onboarding. Levy also discusses why AI diffusion beyond coding will be slower, the importance of systems of record going headless, and advises founders to become 'wired in' and focus on enterprise distribution.

## Key points

- Levy is bullish on application-layer companies ('Neolabs'), arguing there is a large gap between model capability and enterprise workflow that requires bridging, creating multiple layers of startup opportunity.
- Model providers are temporarily subsidizing inference, but gross margin realities and non-economic actors like Meta, SpaceX, China, and Nvidia will drive token costs down, benefiting the application layer.
- Box started as secure cloud storage and sharing, pivoted to enterprise in the early 2010s, and now holds hundreds of billions of files; it is reinventing itself around AI by building agents that extract metadata, answer questions, and automate workflows from unstructured content.
- Hero use cases for Box AI include reading millions of contracts or research documents to extract structured data and deploying long-running background agents for processes like bank onboarding, which previously took weeks and now can happen in hours.
- Box built an 'agentic harness' that leverages deep knowledge of its file system, permissions, and search behavior to achieve meaningfully better accuracy and latency than simply giving an LLM API access to data.
- Box evaluates models using two sets of evals: a public 'complex work eval' across verticals and an internal holdback eval based on Box employee usage; they track incremental model improvements closely.
- Model performance on Box's use cases closely correlates with coding benchmarks, with one exception: Gemini is disproportionately better at some knowledge work tasks due to tool use.
- Customers primarily use Box's default model, but for high-volume workflow agents they run their own evals to select models based on cost and accuracy; open-weight models are seeing adoption but remain early, with issues like token inefficiency and random language switching.
- Box Labs focuses on improving agent accuracy through applied research, including hill climbing on accuracy and building maps of problem sets to go from 70% to 97% accuracy on document extraction.
- Levy argues that systems of record must both build excellent first-party agents and go headless via APIs/MCP to be used by external agents; he cites Salesforce's Agentforce as a smart move and notes he uses Salesforce more via MCP.
- He predicts that in five years, 90% of enterprise tokens will be from background agents that users didn't explicitly initiate, with UIs shifting from chat to dashboards, workflows, and task queues.
- Diffusion of AI into non-coding knowledge work is much slower because unlike code (pure text, high value per line, technical users), most knowledge work is constrained by external factors, access controls, data fragmentation, and change management.
- Coding became the fastest diffusion because code is text, models are heavily trained on it, labs evaluate on coding, and users can fix issues; legal is closer to coding, while sales is farther away.
- Box benefits from having all unstructured data in Box, good data hygiene, and AI FTEs; they run internal token leaderboards to identify best practices and spread them across teams.
- Levy advises founders to get 'wired in' via a curated Twitter feed, and to focus on distribution because AI makes building faster; companies that can get to enterprise customers will win the applied layer.

## Notable quotes

> "You know, I'm pretty long obviously the application layer like I'm also equally very biased like I'm I'm I have a very concentrated bet u with very limited diversification on uh on it working out that you still want to buy technology that that sort of understands your workflow and can get to the the core enterprise data." — Aaron Levy
> "And honestly, if I were one of the, you know, two or three biggest labs, I think I'd prefer this outcome too because back to your antitrust point, like at some point you'll just be nationalized if you're the only thing that that is is sort of exists as intelligence. So, you kind of want a little bit of healthy competition in this ecosystem anyway." — Aaron Levy
> "Most of the enterprise uh is sort of made up of these processes and workflows that are kind of just happening behind the scenes. Sometimes they're happening with computers and computers are running these things or sometimes they're happening with other people that are doing these things or sometimes they should be happening with people but you could never afford to have them happen with people so they just didn't happen." — Aaron Levy
> "In coding, you get access to basically most of the stuff ever relevant for your job. In knowledge work, you're like you're like, 'Hey, Sally, can you open up that that sort of file share for me? Can you open up that that, you know, sort of project because I didn't get access to it. How do you make sure the agent has access to those set of things?' All of that work has to get done." — Aaron Levy

## People mentioned

- Aaron Levy
- Doug Leone
- Stan Druckenmiller
- Matthew McConaughey
- Jesse

## Topics

`enterprise-ai` `ai-agents` `application-layer` `content-management` `technology-adoption` `founder-advice`
