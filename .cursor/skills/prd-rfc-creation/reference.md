# RFC Structure and Implementation Prompt Reference

## RFC Document Sections

Each RFC file should include these sections. Adapt depth to complexity; every section need not be lengthy.

### Header and identity
- **Identifier**: e.g. RFC-001
- **Title**: Clear, descriptive
- **Summary**: 1–2 paragraphs on what the RFC covers

### Scope and relationships
- **Features/requirements addressed**: Bulleted list (with feature IDs from features.md if applicable)
- **Depends on**: Previous RFCs (by ID) that must be complete first
- **Enables**: Later RFCs that build on this one
- **Complexity**: Low | Medium | High

### Technical content
- **Technical approach**: Architecture, design decisions, key algorithms or business logic
- **API contracts / interfaces**: Endpoints, request/response formats, or module interfaces
- **Data models / schema**: New or changed entities, fields, relationships
- **State management**: How state is stored, updated, and passed (if applicable)
- **File structure**: Suggested or required layout (modules, packages, key files)
- **Error handling**: Strategies, error codes, fallbacks
- **Testing strategy**: Test types, key scenarios, edge cases
- **Performance**: Expectations, optimization notes, budgets if any
- **Security**: Auth, authorization, safeguards relevant to this RFC
- **Accessibility / i18n**: A11y and localization considerations if relevant

### Implementation guidance
- **Acceptance criteria**: Testable conditions per feature (can be checklist)
- **Implementation considerations**: Technical challenges, edge cases, rules from RULES.md that apply
- **Third-party dependencies**: Libraries, services, versions
- **Constraints**: Coding standards, compatibility, compliance that affect this RFC

### Optional (when useful)
- **Component / data flow**: Text descriptions or ASCII diagrams
- **Pseudocode**: For non-obvious algorithms
- **Logging and monitoring**: What to log, what to monitor

---

## Implementation Prompt Creation Rules

When generating `implementation-prompt-RFC-[ID].md`:

1. **Source**: Use `RFCs/implementation-prompt-template.md` in the project if it exists; otherwise use the skill’s `implementation-prompt-template.md`.
2. **Read the full template** before making changes.
3. **Replace only** these placeholders (exact match, all occurrences):
   - `[ID]` → the RFC’s numeric ID (e.g. `001`)
   - `[Title]` → the RFC’s title (e.g. `User Authentication`)
   - `[brief description]` → a short summary of the RFC’s purpose (1–2 sentences)
4. **Do not**:
   - Add, remove, or rename sections
   - Change headings or structure
   - Paste in full RFC content (the prompt should reference the RFC file, not duplicate it)
   - Modify any other template text
5. **Verify**: All three placeholders replaced; file structure identical to template.

The implementation prompt is a **short directive** to implement one RFC; detailed specs stay in the RFC document itself.
