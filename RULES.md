# Project Rules

These rules establish technical guidelines, development standards, and implementation priorities for the Takeoff Calculator backend. They are derived from the Product Requirements Document (PRD.md) and features list (features.md) and are intended for AI-assisted and human development.

**Project summary:** The Takeoff Calculator is a Rust-based backend that converts pixel-defined geometry into real-world measurements. It provides a single source of truth for quantity (and cost-ready) estimation from drawings or design coordinates, with first-class support for units, scales, and grouped aggregates. Target users are app developers and integration developers building takeoff, estimation, or design tools (web or Node). The backend runs the same Rust core in Node (NAPI) and web (WASI); no shipped UI—consumers build their own. Scope: inputs (pixel coordinates/lengths, multiple scales), outputs (lengths, areas, counts, grouped aggregates in chosen units), and delivery as library/API only. Out of scope: exported end-user UI. Key constraints: type-safe Rust core, consistent NAPI and WASI behavior, clear separation between core and bindings, and no silent failures on invalid input.

---

## 1. Technology Stack

| Component       | Technology                    | Version / Notes                          |
|----------------|-------------------------------|------------------------------------------|
| Language       | Rust                          | Edition 2021; use stable toolchain       |
| Core crate     | `takeoff_core`                | Pure computation; no I/O                 |
| Serialization  | serde                         | For API contracts and error serialization|
| Node bindings  | NAPI (napi-rs or equivalent)  | In `packages/bindings`                   |
| Web bindings   | WASI / WASM                   | Same bindings package; browser + Node    |
| Build / test   | cargo, cargo test             | Standard Rust harness                    |
| Benchmarks     | Rust benchmark harness        | Run in CI; guard regressions             |
| Publishing     | npm package (bindings)        | Semver for bindings; optional crates.io  |

Only add dependencies explicitly needed for scales, units, geometry, or bindings; avoid unnecessary crates.

---

## 2. Technical Preferences

### 2.1 Naming Conventions

- **Rust crates:** `snake_case` (e.g. `takeoff_core`).
- **Modules and files:** `snake_case` (e.g. `scale.rs`, `measurement.rs`).
- **Types and structs:** `PascalCase` (e.g. `Scale`, `Measurement`, `InvalidScale`).
- **Functions and variables:** `snake_case` (e.g. `create_scale`, `total_area`).
- **Constants:** `SCREAMING_SNAKE_CASE` where appropriate.
- **Error variants:** `PascalCase` and descriptive (e.g. `EmptyGeometry`, `UnknownUnit`).

### 2.2 Code Organization

- **Core logic:** All measurement, scale, unit, and group logic lives in `crates/takeoff_core`. No I/O, no bindings-specific code in core.
- **Bindings:** Node and WASI bindings in `packages/bindings`; thin wrappers over the core API. No business logic in bindings.
- **Structure within core:** Organize by domain (e.g. `scale`, `measurement`, `group`, `unit`). Shared types and error types in clearly named modules.
- **Tests:** Colocated with source (`#[cfg(test)]` in same module or `*_test.rs`). Integration and accuracy tests in appropriate crate or `tests/`.
- **Avoid:** Circular dependencies between core modules; putting NAPI/WASI concerns in `takeoff_core`.

### 2.3 Architectural Patterns

- **Layered separation:** Core (pure computation) → bindings (FFI and API surface). Data flow: caller → bindings → core → bindings → caller.
- **Scale and units:** First-class abstractions. Scale has ratio and unit; measurements attach to a scale; groups inherit scale from their measurements. No global scale requirement.
- **Measurements:** Typed kinds (polygon, polyline, rectangle, count) with single, documented formulae: polygon area = Shoelace; polyline length = sum of segment lengths; rectangle area = width × height; count = item count.
- **Groups:** First-class; own a set of measurements. Aggregates (total area, total length, point count, item count) recompute when inputs or scales change—no stale aggregates.

### 2.4 Data Handling and API

- **Scales:** Create with ratio and unit; list where needed. Zero or invalid ratio/unit must not be accepted.
- **Measurements:** Create by kind; attach scale; get length/area/volume in requested unit. All invalid inputs (empty geometry, zero scale, unknown unit) return typed errors—never silent wrong values.
- **Errors:** Typed, serializable variants (e.g. `InvalidScale`, `UnknownUnit`, `EmptyGeometry`) in core; exposed through bindings so Node and WASI get the same error shape. Document all error variants.
- **State:** No hidden global state; context and scales explicit in API. Groups hold references to measurements; scale is per-measurement.

### 2.5 Performance

- **Target:** Core calculations at native speed; no intentional slowdown in core.
- **Benchmarks:** Suite covering representative workloads (e.g. large polygon set, many groups). Run in CI; no significant regressions without explicit approval.
- **Optimization:** Prefer clear, correct code first; optimize hot paths when benchmarks justify it. Avoid blocking operations in core; core is CPU-bound computation only.

### 2.6 Security and Validation

