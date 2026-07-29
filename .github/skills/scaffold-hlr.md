---
name: scaffold-hlr
description: Transforms state-transition matrices and vocabulary terms into normative High-Level Requirements (HLRs).
---

# Skill Specification: scaffold-hlr

## Input Contract
- Approved State-Transition Matrix from `decompile-story`.
- Target Vocabulary document (e.g., `docs/normative/vocabulary.md`).

## Execution Protocol
1. Transform state transitions and domain boundaries into normative "WHAT" obligations.
2. Enforce modal verb rules: `shall` for mandatory obligations, `should` for recommendations, `may` for permissions.
3. Validate that every technical term matches a canonical entry in the target vocabulary document.
4. Ensure target Flesch-Kincaid Grade Level $\le 12$.
5. Group requirements into logical functional sections matching the system story lifecycle.

## Output Artifact Contract
MUST return proposed candidate High-Level Requirements strictly in chat memory for human review prior to disk write to the target HLR document path (e.g., `docs/normative/HLR_replay.md`).