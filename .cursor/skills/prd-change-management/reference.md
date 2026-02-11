# PRD Change Management – Reference

## Change classification categories

| Category | Description |
|----------|-------------|
| **New Feature** | Entirely new functionality not in the original PRD |
| **Feature Modification** | Changes to existing planned features (behavior, scope, or acceptance criteria) |
| **Feature Removal** | Removing previously planned features |
| **Scope Change** | Fundamental changes to project scope or objectives |
| **Technical Change** | Changes to technical approach, architecture, or stack |
| **Timeline Change** | Changes to delivery schedule or milestones |

## Size and priority

**Size**

- **Small**: Localized change; few artifacts affected; low rework (e.g. wording, one user story).
- **Medium**: Multiple features or RFCs; moderate rework or new work (e.g. new flow, revised scope of 2–3 features).
- **Large**: Broad impact; many RFCs/features; significant rework or new phase (e.g. new platform, major scope pivot).

**Priority**

- **Must-have**: Required for current release or to avoid unacceptable risk; should not be deferred without explicit stakeholder agreement.
- **Nice-to-have**: Desirable but can be deferred to a later version without blocking success.

## Change classification table (use in analysis)

Use this structure when listing classified changes:

```markdown
| ID | Change summary | Category | Size | Priority |
|----|----------------|----------|------|-----------|
| C1 | …              | New Feature | Medium | Must-have |
| C2 | …              | Feature Modification | Small | Nice-to-have |
```

Assign short IDs (C1, C2, …) so you can reference them in impact analysis and recommendations.

## Recommendation block template (per change)

For each change, use a block like:

```markdown
### Change C[N]: [Short title]

- **Decision**: [Implement now | Schedule for sprint X / Phase Y | Separate phase/release | Defer to v2 – reason]
- **Steps**:
  1. [Concrete step]
  2. [Concrete step]
  3. …
- **Owner/trigger**: [Who does what or what event triggers next step]
```

## Stakeholder message template

When drafting key messages in the communication plan:

- **Engineering**: What is changing in scope or specs; which RFCs/features are affected; refactor or test implications.
- **Product**: What is changing in PRD/acceptance criteria; impact on roadmap and priorities.
- **Leadership**: Business impact, timeline/resource impact, and any approval needed.
- **Users** (if applicable): What is new, changed, or removed and when they will see it.

Keep each group to 1–2 bullets unless the change set is large.
