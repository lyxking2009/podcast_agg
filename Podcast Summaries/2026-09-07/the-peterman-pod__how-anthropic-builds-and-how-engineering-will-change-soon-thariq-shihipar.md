---
podcast: "The Peterman Pod"
episode: "How Anthropic Builds And How Engineering Will Change Soon | Thariq Shihipar"
published: 2026-09-07
duration: 1h10m58s
audio_url: "https://anchor.fm/s/106346b90/podcast/play/125311415/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-8-7%2F431337002-44100-2-0c0e061f24695.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/peterman-pod/episodes/How-Anthropic-Builds-And-How-Engineering-Will-Change-Soon--Thariq-Shihipar-e3oemvn"
transcript_source: web
generated_at: 2026-09-07T22:14:52Z
model: "deepseek-v4-pro"
guid: "ce4c02c6-4ce2-4c8d-8383-c8580b6a38b4"
---

# How Anthropic Builds And How Engineering Will Change Soon | Thariq Shihipar — The Peterman Pod

## TL;DR

Thariq Shihipar from Anthropic shares how the company uses Claude as a thought partner and system builder, emphasizing that engineers should treat AI as a collaborator, invest in harnesses and loops, and focus on taste, verification, and technical judgment. He explains that model improvements make the harness more complex, that autonomous coding requires well-specified tasks and robust testing, and that sharing work externally increases luck surface area. The episode covers onboarding, knowledge work automation, computer use, code maintenance, and career advice for engineers in an AI-driven industry.

## Key points

