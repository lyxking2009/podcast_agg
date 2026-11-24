#!/usr/bin/env python3
"""Summarize 2026-11-24 transcripts with DeepSeek and write markdown digests."""
import json, glob, os, re, sys, time, threading, urllib.request
from datetime import datetime, timezone

ROOT = "/Users/yuxinglin/repos/podcast_agg"
TDIR = os.path.join(ROOT, "data/transcripts_20261124")
OUTDIR = os.path.join(ROOT, "Podcast Summaries", "2026-11-24")
STATE = os.path.join(ROOT, "data/state.json")
KEY = "sk-ce9e0dec0ba044b085a483335c529f61"
MODEL = "deepseek-v4-pro"
MAXCHARS = 120000

SYS = """You summarize podcast episodes for a busy listener who wants to skim daily.
You will be given the show title, episode title, and a transcript (or an episode description if a transcript was unavailable).
Preprocessing rules:

Ignore all sponsor segments, ad reads, and host-read promotions. Do not extract quotes, key points, or any content from these segments.
If the transcript is fewer than 200 words or clearly incomplete, return a JSON object where all fields are null and include an additional "error" field with a brief explanation.
If the transcript is in a language other than English, summarize in English regardless.

Produce a structured JSON object with EXACTLY these fields and nothing else:
{
  "tldr": "2-3 sentences, max 200 words. Headline summary of the episode.",
  "key_points": ["As many bullets as the content actually warrants. Each bullet must be a distinct claim, finding, or recommendation. Make each bullet informative and self-contained: up to ~200 words, with specific names, numbers, mechanisms, and reasoning."],
  "notable_quotes": [{"speaker": "Name, or 'Host' or 'Guest' if unidentifiable — never null", "quote": "verbatim quote", "approx_timestamp": "HH:MM:SS or null"}],
  "people_mentioned": ["names of notable people discussed or interviewed"],
  "topics": ["3-7 lowercase hyphenated tags, e.g. 'venture-capital', 'mental-health'"],
}
Field-level rules:
tldr: max 200 words. Be concrete — prefer specific names, numbers, and claims over vague summaries.
key_points: emit as many bullets as the content genuinely supports. Each bullet must be a distinct claim, finding, or recommendation.
notable_quotes: must be verbatim. If the input is a description rather than a transcript, return an empty array. Never use null for speaker.
topics: lowercase and hyphenated. Consistent across episodes.

Output ONLY the JSON object. No prose before or after, no Markdown fences."""

lock = threading.Lock()


def slugify(s, maxlen=200):
    s = re.sub(r"[^a-z0-9]+", "-", (s or "").lower()).strip("-")
    return s[:maxlen].strip("-")


def fmt_duration(d):
    d = (d or "").strip()
    if not d:
        return ""
    if re.fullmatch(r"\d+", d):
        sec = int(d)
    else:
        parts = [p for p in re.split(r"[:.]", d) if p.strip().isdigit()]
        if len(parts) == 3:
            h, m, s = map(int, parts)
            sec = h * 3600 + m * 60 + s
        elif len(parts) == 2:
            m, s = map(int, parts)
            sec = m * 60 + s
        else:
            return d
    h, rem = divmod(sec, 3600)
    m, s = divmod(rem, 60)
    return (f"{h}h" if h else "") + (f"{m}m" if h or m else "") + f"{s}s"


def truncate(t):
    if len(t) <= MAXCHARS:
        return t
    head = int(MAXCHARS * 0.78)
    return t[:head] + "\n\n[... middle of transcript omitted ...]\n\n" + t[-(MAXCHARS - head):]


def call_deepseek(rec):
    txt = truncate(rec["transcript"])
    user = (f"Show: {rec['show']}\nEpisode: {rec['title']}\n--- TRANSCRIPT BEGIN ---\n"
            f"{txt}\n--- TRANSCRIPT END ---\n\nReturn the JSON object now.")
    body = {"model": MODEL,
            "messages": [{"role": "system", "content": SYS}, {"role": "user", "content": user}],
            "temperature": 0.3, "max_tokens": 8192, "response_format": {"type": "json_object"}}
    data = None
    last = None
    for attempt in range(3):
        body["max_tokens"] = 8192 if attempt == 0 else 16384
        data = json.dumps(body).encode()
        try:
            req = urllib.request.Request("https://api.deepseek.com/chat/completions", data=data,
                                         headers={"Authorization": f"Bearer {KEY}",
                                                  "Content-Type": "application/json"})
            with urllib.request.urlopen(req, timeout=600) as r:
                d = json.loads(r.read())
            content = d["choices"][0]["message"].get("content") or ""
            content = content.strip()
            if content.startswith("```"):
                content = re.sub(r"^```[a-z]*\n?|\n?```$", "", content).strip()
            obj = json.loads(content)
            if obj.get("tldr") is None:
                raise ValueError(f"all-null summary: {obj.get('error')}")
            return obj
        except Exception as e:
            last = e
            print(f"  retry {attempt+1} {type(e).__name__}: {str(e)[:160]}", flush=True)
            time.sleep(3 * (attempt + 1))
    raise RuntimeError(str(last)[:300])


