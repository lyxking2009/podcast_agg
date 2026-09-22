---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "🔬 An Oscar, Two Asteroids, and the Algorithm in Your sklearn: John Platt on AI for Science"
published: 2026-09-22
duration: "2h1m26s"
audio_url: "https://api.substack.com/feed/podcast/216845528/6eca91d6e6b9017b8cc853a8a8dd27b8.mp3"
episode_url: "https://www.latent.space/p/john-platt"
transcript_source: youtube_autocaptions
generated_at: 2026-09-22T22:08:37Z
model: "deepseek-v4-pro"
guid: "substack:post:216845528"
---

# 🔬 An Oscar, Two Asteroids, and the Algorithm in Your sklearn: John Platt on AI for Science — Latent Space: The AI Engineer Podcast

## TL;DR
John Platt discusses ERA (Empirical Research Agent), a system at Google that maps scientific problems to code optimized against a scoring function. He explains the distinction between predictive and descriptive models, warns about overfitting and multiple hypothesis testing, and highlights successes in contrail avoidance, wildfire detection, and climate modeling. Platt also touches on fusion energy, quantum computing, and advice for young scientists in the AI-for-science era.

## Key points
- John Platt is a Google Fellow and Head of Applied Science at Google Research, known for Platt scaling and sequential minimal optimization (SMO) for SVMs; he entered college at 14, PhD at 18 at Caltech under John Hopfield, and has named two asteroids and won an Academy Award for technical developments in 2006.
- ERA (Empirical Research Agent) reframes scientific problems as 'scorable tasks' where the goal is to maximize a score; it uses Gemini to generate and mutate Python notebooks, employing Monte Carlo tree search with upper confidence bound (UCB) to select promising candidates, and can incorporate papers as starting points.
- ERA has been applied to diverse problems: fitting statistical models, asymptotic expansions of differential equations, super-resolution of satellite CO2 measurements, forecasting COVID and flu for CDC, and counterfactual climate models for contrail warming.
- A key distinction is predictive vs descriptive models: predictive models minimize error on data, while descriptive models attempt to capture underlying physics for extrapolation; scientists must ensure the model remains descriptive and rigorous.
- Overfitting is a major risk with autonomous code evolution; Platt stresses the need for hidden holdout sets and extreme rigor to avoid fooling yourself, especially given Goodhart's law that any metric becomes a target and loses value.
- The contrail project estimates contrails contribute ~1% of anthropogenic global warming; they form in ice-supersaturated regions, and avoiding them by dropping two flight levels is low cost. Google built satellite-based detection models, and ERA helped solve a 2-year stuck problem in estimating the reflected sunlight component of contrail forcing.
- For wildfires, Google's FireSat project (with Earth Fire Alliance and Muon Space) aims to deploy 50-80 satellites with midwave IR sensors to detect fires as small as 5 meters within 15-20 minutes, enabling early intervention; WHO estimates 300,000 excess deaths per year from wildfire smoke.
- On fusion, Platt explains the Lawson criterion (product of density, temperature, and confinement time) and notes that different fusion approaches have different Achilles heels; he reports progress with field-reversed configurations at TAE and Google DeepMind's work on tokamak disruption avoidance, suggesting commercial fusion may be within a decade.
- Climate modeling remains difficult due to non-stationarity and low data for future projections; Platt distinguishes weather (predicting trajectories up to 15 days) from climate (statistics of the attractor) and notes that AI has revolutionized weather but not yet climate.
- On quantum computing, Platt sees steady progress, with Google's quantum team executing a roadmap and achieving quantum error correction milestones like Willow; he cautions that current devices are still noisy intermediate-scale quantum (NISQ) and finicky, but is optimistic about superconducting qubits.
- Advice for young scientists: combine deep domain expertise with hands-on use of AI tools; preserve 20% time for exploration; humans are still needed for creativity, rigor, and philosophy; avoid being an 'empty suit' by maintaining technical judgment.
- Historical anecdotes: Platt took Richard Feynman's 1982 class on physics of computation at Caltech, coining 'convolutional net'; he discovered asteroids using Palomar photographs and stereoscopes; and his early work on elasticity simulation contributed to Pixar's Academy Award.
- Nvidia's pivot to deep learning was influenced by a graduate school friend who convinced Jensen Huang in a 15-minute conversation, though Platt notes GPUs became significantly faster than CPUs only around the ImageNet/speech recognition era.

## Notable quotes

> "A descriptive model is actually what science is trying to get to, which is okay, it should be able to extrapolate because it has sort of the physics or the actual some description of reality that's captured within it." — John Platt (~00:23:10)
> "you're not so much now in the details of oh oh I have to import this CSV file or I have to get this database to work or whatever. It's you're now sort of thinking almost like deeply philosophically about your actual scientific problem, not down in the grungy goop of of of worrying about, you know, databases." — John Platt (~00:16:43)
> "So you can actually there's a lot of sort of tricks you can do because it's not that the underlying thing that's that's altering the code or the underlying thing that's sort of making the decisions is not a random process. It's an AI itself that is smart and knows about things and knows a lot about the world." — John Platt (~00:06:07)

## People mentioned

- John Platt
- R.J. (co-host)
- John Hopfield
- Michael Brener
- Richard Feynman
- Carver Mead
- Danny Hillis
- Jensen Huang
- Dave Kirk
- Bill Dally
- Dave Bacon
- Hartmut Neven
- John Preskill
- Yann LeCun
- Brian Marsden
- Jean Shoemaker
- Carolyn Shoemaker

## Topics

`ai-for-science` `program-synthesis` `climate-modeling` `contrail-avoidance` `wildfire-detection` `fusion-energy` `quantum-computing`
