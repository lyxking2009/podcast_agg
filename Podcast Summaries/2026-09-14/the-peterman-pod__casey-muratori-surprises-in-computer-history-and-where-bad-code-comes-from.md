---
podcast: "The Peterman Pod"
episode: "Casey Muratori: Surprises In Computer History And Where Bad Code Comes From"
published: 2026-09-14
duration: "1h57m50s"
audio_url: "https://anchor.fm/s/106346b90/podcast/play/125692505/https%3A%2F%2Fd3ctxlq1ktw2nl.cloudfront.net%2Fstaging%2F2026-8-14%2Fcasey-muratori.mp3"
episode_url: "https://youtu.be/jHLbL1Eg4gM"
transcript_source: youtube_autocaptions
generated_at: 2026-09-14T22:05:08Z
model: "deepseek-v4-flash"
guid: "92258213-3681-4fef-99d6-36088812e38b"
---

# Casey Muratori: Surprises In Computer History And Where Bad Code Comes From — The Peterman Pod

*(Guest: Casey Muratori, video game developer and programming creator. Host: Ryan Peterman. YouTube: https://youtu.be/jHLbL1Eg4gM)*

## TL;DR

Casey Muratori — video game developer, computer historian by avocation, and the person behind the "Big OOPS" and "Only Unbreakable Law" talks — joins Ryan Peterman to dig through the documentary record behind some of computing's most repeated phrases. The centrepiece is Knuth's "premature optimization is the root of all evil," and Muratori's method is the point: he treats computer history as a dig through primary sources — published papers, correspondence, conference proceedings — and reports repeatedly that the amount of context he uncovers is far weirder and richer than the one-line takeaway anyone remembers. Two threads converge around 1974: the "software crisis" of the late 1960s and early 1970s, in which hardware was advancing faster than programmers' ability to decompose problems, forcing abstractions and modularity to become a cultural shift rather than a technical one; and Knuth's exposure to early execution profiling, which reframed performance work as a matter of picking your battles. The episode also ranges over where bad code actually comes from, why design docs before code is a bad idea, Dijkstra's correspondence and the personal side of "Go To Statement Considered Harmful," and whether vibe coding damages the industry.

## Key points

- **The subject is the history behind the phrase, not the phrase.** Muratori says the *significance* of "premature optimization is the root of all evil" is easy to explain — people repeat it and keep reinterpreting it — but the history behind it is much stranger than the received version.
- **The phrase has outlived its context.** It is at least fifty years old, and the effects of it "are still felt to this day" — an extraordinary lifespan for a rule of thumb. Some people read it with nuance; others read it bluntly as "any consideration of performance is not worth it."
- **Thread one: the software crisis.** In the late 1960s and early 1970s, hardware was advancing to the point where people wanted to build payroll systems, data analysis and graphical systems (Sutherland's Sketchpad in the early 1960s), but programmers were not ready. The things we take for granted — libraries, abstractions, breaking a complex system into manageable parts — had to become a *cultural shift*.
- **Thread two: Knuth and execution profiling.** Knuth, at Stanford, was learning that you could take actual execution profiles, find where time was spent, and use that to decide where to invest development effort. Sampling profilers, block profilers and binary-instrumentation profilers were brand new concepts — not tools anyone assumed existed.
- **The convergence, circa 1974.** Knuth was deeply invested in structured programming as a proposed response to the software crisis: programmers could not spend all their time writing tightly optimised assembly. They had to think bigger-picture, pick their battles, and make concessions to maintainability — do not obscure a routine that barely registers in runtime.
- **Why that argument landed then and not now:** there were no massive optimising compiler passes like LLVM. If you wanted a loop unrolled or an invariant hoisted, you did it by hand. The trade-off between performance and clarity was a live, daily decision.
- **The 1970 Stanford study.** Knuth worked with students instrumenting Fortran programs to see where time was being spent — Muratori stresses it was not done in the most rigorous way imaginable, which is itself part of the history.
- **Method: "dumpster diving" through the record.** Muratori describes going back through history and picking through what happened as a genuinely enjoyable experience, and repeatedly says the amount of previously unknown context is "mind-boggling" — including reading correspondence where figures like Dijkstra describe being deeply depressed at the time. Letters, not Twitter replies, which is why the personal dimensions survive.
- **He is explicit about his limits.** He is not a historian and does not do this for a living, so even what he considers a thorough read for a talk is shallow compared with what unused personal papers would reveal.
- **Beyond the phrase:** the discussion ranges across what shocked him in the history, clean code versus horrible performance, how to write high-performance code, where bad code comes from, why design docs before code is a bad idea, and "the only unbreakable law in software engineering."
- **Career material:** how he got into programming, why he did not work in big tech, whether you should work at a startup early on, and what video game engineering is actually like.
- **Modern arguments:** why preventing recursion is reasonable, and whether vibe coding is bad for the industry — the episode opens by explicitly setting aside AI-doom content to talk about software engineering instead.
- **Linked work referenced** includes "The Root of the Root of All Evil," "The Big OOPS: Anatomy of a 35-Year Mistake," `Clean Code, Horrible Performance`, "Where Does Bad Code Come From?" and "The Only Unbreakable Law," plus Knuth's *Structured Programming with go to Statements* and Dijkstra's *Notes on Structured Programming* / *Go To Statement Considered Harmful*.

## Notable quotes

> "The amount of stuff that you learn about what was going on at that time is just mind-boggling." — Casey Muratori
> "I refer to it as dumpster diving, but it's actually a very enjoyable experience to go back through history and try to pick through what happened." — Casey Muratori
> "Dijkstra was depressed at that time. He literally says, in this period I was very depressed because of these reasons." — Casey Muratori
> "They didn't have the ability to do pithy Twitter replies, so it was just on paper." — Casey Muratori, on reading historical correspondence
> "This was a thing people were starting to say: the methods that we're using for programming will not scale to these larger problems that we want to do." — Casey Muratori, on the software crisis
> "There's so much AI doom content out there. I'm hoping that we can just talk about interesting things in software engineering and just see where it goes." — Ryan Peterman
> "Fantastic. I will try to keep the AI mentions to a minimum." — Casey Muratori

## People mentioned

- Casey Muratori — video game developer, programming creator, computer historian, guest
- Ryan Peterman — host, The Peterman Pod, author of developing.dev
- Donald Knuth — computer scientist credited with putting the premature-optimization phrase into a worked example
- Edsger Dijkstra — author of "Go To Statement Considered Harmful" and the structured-programming notes
- Barbara Liskov — referenced for a prior Peterman Pod interview that prompted the software-crisis thread
- Ivan Sutherland — author of Sketchpad, cited as an early graphical-systems milestone

## Topics

`software-engineering` `computer-history` `premature-optimization` `structured-programming` `software-crisis` `profiling` `performance` `clean-code` `code-quality` `programming-culture` `vibe-coding`
