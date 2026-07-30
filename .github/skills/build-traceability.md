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
1. Map each High-Level Requirement upward to its parent paragraph/section in `docs/normative/HLR_replay.md`.
2. Map each Low-Level Requirement upward to its parent High-Level Requirement ID (`HLR-REP-xxx`).
3. For each verification mapping cell, enforce strict symbol syntax:
   - File references must point to existing files (e.g., `core/src/...`).
   - Code symbol references must match exact, un-truncated function or type identifiers.
   - Every verification cell MUST include explicit status tagging: `Status: verified`, `Status: implemented`, or `Status: draft`.
4. Run internal validation against all requirement IDs to ensure zero un-anchored HLR/LLR references.

## Output Artifact Contract
MUST return proposed layered traceability matrix and gap analysis strictly in chat memory for review prior to disk write to the target traceability matrix path.