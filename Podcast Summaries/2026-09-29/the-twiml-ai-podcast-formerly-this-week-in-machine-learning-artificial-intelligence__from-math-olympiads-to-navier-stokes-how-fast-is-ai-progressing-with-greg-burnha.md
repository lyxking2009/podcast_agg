---
podcast: "The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)"
episode: "From Math Olympiads to Navier-Stokes: How Fast Is AI Progressing? with Greg Burnham - #778"
published: 2026-09-29
duration: "1h7m48s"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/MLN3408561388.mp3"
episode_url: "https://twimlai.com/podcast/twimlai/math-olympiads-navier-stokes-how-fast-ai-progressing"
transcript_source: whisper_asr
generated_at: "2026-09-29T22:40:02Z"
model: "deepseek-v4-pro"
guid: "a4ae7f12-bc4d-11f1-bf0c-1fda5ade33f6"
---

# From Math Olympiads to Navier-Stokes: How Fast Is AI Progressing? with Greg Burnham - #778 — The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)

## TL;DR
Epoch AI's Greg Burnham discusses the breakneck pace of AI progress, especially in mathematics, where systems have gone from failing grade-school problems to solving open research problems like the Navier-Stokes Millennium Prize problem in just a few years. He explains Epoch's benchmark aggregation and new evaluations for unsolved math, on-the-fly learning, and AI R&D, emphasizing that while AI now performs intricate computation and leverages existing knowledge, it still lacks the ability to generate genuinely new ideas or recursively self-improve. Burnham argues that continuous capability tracking is critical as AI's strengths and weaknesses increasingly impact the real world.

## Key points
- Epoch AI's Greg Burnham states that AI capabilities have moved from human exam benchmarks to real-world work tasks, and no measurement yet shows a slowdown in improvement across any task.
- Epoch's Capabilities Index aggregates benchmarks over time by stitching sigmoid curves, showing smooth linear progress across model generations; this linear trend reflects both targeted training and broad generalization, but the deep vs shallow distinction remains unclear.
- Benchmark scores are highly correlated across nominally different domains, which could mean labs are explicitly training on diverse tasks (shallow) or models are generalizing deeply from core training, with important implications for predictability.
- Epoch uses AI R&D replication benchmarks as a 'tripwire' for recursive self-improvement: giving models pre-innovation knowledge and asking them to improve metrics; so far they fail to replicate recent human innovations or exhibit 'research taste.'
- Math trajectory: as of fall 2024, grade-school math (GSM8K) was still challenging; OpenAI's o1 preview then spiked high-school competition math; Epoch's FrontierMath (tiers 1-4) went from advanced undergrad to grad-level; by early 2026 most FrontierMath problems were solved.
- In May 2026 an Astra-like internal model solved the unit distance problem (an Erdős problem) by disproving a conjecture most mathematicians believed true; Timothy Gowers suggested a two-part hint might have allowed humans to solve it.
- The Navier-Stokes Millennium Prize problem solution reportedly involved thousands of AI instances sharing ideas and built heavily on prior human work, demonstrating AI's persistence and breadth but not necessarily original theory-building.
- AI's two current math advantages are encyclopedic knowledge of literature and extreme patience for intricate computations; however, it has not yet produced fundamentally new concepts like the derivative or integral.
- Epoch's latest FrontierMath set is composed of unsolved problems humans have tried and failed to solve, automatically verifiable, and tiered from moderately interesting to breakthrough; none of the breakthrough-tier problems have been solved yet.
- When Epoch benchmarked its own internal job tasks, AI was good at literature summaries but poor at creating style-compliant infographics, generating short data-driven insights, and prototyping new open-ended projects.
- An on-the-fly learning benchmark using the board game Earthborne Rangers shows AI models arrive with a strong out-of-the-box score but flat learning curves across repeated plays, whereas humans improve quickly; models improve only across generations, not within sessions.
- Providing AI with detailed human-created strategy guides dramatically improves board game performance, but AI cannot create such guides for itself, indicating limited in-context learning-to-learn.
- Harness-side improvements like multi-agent setups, note-taking tools, and skills show only low-hanging gains and plateau; no current harness configuration cracks strategic on-the-fly learning.
- To address contamination, Epoch plans video game benchmarks using newly released games not in training data, allowing assessment of visual, spatial, and real-time reasoning plus learning on new tasks.
- Epoch is also developing a furniture assembly benchmark where AI must identify assembly mistakes from photos, tracking progress toward physical-world guidance before robotics matures.
- Burnham notes capability thresholds lead to sudden real-world impact, exemplified by the ChatGPT moment, Claude Code software engineering, and a recent AI hacking incident against Hugging Face; continuous tracking helps anticipate such thresholds.

## Notable quotes
> "We see no measurement we have shows any slowdown in how AI is getting better and better at any tasks that we're able to measure." — Greg Burnham
> "We're past the academic exam phase of understanding AI capabilities. And for that, we need high-quality benchmarking, high-quality evaluations. And one of my big questions is, just can AI come up with new ideas?" — Greg Burnham
> "We want to put a tripwire out there so that if AI ever does cross some threshold where it's able to do rapid improvements in AI algorithms themselves, then, uh, we, we want that tripwire to sound." — Greg Burnham

## People mentioned
- Greg Burnham
- Sam Charrington
- Paul Erdos
- Timothy Gowers
- Isaac Newton
- Gottfried Wilhelm Leibniz

## Topics
`ai-capabilities` `ai-benchmarks` `math-ai` `navier-stokes` `self-improvement` `on-the-fly-learning`
