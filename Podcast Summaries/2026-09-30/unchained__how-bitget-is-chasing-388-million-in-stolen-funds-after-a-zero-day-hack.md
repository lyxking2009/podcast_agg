---
date: 2026-09-30
show: Unchained
title: "How Bitget Is Chasing $388 Million in Stolen Funds After a Zero-Day Hack"
guid: 5e80484a-bcf7-11f1-9346-e779a7d83419
transcript_source: show_notes
duration: 1:06:12
link: https://unchainedcrypto.com/how-bitget-is-chasing-388-million-in-stolen-funds-after-a-zero-day-hack
model: deepseek-v4-flash (Hermes manual fallback)
---
# How Bitget Is Chasing $388 Million in Stolen Funds After a Zero-Day Hack

## Overview

Laura Shin interviews Bitget CEO Gracy Chen about the $388 million theft from the exchange's hot and warm wallets.
Chen reconstructs the attack: on September 24, attackers used a zero-day in a third-party security product to slip
fraudulent withdrawal commands into Bitget's wallet system, draining roughly $388 million across chains from Ethereum
and XRP to Zcash - starting with test transfers that stayed below risk thresholds. She explains the decision to switch
wallets back on mid-attack to sweep hot and warm funds into cold storage, how the exchange's 2022-vintage protection
fund covered users, and why withdrawals reopening produced an early net inflow. The second half examines the industry
response - Tether, Circle and NEAR Intents froze funds while THORChain declined - and the roughly $632 thousand frozen
to date, along with scammers impersonating an investment firm to target Chen personally after the hack.

## Key Points

- Attack vector: a zero-day in a third-party security product allowed attackers to forge withdrawal commands inside Bitget's wallet authorisation flow - private keys were never compromised.
- Attack timeline: test transfers of ETH and TRX below alert thresholds, then roughly $361 million in USDT-equivalent transfers across XRP, Zcash, BSC, Base, Arbitrum, Optimism and Avalanche before systems flagged the activity and froze withdrawals.
- Response: Bitget switched wallets back on mid-attack to sweep hot and warm funds into cold storage; CEO Gracy Chen credits SEAL 911 for alerting her team within 90 minutes.
- Recovery is the hard part: Tether and Circle froze part of the stolen assets and NEAR Intents cooperated, but THORChain declined her request, citing permissionless design - Chen's framing is that “permissionless doesn't mean neutral.” Only about $632 thousand had been frozen at the time of recording.
- The exchange's User Protection Fund, created in 2022 and worth roughly $465 million USDT on the day of the hack, covered all stolen balances and is to be replenished above $300 million within a week from corporate reserves audited at more than $1.4 billion.
- Chen ties the method to Bybit's $1.5 billion 2025 theft (North Korea's Lazarus Group) and says IP behavioural patterns and on-chain signatures are consistent with DPRK-linked groups; she declined to formally attribute before the incident report.
- Post-hack, scammers impersonated an investment team to target Chen personally.
- Discussion of whether the IPO remains on track and the significance of net ETH inflows after withdrawals reopened.

## Implications

This is the clearest first-hand account available of a landmark exchange breach, and the lesson is structural rather
than cryptographic: the failure was in the authorisation chain of a vendor, not in key management. For practitioners,
two points stand out. First, the recovery asymmetry - permissionless infrastructure that refuses to freeze funds is
now the binding constraint on loss recovery, not the theft itself. Second, a pre-funded protection reserve turned a
solvency-threatening event into a customer-confidence event, which is the strongest argument yet for exchanges
holding explicit reserves.

## Notable Quotes

- Gracy Chen (as quoted in the episode description): “permissionless doesn't mean neutral” - putting a hard question to DeFi over THORChain's refusal to intervene.
- Gracy Chen (per Unchained's reporting of her X post): “the attacker compromised a critical backend system within our wallet infrastructure, used it to spoof transaction data, and triggered our authorization process to move funds out.”

## People Mentioned

- Gracy Chen - CEO, Bitget
- Laura Shin - host, Unchained
- ZachXBT - on-chain investigator
- SEAL 911 - incident response collective

## Topics

crypto security, exchange hacks, Bitget, North Korea, THORChain, permissionless, fund recovery, cold storage, protection fund
