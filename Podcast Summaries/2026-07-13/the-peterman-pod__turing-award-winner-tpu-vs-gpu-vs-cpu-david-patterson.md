---
title: "Turing Award Winner: TPU vs GPU vs CPU, Computer Architecture, RISC vs CISC | David Patterson"
show: The Peterman Pod
author: Ryan Peterman
pub_date: 2026-07-13
guid: 8310d9bf-0f5a-47ee-8f58-3eec9ea5b065
transcript_source: web
duration: 3551
---

## Key Points

- RISC (Reduced Instruction Set Computer) vs CISC (Complex Instruction Set Computer) is one of the most consequential debates in computer architecture — Patterson, a RISC pioneer, argues simplicity of instruction sets enables faster compiler optimization and better performance-per-watt
- GPUs excel at parallelism for general matrix operations but are not the most efficient architecture for every AI workload — TPUs are purpose-built for neural network inference and training with superior memory bandwidth and lower power draw per FLOP
- CPUs remain essential for serial tasks and control flow; the future is heterogeneous computing where CPU, GPU, and TPU work together rather than competing
- Moore's Law is effectively over for transistor density scaling, but domain-specific architectures (DSAs) are the next source of performance gains — this is Patterson's central thesis from his Turing Award work
- GPU benchmarking is frequently misleading: peak FLOP counts are marketing numbers; effective throughput on real workloads is often 10-30% of peak
- Patterson's career lesson: courage to pursue unpopular ideas (RISC was controversial) combined with rigorous empirical evaluation is the formula for lasting impact
- Advice for younger self: "Pick problems that matter, not problems that are easy to publish"

## Overview

David Patterson, UC Berkeley professor emeritus, Turing Award winner (2017, shared with John Hennessy for RISC architecture contributions), and Distinguished Engineer at Google (where he helped design the TPU), joins Ryan Peterman for a wide-ranging conversation on computer architecture. The episode is structured as a career retrospective and technical deep-dive, walking through the RISC vs CISC debate from the 1980s to its modern implications in AI chip design. Patterson explains why Google built the TPU rather than relying on GPUs, the architectural tradeoffs between CPU/GPU/TPU for AI workloads, and why domain-specific architectures represent the frontier of performance improvement now that Moore's Law has plateaued. The second half turns personal — Patterson's "How to Have a Bad Career" talk (a beloved inverse guide), lessons from five decades in CS, and advice on finding problems worth solving.

## Implications

- Heterogeneous computing (CPU + GPU + TPU + other accelerators) is the architecture of the AI era — general-purpose chips are not going away but will play supporting roles
- Domain-specific architectures will be a major source of competitive advantage for cloud providers — whoever designs the best custom silicon wins inference cost economics
- RISC-V (open-source RISC instruction set) is now well-positioned to challenge ARM in many markets, which Patterson supports
- GPU benchmark inflation is a real problem for AI infrastructure buyers — effective throughput benchmarks on representative workloads are more meaningful than peak specs
- The next generation of computer architects needs to think across the full stack: silicon, compiler, and application layer simultaneously

## Notable Quotes

- "RISC was controversial in 1980. Every major computer company told us we were wrong. The compiler people and the empirical data said we were right." — David Patterson
- "The TPU isn't better than a GPU in general. It's better for the specific thing we needed — training and inference of neural networks. That's what domain-specific means." — David Patterson
- "Moore's Law gave us free performance for 50 years. Now we have to earn it. Domain-specific architectures are how we earn it." — David Patterson
- "Peak FLOPs are a marketing number. If you're buying GPU infrastructure and you're getting 20% utilization, someone sold you a number, not a solution." — David Patterson
- "Pick problems that matter. The best researchers I know didn't chase publications — they chased impact." — David Patterson

## People Mentioned

- David Patterson (UC Berkeley professor emeritus; Google Distinguished Engineer; Turing Award winner 2017)
- John Hennessy (co-Turing Award winner; Stanford president emeritus)
- Ryan Peterman (host, The Peterman Pod)

## Topics

- RISC vs CISC computer architecture
- TPU design and purpose
- GPU vs TPU vs CPU comparison for AI workloads
- Domain-specific architectures (DSAs)
- Moore's Law status and alternatives
- GPU benchmarking accuracy
- RISC-V open-source instruction set
- Heterogeneous computing
- Career advice for computer scientists
- Neural network training hardware
