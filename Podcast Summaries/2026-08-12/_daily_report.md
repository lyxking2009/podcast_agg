# Daily Report — 2026-08-12

**Run type:** Cron pipeline (deepseek-v4-pro) + concurrent manual fallback (deepseek-v4-flash) — merged. Manual fallback ran ~15:00 PDT; cron run completed ~15:22 PDT. Best transcript source kept per episode; state reconciled (processed + 9, failures 0 new).

**Episodes found (RSS):** 9 | **Feed errors:** 5 | **Processed:** 9 (5 YouTube auto-captions + 1 official web transcript + 1 RSS SRT + 1 web/substack + 1 show notes) | **Failures:** 0

## Transcript source breakdown
| Source | Count |
|---|---|
| youtube_autocaptions | 5 |
| web (official/structured) | 2 |
| rss_srt | 1 |
| show_notes | 1 |

## Episodes

| Show | Episode | Source | Links |
|---|---|---|---|
| Animal Spirits Podcast | Depressed Day Traders (EP. 477) | youtube_autocaptions | [[animal-spirits-podcast__depressed-day-traders-ep-477]] |
| Big Technology Podcast | Here's How The AI Bubble Bursts — With Paul Kedrosky | youtube_autocaptions | [[big-technology-podcast__here-s-how-the-ai-bubble-bursts-with-paul-kedrosky]] |
| Lex Fridman Podcast | #500 – Khabib Nurmagomedov: Dagestan, MMA, UFC, Islam, Conor, Fedor & Football | web (official lexfridman.com transcript) | [[lex-fridman-podcast__500-khabib-nurmagomedov-dagestan-mma-ufc-islam-conor-fedor-football]] |
| Masters in Business | BONUS: JP Morgan Co-Head of Global Banking Filippo Gori | rss_srt (Omny) | [[masters-in-business__bonus-jp-morgan-co-head-of-global-banking-filippo-gori]] |
| RiskReversal Pod | Nvidia's Backstop Deal Just Made Every Bank a Bag Holder | web_substack | [[riskreversal-pod__nvidia-s-backstop-deal-just-made-every-bank-a-bag-holder]] |
| The Meb Faber Show - Better Investing | Teaser: Investing in America: The Rise Of A 250-Year Bull Market | show_notes | [[the-meb-faber-show-better-investing__teaser-investing-in-america-the-rise-of-a-250-year-bull-market]] |
| The TWIML AI Podcast | Why Image Generation Needs More Than Bigger Models - #773 | youtube_autocaptions | [[the-twiml-ai-podcast-formerly-this-week-in-machine-learning-artificial-intelligence__why-image-generation-needs-more-than-bigger-models-773]] |
| Unchained | Should Ethereum Really Burn Its Staking Yield to Zero? | youtube_autocaptions | [[unchained__should-ethereum-really-burn-its-staking-yield-to-zero]] |
| Wiser World | 101. Becoming a Global Thinker: A Conversation with The Women in the Arena and Iron Butterfly | youtube_autocaptions | [[wiser-world__101-becoming-a-global-thinker-a-conversation-with-the-women-in-the-arena-and-iro]] |

## Feed errors (5)
| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window (weekly ROLLUP already covered 08-07) |
| Chalk Radio | timeout | ⚠️ No episode in window (feed sporadic; latest Mar 2026) |
| Critics at Large (The New Yorker) | timeout | ⚠️ No episode in window (weekly Thursdays; latest Aug 6, covered) |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window (weekly; latest ~Aug 3) |
| The Edge | timeout | ⚠️ No episode in window (latest #35, ~July) |

## Notes
- Window: 2026-08-12T07:00:00Z .. 2026-08-13T07:00:00Z (Aug 12 PDT). Subscriptions synced from Apple Podcasts DB: 47 subscribed feeds.
- Transcripts: 5 episodes via YouTube auto-captions (Animal Spirits 477, Big Technology w/ Paul Kedrosky, TWIML #773, Unchained Bits+Bips EIP-8361 debate, Wiser World 101). Lex #500 via official lexfridman.com transcript page (episode conducted in Russian, English dub). Masters in Business bonus via Omny SRT from RSS. RiskReversal via Substack post (concurrent run); Meb Faber teaser via show notes (book promo, 13 min).
- Two episodes (RiskReversal, Meb teaser) initially flagged as all-null error cases by the cron run's description-only fallback, but the concurrent manual run found real sources (Substack transcript / show notes) — reconciled, removed from failures.
- State updated: last_run_date → 2026-08-12, +9 processed entries, 0 new failures.


## Late additions (recovered 2026-08-13)

| Show | Episode | Transcript source | Vault |
|---|---|---|---|
| The Rest Is History | 696. Elizabeth I vs The Catholics: Killing the Queen of Scots (Part 6) | show_notes | [[the-rest-is-history__696-elizabeth-i-vs-the-catholics-killing-the-queen-of-scots-part-6]] |

- Recovered from prior-date re-fetch (published after 08-12 run's RSS fetch). Episode page is description-only — summarized from show notes.
