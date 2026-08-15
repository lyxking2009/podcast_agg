---
podcast: "Unchained"
episode: "I Interviewed a North Korean Hacker Posing as a Crypto Dev"
published: 2026-08-13
duration: 18m32s
audio_url: "https://pdrl.fm/98e0b1/traffic.megaphone.fm/LSHML1813057908.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-08-13T23:52:18Z
model: "deepseek-v4-flash"
guid: "dc0e79f4-9753-11f1-8797-4369bcc34b47"
---

# I Interviewed a North Korean Hacker Posing as a Crypto Dev — Unchained

## TL;DR
Laura Shin interviews the people behind one of the most audacious counter-intelligence operations in crypto: security researchers who built a fake DeFi protocol called Ballena Azul, advertised developer jobs, and hired three suspected North Korean IT workers — every virtual machine they issued silently recording. Presented at DEF CON 34 by Mauro Eldritch (BCA LTD), Heiner García (NorthScan) and ANY.RUN, the sting revealed how DPRK IT infiltration actually works: AI-generated identity documents (with Gemini processing artifacts and SynthID watermarks), reconnaissance on day one, VPN infrastructure (Astrill), and AI job-application assistants. The episode unpacks the hiring red flags, the operational security of the operatives, and what companies can do to defend against a threat that the July 31 joint government advisory describes as North Korean IT workers remitting salaries to parent agencies. (Summary built from the published DEF CON 34 coverage; same-day episode, no full transcript yet.)

## Key points
- The researchers posed as a legitimate employer, built a fake DeFi protocol (Ballena Azul), and hired three people they believe were North Korean operatives through a normal hiring process — interviews, contracts, and company-issued work VMs.
- Identity-document forensics: the first hire claimed Pasadena, Texas, then sent a California driver's license and a New York bank account; metadata showed the image had been processed with Google Gemini and carried a SynthID watermark (Google's invisible marker on AI-created/edited images). The second supplied a Texas license, a valid SSN and a Kansas City bank account; the third sent a New York license belonging to someone else with a genuine iPhone 15 photo, GPS stripped.
- Day-one reconnaissance: all three ran dxdiag, systeminfo and wmic to profile their machines and checked the apparent country of their connection; one installed Chrome Remote Desktop and synced his personal Google account to the sandbox, exposing browsing history, saved passwords and extensions, then logged into GitHub on the same machine.
- Tooling: 2fa.cn was used to pass two-factor codes between operators (December's operation used authenticator.cc and otp.ee); AstrillVPN exit nodes ran throughout; infrastructure sat on Vultr and Gorilla Servers.
- Their browsers carried AI job-application and interview-assistance extensions: AIApply, Final Round AI, Simplify Copilot, and a saved-prompts tool for ChatGPT.
- Attribution: the researchers describe the three as suspected Famous Chollima operatives (CrowdStrike's name for North Korea's IT worker operation), under the broader Lazarus umbrella; as of August 11 no government source had confirmed the identification.
- Context: the July 31 joint alert (IC3/eleven governments) says NK IT workers seek contracts intending to remit salaries to parent North Korean agencies, and flags documents "forged or altered using image editing software" among hiring signals; in April, two US facilitators were sentenced for a scheme placing workers at 100+ US companies on 80+ stolen identities, earning North Korea more than $5M.
- Defensive recommendations: periodic identity checks rather than one at hire, in-person verification for remote-first companies, recruiter training, and blocking AstrillVPN; watch for one account reached from many addresses in a short window and profile text that reads like machine translation.

## Notable quotes
> "A successful placement gives the operative a real employee account and real access to source code and internal systems." — The Hacker News, summarizing the researchers' findings
> "These schemes are 'not only a hiring risk' — because once a placement holds, the worker's access is also authorized and expected." — The researchers (via The Hacker News)
> "Nobody exploited anything." — The researchers, describing that all three operatives came in through the hiring process

## People mentioned
- Laura Shin — host, Unchained
- Mauro Eldritch — researcher, BCA LTD
- Heiner García — researcher, NorthScan
- ANY.RUN — malware analysis and threat intelligence provider (co-investigators)

## Topics
- North Korea IT workers, Famous Chollima, Lazarus Group, fake startup sting, DEF CON 34, identity fraud, SynthID watermark, AI-generated documents, AstrillVPN, remote work security, insider threat, crypto hiring risk, Ballena Azul
