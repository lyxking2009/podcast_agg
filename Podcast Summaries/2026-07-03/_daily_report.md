# Daily Podcast Report — 2026-07-03

## Summary
- **Episodes processed:** 12
- **Shows covered:** 10
- **Feed errors:** 4 (Bankless, Chalk Radio, Latent Space, The Edge)
- **Transcript coverage:** 12/12 (100%)
- **Pipeline mode:** Manual fallback (Claude Code hit 600s background task ceiling)

## Transcript Sources
| Source | Count | Shows |
|--------|-------|-------|
| Omny SRT | 3 | Everybody's Business, Masters in Business, Odd Lots |
| Web (show site) | 4 | StarTalk Radio, Unchained ×3 |
| Show notes | 3 | Big Technology Podcast, The Compound and Friends, In Good Company |
| Show notes (Spotify/Apple) | 2 | Empire, The Meb Faber Show |

## Episodes

### [[everybodys-business__america-the-250-year-old-startup|Everybody's Business: America, the 250 Year Old Startup]]
America's 250th birthday examined through a business lens. Economist Martha Gimbel evaluates America Inc. as a startup investment — historically successful but growing concerns about political risk, immigration restrictions, and unsustainable deficits. Trump's $1.2B crypto windfall disclosed. Ranch dressing emerges as America's unlikely cultural ambassador.
**Transcript source:** rss_omny_srt (38,928 chars)

### [[masters-in-business__venture-capital-during-the-ai-revolution-with-mamoon-hamid|Masters in Business: Venture Capital During the AI Revolution with Mamoon Hamid]]
Kleiner Perkins partner Mamoon Hamid on his journey from aspiring astronaut to enterprise software investor. Key insight: 90% of AI revenue is in OpenAI and Anthropic. His 'labor pyramid' framework for AI investing starts with highest-paid workers and works down. Biggest miss: Anthropic Series B due to Zoom meetings — now does all first meetings in person.
**Transcript source:** rss_omny_srt (64,755 chars)

### [[odd-lots__how-a-grocery-store-chain-can-dramatically-lower-the-cost-of-food|Odd Lots: How a Major Grocery Store Chain Can Dramatically Lower the Cost of Food]]
Aldi's Scott Patton reveals how the chain keeps prices low: 90% private label, 4-6 hidden barcodes per product, display-ready cases, and grower relationships that start 3-4 years in advance. Beef at 70-year highs driving poultry substitution. GLP-1 drugs boosting protein and fiber demand. Opening 25,000 sq ft store near Times Square.
**Transcript source:** rss_omny_srt (71,020 chars)

### [[startalk-radio__cosmic-queries-astro-lore-with-moiya-mctier|StarTalk Radio: Cosmic Queries – Astro-lore with Moiya McTier]]
Astrophysicist and folklorist Moiya McTier on the intersection of astronomy and cultural storytelling. Double-majored at Harvard (not pre-approved), wrote a sci-fi novel on a real exoplanet as thesis. Science and folklore are 'two sides of the same coin' — both try to understand the universe.
**Transcript source:** web (51,791 chars from startalkmedia.com)

### [[unchained__how-ethereum-institutional-intends-to-grow-ethereums-market-share|Unchained: How Ethereum Institutional Intends to Grow Ethereum's Market Share]]
Joseph Chalom (Sharplink CEO) on the launch of Ethereum Institutional, backed by 50+ institutions. Ethereum's real competition is institutional inertia, not Solana. Robinhood's Arbitrum L2 is an Ethereum win. $31B+ in tokenized RWAs is the prize. 'Michael Saylor is in a pickle.'
**Transcript source:** web_unchained

### [[unchained__how-one-ens-vote-reignited-the-dao-governance-debate-uneasy-money|Unchained: How One ENS Vote Reignited the DAO Governance Debate]]
ENS founder Nick Johnson voted against Security Council renewal with ~50% of active supply, sparking backlash. ENS proposing to move governance to a foundation. The fundamental question: can any voting mechanism control a protocol with a $130M treasury?
**Transcript source:** web_unchained

### [[unchained__why-bitcoins-lack-of-yield-keeps-straining-its-treasury-companies|Unchained: Why Bitcoin's Lack of Yield Keeps Straining Its Treasury Companies]]
David Lawant (Anchorage Digital) on why Bitcoin's yieldless nature creates constant pressure on treasury companies. Saylor's playbook is evolving (not breaking) through preferred stock and options strategies.
**Transcript source:** web_unchained

### [[the-compound-and-friends__brian-belski-returns|The Compound and Friends: Brian Belski Returns!]]
Episode 249 with returning guest Brian Belski. Discussion of broadening market leadership, tech stock rotation into small/mid-caps, expectations for more volatility, and the potential for no SaaSpocalypse.
**Transcript source:** show_notes

### [[big-technology-podcast__zuckerbergs-disappointment-openais-equity-gamble-alex-karps-rally-cry|Big Technology Podcast: Zuckerberg's Disappointment, OpenAI's Equity Gamble, Alex Karp's Rally Cry]]
Ranjan Roy joins for 10-topic tech discussion. Zuckerberg says AI agent progress isn't going to plan; Meta exploring selling excess compute. Anthropic/OpenAI becoming points of failure in AI trade. Palantir's Karp challenges frontier labs. Should OpenAI give US government 5% equity?
**Transcript source:** show_notes

### [[empire__whats-circles-end-game-robinhood-launches-a-chain-venice-raises-65m|Empire: What's Circle's End Game, Robinhood Launches A Chain & Venice Raises $65M]]
Robinhood launched Robinhood Chain (Arbitrum L2) for real-world assets with Alchemy/BitGo/Chainlink integrations. Stock Tokens for 24/7 trading in 120 countries. Venice raises $65M. Circle's strategic positioning discussed.
**Transcript source:** show_notes

### [[in-good-company__highlights-andrew-forrest-ceo-of-fortescue|In Good Company: HIGHLIGHTS: Andrew Forrest - CEO of Fortescue]]
Andrew Forrest built Fortescue from scratch against a 60-year mining duopoly. Pledged zero fossil fuels by 2030 with no offsets. Using AI for smart energy grids. Philanthropy tackling modern slavery and ocean conservation. Life advice: 'Be useful, and enjoy it.'
**Transcript source:** show_notes

### [[the-meb-faber-show__the-secret-sauce-behind-250-years-of-american-success|Meb Faber Show: The Secret Sauce Behind 250 Years of American Success (McKinsey's Rebecca Anderson)]]
McKinsey Senior Fellow Rebecca Anderson shares her report on what has powered America's economy for 250 years: natural endowments, innovation culture, and institutional foundations. Timely for July 4th semiquincentennial.
**Transcript source:** show_notes

## Feed Errors
| Feed | Error |
|------|-------|
| Bankless | XML: mismatched tag: line 6, column 2 |
| Chalk Radio | timeout |
| Latent Space | XML: mismatched tag: line 6, column 2 |
| The Edge | timeout |

## Notes
- Claude Code launched 12 parallel transcript agents but hit the 600s background task ceiling before any completed. Manual fallback used for all 12 episodes.
- 3 Omny SRT transcripts fetched directly (Everybody's Business, Masters in Business, Odd Lots).
- StarTalk full transcript available from show website.
- 3 Unchained episodes had structured web content from unchainedcrypto.com.
- YouTube transcript API broken (Python 3.11 urllib3 incompatibility) — Empire episode used show notes instead of YouTube auto-captions.
- yt-dlp not available — couldn't download auto-captions as fallback.
