# podcast_agg

Daily summarizer for podcasts you subscribe to in the macOS Podcasts app. Fetches new episodes from each show's RSS feed, gets a transcript (or falls back to the episode description), summarizes with DeepSeek, and writes one Markdown file per episode under `Podcast Summaries/YYYY-MM-DD/`.

## How it works

1. **Discover** — refresh `data/subscriptions.json` from the macOS Podcasts app (skip with `--no-refresh-subs`), then fetch each show's RSS feed in parallel. Keep only episodes published inside the catch-up window.
2. **Transcript** — try in order, stop at the first hit:
   1. RSS-declared transcript URL (Podcast 2.0 SRT / VTT / JSON / HTML).
   2. Headless `claude` CLI invoked with `WebSearch` + `WebFetch` to find a published transcript on the web.
   3. The episode's RSS description (HTML stripped).
3. **Summarize** — DeepSeek (`deepseek-v4-pro` by default) emits a structured JSON summary (`tldr`, `key_points`, `notable_quotes`, `people_mentioned`, `topics`). When the description fallback was used, the output is flagged and the quotes section is left empty.

State (last run date, processed episode GUIDs, failures) is persisted to `data/state.json` (gitignored). If a daily run is missed, the next run processes everything from the day after the last successful run through today, capped at `lookback_cap_days` (default 7). Failed episodes retry up to `max_attempts_per_episode` (default 3) before being marked as skipped.

## Prerequisites

- macOS with the Podcasts app, used by both `refresh-subs` and `daily-digest`'s auto-refresh. On non-macOS hosts pass `--no-refresh-subs` and supply your own `data/subscriptions.json`.
- Rust 1.75+ and `cargo`.
- `claude` CLI on `PATH` for the web-search transcript rung. If missing, that rung is skipped silently and the run falls through to the description fallback.
- A DeepSeek API key.

## Setup

```bash
# 1. build
cargo build --release

# 2. configure the API key (either works)
echo 'DEEPSEEK_API_KEY=sk-...' > .env       # auto-loaded by the binary
# or:
export DEEPSEEK_API_KEY=sk-...
```

`daily-digest` automatically refreshes `data/subscriptions.json` from the macOS Podcasts SQLite library (`~/Library/Group Containers/…/MTLibrary.sqlite`) at the start of every run. Pass `--no-refresh-subs` to skip the refresh — useful when Podcasts isn't installed (e.g., on a CI box), the library file is unreadable, or you want a fully offline run. The standalone `refresh-subs` binary does the same export without running the digest, if you want to refresh without producing summaries.

## Running the daily digest

```bash
# default: process the catch-up window, then commit & push the summaries
cargo run --release --bin daily-digest
```

### CLI flags

| Flag | Description |
|---|---|
| `--repo <PATH>` | Project root. Defaults to the current directory. |
| `--date <YYYY-MM-DD>` | Override "today" (interpreted in `config.timezone`). Useful for backfills or deterministic runs. |
| `--only <TEXT>` | Case-insensitive substring filter on show title. |
| `--no-commit` | Write summaries and update state, but skip `git add` / `git commit` / `git push`. |
| `--no-push` | Commit, but skip `git push`. |
| `--no-refresh-subs` | Skip the auto-refresh of `data/subscriptions.json` at the start of the run. |

### Examples

```bash
# dry-run a single show, no git mutations
cargo run --release --bin daily-digest -- --only "Acquired" --no-commit

# backfill a specific past day
cargo run --release --bin daily-digest -- --date 2026-04-30 --no-push

# normal daily run, but skip pushing (e.g. no remote configured yet)
cargo run --release --bin daily-digest -- --no-push
```

To narrow the window to a single day, set `data/state.json`'s `last_run_date` to yesterday before invoking, or pass `--date` while keeping `lookback_cap_days` at 1 in the config.

## Output

Summaries land at `Podcast Summaries/<published_date>/<show-slug>__<episode-slug>.md`. Each file is YAML frontmatter followed by sections:

```
---
podcast: "<title>"
episode: "<title>"
published: YYYY-MM-DD
duration: 1h27m22s
audio_url: "..."
episode_url: "..."
transcript_source: rss | web | description | skipped
generated_at: <RFC 3339>
model: "<model name>"
guid: "<guid>"
---

# <episode> — <show>

## TL;DR
…

## Key points
- …

## Notable quotes
> "…" — Speaker (~HH:MM:SS)

## People mentioned
- …

## Topics
`tag` `tag` `tag`
```

## Layout

```
src/
├── lib.rs              # shared types (Subscription, Episode, TranscriptSource)
├── config.rs           # data/config.json loader
├── state.rs            # state + catch-up window logic
├── rss.rs              # feed parsing
├── transcript.rs       # three-rung transcript fetcher
├── summarize.rs        # DeepSeek client (with map-reduce for very long inputs)
├── writer.rs           # Markdown output
├── git_sync.rs         # commit & push
├── sqlite_reader.rs    # Apple Podcasts MTLibrary.sqlite reader
└── bin/
    ├── refresh_subs.rs
    └── daily_digest.rs
data/
├── config.json         # tunables (see below)
├── subscriptions.json  # auto-refreshed by daily-digest (also writable via refresh-subs)
└── state.json          # generated/updated by daily-digest (gitignored)
Podcast Summaries/      # generated by daily-digest (gitignored)
```

## Configuration (`data/config.json`)

| Field | Description |
|---|---|
| `timezone` | IANA TZ used for day boundaries when computing the catch-up window. |
| `lookback_cap_days` | Maximum days the catch-up window can extend back. |
| `model` | DeepSeek model name. |
| `max_concurrent_feeds` | Parallel RSS fetches. |
| `max_attempts_per_episode` | Retry budget per episode before marking it skipped. |
| `web_search_timeout_secs` | Per-episode time budget for the `claude` CLI subprocess. |
