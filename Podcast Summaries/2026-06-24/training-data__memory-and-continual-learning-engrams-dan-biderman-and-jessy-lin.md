---
title: "Memory and Continual Learning: Engram's Dan Biderman and Jessy Lin"
show: Training Data
date: 2026-06-24
hosts: ["Sonya Huang", "Shaun Maguire"]
guests: ["Dan Biderman", "Jessy Lin"]
transcript_source: show_notes
---

# Memory and Continual Learning: Engram's Dan Biderman and Jessy Lin

## Key Points
- **Engram's contrarian premise:** Instead of stuffing ever-larger prompts into context windows or bolting on RAG, bake a team's knowledge directly into the model's weights so it knows your company the way an employee of several years does
- **100x fewer tokens:** Matching or beating frontier models while consuming up to 100x fewer tokens through internalized knowledge
- **Memory and continual learning as two sides of the same coin:** Dan Biderman and Jessy Lin argue these are the real bottleneck in AI, not raw intelligence
- **"Always Training" paradigm:** Models should continuously learn from use, not be frozen after training
- **Beyond context windows:** A single Wikipedia article can balloon from a few kilobytes to 80 gigabytes of transformer KV cache; the entire internet compressed into Llama 70B's weights fits in ~100 gigabytes
- **Teams first, then individuals:** Initial focus on enterprise team models before personalized individual models
- **"Dreams" and offline digestion:** The model processes and internalizes new information during downtime, similar to how human sleep consolidates memory
- **Training beats curation:** Automated training on raw data outperforms manual curation of knowledge bases
- **Everyone needs their own model:** Vision of millions of privately trained, always-learning models — "the ChatGPT moment for memory"
- **RAG killer:** Internalized knowledge eliminates the need for retrieval-augmented generation in many use cases

## Overview
Hosted by Sonya Huang and Shaun Maguire of Sequoia Capital, this episode features Dan Biderman and Jessy Lin, co-founders of Engram, who are building a neolab around memory and continual learning. They argue the industry's obsession with ever-larger context windows and retrieval-based architectures has created a crisis: AI models that are brilliant strangers. Working with partners like Microsoft, Notion, and Harvey, the team draws on roots in computational neuroscience and state-space architectures to attack what they see as the real bottleneck in AI.

## Implications
- If Engram's approach works, it could make RAG obsolete for many enterprise use cases
- Personalized models that improve with use represent a fundamental shift from the "one model for everyone" paradigm
- The economic case is compelling: internalized knowledge is far cheaper per query than context-window-heavy approaches
- Privacy implications: models that learn from your data raise important questions about what to remember, forget, and prevent from leaking

## Notable Quotes
- "You don't type into a search box, 'What was I working on yesterday?' — you just know." — Dan Biderman on how AI should work
- "The real ChatGPT moment for memory is the day your model feels like an intern that genuinely got smarter overnight." — Dan Biderman

## People Mentioned
- Dan Biderman — Co-founder, Engram; computational neuroscience background
- Jessy Lin — Co-founder, Engram; state-space architecture expertise
- Sonya Huang — Host, Sequoia Capital
- Shaun Maguire — Host, Sequoia Capital

## Topics
["memory", "continual learning", "Engram", "RAG", "context windows", "personalized AI", "enterprise AI", "knowledge internalization", "state-space architectures", "Sequoia Capital"]
