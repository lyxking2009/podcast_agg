---
podcast: "Empire"
episode: "Cybersecurity Panic, Nationalizing AI, The Model Squeeze & Can't Own Anything"
published: 2026-10-23
duration: "1h6m23s"
audio_url: "https://chrt.fm/track/713ACD/traffic.megaphone.fm/BWG8659369869.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-10-27T22:11:39Z
model: "deepseek-v4-pro"
guid: "087ab366-ce88-11f1-a6dd-93c8a2715f02"
---

# Cybersecurity Panic, Nationalizing AI, The Model Squeeze & Can't Own Anything — Empire

## TL;DR
Empire hosts discuss a mass-exploited PHP vulnerability (known since 2004) that AI models recently turned into a working attack, triggering cascading internet disruptions including the Railway credential leak and an unrelated Indian certificate authority fiasco. The conversation covers accelerating AI-enabled cyber threats, the possibility of nationalizing AI labs, OpenAI's shift toward becoming an AI interface, the rumored Visa–Fireblocks acquisition, and market anxiety over AI capex and 'you can't own anything' sentiment.

## Key points
- A PHP bug known since 2004 but never exploited was independently discovered as exploitable by OpenAI's model, a security researcher, and Wiz. A coordinated patching process leaked, forcing public disclosure and a scramble to patch millions of websites.
- Cloudflare CEO Matthew Prince reported exploit attempts rose from a few hundred per hour pre-disclosure to tens of thousands within 45 minutes and millions per hour afterward, making it background internet radiation.
- Sending crafted data to a PHP web server yields a shell with website permissions, allowing database access and data exfiltration; this likely contributed to cascading failures including the Railway credential leak.
- Railway, a Heroku-like platform, disclosed that all deployed secrets may be compromised; attackers stole about $1M in crypto from leaked credentials, likely monetizing via API keys, email access, and crypto wallets.
- India's largest certificate authority e-Mudra issued keys with private exponents of 4 and 283, was expelled from browsers, and exited the certificate business despite claiming no infrastructure compromise.
- Crypto experienced record hack counts in 2024; now AI models are making conventional software attacks economical, with Zaki and Haseeb expecting a rough 6-12 months before internet hardening and standardized defensive programs.
- Mainstream models like Opus and Astra refuse dual-use defensive tasks, forcing defenders to use 'obliterated' models with guardrails removed; Haseeb and Zaki argue defense doesn't require frontier models but does require compute and dedicated resources.
- Discussion suggests governments should provide defensive cyber services, subsidize code analysis, and possibly use eminent domain to patch vulnerable public infrastructure, as volunteer firefighting-style cybersecurity is unsustainable.
- Haseeb argues AI labs could be nationalized during a bubble pop due to national security and civilizational importance; Santi and Host debate bailouts, with Haseeb noting the US government owned 61% of GM after TARP and could take similar action.
- Haseeb describes OpenAI increasingly acting as an AI router/control plane: allowing spending credits on Kima K3 and GLM 5.3, potentially listing competitor models, and monetizing enterprise relationships, risking falling behind Anthropic in frontier intelligence.
- Reports suggest Visa may buy Fireblocks for $2.5B vs its last $8B valuation; Fireblocks raised ~$1B, so later investors get ~1x, earlier investors modest returns, and common is not wiped out. The product was criticized for focusing narrowly on crypto funds.
- The 'can't own anything' meme reflects fears across stocks, crypto, software, gold, and bonds. Santi notes the rare co-occurrence of 10-year Treasury yield drop >15bps and gold drop >2% same day (only 3 times in 30 years) signals deleveraging, but advises long-term investing in quality names.

## Notable quotes
> "if you send the right data to a web server that is hosting a website in PHP it gives you what is like what you call a shell. It gives you like a command line on the server of whoever was running that runs with the same permissions as the website and usually from there you can get into the database and everything else. So you get access to information and you can start exfiltrating data." — Zaki
> "I suspect the most likely time they get nationalized is when they're at bubble pops." — Haseeb
> "largely defense is it's not trivial but you don't need the you know super geniuses to do defense. You just need to detect a breach you need to shut it off and you go patch your software." — Zaki
> "if you want to use other models, fine. We get it. We prefer you use ours and we'll try to steer you towards ours. But at the end of the day, the thing that we really want to own is being the interface for AI." — Haseeb
> "if you're in spot and just are in quality names, I am very much unbothered, moisturized in my lane." — Santi

## People mentioned
- Haseeb Qureshi
- Zaki Manian
- Matthew Prince
- Greg Brockman
- Sam Altman
- Dario Amodei
- Elon Musk
- Ken Griffin
- Ari Emanuel
- John Wilson

## Topics
`cybersecurity` `artificial-intelligence` `national-security` `venture-capital` `crypto-markets` `market-sentiment` `ai-infrastructure`
