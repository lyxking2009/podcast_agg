# Daily Report — 2026-08-10

**Run type:** Manual pipeline fallback (Claude Code OAuth session expired — `Failed to authenticate: OAuth session expired and could not be refreshed`; pipeline exited after ~25 min without processing, re-fetch confirmed 8 episodes + 5 feed errors).

**Episodes found (RSS):** 8 | **Feed errors:** 5 | **Processed:** 8 (3 YouTube auto-captions + 1 RSS SRT + 4 web/show notes) | **Late catch-up (08-09):** +2 episodes (Acquired, The Rest Is History) | **Failures:** 0

## Transcript source breakdown
| Source | Count |
|---|---|
| youtube_autocaptions | 3 |
| rss_omny_srt | 1 |
| web (structured pages) | 2 |
| show_notes | 2 |

## Episodes

| Show | Episode | Source | Links |
|---|---|---|---|
| Animal Spirits Podcast | Talk Your Book: Why Momentum Investing Works | show_notes | [[animal-spirits-podcast__talk-your-book-why-momentum-investing-works]] |
| Empire | Morpho Is Coming For The $200 Trillion Credit Market \| Paul Frambot | youtube_autocaptions | [[empire__morpho-is-coming-for-the-$200-trillion-credit-market-paul-frambot]] |
| Odd Lots | NYT CEO Meredith Kopit Levien on Running a Media Brand in the Age of AI | rss_omny_srt | [[odd-lots__nyt-ceo-meredith-kopit-levien-on-running-a-media-brand-in-the-age-of-ai]] |
| RiskReversal Pod | AI Is Rewiring Retail Investing with Jim Swartwout & Jay Jacobs | web | [[riskreversal-pod__ai-is-rewiring-retail-investing-with-jim-swartwout-and-jay-jacobs]] |
| Strategic Alternatives | How AI infrastructure and M&A are reshaping corporate finance | show_notes | [[strategic-alternatives__how-ai-infrastructure-and-manda-are-reshaping-corporate-finance]] |
| The Compound and Friends | Is IMAX a Screaming Buy? With Rich Gelfond | show_notes | [[the-compound-and-friends__is-imax-a-screaming-buy-with-rich-gelfond]] |
| The Peterman Pod | Creator of Lean: Handwritten Math Will Change Dramatically \| Leonardo de Moura | youtube_autocaptions | [[the-peterman-pod__creator-of-lean-handwritten-math-will-change-dramatically-leonardo-de-moura]] |
| Y Combinator Startup Podcast | Max Hodak: How Startups Build Speed | web | [[y-combinator-startup-podcast__max-hodak-how-startups-build-speed]] |

## Late catch-up — published 08-09 (missed by yesterday's 3:15 PM run)
| Show | Episode | Source | Links |
|---|---|---|---|
| Acquired | Disney: The Renaissance and the Empire | web | [[acquired__disney-the-renaissance-and-the-empire]] |
| The Rest Is History | 695. Elizabeth I vs The Catholics: The Shadow War (Part 5) | show_notes | [[the-rest-is-history__695.-elizabeth-i-vs-the-catholics-the-shadow-war-(part-5)]] |

## Feed errors (5)
| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window — Friday ROLLUP already covered 08-07; Monday episode (if any) not public yet |
| Chalk Radio | timeout | ⚠️ No episode in window (feed sporadic; latest Mar 2026) |
| Critics at Large (The New Yorker) | timeout | ⚠️ No episode in window (weekly Thursdays; latest Aug 6, covered) |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window (weekly; latest ~Aug 3) |
| The Edge | timeout | ⚠️ No episode in window (latest #35, ~July) |

## Notes
- Manual fallback executed because Claude Code auth failed (OAuth expired, non-retryable — same as 08-09; not a pipeline bug).
- 8 RSS episodes processed with 100% transcript coverage: Empire (69m, Morpho $200T credit market thesis, Morpho Midnight fixed-rate protocol, token/equity design) and Peterman Pod (68m, Lean creator Leonardo de Moura on LLM+Lean verification, zlib-in-Lean week-long proof) via YouTube auto-captions; Odd Lots (60m, NYT CEO Meredith Kopit Levien on media in the AI age, video strategy, talent wars) via RSS Omny SRT; RiskReversal (two-part: retail golden age w/ Jim Swartwout + BlackRock's IQQQ/QTop/QNext w/ Jay Jacobs) and YC Startup School (Max Hodak on startup infrastructure: purchasing, attribution, hiring) via structured web pages; Animal Spirits (momentum factor w/ Nasdaq Dorsey Wright's John Lewis), TCAF (IMAX CEO Rich Gelfond), RBC Strategic Alternatives (AI infra constraints + M&A) via show notes.
- Late catch-up: yesterday's run completed 22:15Z (15:15 PDT) and missed two 08-09 episodes published later in the day — Acquired "Disney: The Renaissance and the Empire" (~4h20m, full web transcript) and The Rest Is History ep. 695 "The Shadow War (Part 5)" (show notes). Written to the 08-09 vault dir; state entries dated 2026-08-09.
- State updated: last_run_date → 2026-08-10; +8 processed entries (08-10) + 2 processed entries (08-09 late); 0 new failures.

## Late additions (recovered 2026-08-11)
Two 08-10 episodes published after yesterday's run completed (23:56Z) were missed by the 08-10 fetch. Recovered this run via web sources; state entries dated 2026-08-10.

| Show | Episode | Source | Links |
|---|---|---|---|
| Machine Learning Street Talk (MLST) | AI Is Learning at the Wrong Level of Abstraction — Matthieu Wyart | web | [[machine-learning-street-talk-mlst__ai-is-learning-at-the-wrong-level-of-abstraction-matthieu-wyart]] |
| 硅谷101 | E248｜一个"催发货"AI要跑通260步，和阿里瓴羊朋新宇聊聊中国式FDE | show_notes | [[101__e248-cui-fa-huo-ai-260-bu-ling-yang-peng-xin-yu-fde]] |
