# Skill Specification: align-lexicon

## Input Contract
- Unmapped terms list or `DECOMPILED_CHUNK` block.
- Current `docs/normative/vocabulary.md` file contents.

## Execution Protocol
1. Perform regex/semantic audit of input terms against `docs/normative/vocabulary.md`.
2. Group terms into:
   - Existing Match (canonical term exists).
   - Deprecated Metaphor / Synonym (map to canonical term).
   - New Candidate Term (requires formal definition).
3. Draft definitions for new candidate terms assigned to explicit domain sections (e.g., Section 5, Section 6).
4. Embed state labels directly into parent disposition definitions (e.g., `accepted`, `rejected`, `incomplete` inside `Execution Disposition`).
5. Format proposed entries using `### Term Name` headers with `Prohibited Metaphors:` where applicable.

## Output Artifact Contract
MUST return a structured classification summary and exact Markdown patch block intended for `docs/normative/vocabulary.md`.
