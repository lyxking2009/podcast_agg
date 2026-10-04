---
podcast: "Lenny’s Podcast: Product | Career | Growth"
episode: "OpenAI’s Head of ChatGPT: We’re entering a new era of AI (again) | Tibo Sottiaux"
published: 2026-10-04
duration: 37m22s
audio_url: "https://pscrb.fm/rss/p/api.substack.com/feed/podcast/217274136/13e3f49711c936b294b6ef27d4160818.mp3"
episode_url: "https://www.youtube.com/watch?v=MM-C3JqCXBk"
transcript_source: web
generated_at: 2026-10-04T22:40:00Z
model: "deepseek-v4-flash"
guid: "substack:post:217274136"
---

# OpenAI’s Head of ChatGPT: We’re entering a new era of AI (again) | Tibo Sottiaux — Lenny’s Podcast

## TL;DR
Lenny Rachitsky interviews Tibo Sottiaux, who leads ChatGPT and Codex at OpenAI, recorded at DevDay hours after his team shipped more than 20 products. Sottiaux’s thesis: the coming shift is not a better chatbot but a persistent, always-on agent (“Dots”) that lives on OpenAI’s infrastructure and reaches you across every device — and that will eventually absorb both ChatGPT and Codex. He argues most actions on the internet will soon be taken by agents, that OpenAI is opening a plugin ecosystem with revenue sharing for third-party builders, that the “loops and graphs” era of agent design is a passing phase, and that OpenAI is deliberately sitting on capability it has already built.

## Key points
- **Dots is the bet that one agent replaces the model picker.** A launched dot has no model picker and almost no settings — the only thing a user configures is which channels reach them. Sottiaux expects users to eventually run many dots as a virtual team with assigned roles (he runs one dedicated to monitoring Twitter), but OpenAI deliberately started with one so it can learn usage patterns first. Dots and ChatGPT will converge: all Dots capabilities are meant to ship into ChatGPT to “lift the floor” for its 1.2 billion users.
- **Architecture: the harness is not on your machine.** A dot lives on OpenAI’s infrastructure and connects outward to as many devices as you want — a laptop, a Mac mini, potentially ten devices — which Sottiaux compares to an octopus. “Specialist” dots are the same thing with added guardrails, monitoring, and dedicated hardware.
- **OpenAI trained a stronger model and chose not to release it.** Sottiaux says OpenAI built a next step up in capability beyond “Astra” and held it back, releasing instead a near-Astra model that is much more efficient. He frames safety operationally: “pacing the frontier” means investing ahead on alignment, security, and guardrails; the concrete mechanism is secondary-monitoring compute that watches primary agents, detects high-risk actions, and intervenes on suspected prompt injection. He says the majority of OpenAI’s API-stack investment goes to the safety stack, and that building for 1.2 billion users (including non-technical people) means “we don’t gamble there.”
- **The underrated bet is the ecosystem, not a model.** OpenAI is opening sign-in with ChatGPT (by his count ~16 partners, starting informally with Pi and OpenCode) and plugin extensions inside ChatGPT — effectively letting anyone ship to 1.2 billion users. He confirms revenue sharing for popular, high-usage plugins: when a ChatGPT subscriber spends included usage inside a partner plugin (e.g. Notion, Figma), that company is remunerated. Discovery is driven by retention, extension numbers, and quality — not keywords or write-ups.
- **Agents will take most internet actions.** His three “not priced in” claims: agents will perform the majority of internet actions, models will keep getting cheaper and faster at “incredible” rates, and all modalities will integrate seamlessly. Second-order effect: infrastructure strain — after Notion exposed an MCP, agent traffic arrived in volume and forced Notion to rework its economics.
- **“Loops and graphs” are a passing phase.** Sottiaux says hand-wiring loops, graphs, and fine-tuning agent workflows is not where this goes; the alternative is a system that learns your goals and preferences and absorbs feedback so you never think about the wiring. He concedes the Dots launch is imperfect and that he will learn from opening it to all Pro users.
- **Agent teams expand and shrink.** His own workflow oscillates: when pushing the frontier he builds larger teams of agents; when a new model breakthrough lands, one bigger agent can hold everything in memory and learn, so he shrinks back. He says he still merges code occasionally and that weekends of hand-coding are therapeutic; most of his code is written for him by Codex for analysis.
- **Speed restored flow state — and reshapes hiring.** He says ultra-fast inference is now achievable at roughly Astra-level cost, and that speed and voice control bring a creative state distinct from hand-coding. Typing fast is trending down as a skill; taste, thinking about the user, and connecting to the audience are trending up. OpenAI has more than 120 former YC founders on staff. His example of the “next Tibo” is Ahmed Ibrahim, who joined as a new grad and now owns OpenAI’s compute fleet and applied work and built much of the Codex harness.
- **Autonomy and the cost of mistakes.** He describes a bottoms-up culture (the Decisions API started with four people hacking over a weekend) and a real reset button; he says he took production down on day three at OpenAI and was still employed afterward. He also flags loneliness and context-switching among engineers who talk to agents all day, and proposes reducing configuration fatigue.
- **What he changed his mind about / what annoys him.** He expected today’s capability (Astra specifically) to arrive a year or two later; he did not expect to rely so much on voice; and he revised his view on hiring after seeing how fast younger generations absorb the tech. What annoys him most is the model picker itself — reasoning-effort and multi-agent-versus-ultra choices — which he wants to remove as fast as possible so the app “almost completely disappears.”
- **The concrete demo story.** A dot that understood Dev Day, a production system, and a live demo were connected and pinged him five minutes before the demo to say production was down; he declined its offer to fix it.

## Notable quotes
> "“We haven’t yet released the next step up in capability beyond Astra. We have released a level that is similar near Astra intelligence, but much more efficient.”" — Tibo Sottiaux

> "“the sleeper hit is ecosystem, so opening it all up, and the commitment, the deep commitment we have towards opening it all up.”" — Tibo Sottiaux

> "“Build a good plugin. ... we look at retention extension numbers, we look at how successful the plugin, the quality, and then that’s what we then start to recommend to users in conversations.”" — Tibo Sottiaux

> "“I myself even get fatigued with the model picker and the reasoning efforts and whether to use multi-agent or ultra or what it even does.”" — Tibo Sottiaux

> "“We may not have coders anymore, but we have more builders than ever. And I think there’s something that’s going to remain deeply human about that.”" — Tibo Sottiaux

## People mentioned
- Tibo Sottiaux — Head of ChatGPT (and Codex) at OpenAI
- Lenny Rachitsky — host, Lenny’s Podcast
- Ahmed Ibrahim — OpenAI engineer (new-grad hire; runs compute fleet, built much of the Codex harness)
- Sam Altman — OpenAI CEO (referenced)
