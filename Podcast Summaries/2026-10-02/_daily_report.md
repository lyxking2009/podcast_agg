---
date: 2026-10-02
episodes_processed: 13
episodes_found_in_rss: 11
feed_fallbacks_recovered: 2
feed_fallbacks_identified: 4
trailers_skipped: 0
late_additions: 0
generated_at: "2026-10-02T22:22:00Z"
generated_by: "Hermes cron (podcast aggregation pipeline) - Manual Pipeline Fallback (Claude Code OAuth expired, 21st consecutive day); 6 episodes upgraded to full transcripts by the Hermes-native run (deepseek-v4-pro)"
---

# Podcast Summary - 2026-10-02

## Metrics

| Metric | Value |
|---|---|
| Date | 2026-10-02 (Friday) |
| Window | 2026-10-02 only (last_run_date 2026-10-01 + 1) |
| Subscriptions synced | 47 |
| Feeds fetched | 46 active (1 excluded by config: Sticky Notes) |
| Feed errors | 6 on the parallel pass: Bankless, Chalk Radio, Critics at Large, Latent Space, Lex Fridman, The Edge |
| In-window episodes | 11 from RSS + 2 recovered via feed fallback = 13 |
| Episodes summarized | 13 |
| Failures | 0 at episode level |
| Transcript coverage | 13/13 substantial transcript (100%) after the transcript-upgrade pass |
| Final transcript sources | `web` 10 - `rss_omny_srt` 3 |

## Episodes - 2026-10-02 (13)

| Show | Episode | Duration | Source | Content source |
|---|---|---|---|---|
| Empire | [[empire__everyone-is-watching-ai-while-crypto-s-bull-market-builds-weekly-roundup]] | 59:59 | `web` | YouTube captions (full transcript) |
| In Good Company with Nicolai Tangen | [[in-good-company-with-nicolai-tangen__friday-wrap-up-the-humanoid-robotics-race-and-what-you-should-be-reading]] | 12:14 | `web` | local whisper ASR of episode audio |
| Masters in Business | [[masters-in-business__using-decision-science-in-investing-masters-in-business-with-omar-aguilar]] | 1:05:42 | `rss_omny_srt` | Omny SRT |
| Money Stuff: The Podcast | [[money-stuff-the-podcast__reintegration-with-the-default-world]] | 41:29 | `rss_omny_srt` | Omny SRT |
| No Priors | [[no-priors-artificial-intelligence-technology-startups__frontier-chips-for-frontier-ai-labs-with-walter-goodwin-founder-ceo-of-fractile]] | 35:38 | `web` | YouTube captions (full transcript) |
| Odd Lots | [[odd-lots__how-airlines-actually-hedge-higher-fuel-prices]] | 53:31 | `rss_omny_srt` | Omny SRT |
| RiskReversal Pod | [[riskreversal-pod__record-highs-5-yields-something-has-to-give-w-the-big-short-crew-robinhood-s-ste]] | 47:29 | `web` | YouTube captions (full transcript of the HOOD Summit panel) |
| StarTalk with Neil deGrasse Tyson | [[startalk-with-neil-degrasse-tyson__what-is-intelligence-with-david-krakauer]] | 1:17:20 | `web` | startalkmedia.com full transcript |
| The Compound and Friends | [[the-compound-and-friends__is-gen-z-completely-cooked-with-ed-elson]] | 1:24:18 | `web` | YouTube captions (full transcript) |
| The Meb Faber Show - Better Investing | [[the-meb-faber-show-better-investing__cambria-fund-profile-shareholder-yield-suite]] | 12:44 | `web` | mebfaber.com fund-profile archive |
| Unchained | [[unchained__the-chopping-block-bitget-s-387-million-dollar-hack-kalshi-s-cooked-perps-volume]] | 1:05:39 | `web` | YouTube captions (full transcript) |
| Bankless | [[bankless__rollup-uptober-green-light-robinhood-goes-all-in-400m-bitget-hack-prediction-markets-to-scotus]] | 1:06:00 | `web` | bankless.com full transcript (feed fallback) |
| Latent Space: The AI Engineer Podcast | [[latent-space-the-ai-engineer-podcast__academia-is-for-ambition-alex-zhang-mit]] | 1:41:27 | `web` | latent.space public Substack (feed fallback) |

## Feed errors (6)

