# What the daily podcast digest actually does

A friendly tour for someone seeing this project for the first time. The daily job is one little tool that, end to end, does this: it reads the podcasts you're subscribed to in the macOS Podcasts app, finds new episodes, gets a transcript any way it can, asks an AI to summarize each one, saves the summaries as Markdown files, and commits them to git. Below is the whole story, step by step.

## When it runs

Nothing inside the program decides "now is the time." Some external scheduler (a launchd job, cron, or just you typing the command) kicks it off. There used to be a launchd agent that fired at 5 PM local time every day; it has been removed, so currently the job only runs when you ask it to.

You can always run it yourself:

```
./target/release/daily-digest
```

There are flags for backfilling a past day, narrowing to one show, skipping the commit, and so on — see the README. Everything below describes a default run.

## Step 1 — Wake up and read the config

The program starts up and gets oriented:

- It loads any environment variables in `.env` (most importantly your DeepSeek API key).
- It reads `data/config.json` for things like the timezone, the AI model to use, retry limits, and how many days back it's allowed to look.
- It reads `data/state.json` so it remembers what it already did on previous runs.
- It figures out what "today" means in your configured timezone.

If the API key is missing, the program stops right here. Better to fail loudly than to grind through feeds and crash later.

## Step 2 — Figure out which days to cover

The job isn't just looking at today. It picks a small window of dates so that if you missed a few days — laptop closed, network down, whatever — the next run catches up automatically.

The rule, in plain English:

- Start from the day after the last successful run.
- But never reach back further than the look-back cap (currently 7 days).
- And of course, never start in the future.

If this is the very first run ever, it covers yesterday plus today. Either way, the window then becomes a UTC start/end timestamp so it can be compared against episode publish times later.

## Step 3 — Refresh the subscription list from the macOS Podcasts app

Unless you tell it not to, the program peeks at the macOS Podcasts app's internal database. It copies the file to a temporary spot first (so it doesn't fight with the live app), opens the copy read-only, and pulls out every show you're subscribed to: title, author, RSS feed URL, and the show's website.

That list gets written to `data/subscriptions.json`. If you're on a non-Mac machine, or Podcasts isn't installed, the read just fails quietly — the program keeps using whatever's already in the file from a previous run.

## Step 4 — Fetch all the feeds, in parallel

Now it has the list of shows. For each show, it downloads the RSS feed and parses out every episode. Up to eight feeds are fetched at the same time, so even a long subscription list finishes quickly. For each episode it pulls out the title, publish date, audio URL, episode page URL, duration, description, and any transcript URLs the show declares directly in the feed.

Episodes whose publish dates fall outside the catch-up window are dropped right here. A feed that fails to download — 404, timeout, malformed XML — is logged and skipped. One bad feed can't take down the run.

## Step 5 — For each new episode, try to get a transcript

This is the part that does the most work. Most podcasts don't ship transcripts, so the program has three "rungs" of a ladder and stops on the first one that works.

**Rung 1 — the RSS feed itself.** A small but growing number of podcasts (the ones following the modern Podcasting 2.0 spec) include a transcript URL right in their feed. If so, the program just downloads it and converts whatever format it finds — subtitles in SRT or VTT, structured JSON, or HTML — into plain text.

**Rung 2 — ask Claude to search the web.** If the feed didn't help, the program shells out to the `claude` command-line tool and asks it to find a transcript online. The instructions tell Claude to first try fetching the episode page directly, then the show's main website, then a focused web search using the episode title and the show name, and finally transcript aggregator sites like podscribe and listennotes — and if it finds a real transcript, print it plain, otherwise print the token `NOT_FOUND`. Claude gets up to 12 minutes to come back with something. Whatever it returns is sanity-checked: it has to be at least 600 characters with at least 5 lines, and it can't say `NOT_FOUND`. If the `claude` CLI isn't installed, this whole rung is silently skipped.

**Rung 3 — fall back to the episode description.** If neither of the above worked, the program strips HTML tags out of the RSS description and uses that as the "transcript." The resulting summary will carry a visible warning at the top ("based on description only — full transcript was unavailable") and the notable-quotes section will be left empty, because nothing quoted from a marketing blurb is a real quote from the show.

Whichever rung wins, the resulting text is capped at about 600,000 characters before being passed to the AI — a comfortable ceiling that easily covers a 4-hour episode.

If all three rungs fail, the episode is recorded as a failure. It will be retried on the next run; after three total failed attempts it gets marked as permanently "skipped" so the job stops wasting time on it.

## Step 6 — Send the transcript to DeepSeek for a summary

The program calls DeepSeek's chat API with the model named in the config (currently `deepseek-v4-pro`), the transcript, and the system prompt printed below. It sets the temperature low (0.3, for steady, repeatable output), asks for a JSON response, and gives the model up to 8,192 tokens. If the call hits a transient hiccup — a connection drop, a half-decoded body, a timeout — it gets automatically retried once.

