---
name: prd-rfc-creation
description: Breaks down Product Requirements Documents (PRDs), features lists, and project rules into sequentially implementable Request for Comments (RFC) documents. Use when creating RFCs from a PRD, planning implementation order from product requirements, or when the user asks for RFC generation from features or PRDs.
---

# PRD to RFC Creation

Break down the attached or referenced PRD, features list, and project rules into RFC documents under an **RFCs** folder. RFCs are implemented **strictly one by one** in numerical order (001, 002, …); ordering is critical.

## Input

- **PRD** (e.g. `PRD.md`, `prd.md`)
- **Features list** (e.g. `features.md`)
- **Project rules** (e.g. `.cursor/rules/*`, `RULES.md`) when present

If critical information is missing or unclear, ask specific questions before generating RFCs.

## Output Location

- All RFCs and supporting files: **RFCs/** (project root).
- Master index: **RFCs/RFCS.md**.

---

## Workflow Overview

1. **Implementation order analysis** → dependency graph, phases, sequential numbering.
2. **Feature grouping** → cohesive RFCs, balanced size, dependency-aware.
3. **RFC authoring** → one file per RFC with full structure (see [reference.md](reference.md)).
4. **Implementation prompts** → one prompt per RFC from template (placeholders only).
5. **RFCS.md** → master list, dependency table, sequential roadmap.
6. **Technical specs & constraints** → per-RFC details and project-wide constraints.

---

## 1. Implementation Order Analysis

- Build a **directed dependency graph** of features (describe textually).
- Identify **foundation** components that must come first.
- Find **critical path** items that block others.
- Group into **implementation phases** by dependency.
- Assign **sequential IDs**: RFC-001, RFC-002, … so that:
  - Each RFC is fully implementable only after all previous RFCs are done.
  - No parallel implementation; RFC-N+1 starts only after RFC-N is complete.
- Ensure the sequence is **logical and buildable**; map all feature-to-RFC dependencies.

**Deliverable**: Clear implementation order and dependency rationale (can be summarized in RFCS.md).

---

## 2. Feature Grouping

- Group **related features** into a single RFC when they form a logical unit.
- **Balance size**: avoid trivial (too small) or unmanageable (too large) RFCs.
- Respect **dependencies**: features that build on others go in later RFCs.
- Align groups with the **strict sequential order** from step 1.

---

## 3. RFC Structure and Content

For each RFC file (e.g. `RFCs/RFC-001-Short-Title.md`):

- **Identifier & title**: e.g. RFC-001, clear title.
- **Summary**: What this RFC covers.
- **Features/requirements**: List of addressed features/requirements.
- **Depends on**: Which previous RFCs this builds on.
- **Enables**: Which future RFCs build on this.
- **Complexity**: Low | Medium | High.
- **Acceptance criteria**: Per feature, testable.
- **Technical approach**: Architecture, APIs, data models, state, algorithms, file structure, error handling, testing, performance.
- **Implementation considerations**: Challenges, edge cases, rules from RULES.md, security, i18n, a11y, third-party deps.

Full section checklist and technical specification expectations are in [reference.md](reference.md).

---

## 4. Implementation Prompt Creation

- One prompt file per RFC: **RFCs/implementation-prompt-RFC-[ID].md** (e.g. `implementation-prompt-RFC-001.md`).
- **Template**: Use the project’s `RFCs/implementation-prompt-template.md` if it exists; otherwise use this skill’s [implementation-prompt-template.md](implementation-prompt-template.md).
- **Only** do these replacements in the template:
  - `[ID]` → RFC identifier (e.g. `001`)
  - `[Title]` → RFC title (e.g. `User Authentication`)
  - `[brief description]` → Short summary of the RFC’s purpose
- **Do not** change section headings, add/remove sections, or duplicate RFC body content in the prompt.
- Verify every placeholder is replaced and structure matches the template exactly.

---

## 5. RFCS.md (Master File)

Create **RFCs/RFCS.md** with:

- All RFCs in **strict numerical implementation order**.
- A **dependency graph or table**: which RFCs each RFC builds on and enables.
- **Sequential implementation roadmap** (and optional phase grouping).
- Explicit note that implementation proceeds **strictly in numbered order**.

---

## 6. Technical Specifications and Constraints

Per RFC, include as appropriate:

- **Architecture / data flow** (text descriptions or diagrams).
- **APIs**: endpoints, request/response shapes.
- **Schema**: DB/data model changes, field definitions.
- **State, auth, caching, errors, logging**: as relevant.
- **Algorithms / business logic**: pseudocode or clear steps.
- **Constraints**: coding standards, performance budgets, compatibility, compliance.

Document how each RFC **builds on or extends** prior RFCs. Ensure specs support the chosen sequential order.

---

## Verification Checklist

Before finishing:

- [ ] Every RFC has a unique ID in implementation order (001, 002, …).
- [ ] Dependencies are correct: each RFC only depends on earlier RFCs.
- [ ] Each RFC is implementable after all previous RFCs are done.
- [ ] Implementation prompts use only the three placeholder replacements; template structure is unchanged.
- [ ] RFCS.md lists all RFCs in order and shows dependency relationships.
- [ ] No critical information from the PRD/features is missing from the RFC set.

---

## Additional Resources

- Full RFC section list and implementation-prompt rules: [reference.md](reference.md)
- Default implementation prompt template: [implementation-prompt-template.md](implementation-prompt-template.md)
