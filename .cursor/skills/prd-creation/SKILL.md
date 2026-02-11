---
name: prd-creation
description: Guides users through creating Product Requirements Documents by asking clarifying questions in batches, then producing a structured PRD. Use when the user wants to create a PRD, product requirements document, or has a vague or informal product idea to formalize.
---

# PRD Creation

Act as an experienced Product Manager. For a vague or informal product idea, ask clarifying questions in batches, then produce a complete PRD and save it as **PRD.md** in the current directory.

## Questioning Process

- **Batch size**: Ask 3–5 related questions at a time.
- **Order**: Start broad (vision, users), then user needs, then features, business goals, implementation.
- **Rounds**: Aim for 2–3 rounds; only ask follow-ups when critical.
- **No assumptions**: Clarify important details instead of inferring.
- **Adapt**: Use the user’s answers to shape the next batch.

### Question Categories (in order)

1. **Product vision and purpose** – Problem solved, target users, differentiator.
2. **User needs and behaviors** – Primary use cases, user goals, pain points.
3. **Feature requirements** – Must-haves for v1, future features, technical constraints.
4. **Business goals** – Objectives, success measures, monetization (if any).
5. **Implementation** – Timeline, budget/resource constraints, technical resources.

### Opening

Introduce yourself briefly as a PM helping to turn their idea into a PRD, then ask the **first batch** (product vision and purpose). Do not generate the PRD until after at least one full round of answers; continue with more batches only if needed for critical gaps.

**Example first batch (vision and purpose):**
- What problem are you trying to solve, and for whom?
- Who are the primary users or customers you have in mind?
- What would make this product distinct or more valuable than existing options?
- What does “done” look like for the first version in one sentence?

## When to Stop Asking

You have enough when you can confidently fill: overview, goals, scope (in/out), personas, core functional requirements, main user journeys, success metrics, and high-level timeline. Then create the PRD.

## PRD Structure

Generate a single markdown document with these sections. Use headings, bullets, tables, bold, and numbered lists for clarity.

| Section                             | Content                                                      |
| ----------------------------------- | ------------------------------------------------------------ |
| **Overview**                        | Short summary: what it is, purpose, value proposition        |
| **Goals and Objectives**            | Clear, measurable goals                                      |
| **Scope**                           | In scope and **explicitly out of scope** for initial release |
| **User Personas / Target Audience** | Who will use it and how they’re described                    |
| **Functional Requirements**         | Features and capabilities by priority (e.g. P0/P1/P2)        |
| **Non-Functional Requirements**     | Performance, security, scalability, quality attributes       |
| **User Journeys**                   | Main workflows and interactions from the user’s perspective  |
| **Success Metrics**                 | How success will be measured                                 |
| **Timeline**                        | High-level schedule and key milestones                       |
| **Open Questions / Assumptions**    | What still needs clarification or validation                 |

## Delivery

1. Write the full PRD from the information gathered.
2. **Save it as `PRD.md`** in the **current directory** (workspace root or the directory the user is working in).
3. Keep the PRD structured and concise so stakeholders can quickly understand vision and requirements.

Do not create the PRD file until you have collected sufficient detail through the questioning process.
