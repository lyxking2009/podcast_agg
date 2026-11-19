---
podcast: "Latent Space: The AI Engineer Podcast"
episode: "Designing for Agents — Zeno Rocha, Resend"
published: 2026-11-10
duration: "1h25m52s"
audio_url: "https://podcasts.beehiiv.com/3967c1f2-cb97-4eb0-a5c0-21a1d2579408/01a15fdf-0e93-7e1a-880d-d73752a1f915/01a1c6f7-44d3-72a3-8e65-ffcfae742084/01a1c6f7-4605-726f-ac19-31f9b1763da0.mp3"
episode_url: "https://lspace.beehiiv.com/podcast/s/latent_space_the_ai_engineer_podcast/designing_for_agents_zeno_rocha_resend"
transcript_source: rss
generated_at: 2026-11-13T23:03:38Z
model: "deepseek-v4-pro"
guid: "01a1c6f7-44d3-7eaf-95ea-cb9de51baff2"
---

# Designing for Agents — Zeno Rocha, Resend — Latent Space: The AI Engineer Podcast


## TL;DR
Zeno Rocha discusses building Resend, an email infrastructure platform now powering 4.3 million users with 120k paid customers including Warner Brothers and HBO. He details the underappreciated complexity of email deliverability, abuse prevention, and the strategic use of open source (React Email), MCP integrations, and AI-optimized SEO. He also shares personal lessons on balancing startup ambition with family life.

## Key points
- Resend has reached 4.3 million users and 120,000 paid customers, with notable clients like Warner Brothers and HBO; they surpassed SendGrid's NPM SDK downloads earlier this year and now have more than double.
- Email deliverability requires respecting mailbox provider rules: Gmail, Outlook, and Yahoo throttle based on engagement signals; senders must warm up gradually, space batches, and avoid patterns typical of abusers like buying new domains and blasting emails once.
- Abuse prevention is critical: Resend encountered a Japanese gang that registered 20,000 domains; they detect abuse by checking domain age, website HTTP status, lookalike domains, and sending patterns, blocking quickly and then engaging users to distinguish misuse from abuse.
- Resend initially built on AWS SES but is now building its own infrastructure 'down to the metal'—buying IP blocks, managing IP pools, and establishing direct relationships with mailbox providers to improve throughput, deliverability, and observability.
- AWS SES has hidden costs: users need SNS for events, CloudWatch for logs, SQS for queues, and custom webhooks, leading to a much larger footprint than the low base price suggests.
- Resend created React Email as an open-source top-of-funnel; they wrote integration docs for competitors (SendGrid, MailChimp, Mailgun) to capture developers, then converted them to Resend.
- To scale support without losing free tier, Resend implemented a policy to answer tickets only with links to docs, which forced creation of a knowledge base; with 4.3M users they now use AI and automation but keep free tier for accessibility.
- Domain Connect protocol enables OAuth-like domain setup, reducing DNS misconfiguration support tickets; Resend helped convince Vercel and others to adopt it, though negotiating with providers like GoDaddy and Ionos took months.
- MCP (Model Context Protocol) adoption at Resend grew from 50% month-over-month to doubling monthly, reaching 1.2 million calls by end of August; they built OAuth and RBAC to support remote MCP, now offering 80+ tools covering full dashboard functionality.
- AI/LLM SEO strategy: Resend adds FAQ sections on all pages, llms.txt, markdown versions of pages, and a pricing calculator in markdown because LLMs don't execute JavaScript; this improved accuracy of AI recommendations and ranking.
- Zeno Rocha prioritizes family: he moved to Brazil for second child and chose a design-centric product even if slower, referencing YC advice and Paul Graham's 'infinite game' perspective; he would rather have a close family and $10M ARR than $100M ARR with a distant daughter.
- Enterprise sales at Resend started with one AE; they recognize enterprise requires a different product (SSO, SCIM, audit logs) not just add-ons, and they are building that motion carefully to protect culture.
- The email industry's contact-based pricing is broken for fast-growing AI companies like Mistral; Resend is exploring alternative pricing models to accommodate rapid user growth without penalizing senders.

## Notable quotes
> "So Resend now is powering four point three million users as of this morning. Uh, we have all sorts of customers like, uh, Warner Brothers, HBO." — Zeno Rocha
> "So the whole way of thinking about email marketing and, and email in general is you, you need to do exactly what an abuser wouldn't do. It's the opposite of that." — Zeno Rocha
> "I would rather live in that reality than live in a reality where we're a hundred million dollar company and my daughter doesn't wanna play with me, is not interested in me, and we're, we're not that close." — Zeno Rocha

## People mentioned
- Zeno Rocha
- Paul Graham
- Dharmesh Shah
- Michael Grinich
- Quincy Larson
- Jeff Lawson
- Yoko

## Topics
`email-infrastructure` `developer-experience` `ai-agents` `mcp` `startup-culture` `product-design` `deliverability`
