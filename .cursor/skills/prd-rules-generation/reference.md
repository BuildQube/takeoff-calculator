# RULES.md Structure Reference

Reference structure for generating RULES.md from a PRD and features list.

---

## Document Structure

```markdown
# Project Rules

[Optional: 1–2 paragraph overview of the project and purpose of these rules]

## 1. Technology Stack

## 2. Technical Preferences

## 3. Development Standards

## 4. Implementation Priorities

## 5. General Guidelines
```

---

## Section 1: Technology Stack

Use a table or structured list. Be explicit about versions when the PRD implies them.

**Example (Rust backend):**

| Component | Technology | Version / Notes |
|-----------|------------|-----------------|
| Language | Rust | 1.75+ (edition 2021) |
| Async runtime | tokio | Latest stable |
| Serialization | serde | serde_json for JSON |
| Testing | cargo test | Standard test harness |
| Bindings | NAPI (Node), WASI (Web) | As specified in PRD |

**Example (full-stack):**

| Layer | Technology | Version |
|-------|------------|---------|
| Frontend | React | 18.x |
| Backend | Node.js | 20 LTS |
| Database | PostgreSQL | 16 |
| API | REST / JSON | — |

Only include what the PRD/features explicitly or clearly imply.

---

## Section 2: Technical Preferences

### 2.1 Naming Conventions

- **Files**: `snake_case` for Rust; `PascalCase` for components
- **Variables/Functions**: `snake_case`
- **Types/Structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE` or `UPPER_CAMEL` per language convention

Include 1–2 examples per category relevant to the stack.

### 2.2 Code Organization

- Folder structure (e.g. `src/{domain}/`, `crates/` for workspace)
- Modularity rules (single responsibility, avoid circular deps)
- Where to put shared types, utilities, config

### 2.3 Architectural Patterns

- Layered, hexagonal, or other pattern from or implied by PRD
- Data flow direction
- Boundaries between modules/crates/packages

### 2.4 Data Handling and API Interactions

- Request/response shapes (typed, validated)
- Error representation
- State management approach if applicable

### 2.5 Performance

- Any performance budgets from the PRD
- Benchmarking expectations
- Optimization guidelines (when to optimize vs when to defer)

### 2.6 Security

- Input validation
- Secrets management
- Auth patterns if applicable

---

## Section 3: Development Standards

### 3.1 Testing

- Unit tests: what to cover, where to place them
- Integration tests: scope and location
- Coverage expectations (if PRD specifies)
- Mocking/stubbing approach

**Example:** "Unit tests for all public API functions. Integration tests for critical user flows. Aim for ≥80% coverage on core modules."

### 3.2 Documentation

- Inline comments and docstrings
- API documentation (e.g. Rustdoc, JSDoc)
- README expectations

### 3.3 Error Handling and Logging

- Use `Result`/`Option` (or equivalents); avoid silent failures
- Log levels: when to use debug, info, warn, error
- Structured logging if required

### 3.4 Accessibility (if applicable)

- WCAG level if specified
- Keyboard navigation, screen readers, contrast

### 3.5 Responsive Design (if applicable)

- Breakpoints
- Mobile-first vs desktop-first

---

## Section 4: Implementation Priorities

- **Core (P0/Must-have):** Requirements that must be complete and correct before release
- **Enhancements (P1/P2):** Nice-to-have; can be deferred or simplified if needed
- **Phases:** Any phased rollout or milestones from the PRD
- **Quality thresholds:** Coverage, performance, or correctness bars that must be met

**Example:** "F1–F5 and F10 are core. F6–F7, F11–F12 are important but can be phased. All core features must pass acceptance criteria and benchmarks before merge."

---

## Section 5: General Guidelines

### Requirements Adherence

- Follow the PRD and features list precisely
- Do not add features or change behavior outside documented scope
- When requirements conflict, flag for clarification

### Code Quality

- Readable, maintainable, consistent with project style
- Avoid duplication; extract shared logic

### Completeness

- No TODOs, placeholders, or stub implementations without explicit approval
- Deliver working, tested code for each feature

### Handling Uncertainty

- If a requirement is ambiguous: ask, document assumptions, or flag for review
- Do not guess at business-critical behavior

### Communication

- Prefer asking for clarification over incorrect implementation
- Document non-obvious decisions in code or ADRs

---

## Example RULES.md Snippet

```markdown
## 2. Technical Preferences

### Naming
- **Rust crates**: `snake_case` (e.g. `takeoff_core`)
- **Modules**: `snake_case`
- **Types/structs**: `PascalCase`
- **Functions/variables**: `snake_case`

### Structure
- Core logic in `takeoff_core`
- Bindings in separate crates: `takeoff-node`, `takeoff-wasi`
- Tests colocated with source: `mod.rs` + `mod_test.rs` or `#[cfg(test)]`

### Architecture
- Pure computation in core; no I/O
- Bindings are thin wrappers over core API
- Scale and units as first-class abstractions; avoid magic numbers
```

---

## Anti-Patterns

- **Too vague:** "Use good practices" → specify what "good" means
- **Too rigid:** Over-specifying when the PRD leaves room for choice
- **Contradictory:** Rules that conflict with each other or the PRD
- **Out of scope:** Adding rules for technologies or concerns not in the PRD
