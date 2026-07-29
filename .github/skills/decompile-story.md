---
name: decompile-story
description: Decompiles narrative design documents into domain boundaries, state-transition matrices, unmapped terms, and non-deterministic leak analyses.
---

# Skill Specification: decompile-story

## Input Contract
- Target file path to a Concept of Operations or narrative design story (e.g., `docs/design/replay_system_story.md`).

## Execution Protocol
1. Read the input narrative document.
2. Perform logical decomposition according to NASA SEH 4.2.
3. Identify domain boundaries and strip metaphors or non-standard technical nomenclature.
4. Extract state vectors, cycle tick dependencies ($t_k$), invariants, and transitions.
5. Catalog all terms not yet formally bound to canonical definitions in `docs/normative/vocabulary.md`.
6. Perform non-deterministic leak analysis across state variables, timing boundaries, arithmetic bounds, and external I/O primitives.

## Output Artifact Contract
MUST return a single, machine-parseable `DECOMPILED_CHUNK` block strictly in chat memory for human review during Phase 1A, formatted exactly as follows:

### DECOMPILED_CHUNK
- **Source Artifact:** [Input path]
- **Target Domain Boundary:** [Primary Execution Domain | System Boundaries & External Interfaces]
- **Vocabulary Mapping (Unmapped Candidates):**
| Raw Input Term | Canonical `vocabulary.md` Term / Status |
| :--- | :--- |
| ... | ... |
- **State Transition Matrix:**
| Step / Tick | Trigger / Event | Current State ($S_k$) | Next State ($S_{k+1}$) | Active Invariant |
| :--- | :--- | :--- | :--- | :--- |
| ... | ... | ... | ... | ... |
- **Unresolved Gaps & Non-Deterministic Leaks:**
- [List of unresolved gaps, ambiguities, or non-deterministic leaks]