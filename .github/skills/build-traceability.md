# Skill Specification: build-traceability

## Input Contract
- Target HLR document (`docs/normative/HLR_replay.md`).
- Parent ConOps story document (`docs/design/replay_system_story.md`).
- Current `docs/normative/traceability_matrix.md` content.

## Execution Protocol
1. Map each `HLR-REPLAY-xxx` requirement upward to its parent paragraph/section in `replay_system_story.md`.
2. Reserve downward mapping slots for Low-Level Requirements (`LLR_replay.md`) and formal verification proofs (`Kani::proof_*`).
3. Audit for orphaned requirements or un-covered ConOps invariants.
4. Format output as an FAA AR-08-32 bi-directional traceability matrix block.

## Output Artifact Contract
MUST return proposed traceability matrix content for `docs/normative/traceability_matrix.md`.