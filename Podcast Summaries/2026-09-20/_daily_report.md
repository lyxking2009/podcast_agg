---
date: 2026-09-20
episodes_processed: 3
episodes_found_in_rss: 4
feed_fallbacks_recovered: 0
generated_at: 2026-09-20T22:05:00Z
generated_by: "Hermes cron (podcast aggregation pipeline)"
---

# Podcast Summary — 2026-09-20

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-09-20 (Sunday) |
| Catch-up window | 2026-09-19 00:00 PT → 2026-09-20 23:59 PT (UTC 2026-09-19T07:00 → 2026-09-21T06:59:59) |
| Subscriptions synced | 47 (from macOS Podcasts MTLibrary.sqlite) |
| Episodes found via RSS | 4 |
| Episodes summarized | 3 |
| Episodes skipped | 1 (30-second cross-promo trailer) |
| Transcript coverage | 3/3 summarizable episodes (100%) |
| Feed errors | 0 |
| Late additions to 2026-09-19 | 1 (The Investor's Podcast — RWH072) |
| Model | deepseek-v4-pro |

Sunday volume is light — three substantive episodes plus one network cross-promo across all 47 feeds.

## Transcript sources

| Source | Count |
|---|---|
| RSS-declared (`podcast:transcript`) | 0 |
| Web (podscripts / YouTube captions / publisher transcript) | 3 |
| Episode description fallback | 0 |
| Skipped (trailer) | 1 |

Rung 1 struck out: the only RSS-declared transcripts this window belonged to the Odd Lots cross-promo (107 words — below the 150-word floor). All three real episodes were recovered at Rung 2:

- **The Investor's Podcast (RWH072)** — full 22,079-word transcript from podscripts.co (YouTube captions were the backup at 21,423 words).
- **Lenny's Podcast (Peter Sellis)** — 19,363 words via YouTube auto-captions (`97LRJUUPy_w`); the Substack post itself is paywalled.
- **Critics at Large / In the Dark ("The Journey," Ep. 1: Gifts)** — the official 8,862-word Condé Nast transcript, scraped from the episode page's `transcriptUrl` field on newyorker.com.

## Episodes summarized

| Show | Episode | Source | Words | File |
|---|---|---|---|---|
| The Investor's Podcast (We Study Billionaires) | RWH072: The Making of A Money Master w/ Rob Vinall | web (podscripts) | 22,079 | `2026-09-19/the-investor-s-podcast-we-study-billionaires-the-investor-s-podcast-network__rwh072-the-making-of-a-money-master-w-rob-vinall.md` |
| Lenny's Podcast: Product \| Career \| Growth | 90 minutes of unfiltered product advice from Snap and Discord's product chief \| Peter Sellis | web (YouTube captions) | 19,363 | `2026-09-20/lenny-s-podcast-product-career-growth__90-minutes-of-unfiltered-product-advice-from-snap-and-discord-s-product-chief-pe.md` |
| Critics at Large \| The New Yorker | Introducing: "The Journey," from In the Dark | web (Condé Nast transcript) | 8,862 | `2026-09-20/critics-at-large-the-new-yorker__introducing-the-journey-from-in-the-dark.md` |

The TIP episode published 2026-09-19 17:00 PT, so its summary is filed under the 2026-09-19 folder per the published-date convention; it was fetched in today's run because the feed item appeared after yesterday's cutoff.

## Skipped

| Show | Episode | Reason |
|---|---|---|
| Odd Lots | Introducing: Bloomberg Money | 30-second network cross-promo; RSS transcript is 107 words, description 157 chars. Recorded as `skipped_trailer`. |

## High spots

- **Rob Vinall (RV Capital)**: 15.5% annualized net since 2008 (>1,200% cumulative), ~10-stock portfolio, "owner return" framework, survived a 47.6% drawdown in 2022; now finding value in Chinese consumer/tech and beaten-down software.
- **Peter Sellis (Snap, Discord)**: teams designed like "terrorist organizations," why the median PM is net negative, Snap's ad-business structural constraints, and growth that comes from core users rather than new surfaces.
- **Ava Kofman / In the Dark**: how Guojun Xuan and Silvia Zhang assembled dozens of children via surrogates — with details of Kayla Elliott's whistle-blowing and the surrogacy industry's regulatory gaps.
