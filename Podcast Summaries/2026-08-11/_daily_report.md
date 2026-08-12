# Daily Report — 2026-08-11

**Run type:** Manual pipeline fallback (Claude Code OAuth session expired — `Failed to authenticate: OAuth session expired and could not be refreshed`; 4th consecutive day. Pipeline exited ~2 min after launch; manual fallback executed instead.)

**Episodes found (RSS):** 9 | **Feed errors:** 5 | **Processed:** 9 (1 RSS SRT + 5 web/structured + 3 show notes) | **Late catch-up (08-10):** +2 episodes (MLST, 硅谷101) | **Failures:** 0

## Transcript source breakdown
| Source | Count |
|---|---|
| rss_omny_srt | 1 |
| web (structured pages/transcripts) | 5 |
| show_notes | 3 |

## Episodes

| Show | Episode | Source | Links |
|---|---|---|---|
| Dwarkesh Podcast | Ryan Greenblatt – Human level AIs might build runaway superintelligences by 2032 | web | [[dwarkesh-podcast__ryan-greenblatt-human-level-ais-might-build-runaway-superintelligences-by-2032]] |
| Everybody's Business | One Weird Trick to Get Your Credit Card Charge Reversed? Fraud. | rss_omny_srt | [[everybodys-business__one-weird-trick-to-get-your-credit-card-charge-reversed-fraud]] |
| In Good Company with Nicolai Tangen | Bonus: Advice to young people | show_notes | [[in-good-company-with-nicolai-tangen__bonus-advice-to-young-people]] |
| Invest Like the Best with Patrick O'Shaughnessy | Eric Vishria - A Decade of Lessons Investing in Software & Hardware (EP.486) | web | [[invest-like-the-best-with-patrick-o-shaughnessy__eric-vishria-a-decade-of-lessons-investing-in-software-hardware-invest-like-the-best-ep-486]] |
| StarTalk Radio | Super Solar Storms with Lika Guhathakurta | show_notes | [[startalk-radio__super-solar-storms-with-lika-guhathakurta]] |
| The Compound and Friends | Tradeable Lows, Nvidia's Data Center Finance Deal, Surprise Comebacks for Schwab and Expedia | show_notes | [[the-compound-and-friends__tradeable-lows-nvidias-data-center-finance-deal-surprise-comebacks-for-schwab-and-expedia]] |
| The Peter McCormack Show | #200 - Sebastian Thrun - The Singularity Is Here & We Are Completely Unprepared | show_notes | [[the-peter-mccormack-show__200-sebastian-thrun-the-singularity-is-here-and-we-are-completely-unprepared]] |
| Unchained | Sam MacPherson on Why Spark Benefited So Much From the KelpDAO Hack | web | [[unchained__sam-macpherson-on-why-spark-benefited-so-much-from-the-kelpdao-hack]] |
| Y Combinator Startup Podcast | Peter Steinberger: "Fun Is Velocity" | web | [[y-combinator-startup-podcast__peter-steinberger-fun-is-velocity]] |

## Late additions (recovered 2026-08-11) — published 08-10
| Show | Episode | Source | Links |
|---|---|---|---|
| Machine Learning Street Talk (MLST) | AI Is Learning at the Wrong Level of Abstraction — Matthieu Wyart | web | [[machine-learning-street-talk-mlst__ai-is-learning-at-the-wrong-level-of-abstraction-matthieu-wyart]] |
| 硅谷101 | E248｜一个"催发货"AI要跑通260步，和阿里瓴羊朋新宇聊聊中国式FDE | show_notes | [[101__e248-cui-fa-huo-ai-260-bu-ling-yang-peng-xin-yu-fde]] |

## Feed errors (5)
| Feed | Error | Fallback result |
|---|---|---|
| Bankless | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window — ROLLUP already covered 08-07; no Tue 08-11 episode surfaced in web search |
| Chalk Radio | timeout | ⚠️ No episode in window (feed sporadic; latest Mar 2026) |
| Critics at Large (The New Yorker) | timeout | ⚠️ No episode in window (weekly Thursdays; latest Aug 6, covered) |
| Latent Space: The AI Engineer Podcast | XML: mismatched tag: line 6, column 2 (Flightcast ~167B) | ⚠️ No episode in window (weekly; latest ~Aug 3) |
| The Edge | timeout | ⚠️ No episode in window (latest #35, ~July) |

## Notes
- Manual fallback executed because Claude Code auth failed (OAuth expired — 4th consecutive day, 08-08/09/10/11; not a pipeline bug).
- 9 RSS episodes processed with 100% transcript coverage: Dwarkesh (2h12m full debate transcript with Redwood's Ryan Greenblatt on RSI timelines ~2031) and YC Startup School (Peter Steinberger's OpenClaw talk, full Root Access transcript) via structured web pages; Everybody's Business (16m, The Big Take cross-post on the chargeback/friendly-fraud surge) via RSS Omny SRT; Invest Like the Best EP.486 (Benchmark's Eric Vishria, "Sandcastles & Silicon" show notes + partial transcript), Unchained (Sam MacPherson post-KelpDAO post-mortem via crypto.news coverage), and MLST late catch-up (Matthieu Wyart on abstraction/hierarchy) via structured web pages; StarTalk (Super Solar Storms w/ Lika Guhathakurta — full transcript Patreon-gated, used show notes), TCAF (Tradeable Lows / Nvidia data center finance / Schwab-Expedia comebacks, via YouTube description), IGC bonus (Advice to young people), PMcC #200 (Sebastian Thrun singularity), and 硅谷101 E248 (Chinese FDE w/ Lingyang CEO) via show notes.
- Late catch-up (08-10): yesterday's run completed 23:56Z (16:56 PDT) but its RSS fetch ran at the top of the window and missed two episodes published later in the day — MLST "AI Is Learning at the Wrong Level of Abstraction" (Matthieu Wyart, structured podcasters.spotify.com page) and 硅谷101 E248 (fireside show notes). Written to the 08-10 vault dir; state entries dated 2026-08-10.
- State updated: last_run_date → 2026-08-11; +9 processed entries (08-11) + 2 processed entries (08-10 late); 0 new failures.
