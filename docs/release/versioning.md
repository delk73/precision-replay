# Release Candidate Versioning

Release versions use `vMAJOR.MINOR.PATCH`.

Release candidates use `vMAJOR.MINOR.PATCH-rcN`.

Evidence package IDs use `precision-replay-vMAJOR.MINOR.PATCH-rcN`.

Release and evidence paths use the candidate ID.

For this candidate:

- Release version: `v0.1.0`
- Release candidate: `v0.1.0-rc1`
- Evidence package ID: `precision-replay-v0.1.0-rc1`
- Release path: `docs/release/v0.1.0-rc1/`
- Evidence path: `docs/evidence/v0.1.0-rc1/`

Firmware payload labels use the candidate ID.

MVP is sprint/process language only and must not be used in durable release/evidence paths, payload labels, readiness documents, or package IDs.

Existing requirement identifiers such as `HLR-MVP-*` and `LLR-REPLAY-MVP-*` are not renamed automatically. Identifier renames require explicit traceability approval.
