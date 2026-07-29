---
name: align-lexicon
description: Audits domain terms against the normative vocabulary, classifies definitions, and drafts markdown patches.
---

# Skill Specification: align-lexicon

## Input Contract
- Unmapped terms list or decompiled story block.
- Target vocabulary document (`docs/normative/vocabulary.md`).

## Execution Protocol
1. Audit input terms against canonical entries in the target vocabulary document.
2. Classify terms into three distinct categories:
   - **Existing Match:** Canonical term already defined in vocabulary.
   - **Deprecated Metaphor / Synonym:** Term mapped to existing canonical equivalent.
   - **New Candidate Term:** Term requires formal definition.
3. Draft formal, metaphor-free definitions for new candidate terms assigned to logical domain sections.
4. Format proposed additions to match the target vocabulary document's structure, including explicit deprecations or prohibited metaphors where applicable.

## Output Artifact Contract
MUST return a structured classification summary and exact Markdown patch block strictly in chat memory for human review prior to disk write.