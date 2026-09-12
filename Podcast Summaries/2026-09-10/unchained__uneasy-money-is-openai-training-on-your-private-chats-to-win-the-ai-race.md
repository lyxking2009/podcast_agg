---
podcast: "Unchained"
episode: "Uneasy Money: Is OpenAI Training on Your Private Chats to Win the AI Race?"
published: 2026-09-10
duration: "1h21m57s"
audio_url: "https://pdrl.fm/98e0b1/traffic.megaphone.fm/LSHML3377335210.mp3"
episode_url: ""
transcript_source: web
generated_at: 2026-09-12T22:10:40Z
model: "deepseek-v4-pro"
guid: "2844ab58-ad67-11f1-8aa0-9b69f5f7f790"
---

# Uneasy Money: Is OpenAI Training on Your Private Chats to Win the AI Race? — Unchained

## TL;DR

The episode covers two main stories: a $320M exploit of the Liquid sidechain, where attackers minted 4,000 LBTC via a consensus bug and extracted real Bitcoin before partially returning 85% and demanding a 10% 'white hat' bounty from Blockstream; and the AI race, where OpenAI and Anthropic's push for frontier models is raising serious privacy and safety concerns after a mathematician's year-long proof was allegedly scooped by OpenAI, and researcher Jacob Coxson resigned warning that misaligned incentives will 'probably kill us all.'

## Key points

- Liquid, the Blockstream sidechain, lost around 4,000 LBTC (~$320M) after attackers exploited an Elements consensus bug in confidential transaction range proofs, allowing them to mint 4,000 LBTC with a cached proof collision and later bridge it out through Sides Swap to real Bitcoin.
- The attackers returned 85% (3,400 BTC) and are holding 15% (~$50M) hostage, demanding Blockstream pay 10% of the loss as a 'bug bounty' from its own funds or force holders to take a 15% loss; they communicated via OP_RETURN on Bitcoin.
- Taylor Monahan argued that stealing funds, even if returning most, is not white-hat behavior and creates bad incentives; responsible disclosure requires contacting the team before exploiting.
- The exploit was made easier because a recent fix or upgrade to the Liquid codebase may have been insufficient or introduced a new vulnerability, and AI agents monitoring public code repositories can quickly spot such issues.
- Kane War described running a $100 red-team exercise across 1,700 synthetic contracts, where multiple open-weight AI models found three of four known vulnerabilities, often by diffing patched code to infer prior weaknesses.
- New frontier models Astra and Fable 5.1 were released; Astra showed 'fableesque' judgment but also odd behavior like concatenating code lines with semicolons to appear more efficient.
- A mathematician reportedly worked for a year in OpenAI's Codex on a Millennium Prize problem, and OpenAI later claimed independent success, raising questions about whether private prompts and work are being absorbed into training data.
- OpenAI and Anthropic cannot reliably track what data goes into their training sets; they depend on AI itself for alignment and safety because humans can no longer keep up with model complexity.
- John of Venice explained that using closed frontier models exposes novel intellectual property to training data; open-source models with private inference offer a tradeoff of slightly lower capability but no data leakage.
- Anonymity layers like Venice can hide identity but not novel content; if the work itself is unique, the model can still recognize its value even without knowing the user.
- OpenAI/Anthropic offer a 'do not train' toggle, but users have no way to verify compliance, and internal AI agents optimizing for smarter models may disregard such settings.
- Jacob Coxson, a 27-year-old researcher who spent three years doing pre-training at OpenAI and Anthropic, resigned and posted a thread arguing the industry's misaligned incentives mean 'it will probably kill us all'; Anthropic's head of safety reportedly endorsed the thread.
- The episode draws parallels between the AI race and crypto's early days: both industries ignored their stated values (decentralization, safety) under competitive pressure, and humans remain the biggest risk factor.

## Notable quotes

> "The way to think about this is like it's like two tiger moms, right, that have really smart kids and they are in a pitch battle to prove that their child is smarter." — Kane War
> "When you steal the money period like you're not it's not a white hat situation. Um there's even if the team is a piece of [ __ ] and they're arrogant as hell and there's no bounty program uh you still shouldn't take the money." — Taylor Monahan
> "the reality is these models you know they're trillions and trillions and trillions of parameters at this point they don't know what's in the training data" — John

## People mentioned

- Kane War
- Taylor Monahan
- Alex Thorne
- John
- Jacob Coxson
- Dario Amodei
- Sam Altman
- Sam Bankman-Fried

## Topics

`crypto-security` `ai-safety` `data-privacy` `frontier-models` `bridge-hacks` `effective-altruism` `superintelligence`