- Onboarding at Anthropic: Treat Claude as a thought partner and provide full context; ask 'Can Claude do it? If not, why not?' and consider building systems that build systems rather than just products. Technical onboarding no longer requires an onboarding buddy; the buddy remains for social/cultural integration and buy-in.
- External vs internal AI perception gap: Anthropic engineers often no longer write code directly; Thariq says some enterprise customers haven't typed a line in six months. Anthropic culture views exploring autonomous capabilities as part of the job, even if a day spent trying to automate fails, because it reveals model limitations to improve; average engineers optimize for output and avoid risky automation investments.
- Knowledge work is reducible to code: Thariq uses Claude Code for accounting via Python instead of Excel and video editing via FFmpeg; most knowledge work can be broken into code-like steps and handled by coding agents, giving technical people a major advantage.
- Harness importance grows with model capability: The harness is as important as the model; as models improve, harness complexity increases to enable more autonomy. Examples include Auto mode, a classifier that handles permission prompts during hours-long runs, sandboxing, and Artifacts as a form of prompting to represent completed work.
- Autonomous code changes depend on specification quality: For a well-written ticket or spec, Claude can usually complete the task autonomously; the hard part is resolving ambiguity, discovering unknowns, and deciding what to build. A common failure mode is giving Claude a one-sentence task and iterating endlessly instead of clarifying intent upfront.
- New roles for interns and juniors: With Claude handling implementation glue, internships shift from writing React code to solving new types of problems like creating evals for AI coding behaviors, ensuring performance across millions of users, and proactively exploring novel work; spec writing and ambiguity resolution become more valuable.
- Computer use still faces edge cases: Models like Opus 5 are strong at computer use, but issues like password managers (e.g., 1Password) block autonomous typing. Computer use is slower because small models lack the needed knowledge, and browser state is harder to reverse than code state; APIs and MCPs increasingly replace literal screen clicking.
- Compute usage is more about capability exploration than output: Anthropic employees rarely hit usage limits because high compute is spent exploring model capabilities, testing math, or trying diverse approaches; individuals can replicate advanced workflows at home by raising abstraction level and setting up monitoring loops rather than burning tokens on narrow refactors.
- Loop engineering for software development: Loop engineering means setting up a system that prompts Claude repeatedly; examples include triaging feedback, implementing changes, and monitoring issues. Effective loops require good verification, well-designed skills, and appropriate data sources; at Anthropic, such loops significantly increase velocity.
- Next industry shift: Extending Claude Code across the entire SDLC (feedback, code review, CI/CD, incident management) and creating routines/loops is the next big change. Generative interfaces like Artifacts will become a primary way to interact with Claude, including mobile-readable reports and interactive diagrams.
- Model selection will fade: Currently Thariq uses the smartest model (Fable) for planning, brainstorming, and spec creation, and Claude Opus 5 for implementation and execution with verification harnesses. Within a year, he predicts users won't need to think much about which model to use; model-specific selection is not a durable skill.
- Prompting is more than the prompt: Effective prompting includes accumulated context, skills, data, and harness setup; model-specific quirks must be learned and unlearned, but the meta-skill of learning to work with new models is durable. Anthropic removed 80% of Claude Code's system prompt, including examples from tool descriptions, because models became more imaginative and examples were mostly negative.
- System prompt optimization is a full-time effort: Adjusting the system prompt involved removing lines, running internal/external evals, and monitoring user feedback; some behaviors (like Claude telling users to sleep) have no eval and must be caught through internal reports. The learnings were shared publicly to save others the iteration.
- Achieving tasteful output: For subjective work like frontend, give references in code (HTML, Figma) rather than screenshots; stay in the loop and develop domain taste. High-taste users like mathematicians (e.g., Lev reviewing the Riemann Hypothesis attempt) are essential to recognize good output; you can also ask Claude to help you develop taste by curating references.
- Writing norms at Anthropic: The rule is 'if I would be happy to show someone the prompt, I would send them the output.' Context gathering and data readouts are AI-generated and reviewed; pitches, essays, and tweets remain human-written because individual words carry intention. For large PRs, Thariq attaches an artifact of all prompts to show work.
- Code maintenance priorities shift: Naming and stylistic consistency become less important; verification harnesses, skills (like simplify), and massive testing code (100x more than before) are critical. Knowing the direction of the codebase (e.g., multiplayer vs single-player) guides what to emphasize, such as replay/undo for multiplayer.
- Preventing incidents with high code velocity: Anthropic aims for dream testing/deployment environments by replaying requests across mock databases, using fixtures, and chaos monkey testing; custom tools and personal Claude Code setups (calendar scheduling, email) are common. Incidents increase with speed, but Claude can also be used to improve uptime.
- Visibility and luck surface area: Thariq advises engineers to share work externally by choosing a project, working hard, shipping it, and writing about it; this expands luck surface area. He got his Anthropic job after sharing an interpretability visualization from a Goodfire fellowship; he cautions against using Claude to manage social media, emphasizing authenticity.
- Should people learn to code: Yes, because being technical is critical to evaluate AI outputs and understand system constraints. Thariq says 'coding is solved' means software no longer gets stuck in the same high-variability ways, but technical knowledge remains essential, similar to how technical CEOs like Zuckerberg and Musk operate without writing daily code.
- Advice to younger self: Believe in yourself and take bold moves; share original work; learn from mentors; balance boldness with learning from others' experience. Many early ideas were never shared; there is no universal advice, but both confidence and humility matter.

## Notable quotes

> "I think the number one tip we have for both people inside and outside Anthropic is that if you treat Claude like a thought partner and give it the context that you need, then you can usually figure out the next steps." — Thariq (~00:01:08)

> "Harness engineering is definitely this mix of science and art. I think it's very unintuitive in a lot of different ways, but I think it has just big abilities to unlock new parts of model behavior." — Thariq (~00:08:06)

> "I think most knowledge work is reducible to code and coding agents, if you think about it well." — Thariq (~00:06:07)

> "Turning every part of your software development lifecycle into kind of like a routine or a loop using Claude Code or something like that, I think is probably where things are headed." — Thariq (~00:23:38)

> "I think being technical is really, really important. Knowing how computers work, how computer programs work, how languages work, what are the hard things?" — Thariq (~01:01:14)

> "You have to choose an interesting project to work on. You have to work really hard on it. You have to lock in, and then you have to ship it and write about it." — Thariq (~00:56:45)

## People mentioned

- Ryan Peterman
- Thariq Shihipar
- Boris
- Jared
- Lev
- Terence Tao
- Andrej Karpathy
- Mark Zuckerberg
- Elon Musk

## Topics

`software-engineering` `ai-coding` `harness-engineering` `loop-engineering` `career-advice` `prompting` `technical-skills`
