# PRD → RFC Creation Checklist

Use this checklist when generating RFCs from a PRD and features list.

## Before Generating

- [ ] PRD and features list (e.g. features.md) are read and understood
- [ ] Project rules (RULES.md, .cursor/rules) located and reviewed if present
- [ ] Missing or ambiguous requirements clarified with the user (or explicitly noted as assumptions)

## Implementation Order

- [ ] Dependency graph of features/components described (text or table)
- [ ] Foundation and critical-path items identified
- [ ] RFCs numbered in strict implementation order (001, 002, …)
- [ ] Each RFC depends only on earlier-numbered RFCs
- [ ] Sequence is buildable: each RFC is implementable after all previous RFCs are done

## Per-RFC Content

- [ ] Unique ID and title
- [ ] Summary and list of features/requirements
- [ ] “Depends on” and “Enables” explicitly listed
- [ ] Complexity (Low/Medium/High)
- [ ] Acceptance criteria (testable)
- [ ] Technical approach, APIs, data/schema, state, errors, testing, performance, security as needed
- [ ] Implementation considerations and constraints

## Implementation Prompts

- [ ] One implementation-prompt-RFC-[ID].md per RFC
- [ ] Template used: project’s RFCs/implementation-prompt-template.md or skill’s template
- [ ] Only [ID], [Title], [brief description] replaced; no other template changes
- [ ] All placeholders replaced; structure unchanged

## RFCS.md

- [ ] All RFCs listed in numerical order
- [ ] Dependency table or graph included
- [ ] Sequential implementation roadmap clear
- [ ] Note that implementation is strictly one-by-one in order

## Final

- [ ] All RFC files in RFCs/ folder
- [ ] No PRD/features scope dropped without justification
- [ ] RFCs are specific enough to implement without extra clarification and flexible enough for reasonable engineering decisions
