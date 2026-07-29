# Skill Specification: scaffold-hlr

## Input Contract
- Approved State-Transition Matrix from `decompile-story`.
- Authoritative `docs/normative/vocabulary.md`.

## Execution Protocol
1. Transform state transitions into normative "WHAT" obligations.
2. Enforce modal verb rules: `shall` for mandatory obligations, `should` for recommendations, `may` for permissions.
3. Validate that every technical term matches an entry in `docs/normative/vocabulary.md`.
4. Ensure target Flesch-Kincaid Grade Level $\le 12$.
5. Group requirements into logical functional sections matching the system story lifecycle.

## Output Artifact Contract
MUST return proposed candidate High-Level Requirements formatted for inclusion in `docs/normative/HLR_*.md`.
