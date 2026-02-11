---
name: prd-rules-generation
description: Generates a comprehensive RULES.md file from a Product Requirements Document and features list. Establishes technical guidelines, development standards, and implementation priorities for AI-assisted development. Use when creating RULES.md from a PRD, generating project rules from product requirements, or when the user asks for rules or guidelines based on a PRD/features.
---

# PRD to RULES.md Generation

Generate a comprehensive `RULES.md` from the attached or referenced PRD and features list. Act as an expert software architect: create clear, actionable rules that ensure consistency, quality, and alignment with project requirements for AI-assisted development.

## Input

- **PRD** (e.g. `PRD.md`, `prd.md`)
- **Features list** (e.g. `features.md`)

If critical information is missing or unclear, ask specific questions before generating rules. Do not proceed with assumptions that could lead to incorrect or conflicting guidelines.

## Output

- **RULES.md** in the project root (or specified location)

---

## Workflow Overview

1. **Project overview** → Brief summary of the project from the PRD.
2. **Technology stack** → Core technologies, versions, libraries.
3. **Technical preferences** → Naming, structure, architecture, data, performance, security.
4. **Development standards** → Testing, docs, errors, logging, a11y, responsive design.
5. **Implementation priorities** → Core vs enhancements, phases, quality thresholds.
6. **General guidelines** → Requirements adherence, code quality, completeness, communication.
7. **Format and finalize** → Clean markdown, logical structure, actionable rules.

Full section structure and examples are in [reference.md](reference.md).

---

## 1. Project Overview

Before generating rules, provide a brief overview:

- What the project does and its value proposition
- Target users / personas
- Scope (in-scope vs out-of-scope)
- Key technical constraints mentioned in the PRD

This grounds the rules in context. Include 2–4 short paragraphs maximum.

---

## 2. Technology Stack Definition

Extract or infer from the PRD and features:

- **Core technologies** (language, runtime, frameworks)
- **Version pinning** — specify latest stable versions where possible
- **Required libraries** — only those explicitly or clearly implied
- **Tools** — build, test, lint, CI if mentioned

Use a table or bullet list. Be specific; avoid vague "use appropriate versions" unless the PRD leaves it open.

---

## 3. Technical Preferences

Define rules for:

| Area | What to specify |
|------|-----------------|
| Naming | Files, components, variables, types — conventions and examples |
| Structure | Folder layout, modularity, separation of concerns |
| Architecture | Patterns (e.g. layered, hexagonal), data flow direction |
| Data & state | Handling, validation, serialization, API contracts |
| Performance | Budgets, optimization expectations, benchmarks if mentioned |
| Security | Auth, input validation, secrets, compliance if applicable |

Rules should be specific and actionable. Prefer examples over abstract descriptions.

---

## 4. Development Standards

Define requirements for:

- **Testing** — types (unit, integration, e2e), coverage expectations, mocking
- **Documentation** — inline docs, API docs, README expectations
- **Error handling** — patterns, logging, user-facing vs internal errors
- **Logging** — levels, format, what to log
- **Accessibility** — standards (e.g. WCAG), when applicable
- **Responsive design** — expectations if a UI is in scope

Scope each rule to what the PRD actually covers. Omit sections not relevant (e.g. no a11y rules for a pure backend).

---

## 5. Implementation Priorities

Align rules with PRD priorities:

- **Core vs enhancements** — Must-have vs nice-to-have from the features list
- **Phased approach** — Any phases or milestones defined in the PRD
- **Quality thresholds** — Coverage, performance, or correctness bars

Ensure the rules reflect the PRD’s prioritization so AI assistance focuses effort correctly.

---

## 6. General Guidelines

Establish cross-cutting rules:

- **Requirements adherence** — Follow PRD and features precisely; do not invent scope
- **Code quality** — Readability, maintainability, consistency
- **Completeness** — No TODOs, placeholders, or stub implementations without explicit approval
- **Uncertainty** — How to handle ambiguity: ask, document assumptions, or flag for review
- **Communication** — When to ask for clarification vs proceed with best judgment

These rules apply to all implementation work guided by RULES.md.

---

## 7. RULES.md Format and Finalization

- Use clean, well-structured markdown with clear headings
- Organize logically; match the flow in [reference.md](reference.md)
- Make rules **specific, actionable, and unambiguous**
- Include short examples where they add clarity
- Be specific enough to guide development; flexible enough for creative problem-solving where appropriate

---

## Verification Checklist

Before finishing:

- [ ] Project overview is accurate and concise
- [ ] Technology stack matches or reasonably extends the PRD
- [ ] Technical preferences cover naming, structure, architecture, data, performance, security
- [ ] Development standards cover testing, docs, errors, logging (plus a11y/responsive if relevant)
- [ ] Implementation priorities align with PRD/features MoSCoW or priority levels
- [ ] General guidelines cover requirements adherence, quality, completeness, uncertainty
- [ ] Rules are actionable; no vague or contradictory guidance
- [ ] Critical PRD constraints are reflected in the rules

---

## Additional Resources

- Full RULES.md section structure and example content: [reference.md](reference.md)
