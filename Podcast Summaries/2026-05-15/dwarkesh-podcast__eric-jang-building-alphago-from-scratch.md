---
show: "Dwarkesh Podcast"
episode: "Eric Jang – Building AlphaGo from scratch"
date: 2026-05-15
guid: "substack:post:197852876"
transcript_source: "youtube_autocaptions"
---

# Dwarkesh Podcast — Eric Jang – Building AlphaGo from scratch

## Key Points

- Eric Jang, former VP of AI at Onex and DeepMind Robotics researcher, chose to rebuild AlphaGo from scratch during his sabbatical rather than relaxing, driven by his passion for building things and fascination with how deep networks 'tamed Go's brutal search space'
- Modern implementations like KataGo require approximately 40 times less training compute than the original AlphaGo, making what once required massive lab resources now achievable on small budgets with open tools
- Go's astronomical game tree complexity (wider and deeper than chess even with shared transpositions) makes brute force approaches hopeless, requiring the sophisticated combination of Monte Carlo Tree Search (MCTS) with learned neural network guidance
- AlphaGo's core architecture uses a shared neural network trunk with two heads: a scalar value function and a 19x19 policy network, where the value head provides quick position evaluation and the policy head guides move selection
- The PUCT (Predictor + Upper Confidence bounds applied to Trees) algorithm balances exploitation of known good moves with exploration of uncertain branches, with exploration bonus fading as visit counts accumulate, shifting from trying new branches to trusting the best ones
- Training involves pretraining on expert games to predict moves and outcomes (which alone beats many humans), followed by self-play where MCTS visit distributions become training targets for the policy network, creating a virtuous cycle of improvement
- The key insight is that modest-depth neural networks can 'amortize vast combinatorial searches' by learning to collapse long game sequences into quick judgments, similar to how AlphaFold compresses complex physics simulations
- Network architecture choices depend on constraints: ResNets work well for small budgets, Transformers excel with abundant data and need for global aggregation, while KataGo mixes global features to connect distant board positions
- MCTS can underperform when the value network is poorly trained or simulation counts are low, requiring careful bootstrapping on smaller boards and ensuring diverse late-game scenarios during training to ground the value estimates properly

## Overview

This technical deep dive opens with Eric Jang explaining his motivation for rebuilding AlphaGo during sabbatical, driven by fascination with how neural networks solved Go's previously intractable search complexity. The conversation immediately shifts into a detailed technical explanation of Go's rules and why the game presents such computational challenges, with Jang using this as a blackboard-style teaching session. The heart of the discussion focuses on the elegant marriage of Monte Carlo Tree Search with neural network guidance - specifically how PUCT algorithm balances exploration and exploitation, how the dual-headed network architecture (value and policy) enables efficient position evaluation, and how the training process creates a self-improving loop through expert game pretraining followed by self-play distillation. Jang emphasizes the profound efficiency gains achieved by modern implementations, noting that what once required massive computational resources can now be accomplished on modest budgets, and draws compelling parallels between AlphaGo's approach to search compression and other breakthrough AI systems like AlphaFold.

## Implications

For AI practitioners, this conversation illuminates how combining classical algorithms (MCTS) with modern deep learning can solve seemingly intractable problems, suggesting similar hybrid approaches may work in other domains. Engineers will appreciate the practical insights about network architecture choices, training strategies, and the importance of proper bootstrapping procedures. The dramatic reduction in compute requirements from original AlphaGo to modern implementations demonstrates how algorithmic improvements can democratize access to powerful AI capabilities, making previously lab-exclusive techniques accessible to smaller teams and budgets. The discussion also highlights the value of learning from games and simulations as a pathway to developing robust AI systems that can handle complex decision-making scenarios.

## Notable Quotes

- Eric Jang: 'I love building things, and AlphaGo is what pulled me into AI because deep nets somehow tamed Go's brutal search'
- Eric Jang: 'With KataGo, we saw approximately 40 times less training compute. So, what once needed a big lab now runs on a small budget with open tools'
- Eric Jang: 'Go's game tree is astronomically wide and deep, even with shared transpositions, so brute force is hopeless'
- Eric Jang: 'A network can learn that shortcut and collapse long playouts into a quick judgment. That trims depth while a strong policy prior prunes breadth'
- Eric Jang: 'The deeper point is that a modest depth net can amortize a vast combinatorial search, much like AlphaFold compresses heavy physics'

## People Mentioned

- Eric Jang — Former VP of AI at Onex, formerly at DeepMind Robotics, currently rebuilding AlphaGo on sabbatical
- Dwarkesh — Podcast host conducting the interview

## Topics

- AlphaGo
- Monte Carlo Tree Search
- Neural Networks
- Game AI
- PUCT Algorithm
- Deep Learning
- KataGo
- Self-Play Training
- Go Strategy
- AI Architecture
