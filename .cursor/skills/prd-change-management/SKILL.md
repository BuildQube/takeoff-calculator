---
name: prd-change-management
description: Analyzes and integrates proposed changes to an existing Product Requirements Document (PRD) while development is in progress. Classifies changes, assesses impact on features and RFCs, recommends implementation strategy, and produces updated PRD sections and communication plans. Use when incorporating PRD changes mid-development, analyzing change impact, or when the user has proposed PRD modifications and wants minimal disruption to ongoing work.
---

# PRD Change Management

Act as an expert product manager and change management specialist. Analyze the original PRD, current development status, and proposed changes; then recommend how to integrate changes with minimal disruption to ongoing development.

## Input

- **Original PRD**: `PRD.md`, `prd.md`, or user-specified path.
- **Current development context**: What is done vs in progress (e.g. `features.md`, `RFCs/`, implemented vs open RFCs).
- **Proposed changes**: Attached text, a separate change document, or a diff/description of what should change in the PRD.

If critical information is missing (e.g. no dev status, unclear scope of proposed changes), ask specific questions before proceeding. Do not guess.

For a systematic pass, use [checklist.md](checklist.md).

---

## Output Order

1. **Summary** – Overall assessment of proposed changes and their impact (2–4 paragraphs).
2. **Detailed analysis** – Follow the six sections below.
3. **Recommendations** – Clear, actionable recommendation per change with concrete next steps.

---

## 1. Change Classification

For each proposed change:

- **Category** (one per change):
  - **New Feature** – Functionality not in the original PRD.
  - **Feature Modification** – Changes to existing planned features.
  - **Feature Removal** – Removing previously planned features.
  - **Scope Change** – Fundamental change to project scope or objectives.
  - **Technical Change** – Change to technical approach or architecture.
  - **Timeline Change** – Change to delivery schedule or milestones.
- **Size**: Small | Medium | Large.
- **Priority**: Must-have | Nice-to-have.

Use the table format in [reference.md](reference.md) for consistency.

---

## 2. Impact Analysis

For each change, assess:

- **Affected artifacts**: Components, features (from `features.md`), and RFCs (from `RFCs/`) that are touched.
- **Timeline and resources**: Effect on schedule and capacity.
- **Technical dependencies**: Ripple effects, breaking changes, refactors.
- **Completed or in-progress work**: What already exists that this change affects.
- **Testing and validation**: New or changed tests, regression risk.
- **User experience and product coherence**: UX and product consistency impact.

Summarize per change; call out high-impact items explicitly.

---

## 3. Implementation Strategy

For each change, recommend one of:

- **Implement now** – Integrate into current sprint.
- **Schedule for future sprint** – Assign to a specific phase/sprint with brief rationale.
- **Separate phase or release** – Implement as a distinct phase or release.
- **Defer** – Move to a future version; state reason.

Also:

- **Refactoring**: What already-implemented parts need refactoring and why.
- **Parallel work**: Where parallel work streams could reduce disruption.
- **Testing strategy**: How to validate the change (e.g. regression, new tests, acceptance criteria).

Be specific (e.g. “RFC-003 and RFC-005”; “Phase 2 after MVP”).

---

## 4. Documentation Updates

Produce or specify:

- **Updated PRD sections** – Revised text that incorporates the change; use clear “Before / After” or “Revised” callouts where useful.
- **Modified user stories and acceptance criteria** – List changes and provide updated wording.
- **Technical specifications** – Revisions to any impacted specs.
- **Timeline and milestones** – Updated dates/phases if the change affects schedule.

Save revised PRD content as requested (e.g. `prd-updated.md` or inline in the analysis). If the user does not specify a file, propose one (e.g. `prd-updated.md` in the same directory as the source PRD).

---

## 5. Communication Plan

- **Stakeholders**: Who must be informed (e.g. eng, product, leadership, users).
- **Key messages**: 1–2 bullet points per stakeholder group.
- **Sync points**: When and how to align with the development team (e.g. change review, backlog refinement).
- **Review/approval**: Suggested change review or approval steps (e.g. PRD change review meeting, sign-off checklist).

---

## 6. Risk Assessment

- **Risks of implementing mid-development**: List with severity (High / Medium / Low).
- **Mitigation**: One or two concrete mitigations per material risk.
- **Product quality and technical debt**: Impact and how to limit it.
- **Business risk of not implementing**: What happens if the change is deferred or rejected.

---

## Recommendations Deliverable

End with a **per-change recommendation** that includes:

- **Decision**: Implement now | Schedule (when) | Separate phase/release | Defer (reason).
- **Concrete steps**: 3–5 actionable steps (e.g. update PRD section X, adjust RFC-004 acceptance criteria, add regression tests for Y, notify Z by date).
- **Owner/trigger**: Who or what triggers the next step (e.g. “Product updates PRD”; “Eng leads refactor after RFC-003 merge”).

Be specific so the team can execute without further clarification.

---

## Principles

- **Minimize disruption**: Prefer options that protect in-progress work and avoid rework where possible.
- **Evidence-based**: Tie impact and recommendations to specific PRD sections, features, and RFCs.
- **Actionable**: Every recommendation should be implementable; avoid vague or generic advice.
- **Traceable**: Clearly link each proposed change to its classification, impact, strategy, and documentation updates.

For classification and output templates, see [reference.md](reference.md).