- **Input validation:** Reject invalid inputs at API boundary and in core: empty geometry, zero or negative scale, unknown unit, invalid measurement references. Return typed errors, do not panic for bad input where a clear error is possible.
- **Secrets:** Not applicable for v1 (no auth or secrets in scope). If added later, never commit secrets; use environment or secure config.
- **No trust of caller:** Validate all inputs; do not assume bindings have already validated (defense in depth for core).

---

## 3. Development Standards

### 3.1 Testing

- **Unit tests:** All public API functions and critical paths in `takeoff_core`. Cover formulae (Shoelace, segment lengths, rectangle, count), scale/unit conversion, and group aggregates. Use `cargo test`; support `#[cfg(test)]` and test modules.
- **Integration / bindings:** Smoke tests for NAPI and WASI (same inputs produce same outputs). Tests in `packages/bindings` (e.g. `__test__/`, `.spec.ts`) for public API behavior.
- **Accuracy tests:** Golden or reference datasets for known pixel + scale → length/area. Compare outputs to baseline within stated tolerance (e.g. 0.01% for area). Version baseline dataset in repo or CI artifact.
- **Coverage:** Aim for high coverage on core; all error paths and edge cases (empty geometry, zero scale, unknown unit, degenerate polygons) must be covered.
- **Mocking:** Prefer real types in core tests; use mocks/fakes only where external I/O would otherwise be required (minimal in current scope).

### 3.2 Documentation

- **Rustdoc:** Document all public types, functions, and error variants in `takeoff_core`. Include formulae and units where relevant (e.g. polygon area, scale ratio).
- **README:** Root README explains project, build, test, and how to run benchmarks. Bindings package README explains npm usage and Node/WASI entry points.
- **API surface:** Document scale (create, list), measurements (create by kind, attach scale, get length/area/volume in unit), groups (create, add measurements, get aggregates), and all error types. Keep supported units list explicit in docs.

### 3.3 Error Handling and Logging

- **Errors:** Use `Result` in core; no silent failures. Return typed errors for invalid scale, unknown unit, empty geometry, invalid measurement reference. Errors must be serializable for NAPI/WASI.
- **Logging:** If logging is added (e.g. in bindings or sample-app), use structured levels (e.g. debug for verbose, error for failures). Core remains free of logging unless explicitly scoped; prefer errors over log-and-continue for invalid input.
- **Panics:** Reserve for programming bugs (e.g. unreachable code), not for invalid user input. Invalid input → `Result::Err` with a typed variant.

### 3.4 Accessibility and Responsive Design

- **Not applicable** for v1. Backend is library/API only; no UI. If a future consumer UI is built, accessibility and responsive rules can be added then.

---

## 4. Implementation Priorities

- **Core (Must-have for v1):** F1 (pixel geometry → length/area/volume), F2 (multiple scales), F3 (imperial and metric units), F4 (groups and aggregates), F5 (stable API from Node and WASI), F10 (typed error handling). These must be complete and correct; acceptance criteria from features.md must pass. Same inputs must produce same outputs from NAPI and WASI.
- **Important (Should-have):** F6 (scale as ratio and unit; clear scale abstraction), F7 (typed measurement kinds with correct formulae), F11 (performance benchmark suite), F12 (accuracy validation / golden tests). Implement in line with PRD; can be phased but should be present for a solid v1.
- **Nice-to-have (Could-have):** F8 (additional units or unit systems). Defer until demanded; extend `Unit` and conversion without breaking existing API.
- **Out of scope this release (Won't have):** F9 (contour/3D/volumetric). Do not implement in v1; no design or code that assumes contour/volume beyond what is already present for future alignment.
- **Quality thresholds:** Accuracy within 0.01% of baseline for area where specified; benchmark suite in CI with no significant regressions; all core features covered by tests and passing acceptance criteria before merge.

---

## 5. General Guidelines

### Requirements Adherence

- Follow PRD.md and features.md precisely. Do not add features or change behavior outside documented scope (e.g. no UI, no export formats in v1 unless scoped).
- When the PRD and features conflict, or a requirement is unclear, flag for clarification rather than guessing.
- Cost estimation is applied on top of quantities (e.g. in app layer); backend focuses on quantities unless otherwise scoped.

### Code Quality

- Prefer readable, maintainable code. Use expressive names; avoid magic numbers (use scale and unit abstractions).
- Avoid duplication; extract shared logic (e.g. conversion, formulae) into clear functions or modules.
- Stay consistent with existing Rust style (rustfmt, clippy) and project layout.

### Completeness

- No TODOs, placeholders, or stub implementations for core or must-have features without explicit approval. Deliver working, tested code per feature.
- Edge cases called out in features (empty geometry, zero scale, unknown unit, degenerate polygons) must be implemented and tested, not left as future work.

### Handling Uncertainty

- If a requirement is ambiguous: ask, document assumptions in code or comments, or flag for review. Do not guess at business-critical behavior (e.g. formulae, unit conversion, scale attachment).
- Document non-obvious decisions (e.g. tolerance for floating-point comparison, rounding policy for aggregates) in code or docs.

### Communication

- Prefer asking for clarification over implementing incorrect behavior. When in doubt on scope (e.g. new units, new measurement kinds), confirm before implementing.
- Document assumptions and open points (e.g. baseline dataset format) where they affect implementation; align with PRD open questions where applicable.
