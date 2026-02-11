---
name: prd-validation
description: Reviews Product Requirements Documents for gaps, clarity, and implementation readiness. Performs gap analysis, improvement recommendations, and produces an enhanced PRD. Use when validating a PRD, reviewing PRD.md or prd.md after creation, or when the user asks for PRD review or implementation-ready feedback.
---

# PRD Validation

Act as an expert product manager. Review the PRD produced by the interactive PRD creation process (or any PRD in the repo) and provide actionable feedback so the document is implementation-ready.

## Input

- **Primary**: `PRD.md` or `prd.md` in the current directory (workspace root or user-specified directory).
- If the user names a different file or path, use that instead.

Read the PRD first; do not generate feedback without reviewing the actual content.

For a systematic gap pass, use [checklist.md](checklist.md).

---

## Step 1: Gap Analysis

Identify critical missing elements in these areas:

### 1. Product fundamentals
- Product vision and problem statement
- Target users and their needs
- Success metrics and scope boundaries

### 2. Technical requirements
- Technology constraints and integrations
- Security, performance, and scalability needs
- Infrastructure requirements

### 3. Business considerations
- Timeline and budget constraints
- Regulatory requirements
- Market factors and business model

### 4. Implementation factors
- Dependencies and third-party requirements
- Team resources and skills needed
- Testing and deployment needs

---

## Step 2: Improvement Recommendations

Provide specific recommendations in these areas:

### 1. Structure and clarity
- All essential sections present (see [prd-creation](.cursor/skills/prd-creation/SKILL.md) for expected structure)
- Ambiguous requirements clarified
- User stories formatted with clear acceptance criteria

### 2. Completeness and feasibility
- Gaps in user journeys filled
- Technical risks or challenges called out
- Alternatives suggested for problematic or vague requirements

### 3. Prioritization and implementation
- MoSCoW (Must / Should / Could / Won’t) or P0/P1/P2 applied consistently
- Critical path requirements identified
- Logical implementation sequence suggested

---

## Deliverables

Produce all of the following.

### 1. Summary of findings
- **Critical gaps**: List with impact (High / Medium / Low).
- **Overall assessment**: 2–3 sentences on readiness and main gaps.

### 2. Specific recommendations
- Concrete suggestions with short examples where helpful.
- Show how to clarify ambiguous requirements (e.g. before/after wording).

### 3. Improved PRD
- Create an enhanced version that addresses the issues found.
- Use clean markdown: headings, tables, bullets, bold.
- **Save as `prd-improved.md`** in the **current directory** (same location as the source PRD unless the user specifies otherwise).

### 4. Quality assessment
- Score the original PRD (1–10) on:
  - **Completeness** – Nothing critical missing.
  - **Clarity** – Unambiguous and scannable.
  - **Feasibility** – Realistic for team and constraints.
  - **User-focus** – Needs and journeys clearly reflected.
- One brief sentence per score.

---

## Principles

- **Practical**: Focus on changes that make implementation easier.
- **Second step**: Assume the PRD came from the interactive creation process; build on it rather than redoing it.
- **Actionable**: Every recommendation should be something the team can apply directly.
