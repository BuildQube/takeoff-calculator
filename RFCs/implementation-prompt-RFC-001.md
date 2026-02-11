# Implementation Prompt: RFC-001 – Typed Error Handling for Invalid Inputs

Implement **RFC-001: Typed Error Handling for Invalid Inputs** in strict accordance with the RFC document.

## Purpose

Define and implement a typed error system in takeoff_core (and expose it through bindings) so that invalid inputs—empty geometry, zero or invalid scale, unknown unit—return clear, serializable errors instead of panics or silent wrong values.

## Instructions

1. **Read the full RFC**: Open and follow `RFCs/RFC-001-*.md` (the RFC file for this ID). All technical details, acceptance criteria, APIs, data models, and constraints are defined there.

2. **Scope**: Implement only what is specified in RFC-001. Do not add functionality that belongs to later RFCs. Do not skip required items from the RFC.

3. **Dependencies**: This RFC builds on prior RFCs (listed in the RFC document). Assume those are already implemented. Use existing interfaces and patterns from the codebase.

4. **Output**:
   - Code, configuration, and any artifacts required by the RFC.
   - Tests that satisfy the RFC's acceptance criteria and testing strategy.
   - Updates to documentation or schema as specified in the RFC.

5. **Verification**: Before considering the task complete, confirm every acceptance criterion in the RFC is met and that no placeholder or stub remains unless the RFC explicitly allows it.

## Reference

- **RFC document**: `RFCs/RFC-001-*.md`
- **Master index**: `RFCs/RFCS.md`

Implement RFC-001 in full according to the RFC document; do not deviate from its scope or add features from other RFCs.
