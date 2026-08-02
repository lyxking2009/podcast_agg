---
guid: ab0875ce-554c-11f1-b770-9fac1c70deab
title: "Relational Foundation Models for Enterprise Data with Jure Leskovec - #768"
show: "The TWIML AI Podcast"
date: 2026-05-21
duration: "1:06:23"
transcript_source: youtube
---

## Key Points
- Relational Deep Learning (RDL) treats enterprise multi-table databases as heterogeneous graphs and trains graph neural networks directly on raw relational data, eliminating manual feature engineering
- Kumo's Relational Foundation Model 2 (RFM2) performs in-context learning over subgraphs, enabling accurate predictions on any database and any task without model training
- The system extracts labeled subgraph examples from historical data, encodes them in a domain-agnostic way, and runs a single forward pass of a pre-trained frozen transformer to produce predictions in under a second
- RFM2 outperforms best supervised models by ~5% relative accuracy out-of-the-box, and ~12% when fine-tuned, delivering double-digit lifts over hand-engineered feature pipelines
- Real-world deployments include DoorDash (restaurant recommendations, hundreds of millions in revenue impact), Reddit (near double-digit clickthrough rate increase for ads), and Coinbase (fraud detection over the entire Bitcoin blockchain network)
- The same graph-based architecture principle underlies Leskovec's AI Virtual Cell project, which represents biological cells as graphs of proteins and aggregates upward to patient-level foundation models trained on single-cell RNA-seq data
- Explainability is achieved by running the differentiable model backwards to identify which tables, columns, and cells drove the prediction, then using an LLM to generate human-readable text explanations
- The key bottleneck in production is not model accuracy but connecting predictions to downstream decision-making processes and business rules
- Agents are a natural consumer of RFM2 as a tool, since agentic workflows require grounded, hallucination-free predictions over enterprise data rather than LLM common-sense guesses
- RelBench (Stanford) and SALT (SAP) are the two main multi-table benchmarks; nearly all existing ML benchmarks are single-table, which Leskovec argues is an already-solved and unrealistic problem

## Overview
Jure Leskovec, co-founder and chief scientist at Kumo and Stanford CS professor, joins Sam Charrington to discuss two lines of work: the AI Virtual Cell project (building hierarchical foundation models over single-cell RNA-seq and protein structure data) and Relational Deep Learning, the idea that enterprise multi-table databases should be treated as heterogeneous graphs on which neural networks can train directly. He explains how Kumo's Relational Foundation Model 2 (RFM2) extends this to zero-shot in-context learning—making accurate predictions on unseen databases and tasks in a single forward pass without any model training. The conversation covers benchmarking, real-world deployments at DoorDash, Reddit, and Coinbase, explainability, cold-start robustness, and the emerging role of these models as prediction tools for AI agents.

## Implications
Relational Foundation Models represent the same paradigm shift for structured enterprise data that transformers brought to vision and language: learning directly from raw data instead of hand-crafted features. If this generalizes, the long tail of enterprise prediction tasks—fraud, churn, recommendations, sales forecasting—could be served by a single pre-trained model called at inference time by AI agents, removing the months-long data science cycle currently required for each new task.

## Notable Quotes
> "It can make accurate predictions on any database and any predictive task without any model training."

> "AI has not transformed this structured data space in the same way as computer vision or natural language understanding have been fundamentally transformed by AI."

> "Your features are just some arbitrary human biased summary statistic of your data that you kind of dreamt up with."

> "It's a single forward pass of a pre-trained frozen neural network. There is no loop. Nothing like that. You get the answer in I don't know two seconds, half a second."

> "Nearly a double-digit increase in clickthrough rates. Usually an entire team increases maybe 1% accuracy year over year."

> "For agents to be effective they need APIs that are agent or agentic friendly."

> "A foundation model—an agent can just ask 'predict me probability of churn under this definition under this data' and you get the answer half a second later."

## People Mentioned
- Jure Leskovec (co-founder and Chief Scientist at Kumo; Professor of Computer Science at Stanford University)
- Sam Charrington (host, The TWIML AI Podcast)

## Topics
relational-deep-learning, graph-neural-networks, foundation-models, enterprise-AI, in-context-learning, multi-table-prediction, fraud-detection, recommendation-systems, AI-virtual-cell, single-cell-RNA-seq, explainability, agentic-AI, Kumo, RelBench, graph-transformers
