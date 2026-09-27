# Podcast Summaries — 2026-09-15

- **Episodes in vault:** 4
- **Run:** 2026-09-15 15:00 PT cron (manual fallback — Claude Code OAuth expired for the 12th consecutive day, 08-08 → 09-15)
- **Coverage:** 4/4 episodes (100%)
- **Prior-date diff:** 2026-09-14 re-fetched, 7 episodes — **all already in `state.json`**, no late additions
- **Feed errors:** 5 — Bankless, Latent Space, Chalk Radio, Critics at Large, The Edge. Bankless and Latent Space were both recovered by direct feed fetch; **neither has an in-window episode**. The other three were not re-searched (manual-mode budget) — all three were confirmed empty-window yesterday (Chalk Radio latest 2026-03-05, Critics at Large 2026-09-03, The Edge 2026-07-07).

## Episodes

- [[machine-learning-street-talk-mlst__how-physical-ai-learns-across-language-video-and-action-ming-yu-liu]]  ·  `youtube_autocaptions` (YouTube auto-captions — duration verified 1558s vs RSS 1557s)
- [[startalk-radio__why-do-we-exist-with-hakeem-oluseyi]]  ·  `show_notes` (episode page show notes; full transcript Patreon-gated at $5)
- [[training-data__boxs-aaron-levie-on-reinventing-yourself-in-the-ai-age-and-enterprise-diffusion]]  ·  `youtube_autocaptions` (YouTube auto-captions, 710KB VTT)
- [[unchained__guy-young-on-why-ethena-launched-a-neobank-on-top-of-its-stablecoin]]  ·  `web` (published coverage of the EthenaPay launch; Unchained's YouTube upload carries no auto-captions)

## Feed errors

Both failing Flightcast feeds were checked by direct curl (`Accept-Encoding: identity`). Neither published an in-window episode.

- **Bankless** — Flightcast ~167-byte error (`XML: mismatched tag: line 6, column 2`). Direct fetch OK (22.1MB, 1372 items). Latest = **2026-09-14** *Crypto is Ready for Onchain Options | Nick Forster, CEO of Derive* — **no in-window episode** (already covered in the 2026-09-14 vault).
- **Latent Space: The AI Engineer Podcast** — Flightcast ~167-byte error. Direct fetch OK (13.4MB, 222 items). Latest = **2026-09-14** *Humanity's Last Invention — Richard Socher of Recursive* — **no in-window episode** (already covered in the 2026-09-14 vault).
- **Chalk Radio** — timeout. Not re-fetched (manual-mode budget); latest confirmed 2026-03-05 — none in window.
- **Critics at Large | The New Yorker** — timeout. Not re-fetched (manual-mode budget); latest confirmed 2026-09-03 — none in window.
- **The Edge** — timeout. Not re-fetched (manual-mode budget); latest confirmed 2026-07-07 — none in window.

## Unresolved failures

- None added today. All 4 in-window episodes were processed.

## Notes

- **Claude Code was unavailable again.** `run_podcast_pipeline.py --dates "2026-09-15"` exited with `Failed to authenticate: OAuth session expired and could not be refreshed` (12th consecutive day). Manual pipeline fallback completed all 4 episodes.
- **Episode count differed between fetches.** My standalone `fetch_episodes_parallel.py` returned 4 episodes / 5 feed errors; the pipeline's internal re-fetch saw 3 episodes / 15 feed errors. Per the documented inconsistency note, the episode count was trusted over the error count and the fuller fetch was used.
- **`yt-dlp ytsearch` + duration verification paid off twice.** Neither MLST nor Unchained surfaced via `site:youtube.com` search. Direct `yt-dlp --flat-playlist --print "%(id)s | %(title)s | %(duration)s"` queries found MLST at 1558s (RSS 1557s) and Unchained at 3129s (RSS 3128s) — both accepted in a single call with no wasted searches. The Unchained full upload carries a **different YouTube title** (*Guy Young on How to Build a 10 Times Better Product on Blockchain Rails*), so an RSS-title search would have missed it.
- **Unchained had no usable transcript.** The duration-verified YouTube upload exposes **no subtitles at all** (`There are no subtitles for the requested languages`, confirmed via `--list-subs` and an `en.*,en-orig` attempt). The show's own pages are description-only, so the episode was summarised from published coverage of the EthenaPay launch, labelled `web` with a scope note in the file.
- **StarTalk was Patreon-gated (second time this pattern has appeared).** `startalkmedia.com/show/why-do-we-exist-with-hakeem-oluseyi` renders the episode description and full tag list, but gates the verbatim transcript behind the $5 Patreon tier, and no matching StarTalk YouTube upload exists yet (searches surfaced only older Oluseyi episodes). Summarised as `show_notes`; the `## Notable quotes` section is omitted because the source carries no speaker-attributed quotes.
- **MLST link hub bypassed the blocked domain again.** MLST is Anchor.fm-hosted and its own domain is blocked by `web_extract`; the RSS `episode_url` (`podcasters.spotify.com/...`) resolved directly to the show page, from which the YouTube ID was obtained. Note the episode is a **paid partnership with NVIDIA**, disclosed in the summary.
- **Concurrent sibling run detected and reconciled.** A second agent (`model: deepseek-v4-pro`) wrote a duplicate Training Data file with an apostrophe-hyphenated slug (`box-s-aaron-levie` vs my `boxs-aaron-levie`, same GUID `d2e9aa74-b080-11f1-8007-f31e2b6b4e53`) and updated `state.json` with 3 processed entries before stopping (no `_daily_report.md`, no failures). Its entry set omitted StarTalk. Reconciliation: duplicate removed by GUID, report rewritten to cover all 4 episodes, state merged — StarTalk added, and the Training Data / Unchained `transcript_source` values corrected to the verified sources (`youtube_autocaptions` and `web` respectively; the sibling had recorded `web` and `description`).

## Late additions (recovered 2026-09-17)

Feed: 硅谷101 — E251 published 2026-09-15 (after the prior run's fetch):

- [[硅谷101__e251-推理芯片之战-聊聊groq-cerebras与openai三大路径与bill-dally的设计哲学]] — `show_notes`
