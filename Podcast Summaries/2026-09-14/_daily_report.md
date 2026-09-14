# Podcast Summaries — 2026-09-14

- **Episodes in vault:** 10
- **Run:** 2026-09-14 15:00 PT cron (manual fallback — Claude Code OAuth expired for the 11th consecutive day)
- **Coverage:** 10/10 episodes (100%)
- **Prior-date diff:** 2026-09-13 re-fetched, 5 episodes — **2 late additions recovered** (Acquired *The Home Depot*, The Rest Is History 705), see the 2026-09-13 vault
- **Feed errors:** 5 — Bankless, Latent Space, Chalk Radio, Critics at Large, The Edge. Bankless and Latent Space **both published in-window episodes**, recovered by direct feed fetch; the other three have no in-window episodes.

## Episodes

- [[animal-spirits-podcast__talk-your-book-why-aren-t-there-more-ipos]]  ·  `show_notes` (show notes (no transcript published))
- [[bankless__crypto-is-ready-for-onchain-options-nick-forster-ceo-of-derive]]  ·  `rss_vtt` (Flightcast RSS transcript (VTT) — recovered via direct feed fetch)
- [[empire__the-six-tokens-to-own-this-cycle-yan-liberman]]  ·  `youtube_autocaptions` (YouTube auto-captions)
- [[flirting-with-models__lucas-schuermann-swapping-out-perpetual-futures-s7e34]]  ·  `show_notes` (show notes (no transcript published))
- [[latent-space-the-ai-engineer-podcast__humanity-s-last-invention-richard-socher-of-recursive]]  ·  `web_substack` (Substack episode page)
- [[machine-learning-street-talk-mlst__speech-recognition-is-not-a-solved-problem-pavan-muddireddy]]  ·  `web` (structured web source)
- [[odd-lots__openai-president-greg-brockman-on-doing-business-in-the-wake-of-hugging-face]]  ·  `rss_omny_srt` (Omny RSS transcript (SRT))
- [[riskreversal-pod__danny-moses-debt-gold-and-the-fed-what-s-really-driving-markets]]  ·  `web_substack` (Substack episode page)
- [[the-peterman-pod__casey-muratori-surprises-in-computer-history-and-where-bad-code-comes-from]]  ·  `youtube_autocaptions` (YouTube auto-captions)
- [[wiser-world__103-seven-questions-to-ask-about-any-democracy]]  ·  `show_notes` (show notes (no transcript published))

## Feed errors

All five feeds errored in the parallel fetch; all five were checked by direct curl (`Accept-Encoding: identity`).

- **Bankless** — Flightcast ~167-byte error. Direct fetch OK (22.1MB, 1371 items). Latest = **2026-09-14** *Crypto is Ready for Onchain Options | Nick Forster, CEO of Derive* — **in window, recovered** (guid `flightcast:01M2FJXD36WWSVB61P3AZ9T0X9`, RSS `<podcast:transcript>` VTT `01M2FJXD36WWSVB61P3AZ9T0X9.vtt` fetched directly).
- **Latent Space: The AI Engineer Podcast** — Flightcast ~167-byte error. Direct fetch OK (13.4MB, 222 items). Latest = **2026-09-14** *Humanity's Last Invention — Richard Socher of Recursive* — **in window, recovered** from the public Substack page `latent.space/p/recursive`.
- **Chalk Radio** — timeout. Direct fetch OK (628KB). Latest = 2026-03-05 — **none in window**.
- **Critics at Large | The New Yorker** — timeout. Direct fetch OK (1.4MB). Latest = 2026-09-03 — **none in window**.
- **The Edge** — timeout. Direct fetch OK (126KB). Latest = 2026-07-07 — **none in window**.

## Late additions (2026-09-13, recovered today)

Both published after the 09-13 run's RSS fetch. Written into the 2026-09-13 vault and appended to that date's report.

- **Acquired** — *The Home Depot* (guid `68b66323-6e74-4c90-8157-ef3f1ed0d042`, 3h35m) · `rss_transistor` (full transcript from `share.transistor.fm/s/8ecb4ed4/transcript.txt`)
- **The Rest Is History** — *705. Queen Victoria's Revenge: The Mad Emperor of Abyssinia (Part 1)* (guid `5c0bf304-adf5-11f1-a227-3f46219e3faa`) · `show_notes` (description-only pages; no transcript published)

## Unresolved failures

- None added today.

## Notes

- **Claude Code was unavailable again.** `run_podcast_pipeline.py` exited ~20s after launch with `Failed to authenticate: OAuth session expired and could not be refreshed` (11th consecutive day). Manual pipeline fallback completed all 12 items (10 for today + 2 late 09-13).
- **Rung 1 paid off twice today.** Odd Lots supplied an Omny SRT via the RSS transcript tag, and the two feed-error shows both had non-search-dependent sources: Bankless publishes a `<podcast:transcript>` VTT on the Flightcast feed, and Latent Space's Substack page carries the full transcript. Because the feeds themselves failed, both were only reachable via a direct curl of the feed URL — the first time the Bankless VTT transcript tag has been usable.
- **YouTube duration verification worked as documented.** Empire's `ytsearch5` hit (NIv7MNKqnEQ) reported 3569s against the RSS `duration` of 3568s — accepted in the same call, no wasted searches.
- **Peterman Pod link-hub pattern confirmed again:** the `podcasters.spotify.com` episode page supplied both the YouTube ID (`jHLbL1Eg4gM`, 1.08MB VTT) and the developing.dev transcript link in a single `web_extract`, avoiding a YouTube search entirely.
- **MLST:** the `podcasters.spotify.com` link hub carried the full structured description plus a complete timestamp map and reference list — used directly as a `web` source (the show's own domain is blocked by `web_extract`).
- **Wiser World** has no YouTube upload and its site lags publication, so the episode was summarised from its own show notes, which list the seven democratic principles explicitly.
- **Flirting with Models** published an unusually complete episode description (RFQ/OLP mechanics, perp funding versus the swap structure) — used as `show_notes` rather than falling back to RSS description.
- **Item accounting:** the 2-day RSS fetch returned 13 episodes (3 already in `state.json`, 10 new). Two further episodes came from feed-error recovery (Bankless, Latent Space), giving **12 new episodes written** — 10 into the 2026-09-14 vault and 2 into the 2026-09-13 vault.
