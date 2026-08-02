---
podcast: "Training Data"
episode: "ElevenLabs' Mati Staniszewski: How Voice Becomes the Interface for Everything"
published: 2026-05-08
duration: 26m48s
audio_url: "https://pscrb.fm/rss/p/traffic.megaphone.fm/CPUAI5362828253.mp3"
transcript_source: web
generated_at: 2026-05-09T00:25:51.734117+00:00
model: "deepseek-v4-pro"
guid: "41e0e0de-4a72-11f1-8f25-0bf25b7d3e24"
---

# ElevenLabs' Mati Staniszewski: How Voice Becomes the Interface for Everything — Training Data

## TL;DR
ElevenLabs CEO Mati Staniszewski recounts how the company beat large foundation models by staying laser-focused on audio research, applying transformer and diffusion models to text-to-speech when few others were. Early viral moments—from book authors to no-face YouTube channels to dubbing world leaders—fueled adoption, and the company is now powering voice agents for healthcare, gaming, and education. Staniszewski predicts voice will become the universal interface, aiming to pass the Turing test this year, while addressing challenges like enterprise integration, European talent dynamics, and deepfake authentication.

## Key points
- ElevenLabs outcompeted big labs by focusing exclusively on audio, an area that had seen little prior research innovation; co-founder Piotr built a rock-star team that applied diffusion and transformer architectures to text-to-speech, enabling models that understand context and deliver natural tonality and emotion.
- Audio data is fundamentally different from text: there is far less high-quality clean audio, most audio lacks accurate transcriptions, and labeling requires capturing non-verbal elements (emotion, speaker identity, etc.)—ElevenLabs built a specialized pipeline with voice coaches and labelers to create this training data.
- The model architecture differs from LLMs in key ways: the system must predict sound sequences, account for both preceding and following context (e.g., sarcasm changing the meaning of a sentence), and decouple voice identity from content by learning to encode and decode voice characteristics without hard-coding features like gender or age.
- Early adoption came from unexpected prosumer use cases: a beta tester pasted an entire book into the tweet-length text box, uploaded the AI narration to platforms that banned AI content (and it passed as human), triggering viral demand among book authors; later, no-face YouTube channels and AI-generated comedy videos drove explosive growth.
- A major inflection point was the release of multilingual dubbing and voice cloning; one viral example was the Lex Friedman interview with Prime Minister Narendra Modi, where ElevenLabs dubbed both speakers into Hindi and English, gaining massive traction in India and the US.
- Voice agents are rapidly being adopted across industries: Hippocratic AI automates nurse-patient calls to check medication adherence, Deutsche Telekom builds internal voice customer support, and Chess.com is experimenting with AI voice companions modeled after Magnus Carlsen or Hikaru Nakamura to coach players.
- Enterprise bottlenecks are less about voice quality and more about integrations: companies need deep connections to Twilio, CRM systems, and telecom infrastructure; ElevenLabs addresses this by offering a conversational AI stack with built-in RAG and function-calling, and by progressively building an integration library.
- ElevenLabs uses a cascading architecture (speech-to-text → LLM → text-to-speech) in production for reliability, but is developing a ‘duplex’ end-to-end model that promises lower latency and greater contextual expressivity; the company aims to achieve a voice Turing test pass this year or early 2026, though the cascading vs. duplex trade-off in reliability vs. expressivity remains an engineering challenge.
- To manage coop-etition with foundation model providers, ElevenLabs remains LLM-agnostic, often cascading multiple LLMs for fallback, and focuses on the audio layer as a complementary piece; Staniszewski views most companies as partners and believes the competition will drive the whole space forward.
- Customers prioritize three things: quality (expressiveness in English and other languages), latency (critical for conversational agents), and reliability at scale (demonstrated by handling millions of simultaneous interactions in Epic Games’ Darth Vader experience without degradation).
- Staniszewski’s long-term vision centers on voice as the background interface: always-on tutors for personalized learning, real-time translation devices (a Babel Fish concept) enabling seamless cross-cultural communication, and agent-to-agent interaction for tasks like booking restaurants, with voice as the primary modality.
- Despite being a remote company headquartered in London, ElevenLabs benefited from European talent that is passionate and hardworking, and a global-first mindset prompted by Europe’s multilingual reality; key disadvantages include a less mature founder ecosystem and regulatory drag from the EU AI Act.
- Safety measures include robust audio provenance (every generated clip traces back to an account), moderation on voice and text levels, and collaboration with academic groups like UC Berkeley to build detection models; Staniszewski acknowledges the cat-and-mouse nature of open-source generation and evolving authentication needs.

## Notable quotes
> "The ideal—like we would love to prove that it's possible this year. Like, we can cross the Turing test of speaking with an agent and you just would say, like, this is speaking another human. I think it's a very ambitious goal, but I think it's possible." — Mati Staniszewski
>
> "In audio, the data—first of all, there's much less of the high quality audio that actually would get you the result you need. And then the second, it frequently doesn't come with transcription or with a high accurate text of what was spoken. ... And then there's a third component ... not only what was said, so the transcript of the audio, but also how well was it said. What emotions did you use? Who said it? What are some of the non-verbal elements that were said?" — Mati Staniszewski
>

## People mentioned
- Mati Staniszewski
- Piotr (ElevenLabs co-founder)
- Lex Friedman
- Narendra Modi
- Richard Feynman
- Magnus Carlsen
- Gary Kasparov
- Hikaru Nakamura
- Demis Hassabis
- Dario Amodei
- will.i.am
- Bret Taylor

## Topics
`text-to-speech` `voice-agents` `audio-ai` `conversational-ai` `language-translation` `startup-competition` `europe-tech`

