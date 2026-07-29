# Skill Specification: derive-llr

## Input Contract
- Authoritative high-level requirements file: `docs/normative/HLR_replay.md`.
- Authoritative vocabulary file: `docs/normative/vocabulary.md`.

## Execution Protocol
1. Read and parse all HLR identifiers in `docs/normative/HLR_replay.md`.
2. Enforce a strict 1:1 derivation mapping from `HLR-RPL-001` through `HLR-RPL-036` to `LLR-RPL-001` through `LLR-RPL-036`.
3. For each derived LLR, specify implementation-constraining low-level obligations for:
   - deterministic data structures (field ordering, width/representation assumptions, serialization boundaries when relevant),
   - fixed-point numeric bounds and rounding semantics where applicable,
   - explicit state transition functions and pre/post-conditions,
   - explicit error condition labels and deterministic handling paths.
4. Use normative language (`shall`) and preserve canonical terminology from `docs/normative/vocabulary.md`.
5. Do not alter, renumber, or reinterpret existing HLR IDs; only derive corresponding low-level constraints.

## Output Artifact Contract
MUST write a complete low-level requirements baseline to `docs/design/LLR_replay.md` with IDs `LLR-RPL-001` through `LLR-RPL-036`, preserving one-to-one traceability back to `HLR-RPL-001` through `HLR-RPL-036`.