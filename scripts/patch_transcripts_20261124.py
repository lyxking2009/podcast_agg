#!/usr/bin/env python3
"""Patch transcript records for the 5 episodes that fell back to thin descriptions
or a false-positive web page (Wikipedia)."""
import json, glob, os, re, html, urllib.request

TDIR = "/Users/yuxinglin/repos/podcast_agg/data/transcripts_20261124"

PATCH = {
    # Moonshots EP #315 -> YouTube captions (verified: "Jensen is back", $1.09T, EP #315)
    "5cc398a8-e822-11f1-b097-0f454450a69c": ("/tmp/yt_CDN5X5z4gLw.txt", "youtube_autocaptions",
                                             "https://youtu.be/CDN5X5z4gLw"),
    # Y Combinator -> YouTube captions (verified: Boom Supersonic first full-scale jet engine core)
    "24c46131-e4f1-4f43-97d2-5de83de089ae": ("/tmp/yt_-ieJwYq4j-o.txt", "youtube_autocaptions",
                                             "https://youtu.be/-ieJwYq4j-o"),
    # TWIML #786 -> YouTube captions (verified: Thore Graepel, AlphaGo move 37)
    "9294c3be-e83e-11f1-84c1-ffe7c3ba21de": ("/tmp/yt_wDs1SMHR8es.txt", "youtube_autocaptions",
                                             "https://youtu.be/wDs1SMHR8es"),
    # Invest Like the Best EP.502 -> Kazuha structured transcript summary
    "9a5d37c6-e7d9-11f1-b2ec-930476c9d7cf": ("/tmp/kazuha_iltb.txt", "web_kazuha_summary",
                                             "https://kazuha.ai/sources/invest-like-the-best-with-patrick-o-shaughnessy/posts/scott-wu-on-cybersecurity-compute-and-the-future-of-engineering-invest-like-the"),
}

def startalk_desc():
    UA = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0 Safari/537.36"
    u = "https://startalkmedia.com/show/neutrinos-on-ice-with-francis-halzen"
    h = urllib.request.urlopen(urllib.request.Request(u, headers={"User-Agent": UA}), timeout=40).read().decode("utf-8", "replace")
    t = re.sub(r"(?is)<(script|style|nav|footer|head).*?</\1>", " ", h)
    t = re.sub(r"(?i)<br\s*/?>|</p>|</div>|</li>|</h[1-6]>", "\n", t)
    t = html.unescape(re.sub(r"<[^>]+>", " ", t))
    lines = [re.sub(r"\s+", " ", l).strip() for l in t.splitlines()]
    lines = [l for l in lines if l and len(l) > 2]
    txt = "\n".join(lines)
    i = txt.find("About This Episode")
    j = txt.find("Get the most out of StarTalk")
    if i >= 0 and j > i:
        txt = txt[i:j]
    return txt.strip()

PATCH["b08ef052-68b5-45b7-913d-1bd7041b4a6e"] = (None, "description", "startalkmedia_show_page")

# fast lookup guid -> record path
paths = {}
for p in glob.glob(os.path.join(TDIR, "*.json")):
    d = json.load(open(p))
    paths[d["guid"]] = p

for guid, (txtpath, src, ref) in PATCH.items():
    p = paths.get(guid)
    if not p:
        print("MISSING", guid)
        continue
    d = json.load(open(p))
    txt = startalk_desc() if txtpath is None else open(txtpath).read()
    d["transcript"] = txt[:600000]
    d["transcript_source"] = src
    d["transcript_ref"] = ref
    json.dump(d, open(p, "w"), indent=1, ensure_ascii=False)
    open(p.replace(".json", ".txt"), "w").write(d["transcript"])
    print(f"patched {src:22s} {len(txt.split()):6d}w  {d['show'][:34]:34s} | {d['title'][:50]}")
