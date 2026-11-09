---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "⚡ An Eval for AI Scientists That Knows the Answer Key - Ian Quigley, Leash Bio"
published: 2026-10-26
duration: "16m34s"
audio_url: "https://podcast.latent.space/audio/vhPCo-8K78U.m4a"
episode_url: "https://www.youtube.com/watch?v=vhPCo-8K78U"
transcript_source: web
generated_at: 2026-10-27T22:12:29Z
model: "deepseek-v4-pro"
guid: "youtube:vhPCo-8K78U"
---

# ⚡ An Eval for AI Scientists That Knows the Answer Key - Ian Quigley, Leash Bio — Latent Space: The AI Engineer Podcast

## TL;DR
Ian Quigley from Leash Bio returns to discuss a new evaluation environment where AI agents perform structure-activity relationship (SAR) optimization using pre-measured DNA-encoded library screening data. Agents are given a target and a 100-molecule budget to find the best binder, with instant feedback from the fully measured landscape. Leash has run about 90,000 rollouts across many models and found that more capable models abandon unproductive chemical neighborhoods faster, while weaker models futilely tweak poor molecules; they plan to release the environments, data, and a preprint alongside this episode.

## Key points
- Leash Bio has built a virtual environment for AI agents to tackle medicinal chemistry structure-activity relationship (SAR) problems: agents receive a protein target and a budget of 100 molecules, then iteratively propose and test molecules to find the best binder, using pre-measured DNA-encoded library screening data.
- The environment exploits Leash's counterfactually dense dataset—screening data covering about 1,000 proteins against large chemical libraries—so every agent action receives instant ground-truth feedback, essentially amortizing the latency of real lab experiments.
- Leash has run approximately 90,000 agent rollouts across many model families and levels, generating a large corpus of transcripts that reveals agent hypotheses, search policies, and failure modes.
- Evaluation goes beyond final success: because the entire molecular landscape is known, Leash can assess whether agents found near-optimal binders or settled for suboptimal ones, and can quantify decision-making quality.
- A key behavioral finding is that more performant models abandon unproductive chemical neighborhoods significantly faster than weaker models; weaker models tend to make small, futile modifications to low-scoring molecules and repeatedly retest the same compounds despite limited budgets.
- The reason for this difference in behavior is not yet understood; Leash has only observed the correlation, not the underlying mechanism.
- The agents produce plausible medicinal chemistry narratives in natural language, and when human medicinal chemists reviewed some transcripts, they found the reasoning plausible even when not necessarily correct.
- Leash plans to release the SAR environments as packages along with thousands of rollout transcripts, enabling the community to analyze agent behavior and build on the benchmark.
- Among frontier models, the current best performer was referred to as 'after six' (as spoken in the conversation), with Sonnet 55 and Sol 61 also performing well; GPT-5 was described as expensive and not performant. Open-source models like DeepSeek V4 Flash and others were also tested.
- Leash previously attempted supervised fine-tuning of language models to understand SMILES strings because frontier models were initially poor at converting between SMILES representations; by around February, models began to handle chemistry meaningfully, enabling these iterative tasks.
- Leash has screened about 1,000 proteins against a couple of chemical libraries to date and is currently generating two new environments per day, aiming to scale to 20 per day by 2026-2027, creating a large RL environment for scientific agents.
- The team is interested in whether reinforcement learning progress from these environments transfers to new biology, and they plan to incorporate more structural tool calls and consult decision-making experts on optimal experimental budget allocation.

## Notable quotes
> "We have built this thing where we if you've got scientific agents and you want to know how they're behaving, what mistakes they might make, what how they might learn from physical feedback, you know, if a new training recipe improves scientific decision-making. We've got the capabilities to measure these things at scale." — Ian Quigley
> "And it turns out that more performant models are much better at abandoning a kind of unproductive neighborhoods. And that was actually a really interesting surprise for us." — Ian Quigley
> "And what happens with the the dumber models is that they'll just keep running little subtle changes on molecules that aren't ever going to go anywhere." — Ian Quigley
> "We don't know that at all yet. All we know is that they're abandoning these crappy neighborhoods more quickly." — Ian Quigley

## People mentioned
- Ian Quigley
- Brandon
- RJ

## Topics
`ai-for-science` `drug-discovery` `medicinal-chemistry` `reinforcement-learning` `model-evaluation` `dna-encoded-libraries`
