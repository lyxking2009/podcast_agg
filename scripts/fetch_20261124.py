#!/usr/bin/env python3
"""Fetch all subscription feeds, filter to catch-up window, exclude processed GUIDs."""
import json, re, os, sys
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone, timedelta
from email.utils import parsedate_to_datetime
import urllib.request
import xml.etree.ElementTree as ET

ROOT = "/Users/yuxinglin/repos/podcast_agg"
NS = {
    "itunes": "http://www.itunes.com/dtds/podcast-1.0.dtd",
    "podcast": "https://podcastindex.org/namespace/1.0",
    "content": "http://purl.org/rss/1.0/modules/content/",
}

def load(p):
    with open(p) as f:
        return json.load(f)

subs = load(f"{ROOT}/data/subscriptions.json")
state = load(f"{ROOT}/data/state.json")
cfg = load(f"{ROOT}/data/config.json")
processed = set(state["processed"].keys())
excluded = set(cfg.get("excluded_feeds", []))

# window: from last successful completion through now (+6h buffer for timezone slop)
start_utc = datetime(2026, 11, 23, 23, 19, 48, tzinfo=timezone.utc)
end_utc = datetime.now(timezone.utc) + timedelta(hours=6)

UA = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120 Safari/537.36"

def fetch(feed_url):
    req = urllib.request.Request(feed_url, headers={"User-Agent": UA, "Accept": "application/rss+xml, application/xml, text/xml, */*"})
    with urllib.request.urlopen(req, timeout=40) as r:
        return r.read()

def parse_date(s):
    if not s:
        return None
    s = s.strip()
    try:
        d = parsedate_to_datetime(s)
        if d.tzinfo is None:
            d = d.replace(tzinfo=timezone.utc)
        return d.astimezone(timezone.utc)
    except Exception:
        pass
    for fmt in ("%Y-%m-%dT%H:%M:%S%z", "%Y-%m-%dT%H:%M:%SZ", "%Y-%m-%d"):
        try:
            d = datetime.strptime(s, fmt)
            if d.tzinfo is None:
                d = d.replace(tzinfo=timezone.utc)
            return d.astimezone(timezone.utc)
        except Exception:
            continue
    return None

def text(el):
    return re.sub(r"\s+", " ", "".join(el.itertext())).strip() if el is not None else ""

def handle_sub(sub):
    title = sub["title"]
    url = sub["feed_url"]
    out = {"show": title, "feed_url": url, "ok": False, "error": None, "episodes": []}
    try:
        raw = fetch(url)
    except Exception as e:
        out["error"] = f"fetch: {e}"
        return out
    try:
        root = ET.fromstring(raw)
    except Exception as e:
        out["error"] = f"parse: {e}"
        return out
    chan = root.find("channel")
    if chan is None and root.tag.endswith("feed"):
        chan = root
    if chan is None:
        out["error"] = "no channel"
        return out
    items = chan.findall("item")
    out["ok"] = True
    out["item_count"] = len(items)
    for it in items:
        guid = text(it.find("guid")) or text(it.find("link"))
        if not guid:
            continue
        pd = parse_date(text(it.find("pubDate")))
        if pd is None or not (start_utc <= pd <= end_utc):
            continue
        if guid in processed:
            continue
        enc = it.find("enclosure")
        audio = enc.get("url") if enc is not None else ""
        if not audio:
            for e in it.findall("media:content", {"media": "http://search.yahoo.com/mrss/"}):
                if (e.get("type") or "").startswith("audio"):
                    audio = e.get("url") or ""
                    break
        dur = text(it.find("itunes:duration", NS))
        desc = text(it.find("description"))
        if len(desc) < 40:
            desc = text(it.find("content:encoded", NS)) or desc
        transcripts = []
        for t in it.findall("podcast:transcript", NS):
            transcripts.append({"url": t.get("url"), "type": t.get("type"), "language": t.get("language")})
        out["episodes"].append({
            "guid": guid,
            "title": text(it.find("title")),
            "pub_date_utc": pd.isoformat(),
            "pub_date": pd.astimezone(timezone(timedelta(hours=-8))).strftime("%Y-%m-%d"),
            "audio_url": audio,
            "episode_url": text(it.find("link")),
            "duration": dur,
            "description_html": desc,
            "transcripts": transcripts,
        })
    return out

results = []
with ThreadPoolExecutor(max_workers=8) as ex:
    for res in ex.map(handle_sub, subs):
        if res["show"] in excluded:
            res["skipped"] = "excluded_feed"
        results.append(res)

all_eps = []
for r in results:
    for e in r["episodes"]:
        e["show"] = r["show"]
        e["feed_url"] = r["feed_url"]
        all_eps.append(e)

seen = {}
for e in all_eps:
    seen.setdefault(e["guid"], e)
eps = sorted(seen.values(), key=lambda e: e["pub_date_utc"])

fails = [(r["show"], r["error"]) for r in results if not r["ok"]]
with open(f"{ROOT}/data/_cand_today.json", "w") as f:
    json.dump({"episodes": eps, "feed_errors": fails,
               "window": [start_utc.isoformat(), end_utc.isoformat()]}, f, indent=1)

print(f"feeds ok={sum(1 for r in results if r['ok'])}/{len(results)}  failed={len(fails)}")
print(f"new episodes: {len(eps)}")
for e in eps:
    print(f"  {e['pub_date']}  {e['show'][:40]:40}  {len(e['description_html']):5}d  {e['title'][:70]}")
if fails:
    print("feed errors:", fails)