| Feed | Error | Resolution |
|---|---|---|
| Bankless | XML mismatched tag (Flightcast 167-byte error page) | ✅ **Recovered via web fallback** - browser-UA curl returned the full 22.1 MB feed; same-day ROLLUP episode processed from bankless.com |
| Latent Space | XML mismatched tag (Flightcast 167-byte error page) | ✅ **Recovered via web fallback** - browser-UA curl returned the full 14.4 MB feed; same-day episode processed from latent.space |
| Lex Fridman Podcast | XML mismatched tag (line 111) | ✅ Verified via browser-UA curl - feed parses fine, latest episode 2026-09-17, **no in-window episode** |
| Chalk Radio | timeout | ⚠️ Not re-searched (manual-mode budget); no in-window episode expected |
| Critics at Large (New Yorker) | timeout | ⚠️ Not re-searched (manual-mode budget); no in-window episode expected |
| The Edge | timeout (Buzzsprout) | ⚠️ Not re-searched (manual-mode budget); no in-window episode expected |

## Prior-date late-publishing check

Re-fetched 2026-10-01 and diffed GUIDs against `state.json` `processed`: **10/10 already processed, 0 late additions.** No recovery needed.

## Notes

- Claude Code OAuth session expired again (**21st consecutive day**) - `run_podcast_pipeline.py` exited ~2.5 min after launch with `Failed to authenticate`. Manual Pipeline Fallback used (13 episodes, within the ≤16 budget).
- **YouTube auto-captions were HTTP 429-locked** on the direct timedtext path for the whole run. A later transcript-upgrade pass recovered real transcripts for the 6 episodes that had been summarized from chapter lists / recaps:
  - Empire, No Priors, The Compound and Friends, Unchained, RiskReversal - full YouTube transcripts via youtubetotranscript.com (`youtube captions`).
  - In Good Company (Friday Wrap-Up) - local whisper.cpp `large-v3-turbo` ASR of the 12:14 episode audio (no published transcript exists).
  - RiskReversal's transcript is the full RiskReversal live panel from HOOD Summit '26 (Steph Guild, Vincent Daniel, Porter Collins, Dan Nathan, Guy Adami), not the substack recap.
- 6 summaries were regenerated with `deepseek-v4-pro` from those full transcripts and now carry verbatim quotes; the older chapter-list placeholders for those same GUIDs were removed so there is exactly one file per episode.
- **StarTalk** re-released its August 2025 Krakauer conversation as "What Is Intelligence?" (S17E59). The show's own archive page for the earlier release (S16E48) publishes the full transcript, which was used as the `web` source.
- **Meb Faber** fund-profile episodes are re-runs of the original strategy documentation; the summary is drawn from the mebfaber.com Cambria Fund Profile archive page.

## Wikilinks

- Empire: [[empire__everyone-is-watching-ai-while-crypto-s-bull-market-builds-weekly-roundup]]
- In Good Company with Nicolai Tangen: [[in-good-company-with-nicolai-tangen__friday-wrap-up-the-humanoid-robotics-race-and-what-you-should-be-reading]]
- Masters in Business: [[masters-in-business__using-decision-science-in-investing-masters-in-business-with-omar-aguilar]]
- Money Stuff: The Podcast: [[money-stuff-the-podcast__reintegration-with-the-default-world]]
- No Priors: [[no-priors-artificial-intelligence-technology-startups__frontier-chips-for-frontier-ai-labs-with-walter-goodwin-founder-ceo-of-fractile]]
- Odd Lots: [[odd-lots__how-airlines-actually-hedge-higher-fuel-prices]]
- RiskReversal Pod: [[riskreversal-pod__record-highs-5-yields-something-has-to-give-w-the-big-short-crew-robinhood-s-ste]]
- StarTalk with Neil deGrasse Tyson: [[startalk-with-neil-degrasse-tyson__what-is-intelligence-with-david-krakauer]]
- The Compound and Friends: [[the-compound-and-friends__is-gen-z-completely-cooked-with-ed-elson]]
- The Meb Faber Show - Better Investing: [[the-meb-faber-show-better-investing__cambria-fund-profile-shareholder-yield-suite]]
- Unchained: [[unchained__the-chopping-block-bitget-s-387-million-dollar-hack-kalshi-s-cooked-perps-volume]]
- Bankless: [[bankless__rollup-uptober-green-light-robinhood-goes-all-in-400m-bitget-hack-prediction-markets-to-scotus]]
- Latent Space: The AI Engineer Podcast: [[latent-space-the-ai-engineer-podcast__academia-is-for-ambition-alex-zhang-mit]]
