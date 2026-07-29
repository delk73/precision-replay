---
name: build-traceability
description: Builds an FAA AR-08-32 layered bi-directional traceability matrix linking story statements, HLRs, LLRs, and vocabulary terms.
---

# Skill Specification: build-traceability

## Input Contract
- Parent ConOps story document (e.g., `docs/design/replay_system_story.md`).
- Target HLR document (e.g., `docs/normative/HLR_replay.md`).
- Target LLR document (e.g., `docs/normative/LLR_replay.md`).
- Target Vocabulary document (e.g., `docs/normative/vocabulary.md`).
- Current traceability matrix document (e.g., `docs/normative/traceability_matrix.md`).

## Execution Protocol
1. Map each High-Level Requirement upward to its parent paragraph/section in the narrative story document.
2. Map each Low-Level Requirement upward to its parent High-Level Requirement obligation and canonical vocabulary terms.
3. Reserve verification mapping slots for formal proof harnesses, test suites, and validation targets.
4. Perform coverage analysis to identify orphaned requirements, unmapped low-level obligations, or un-covered story invariants.
5. Format the output as an FAA AR-08-32 compliant layered bi-directional traceability matrix block.

## Output Artifact Contract
MUST return proposed layered traceability matrix and gap analysis strictly in chat memory for review prior to disk write to the target traceability matrix path.