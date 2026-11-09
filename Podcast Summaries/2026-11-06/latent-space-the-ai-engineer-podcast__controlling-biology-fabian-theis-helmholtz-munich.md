---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Controlling Biology - Fabian Theis, Helmholtz Munich"
published: 2026-11-06
duration: "50m50s"
audio_url: "https://podcasts.beehiiv.com/3967c1f2-cb97-4eb0-a5c0-21a1d2579408/01a15fdf-0e93-7e1a-880d-d73752a1f915/01a1a3f4-4773-7a7d-acdc-731f8540ab5d/01a1b153-728b-7ba1-9c45-88599573e55f.mp3"
episode_url: "https://lspace.beehiiv.com/podcast/s/latent_space_the_ai_engineer_podcast/controlling_biology_-_fabian_theis_helmholtz_munich_1"
transcript_source: rss_vtt
generated_at: 2026-11-09T23:22:25Z
model: "deepseek-v4-pro"
guid: "01a1a3f4-4773-7ef4-9673-59b03a12c34b"
---

# Controlling Biology - Fabian Theis, Helmholtz Munich — Latent Space: The AI Engineer Podcast

## TL;DR
Fabian Theis discusses the shift from mapping and prediction to actively steering biological experiments using virtual cell models. He highlights the explosion in perturbation data (e.g., 10M transcriptomes per experiment) and scaling of foundation models like scConcert (350M cells), while cautioning that linear baselines often outperform complex models and metrics can be gamed. Theis also covers organoid modeling, Lab Compass for inverse experimental design, Helmholtz Munich's interdisciplinary training, and Germany's startup scaling challenges.

## Key points
- Fabian argues that renewed pharma interest in virtual cells is driven by data generation at scale, pointing to Biohub's $1.8B collaboration with DOE, NIH, Meta, and Google linked to the Human Cell Atlas, and notes that startups in Europe and China are exploding as pharma desperately needs cell-type-specific screens.
- He cautions that equating virtual cell with perturbation prediction is too simplistic, but says perturbation prediction models—world models that take state X and action A to predict next state X'—are taking off, with early work like his lab's 2019 scGen showing nonlinear latent-space shifts can outperform linear baselines.
- Data scale has reached new levels: Pars Bio's 'giga lab' measured 10 million transcriptomes in a single experiment, and perturbation spaces now include hundreds of cell lines, full CRISPR knockouts, and thousands of drugs, moving beyond the scarce control-vs-disease comparisons.
- Perturbations are defined broadly as any input to a cell—CRISPR knockouts, drug treatments, media conditions, Yamanaka factors, or intercellular signaling—and a complete virtual cell model should handle combinations, time, and disease context.
- Organoids are emerging as multicellular model systems: mini brains that compute, beating heart organoids, and cancer organoids, but they lack immune systems and vasculature; modeling them is unsettled, with current representations often reducing to cell distributions or graphs.
- For spatial and H&E pathology data, Fabian says a categorization language analogous to 'cell type' is missing for local niches of cells, and current methods either treat them as distributions or need graph-based representations.
- On inductive bias, Fabian cites AlphaFold as an example where initial models hard-coded rotational invariance, but later versions learned it automatically with more data; however, in the data-poor perturbation regime, hybrid models with some priors may still be necessary.
- He stresses the need for scaling laws to justify foundation models, noting early papers lacked them; his lab's contrastive learning model scConcert shows scaling behavior with 350 million cells, up to 100M parameters, and covers 16 species, enabling cross-species latent spaces.
- Baseline models remain surprisingly competitive: logistic regression can be state-of-the-art for cell type prediction within an organ, and linear models often match complex ones for simple perturbation tasks; his lab's scTab demonstrated nonlinearity adds value for cross-organ prediction.
- The Arc Institute's virtual cell challenge revealed that fixed metrics could be exploited; after removing metric exploitation, the winning model was a flow matching approach, and Bo Wang's Nature paper further showed the metric itself was problematic.
- Flow matching models like CellFlow (accepted at Nature Methods) generate perturbation effects much better than linear or pseudo-linear models but depend heavily on a good latent space, which scConcert provides.
- Lab Compass is an inverse generative model that uses a forward world model to tell researchers what perturbations are needed to reach a desired cell state; it was validated with 3-4 lab-in-the-loop cycles in hematopoiesis, moving from active learning toward RL-driven experimentation.
- Helmholtz Munich's Computational Health Center, led by Fabian, has ~45 group leaders and 500-600 staff, making it Europe's largest academic health AI center; it has no undergraduate teaching, focuses on PhDs/postdocs, and encourages each PI to spin off a startup within three years.
- Germany's startup ecosystem is competitive for early-stage funding but bureaucratic and weak at scaling; later-stage financing (Series A/B) often requires going to the US, and EU regulations like the AI Act complicate health data access, though the European Health Data Space may improve cross-border patient data.
- Fabian's 'Y model' training pairs each student with a methods advisor and a domain advisor, requiring attendance at two lab meetings; this has produced diverse outcomes, with recent hires to NVIDIA and flow into pharma, startups, and academia.
- For commercialization, platform companies often pivot to drug development because investors want a clear path; Fabian sees data generation combined with modeling as the real moat, exemplified by the Cancer Organoid Perturbation Atlas consortium where pharma pays for early access.
- Fabian's lab created widely used open-source tools Scanpy, Anndata, and scVerse, which have attracted many contributors and are central to single-cell analysis.

## Notable quotes
> "From mapping, we went to prediction. But now we can go to actually steering the next experiment." — Fabian Theis
> "the honest learning from, from big scale machine learning is that at some point data beats human-made additional concepts in these models." — Fabian Theis
> "the ultimate vision being that we want to RL everything. We want to do RL also in, in this loop." — Fabian Theis
> "I know you wanna do the most fancy deep learning thing, but please, please, you know, have the baseline." — Fabian Theis

## People mentioned
- Fabian Theis
- RJ Hanocki
- Brandon Anderson
- John Platt
- Bo Wang

## Topics
`virtual-cell` `perturbation-prediction` `single-cell-transcriptomics` `organoids` `ai-for-science` `startup-ecosystem`
