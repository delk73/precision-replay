---
name: scaffold-llr
description: Transforms High-Level Requirements into implementation-constraining Low-Level Requirement skeletons.
---

# Skill Specification: scaffold-llr

## Input Contract
- Target High-Level Requirements document (e.g., `docs/normative/HLR_replay.md`).
- Target Vocabulary document (e.g., `docs/normative/vocabulary.md`).

## Execution Protocol
1. Read and parse all High-Level Requirement obligations from the input HLR document.
2. Derive one or more Low-Level Requirements (LLRs) for each HLR to capture full implementation constraints.
3. For each derived LLR, specify explicit low-level obligations covering:
   - Deterministic data structure layout (field ordering, bit-width boundaries, representation assumptions).
   - Fixed-point arithmetic precision, numerical scale/bounds, and rounding semantics.
   - Deterministic state transition functions, pre-conditions, post-conditions, and active invariants.
   - Explicit error labels, error codes, and deterministic handling paths.
4. Enforce normative language (`shall`) bound strictly to canonical terms defined in the target vocabulary document.
5. Group requirements logically under parent HLR identifiers, maintaining upstream parent traceability.

## Output Artifact Contract
MUST return proposed candidate Low-Level Requirements strictly in chat memory for human review prior to disk write to the target LLR document path (e.g., `docs/normative/LLR_replay.md`).