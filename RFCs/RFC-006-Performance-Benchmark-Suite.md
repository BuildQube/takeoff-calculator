# RFC-006: Performance Benchmark Suite

## Identifier and title

- **Identifier**: RFC-006
- **Title**: Performance Benchmark Suite

## Summary

Add a performance benchmark suite for the takeoff core (and optionally bindings) so that calculations run at native speed and regressions are guarded. Benchmarks cover representative workloads (e.g. large polygon set, many groups) and run in CI. No significant regressions without explicit approval (e.g. tracked in CI or documented process).

## Features / requirements addressed

- **F11**: Performance benchmark suite (Should have)

## Depends on

- RFC-001 through RFC-004 (core); optionally RFC-005 (bindings) for bindings benchmarks.

## Enables

- None (quality guard).

## Complexity

Medium.

---

## Acceptance criteria

- [ ] Benchmark suite runs in CI.
- [ ] Benchmarks cover representative workloads: e.g. large polygon set, many groups.
- [ ] No significant regressions without explicit approval (e.g. CI fails or flags regression; process documented).
- [ ] Document how to run benchmarks locally and how to interpret results.
- [ ] Backend-only performance (no UI); core and optionally bindings.

---

## Technical approach

- **Harness**: Use Rust benchmark harness (e.g. `criterion`) in `takeoff_core` (and optionally in `packages/bindings` for JS-callable benchmarks).
- **Workloads**: (1) Large polygon set: e.g. hundreds or thousands of polygons; create scales and measurements, compute area/length in a loop. (2) Many groups: e.g. many groups each with multiple measurements; request aggregates for all groups. (3) Optional: bindings round-trip (create scale/measurement/group from JS, get results) to measure FFI overhead.
- **CI**: Run benchmarks in CI; either compare to baseline (e.g. previous run or checked-in baseline) or fail if execution time exceeds a threshold. Document policy: e.g. “no more than X% regression” or “benchmark run required for PR merge.”
- **Environment**: Document baseline environment (OS, CPU) if results are sensitive; optional artifact storage for historical comparison.

## API contracts / interfaces

- Benchmarks are not part of the public API; they are dev/CI tools. Entry points: `cargo bench` in core; optionally npm script in bindings for JS-driven benchmarks.

## File structure

- `crates/takeoff_core/benches/` (or similar): benchmark binaries or integration benchmarks.
- `packages/bindings/benchmark/`: if bindings benchmarks exist, keep existing structure or add benchmark script.
- CI workflow: run `cargo bench` (and optionally bindings benchmark); compare or record results.

## Testing strategy

- Benchmarks themselves are the test of performance; no separate “correctness” here (covered in RFC-003, RFC-007). Optionally sanity-check that benchmark code produces correct results (e.g. assert on one known value).

## Performance

- Benchmarks should complete in a reasonable time for CI (e.g. under a few minutes). Use smaller datasets if needed for CI and document “full” dataset for local runs.

## Implementation considerations

- Avoid benchmark-only code paths that differ from production; use same APIs as production.
- Rules from `.cursor/rules`: performance optimization notes; minimize async overhead if benchmarks touch async code (likely not in core).

## Constraints

- CI must be able to run the suite; no flaky or environment-dependent failures without documentation.
