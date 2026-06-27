# v0.1.0-rc1 Evidence Package Definition

## Purpose

The `precision-replay-v0.1.0-rc1` evidence package is the retained inspection surface for `v0.1.0-rc1` readiness. It records the release candidate identity, repository baseline, validation outputs, bounded proof status, requirement references, and one hardware-backed replay artifact needed for review.

This package is not a certification package. It does not claim DO-178C compliance, DAL A compliance, tool qualification, hardware qualification, full release authority, or full arithmetic closure.

## Required Package Contents

Each `v0.1.0-rc1` evidence package shall retain or reference:

- [ ] release or candidate identifier
- [ ] baseline commit SHA
- [ ] repository identity
- [ ] Rust toolchain and target context used for evidence-producing commands
- [ ] validation command results
- [ ] Kani proof result references for proof-backed math claims
- [ ] requirement references, including `HLR-MVP-*` / `LLR-REPLAY-MVP-*` identifiers and applicable math requirements
- [ ] SVCP and traceability references
- [ ] retained hardware-backed replay artifact path
- [ ] deferred surfaces and non-applicable evidence surfaces
- [ ] provenance timestamp or capture date

Retained paths shall be explicit. The package shall not depend on unstated local workspace state.

## Required Validation Evidence

For `v0.1.0-rc1` readiness, retain the command result, date or timestamp, toolchain context, target context where applicable, and pass/fail outcome for the repository validation surfaces already defined by the engineering contract:

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace --locked`
- [ ] `cargo test --workspace --locked`
- [ ] `cargo clippy --workspace --locked -- -D warnings`
- [ ] compile-only embedded target checks currently present in repository CI or repository-owned validation wrappers

These records capture existing validation surfaces only. This definition does not add new CI requirements, default Kani CI execution, release bundle automation, generated traceability, or hardware execution tooling.

## Required Proof-Status Evidence

The package shall reference the current proof authority and proof-status documents rather than restating proof coverage as broader than documented:

- [ ] `docs/verification/SVCP_math.md`
- [ ] `docs/normative/traceability_matrix.md`
- [ ] retained `cargo kani -p verification` result for proof-backed math claims
- [ ] explicit statement if the candidate is not claiming proof-backed math readiness

Proof status is bounded and partial where those documents say it is bounded and partial. In particular, active multiplication and division proof slices do not imply full arithmetic closure, full overflow/trap proof coverage, or generalized release proof authority.

## Required Hardware-Backed Evidence

The `precision-replay-v0.1.0-rc1` evidence package shall retain one hardware-backed replay artifact captured under the `v0.1.0-rc1` firmware payload label. The artifact shall identify:

- [ ] replay vector
- [ ] host/reference result or digest
- [ ] target result or digest
- [ ] transcript or log path
- [ ] target identity
- [ ] toolchain and execution context
- [ ] pass/fail verdict against the host/reference expectation

The previous raw transcript captured the historical `mvp-rc1` payload label and is not reused as `v0.1.0-rc1` evidence. A new hardware recapture is required after the firmware payload label change.

This is one retained hardware-backed replay artifact for `v0.1.0-rc1` readiness. It is not a claim of generalized hardware validation coverage.

## Deferred Surfaces

The following surfaces are deferred for this `v0.1.0-rc1` evidence package definition:

- [ ] full multiplication closure
- [ ] division behavior outside the documented bounded proof domains
- [ ] full overflow/trap proof coverage
- [ ] generated traceability
- [ ] release bundle validator implementation
- [ ] Kani in default CI
- [ ] hardware qualification
- [ ] tool qualification
- [ ] certification claims

## Non-Claims

This evidence package definition makes these non-claims explicitly:

- This is not a DO-178C package.
- This is not a DAL A claim.
- This is not tool qualification evidence.
- This is not hardware qualification evidence.
- This is not proof of full arithmetic closure.
- This is not a general hardware validation claim.
