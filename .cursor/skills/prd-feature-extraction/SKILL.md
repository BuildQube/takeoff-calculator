---
name: prd-feature-extraction
description: Extract and organize features from a Product Requirements Document into a structured features.md with unique IDs, categories, MoSCoW priority, acceptance criteria, and complexity. Use when extracting features from a PRD, creating features.md for implementation planning, or when the user asks for a feature list from product requirements.
---

# PRD Feature Extraction

Act as an expert product manager and technical lead. Extract all features from the attached or referenced PRD and produce a comprehensive **features.md** for implementation planning.

## Input

- **Primary**: `PRD.md` or `prd.md` (workspace root or user-specified path). Read the PRD first.
- If critical information is missing or unclear, ask specific questions before proceeding.

## Before Extracting

1. **Brief product overview**: Summarize the product (vision, target users, core value) in 1–2 paragraphs.
2. **Clarification**: If the PRD lacks enough detail to define discrete, implementable features, ask targeted questions to fill gaps.

---

## Extraction Process

### 1. Feature identification

- Extract every explicit and implicit feature in the PRD.
- Make each feature **discrete, specific, and implementable**.
- Assign a **unique ID** per feature: `F1`, `F2`, `F3`, …

### 2. Feature categorization

- Group features by **logical category** (e.g. User Authentication, Dashboard, Reporting).
- Mark **core** vs **enhancement** vs **nice-to-have**.
- Tag by **user type/persona** where it matters.

### 3. Prioritization (MoSCoW)

| Priority        | Meaning                                   |
| --------------- | ----------------------------------------- |
| **Must have**   | Critical for MVP / initial release        |
| **Should have** | Important but not blocking first release  |
| **Could have**  | Desirable; can be deferred                |
| **Won't have**  | Out of scope this release; note for later |

- Apply one MoSCoW level per feature.
- Account for **dependencies** when assigning priority.

### 4. Feature detailing (per feature)

- **Description**: Clear, concise; understandable by technical and non-technical readers.
- **Acceptance criteria**: Testable conditions for “done.”
- **Technical notes**: Constraints, integrations, non-functional needs.
- **Edge cases**: Special handling, error cases, boundaries.

### 5. Implementation complexity

- **Complexity**: Low | Medium | High (relative).
- **Integrations**: Third-party or external systems required.
- **Risks**: Notable technical or delivery challenges.

### 6. features.md creation

- Write **features.md** in the workspace (root or path specified by the user).
- Use the structure below so the doc is consistent and easy to navigate.

---

## features.md Structure

Use this structure when creating **features.md**:

```markdown
# [Product Name] – Features

## Product overview
[1–2 paragraph summary from the PRD]

## Table of contents
- [Summary](#summary)
- [By category](#by-category) (link each category section)
- [By priority](#by-priority)
- [Feature specifications](#feature-specifications)

## Summary
| Priority    | Count |
| ----------- | ----- |
| Must have   | N     |
| Should have | N     |
| Could have  | N     |
| Won't have  | N     |

**By category:** [Category A: N] [Category B: N] …

## By category
[List category names and feature IDs under each]

## By priority
[Must have / Should have / Could have / Won't have – list feature IDs under each]

## Feature specifications

### [Category name]

#### F1: [Feature title]
- **Priority:** Must have | Should have | Could have | Won't have
- **Category:** [Category]
- **Persona:** [if applicable]
- **Description:** [Clear description]
- **Acceptance criteria:**
  - Criterion 1
  - Criterion 2
- **Technical notes:** [Constraints, integrations]
- **Edge cases / special handling:** [If any]
- **Complexity:** Low | Medium | High
- **Integrations / risks:** [If any]

[Repeat for F2, F3, …]
```

---

## Checklist before finalizing

- [ ] Every feature has a unique ID (F1, F2, …).
- [ ] Each feature has one MoSCoW priority.
- [ ] Acceptance criteria are testable and specific.
- [ ] Table of contents and summary counts are accurate.
- [ ] Descriptions are clear for both technical and non-technical readers.

For a compact extraction checklist, see [checklist.md](checklist.md).
