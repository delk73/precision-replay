# Low-Level Requirements - Release Candidate Readiness Evidence (LLR-MVP)

These low-level requirements refine the release candidate readiness surface for the retained release evidence package, one hardware-backed replay validation artifact, and the release candidate readiness statement. They define required contents and retained paths only; they do not implement artifact generation, bundle validation, hardware capture, generated traceability, certification evidence, or local merge gate changes. Existing `LLR-REPLAY-MVP-*` identifiers are stable requirement identifiers and are not renamed by release candidate identity alignment.

`engineering_contract.md` remains the workflow and evidence-boundary authority for active local merge preconditions, release evidence boundaries, hardware validation boundaries, and certification-readiness deferrals.

## 1. Release Evidence Package (LLR-REPLAY-MVP-EVD)

### LLR-REPLAY-MVP-EVD-001: Release Evidence Manifest Contents
The release candidate evidence package shall include a manifest that identifies, at minimum:
1. release or candidate identifier,
2. baseline commit SHA,
3. source repository identity,
4. toolchain and target context used for evidence-producing commands,
5. validation outputs included in the package,
6. applicable proof results included in the package,
7. requirement and verification status references,
8. retained artifact paths for package contents,
9. known deferred or non-applicable evidence surfaces, and
10. package creation timestamp or equivalent provenance marker.

The manifest shall reference retained paths rather than relying on unstated local workspace state.
*Traces to: HLR-MVP-EVD-001*

### LLR-REPLAY-MVP-EVD-002: Release Evidence Bundle Validation Expectations
The release candidate evidence bundle shall be considered valid only when the retained package can be checked for:
1. manifest presence,
2. referenced artifact path presence,
3. baseline SHA and repository identity presence,
4. validation output references for applicable commands,
5. proof-result references when proofs are applicable under `engineering_contract.md`,
6. requirement and verification status references, and
7. explicit recording of deferred or non-applicable evidence surfaces.

This requirement defines expected validation behavior for the future bundle validator. It does not add, implement, or require a bundle validator in this change.
*Traces to: HLR-MVP-EVD-001*

## 2. Hardware-Backed Replay Artifact (LLR-REPLAY-MVP-HW)

### LLR-REPLAY-MVP-HW-001: Retained Hardware Replay Artifact Contents
The release candidate hardware-backed replay validation artifact shall include, at minimum:
1. the deterministic replay vector under test,
2. the host/reference result or digest,
3. the embedded/target result or digest,
4. a capture transcript or execution log,
5. target identity,
6. toolchain and execution context,
7. pass/fail verdict against the host/reference expectation, and
8. retained artifact path.

The artifact shall show execution through the embedded target path for one defined deterministic replay vector. It does not implement hardware capture or establish generalized hardware-backed replay validation coverage.
*Traces to: HLR-MVP-HW-001*

## 3. Release Candidate Readiness Statement (LLR-REPLAY-MVP-BND)

### LLR-REPLAY-MVP-BND-001: Release Candidate Readiness Statement Structure
The release candidate readiness statement shall explicitly identify:
1. active-covered surfaces,
2. bounded surfaces and their stated limits,
3. deferred surfaces,
4. non-certified surfaces,
5. applicable retained release evidence package path,
6. applicable retained hardware-backed replay artifact path,
7. requirement and verification status references, and
8. an explicit statement that certification compliance is not claimed.

The readiness statement shall distinguish release candidate readiness evidence from formal certification evidence, release authority, generalized hardware validation authority, generated traceability authority, broader Kani proof authority, and ordinary local merge gates.
*Traces to: HLR-MVP-BND-001*
