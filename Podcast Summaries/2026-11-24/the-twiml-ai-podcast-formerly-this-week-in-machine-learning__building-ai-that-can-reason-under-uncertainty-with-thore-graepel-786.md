---
podcast: "The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)"
episode: "Building AI That Can Reason Under Uncertainty with Thore Graepel - #786"
published: 2026-11-24
duration: "1h10m19s"
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/MLN2250038870.mp3"
episode_url: "https://twimlai.com/podcast/twimlai/building-ai-that-can-reason-under-uncertainty"
transcript_source: web
generated_at: 2026-11-24T23:05:10Z
model: "deepseek-v4-pro"
guid: "9294c3be-e83e-11f1-84c1-ffe7c3ba21de"
---

# Building AI That Can Reason Under Uncertainty with Thore Graepel - #786 — The TWIML AI Podcast (formerly This Week in Machine Learning & Artificial Intelligence)

## TL;DR
Thore Graepel, a key contributor to DeepMind's AlphaGo, argues that next-token prediction alone will not yield AI capable of true discovery or reliable reasoning under uncertainty. Drawing on AlphaGo's Move 37—where game-tree search overrode a policy network's 1/10,000 probability assignment—he advocates a neurosymbolic approach that explicitly represents beliefs, tests hypotheses with evidence, and uses probability theory to reduce uncertainty. His team at Meta is building systems that separate knowledge from reasoning, using large language models only for small, well-scoped semantic interpretation tasks.

## Key points
- Thore Graepel argues that next-token prediction cannot achieve reliable AI reasoning because tokens cannot be true or false—only plausible next tokens—so systems must explicitly represent beliefs about the world as propositions that can be evaluated against evidence.
- AlphaGo's famous Move 37 illustrates the need for a dedicated reasoning process: the neural policy network assigned the move a probability of only 1 in 10,000, but the game tree search selected it by evaluating future positions, showing that imitation-based pattern matching rarely produces novel, surprising moves; true discovery requires reasoning through consequences.
- Graepel advocates a neurosymbolic approach that combines symbolic logic and knowledge representation with adaptive neural learning, calling it a pendulum swing back toward integrating the two strands of AI after recent neural progress.
- He frames reasoning as a game in which the internal belief state is the 'board' and moves include framing the question, forming hypotheses, decomposing problems into subproblems, pulling in external evidence, and updating probabilities; the objective is to reduce uncertainty about the world.
- Probability theory serves as the 'calculus of uncertainty': hypotheses are assigned probabilities, evidence increases or decreases belief, and logical relationships between hypotheses must be reflected in coherent probability assignments; this allows non-binary conclusions and quantified confidence.
- In a simple example—'What is the capital of France?'—the system first questions the premise and frames the question, proposes hypotheses (Paris, London, Madrid, even 'Mickey Mouse'), pulls evidence like a Wikipedia snippet, and judges whether the snippet supports the hypothesis; the final answer is not 100% but the highest probability with an attached reasoning trace and evidence provenance.
- The approach separates knowledge from reasoning: explicit knowledge representation stores certified facts with confidence and evidence, allowing auditing, editing, guardrails, and access control—unlike implicit knowledge hidden in transformer weights.
- Graepel emphasizes using large language models only as small, well-scoped semantic interpreters—for example, judging whether a text snippet supports a statement—rather than as reasoning engines; this decomposition reduces hallucinations and enables many calls to smaller models, improving compute efficiency and enabling edge deployment and data sovereignty.
- The doctor example illustrates optimal information gathering: asking a patient about symptoms has high information gain per cost ('bits per buck') compared to an immediate imaging scan, so a reasoning system should choose next moves that maximize expected reduction in uncertainty per unit cost.
- Decision models that output numbers/vectors instead of strings—such as those from Meta's JEPA work—are exciting because they can produce calibrated probabilities, lower latency, and less compute; they can serve as components for prior probabilities and evidence interpretation within the reasoning process.
- The reasoning patterns are universal and finite: framing, hypothesis generation, decomposition, evidence retrieval, tool calls, and synthesis; the policy for sequencing these moves can be learned via reinforcement learning with uncertainty reduction as reward, leading to two scaling axes: knowledge base growth and reasoning policy improvement.
- Frames are critical for knowledge representation: claims like 'Paris is the capital of France' or 'Gandalf is 800 years old' depend on the frame (e.g., current vs historical, literary vs pet); a good knowledge base manages frames to avoid false contradictions.
- Current benchmarks like GPQA Diamond measure only final answers and may miss whether intermediate reasoning steps are consistent or whether the system is post-hoc rationalizing; Graepel calls for benchmarks that objectively score reasoning traces to distinguish rigorous reasoning from guessing.
- The system can actively build knowledge by processing claims from sources like medical papers, decomposing each claim, verifying against evidence, and depositing certified conclusions into the explicit knowledge base, enabling continuous learning.

## Notable quotes
> "tokens cannot be true or false. there can just be um a plausible next token. But if you work for example with propositions about the world, then those can be true or false." — Thore Graepel
> "the scientific method uh in my estimation is the only reliable way to create new knowledge." — Thore Graepel
> "we fundamentally distrust um the knowledge that's in large language models. And we believe that the crucial step to make to make AI reasoning systems uh reliable and trustworthy is to separate the knowledge from the reasoning process." — Thore Graepel
> "So instead of making one call to a big model, you make many calls to smaller models and that gives you a fundamentally different profile." — Thore Graepel

## People mentioned
- Thore Graepel
- Sam Charrington
- David Silver
- Michael Redmond
- Douglas Lenat
- E.T. Jaynes
- Daniel Kahneman

## Topics
`neurosymbolic-ai` `reasoning-under-uncertainty` `alphago` `scientific-method` `knowledge-representation` `large-language-models` `probabilistic-reasoning`