In the very rare case where a transcript somehow exceeds two million characters, the program splits it into chunks, summarizes each chunk separately, and then asks the model one more time to merge the partial summaries into one. This is a safety net you'll almost never see fire.

### The exact prompt the model receives

This is the system prompt, verbatim:

```text
You summarize podcast episodes for a busy listener who wants to skim daily.
You will be given the show title, episode title, and a transcript (or an episode description if a transcript was unavailable).
Preprocessing rules:

Ignore all sponsor segments, ad reads, and host-read promotions. Do not extract quotes, key points, or any content from these segments.
If the transcript is fewer than 200 words or clearly incomplete, return a JSON object where all fields are null and include an additional "error" field with a brief explanation.
If the transcript is in a language other than English, summarize in English regardless.

Produce a structured JSON object with EXACTLY these fields and nothing else:
{
  "tldr": "2-3 sentences, max 200 words. Headline summary of the episode.",
  "key_points": ["As many bullets as the content actually warrants — use more when the episode covers many distinct topics, and use fewer (even just a handful) when the episode is short or narrow. Each bullet must be a distinct claim, finding, or recommendation — not a restatement of the episode premise or topic. Make each bullet informative and self-contained: up to ~200 words, with specific names, numbers, mechanisms, and reasoning rather than vague gestures."],
  "notable_quotes": [{"speaker": "Name, or 'Host' or 'Guest' if unidentifiable — never null", "quote": "verbatim quote", "approx_timestamp": "HH:MM:SS or null"}],
  "people_mentioned": ["names of notable people discussed or interviewed"],
  "topics": ["3-7 lowercase hyphenated tags, e.g. 'venture-capital', 'mental-health'"],
}
Field-level rules:

tldr: max 200 words. Be concrete — prefer specific names, numbers, and claims over vague summaries.
key_points: do not target a fixed count — emit as many bullets as the content genuinely supports. A short or thin episode may justify only a few bullets; a wide-ranging one may justify many. Each bullet must be a distinct claim, finding, or recommendation — no filler, no restatements of the episode premise. Make each bullet informative and standalone (up to ~200 words): name the specific people, numbers, mechanisms, examples, and reasoning involved, so a reader who skims only this section understands the substance without listening.
notable_quotes: must be verbatim. Prefer quotes that are surprising, counterintuitive, or the clearest expression of the episode's central argument. If the input is a description rather than a transcript, return an empty array. Never use null for speaker — fall back to "Host" or "Guest".
topics: lowercase and hyphenated. Consistent across episodes (e.g. always "artificial-intelligence", not "AI" or "machine learning").
content_warnings: flag graphic content, heavy news, distressing topics, or anything a listener may want a heads-up about. Return an empty array if there are none.

Output ONLY the JSON object. No prose before or after, no Markdown fences.
```

The user message that accompanies the prompt is just the show name, the episode name, and the transcript wrapped in `--- TRANSCRIPT BEGIN ---` / `--- TRANSCRIPT END ---` markers, plus a final line saying "Return the JSON object now."

## Step 7 — Write the summary to a Markdown file

The model's JSON response gets rendered into a Markdown file under `Podcast Summaries/<published-date>/<show-slug>__<episode-slug>.md`. Each file starts with a small YAML front-matter block (show name, episode name, publish date, duration, audio URL, episode page URL, where the transcript came from, when it was generated, which model produced it, and the episode's GUID), followed by these sections in order:

- A heading: `# <episode title> — <show>`
- An optional warning banner, only if the summary was based on the description rather than a real transcript
- **TL;DR** — the headline 2–3 sentences
- **Key points** — bullet-by-bullet substance
- **Notable quotes** — with speaker and rough timestamp where known
- **People mentioned**
- **Topics** — short tags

Show and episode names get slugified for the filename, and very long titles are truncated.

## Step 8 — Remember what was done

After each episode, the program saves `data/state.json` immediately, so a crash mid-run never loses progress. The state file records which episodes were processed (and by which transcript route), and which ones failed (with the reason and an attempt counter). When the whole pass is done, the program stamps `last_run_date` and `last_run_completed_at` so tomorrow's catch-up window starts from the right place.

## Step 9 — Commit and push to git

Finally, unless you passed `--no-commit`, the program runs git on your behalf: it stages the `Podcast Summaries` folder and `data/state.json`, makes a commit with a message like `digest: 2026-05-08 (4 episodes, window 2026-05-07..2026-05-08)`, and pushes — unless you also passed `--no-push`. A failed push is logged but doesn't fail the run; your commit stays safely on the local branch.

---

That's the whole loop. If you remember just one sentence, remember this: **the daily job reads your subscriptions, finds new episodes, gets a transcript any way it can, asks DeepSeek for a structured summary, saves it as Markdown, and commits the result.**
