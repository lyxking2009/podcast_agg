---
date: 2026-10-01
show: Machine Learning Street Talk (MLST)
title: "How a Voice Agent Learns the Rhythm of Conversation - Shawn Wen"
guid: 42b10fda-0ad2-4d6f-8fa6-14a3fca50214
transcript_source: web
duration: 1:10:09
link: https://podcasters.spotify.com/pod/show/machinelearningstreettalk/episodes/How-a-Voice-Agent-Learns-the-Rhythm-of-Conversation--Shawn-Wen-e3pndhc
model: deepseek-v4-flash (Hermes manual fallback)
---
# How a Voice Agent Learns the Rhythm of Conversation - Shawn Wen

## Overview

Tim Scarfe talks with Tsung-Hsien (Shawn) Wen, CTO of PolyAI, about why voice agents are substantially harder
than text agents. Voice adds time to the problem: a good conversation depends on adapting to the person on the line,
not merely on reasoning to the best answer. Wen describes Dialog-RSN-1, an audio-native model that first predicts a
turn-taking signal, then replies in text with citations, and writes the transcript last so enterprises can audit it.
They cover why training on real noisy calls with synthetic noise added beats over-cleaned audio, what a voice agent
should do while it thinks, why a voice with a hint of regional accent outperforms a generic one, and why public
benchmarks fall short for voice. The last stretch widens to harness engineering, cognitive debt, and the shift from
producing content to checking it.

## Key Points

- Voice agents are harder than text agents because voice adds a temporal dimension - a good conversation requires adapting to the caller, not just producing the best answer.
- PolyAI's audio-native model Dialog-RSN-1 predicts a turn-taking signal first, then replies in text with citations, and writes the transcript last specifically so enterprises can audit the interaction.
- Training data: real, noisy calls with synthetic noise added. Over-cleaned audio actually made the new model worse - a direct lesson about how much real-world signal lives in the noise.
- Latency management is a design problem, not just an infrastructure one: what the agent should do while it thinks is part of the conversational experience.
- Voice design: a voice with a hint of regional accent beats a generic one; the uncanny valley is a real constraint on adoption.
- Public benchmarks fall short for voice agents, and enterprises increasingly want to own their own agent harness rather than rent one.
- The closing arc covers whether behaviour belongs in the harness or in the model weights, cognitive debt from working alongside agents, Wispr Flow, building tools agents can use, and whether slop is in the eye of the reader.

## Implications

This is the most concrete engineering account of voice-agent design in recent memory, and the over-cleaned-audio
finding is the standout: cleaning your training data can destroy the signal the model needs. For teams building
enterprise agents, the auditability design - turn-taking signal, then cited text reply, then transcript written last -
is a reusable pattern. The harness-versus-weights discussion is also directly relevant to anyone deciding how much of
their agent's behaviour to encode outside the model. This record is built from the episode's structured description and
full chapter list rather than a verbatim transcript, so chapter titles appear as topics and not as quotes.

## Notable Quotes

- (structured show notes / chapter list only, no verbatim transcript)

## People Mentioned

- Tsung-Hsien (Shawn) Wen - CTO, PolyAI
- Tim Scarfe - host, MLST

## Topics

voice agents, turn-taking, audio-native models, latency, synthetic noise, benchmarking, agent harness, cognitive debt, enterprise AI
