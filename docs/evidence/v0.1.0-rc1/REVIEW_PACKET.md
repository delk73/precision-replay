# Review Packet for v0.1.0-rc1 Evidence

## Package Entry Point

- Package ID: `precision-replay-v0.1.0-rc1`
- Release version: `v0.1.0`
- Release candidate: `v0.1.0-rc1`
- Repository: `delk73/precision-replay`
- Evidence path: `docs/evidence/v0.1.0-rc1/`
- Manifest: `manifest.toml`

## Retained Files

The retained evidence files are listed in `manifest.toml` and currently include:

- `evidence_package.md`
- `hardware_replay_artifact.md`
- `hardware_replay_transcript.txt`
- `stm32_flash_capture_procedure.md`

Context files for this review step are:

- `REPRODUCING.md`
- `HARDWARE_SETUP.md`
- `REVIEW_PACKET.md`

## Supported Claim

The package supports only this bounded claim:

- one retained STM32F446 hardware-backed replay observation is recorded for this release candidate

Reviewers should not infer broader replay, certification, hardware, timing, or proof coverage from this package.

## Excluded Claims

This package explicitly excludes claims of:

- certification compliance
- tool qualification
- hardware qualification
- timing behavior
- board-family validation
- full arithmetic proof closure

## Deferred Proof Debt Reference

Deferred proof and verification surfaces remain governed by the existing repository status documents, including:

- `docs/normative/software_criteria.md`
- `docs/verification/SVCP_math.md`
- `docs/normative/traceability_matrix.md`
- `docs/evidence/v0.1.0-rc1/evidence_package.md`

This packet does not restate, close, or expand those proof obligations.

## Expected Review Posture

A reviewer should:

1. Start with `manifest.toml` for package identity and retained membership.
2. Read `evidence_package.md` for the package-level evidence index.
3. Read `hardware_replay_artifact.md` and compare it with `hardware_replay_transcript.txt`.
4. Use `REPRODUCING.md` to distinguish local inspection from hardware recapture.
5. Use `HARDWARE_SETUP.md` and `stm32_flash_capture_procedure.md` to understand the retained hardware observation boundary.
6. Treat excluded claims as non-claims, not as implied future acceptance criteria.

The expected posture is evidence inspection of a narrow retained observation, not certification approval, hardware qualification, timing validation, board-family validation, or proof-closure approval.
