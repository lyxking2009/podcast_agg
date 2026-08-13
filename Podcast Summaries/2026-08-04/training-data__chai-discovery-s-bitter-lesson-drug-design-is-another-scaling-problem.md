---
podcast: "Training Data"
episode: "Chai Discovery's Bitter Lesson: Drug Design Is Another Scaling Problem"
published: 2026-08-04
duration: 47m22s
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/CPUAI9886655614.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-08-06T02:41:20Z
model: "claude-sonnet-5"
guid: "00198340-89dd-11f1-8519-5f4366a34437"
---

# Chai Discovery's Bitter Lesson: Drug Design Is Another Scaling Problem — Training Data

## TL;DR
Chai Discovery co-founders Joshua Meier and Matthew McPartlon describe treating drug design as an engineering/scaling problem rather than trial-and-error science, applying the 'bitter lesson' to biology. Their flagship result: de novo antibody design success rates rose from roughly 0.1% industry baseline in 2024 to about 15-16% with their Chai-2 model, a roughly 150x improvement. Rather than building their own drug pipeline, Chai licenses its models to pharma partners including Eli Lilly, Novartis, argenx, and Pfizer.

## Key points
- Chai Discovery's core thesis: instead of screening millions of molecules to find a hit ('drug discovery'), specify desired molecular properties upfront and have a model generate a candidate directly ('drug design').
- The company's headline metric is the de novo antibody binding hit rate: about 0.1% was state of the art when Chai was founded in 2024; their Chai-2 model reached roughly 15%, a step-change the founders originally hoped might take three to four years.
- Matthew McPartlon emphasizes simplicity as a guiding research principle: Chai-1 had 23 distinct submodules, and simplifying the architecture made it easier to identify true scaling directions.
- Chai deliberately avoids building an internal drug pipeline (unlike Isomorphic Labs) and instead partners with pharma companies (Eli Lilly, Novartis, argenx, Pfizer), so partnership revenue funds further model investment.
- Data sources are twofold: structural data from the Protein Data Bank and massive sequence databases; Chai's models are trained from scratch rather than fine-tuned from existing language models.
- The team scaled hiring opportunistically: AI researchers first, then antibody engineers/scientists once Chai-2 made antibody campaigns viable, then product and infrastructure specialists.
- The founders describe a long-term vision of shrinking the idea-to-testable-hypothesis loop from about nine months to nine days, making currently uneconomical targets (rare diseases, personalized medicine) tractable.

## Notable quotes
> "One of our big guiding principles is just simplicity. So when you look at a model like let's say Chai-1, I think there are 23 distinct submodules in Chai-1." — Matthew McPartlon

> "When we started the company, the state of the art for antibody design was about a 0.1 percent binding rate so 1 in 1,000 of the molecules you design would actually bind in the lab... We got to with our Chai-2 model about a 15 percent success rate." — Joshua Meier

> "Yeah, honestly, I think Josh and I were both surprised at how quickly this worked... we're like, maybe 20 percent hit rate in three or four years." — Matthew McPartlon

> "One thing that I really like about Chai is we're a very bitter lesson pill company. So we really believe in scaling data, scaling models, scaling compute." — Matthew McPartlon

> "If that loop right now takes something like nine months... to go and discover your molecule versus if it takes nine weeks or it takes nine days, each order of magnitude just scales in a very big way the number of ideas you can really sort through." — Joshua Meier

## People mentioned
- Joshua Meier
- Matthew McPartlon
- Pat Grady
- Andy Yeung
- David Baker

## Topics
`drug-discovery` `protein-design` `ai-scaling` `bitter-lesson` `antibody-design` `diffusion-models` `biotech` `pharma-partnerships`