def front_source(src):
    s = (src or "").lower()
    if s.startswith("rss"):
        return "rss"
    if s.startswith("description"):
        return "description"
    return "web"


def write_md(rec, s):
    os.makedirs(OUTDIR, exist_ok=True)
    show_slug = slugify(rec["show"], 60)
    ep_slug = slugify(rec["title"], 80)
    path = os.path.join(OUTDIR, f"{show_slug}__{ep_slug}.md")
    w = ""
    if rec["transcript_source"] == "description":
        w = "\n> ⚠️ Based on episode description only — full transcript was unavailable.\n"
    lines = [
        "---",
        f'podcast: "{rec["show"].replace(chr(34), chr(39))}"',
        f'episode: "{rec["title"].replace(chr(34), chr(39))}"',
        f'published: {rec["pub_date"]}',
        f'duration: "{fmt_duration(rec.get("duration"))}"',
        f'audio_url: "{rec.get("audio_url") or ""}"',
        f'episode_url: "{rec.get("episode_url") or ""}"',
        f'transcript_source: {front_source(rec["transcript_source"])}',
        f'generated_at: {datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")}',
        f'model: "{MODEL}"',
        f'guid: "{rec["guid"]}"',
        "---",
        "",
        f'# {rec["title"]} — {rec["show"]}',
        w,
        "## TL;DR",
        s.get("tldr") or "",
        "",
        "## Key points",
    ]
    for b in s.get("key_points") or []:
        lines.append(f"- {b}")
    quotes = s.get("notable_quotes") or []
    if quotes and rec["transcript_source"] != "description":
        lines += ["", "## Notable quotes"]
        for q in quotes:
            if not isinstance(q, dict) or not q.get("quote"):
                continue
            ts = q.get("approx_timestamp")
            tail = f" (~{ts})" if ts else ""
            lines.append(f'> "{q["quote"]}" — {q.get("speaker") or "Host"}{tail}')
    people = s.get("people_mentioned") or []
    if people:
        lines += ["", "## People mentioned"]
        for p in people:
            lines.append(f"- {p}")
    topics = s.get("topics") or []
    if topics:
        lines += ["", "## Topics", " ".join(f"`{t}`" for t in topics)]
    open(path, "w").write("\n".join(lines) + "\n")
    return path


def update_state(guid, rec, ok, reason=None):
    with lock:
        st = json.load(open(STATE))
        if ok:
            st["processed"][guid] = {"date": "2026-11-24",
                                     "transcript_source": rec["transcript_source"],
                                     "show": rec["show"]}
            st["failures"] = [f for f in st.get("failures", []) if f.get("guid") != guid]
        else:
            fl = [f for f in st.get("failures", []) if f.get("guid") != guid]
            fl.append({"guid": guid, "title": rec["title"], "reason": reason,
                       "first_seen": "2026-11-24", "attempts": 1})
            st["failures"] = fl
        tmp = STATE + ".tmp"
        json.dump(st, open(tmp, "w"), indent=2, ensure_ascii=False)
        os.replace(tmp, STATE)


def work(rec):
    try:
        s = call_deepseek(rec)
        path = write_md(rec, s)
        update_state(rec["guid"], rec, True)
        return "ok", path
    except Exception as e:
        update_state(rec["guid"], rec, False, f"summarize: {str(e)[:200]}")
        return "fail", str(e)[:200]


def main():
    import concurrent.futures
    recs = [json.load(open(p)) for p in glob.glob(os.path.join(TDIR, "*.json"))]
    recs.sort(key=lambda r: r["pub_date_utc"])
    st = json.load(open(STATE))
    done = {g for g, v in st["processed"].items() if v.get("date") == "2026-11-24"}
    recs = [r for r in recs if r["guid"] not in done]
    only = set(sys.argv[1:])
    if only:
        recs = [r for r in recs if r["guid"] in only]
    print(f"summarizing {len(recs)} (skip {len(done)} already done)", flush=True)
    stats = {"ok": 0, "fail": 0}
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:
        futs = {ex.submit(work, r): r for r in recs}
        for fut in concurrent.futures.as_completed(futs):
            r = futs[fut]
            try:
                status, info = fut.result()
            except Exception as e:
                status, info = "fail", str(e)
            stats[status] += 1
            print(f"[{status}] {r['show'][:30]:30s} | {r['title'][:44]} | {info[-80:]}", flush=True)
    print(stats)


if __name__ == "__main__":
    main()
