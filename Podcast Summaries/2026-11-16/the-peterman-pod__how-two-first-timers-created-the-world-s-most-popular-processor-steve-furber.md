---
podcast: "The Peterman Pod"
episode: "How Two First-Timers Created the World’s Most Popular Processor | Steve Furber"
published: 2026-11-16
duration: "55m38s"
audio_url: "https://anchor.fm/s/106346b90/podcast/play/129446274/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-10-14%2F436741870-44100-2-e65e8988c44738.mp3"
episode_url: "https://podcasters.spotify.com/pod/show/peterman-pod/episodes/How-Two-First-Timers-Created-the-Worlds-Most-Popular-Processor--Steve-Furber-e3scsu2"
transcript_source: youtube_autocaptions
generated_at: 2026-11-19T23:10:23Z
model: "deepseek-v4-pro"
guid: "2d3b13ec-a5f9-4374-939b-7aa3e4559306"
---

# How Two First-Timers Created the World’s Most Popular Processor | Steve Furber — The Peterman Pod

## TL;DR
Steve Furber recounts how he and Sophie Wilson, two first-time chip designers at Acorn, created the ARM processor by drawing on Berkeley and Stanford RISC research, leveraging memory bandwidth insights, and simplifying the instruction set. Their 32-bit load-store design, first implemented in 1985, outperformed 16-bit CISC competitors and eventually became the world's most widely used processor architecture, with over 350 billion chips shipped. Furber also discusses the BBC Micro's origins, Acorn's decline, and his later work on the SpiNNaker neuromorphic computing platform.

## Key points
- Steve Furber built his first computer around a Signetics 2650 microprocessor ordered by phone from California, hand-assembled on a PCB using Verowire wiring pen with insulated copper wire; he used it in his PhD research.
- Before microprocessors, hobbyists in the 1970s built computers from TTL (transistor-transistor logic) chips, standard 74-series packages containing logic gates, flip-flops, and 4-bit ALUs; the Cambridge University Processor Group was a student society where Furber learned by doing.
- Furber anodized aluminum in his kitchen sink to create a professional-looking front panel for his first computer, then applied Letraset labels; he also etched PCBs using ferric chloride and a waterproof pen.
- Acorn grew out of CPU Limited founded by Herman Hauser and Chris Curry, who recruited Furber and Sophie Wilson from the Cambridge University Processor Group; Furber did unpaid design work while a PhD student, receiving components in exchange.
- The BBC Micro contract was won with a design based substantially on a machine Furber built at home; he joined Acorn formally in October 1981. The BBC Micro's main board had 102 chips, including two semi-custom ULAs from Ferranti, which each replaced 10-20 TTL packages.
- Herman Hauser pushed Acorn into chip design with the warning: 'in 10 years time there'll be two sorts of computer companies, those that have learned to design chips and those that have gone bust.' Acorn adopted VLSI Technology's chip design software and recruited a small professional chip design team.
- When evaluating 16-bit microprocessors as potential successors to the 6502, Furber and Wilson found them worse in real-time performance (slow interrupt response) and unable to fully use available memory bandwidth, because their microcoded execution required multiple clock cycles per instruction.
- The key performance insight was that useful memory bandwidth, not raw clock speed, determined system performance. 6502-era memory chips could support 4-8 million accesses per second, but 16-bit processors at 4-6 MHz used multiple cycles per access, leaving memory idle.
- Inspired by Berkeley RISC (Dave Patterson) and Stanford (John Hennessy) papers, which showed a postgraduate class could design a competitive microprocessor in a year, Acorn decided to design its own processor. Sophie Wilson created the ARM instruction set, informed by her experience writing Acorn's BASIC interpreters, while Furber designed the microarchitecture.
- Furber built an 808-line BBC Basic reference model of the first ARM processor, describing all internal operations; the silicon design team then translated this into a silicon implementation, with first ARM silicon arriving 18 months after project start.
- ARM was a 32-bit load-store RISC architecture with a three-level pipeline, most instructions executing in one cycle, and load/store multiple instructions that deliberately used sequential addresses to exploit page-mode memory's double-speed access, yielding a potential 25x increase in useful memory bandwidth over the BBC Micro.
- Word sizes are often powers of two because 32 bits was the first width that could hold a full address as a single data item; 64 bits later became standard, but doubling bus width increases pin count and memory chip cost, so 64 is a practical stopping point.
- ARM's success came from being very early to RISC: its simplicity led to small size and low power. Mainstream CISC industry, including Intel, was slow to adopt RISC because of investment in complex instructions and x86 software compatibility; modern x86 chips translate CISC instructions to RISC-like operations internally.
- Visiting Western Design Center in Phoenix in October 1983, Furber and Wilson found a company operating out of two bungalows with school kids designing cell libraries, which convinced them that if such an operation could design a processor, Acorn could too.
- Acorn never built its own fab; fabrication was done by VLSI Technology in San Jose. The early ARM was a four-chip system including processor, memory controller, video controller, and IO controller, used in the Acorn Archimedes.
- In 1990, ARM was spun out as a joint venture with Apple and VLSI Technology because Apple wanted the ARM for the Newton but preferred an independent supplier. Acorn later declined when its share of ARM became worth more than Acorn itself.
- SpiNNaker is a digital, brain-inspired neuromorphic computing platform aiming to put a million ARM processors in a single computer to model brain regions in real time; its unique communication system addresses the brain's high connectivity (~5,000-10,000 connections per neuron).
- Furber discussed differences between synchronous and asynchronous circuits: synchronous designs use a global clock that creates current spikes and radio emissions, while asynchronous designs have local control with lower interference but are harder to design due to poor EDA tool support.
- Furber won a bronze medal at the 1970 International Mathematical Olympiad in Keszthely, Hungary, representing the UK; he studied mathematics at Cambridge before switching to engineering and aerodynamics.
- Furber advises students to choose courses that keep the most doors open and to look for opportunities; he identifies AI as today's major opportunity, citing hardware, software, and ethical dimensions.

## Notable quotes
> "Nobody taught us. We picked up what we could from books and papers and made the rest up." — Steve Furber
> "If they can design a processor, then maybe we can too." — Steve Furber
> "In 10 years time there'll be two sorts of computer companies, those that have learned to design chips and those that have gone bust." — Herman Hauser
> "I literally started the microarchitecture design by sketching an arrangement of registers and barrel shifters and ALUs and so on on a piece of paper, making lots of photocopies and then for each instruction, for each clock cycle, coloring in what was happening on this microarchitecture." — Steve Furber
> "In the last figure I heard is that over 350 billion Arm powered chips have been shipped by Arm's partner network." — Steve Furber

## People mentioned
- Steve Furber
- Sophie Wilson
- Herman Hauser
- Chris Curry
- Dave Patterson
- John Hennessy
- Carver Mead
- Katevenis

## Topics
`arm-processor` `bbc-micro` `risc-architecture` `chip-design` `computer-history` `neuromorphic-computing`
