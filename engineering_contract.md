# precision-replay Engineering Contract (Single-Developer, DAL A Aligned)

This contract defines how precision-replay is developed under a strict single-developer workflow aligned with DO-178C DAL A objectives. It establishes DAL-overlap rigor in day-to-day engineering so that full compliance, independent review, and certification package layering can be added later with minimal process rework.

This document is an engineering execution and evidence-readiness contract. It is intentionally structured to maximize downstream certifiability, but it is not by itself a claim of complete DO-178C DAL A compliance.

Terminology used in this contract:

* **DAL-overlap rigor:** Engineering controls and verification discipline selected to match DAL A intent where feasible in a single-developer workflow.
* **Compliance readiness:** Evidence and process structure prepared so formal compliance activities can be layered with minimal rework.
* **Compliance claim:** We are not making a DO-178C DAL A compliance claim in this contract. A future claim requires objective-by-objective closure, approved lifecycle plans, and completed independent activities with review evidence.

## 1. Deterministic Environment and Build Baseline

* **Toolchain Lock:** All builds and verification runs must use the exact toolchain specified by root `rust-toolchain.toml`.
* **Deterministic Profiles:**
    * `panic = "abort"` for deterministic failure behavior.
    * `lto = "fat"` and `codegen-units = 1` for deterministic optimization behavior.
    * `debug = true` in release for repeatable symbol-level analysis and WCET auditability.
* **Baseline Commands (Required Before and After Change):**
    * `cargo test --workspace`
    * `cargo kani -p verification`

---

## 2. Core Language and Safety Constraints

* **No Std Runtime:** Core logic crates must remain `#![no_std]`.
* **Unsafe Prohibition:** `#![forbid(unsafe_code)]` is mandatory for core and verification crates unless a formally approved exception process is added.
* **Arithmetic Discipline:** `#![deny(clippy::arithmetic_side_effects)]` in core is mandatory.
* **No Blanket Suppressions:** File-wide `#![allow(clippy::arithmetic_side_effects)]` is prohibited.
* **Localized Exceptions Only:** Any arithmetic lint exception must be function-scoped or expression-scoped and include a requirement reference plus rationale in adjacent comments.

---

## 3. Requirements Traceability and Code Annotation

* **Source-of-Truth Rule:** High-level and low-level requirements are maintained in-repo and linked directly to code via doc comments.
* **Traceability Key Requirement:** Every externally used math primitive must include:
    * `Low-Level Requirement` identifier.
    * `Verification Vector` identifier naming the proof/test harness.
* **No Untargeted Logic:** Code without requirement mapping and verification mapping is treated as non-compliant until either mapped or removed.

Example:

```rust
/// # Low-Level Requirement: LLR-REPLAY-MATH-ROUND
/// Deterministic rounding to nearest with ties-to-even in constant time.
///
/// **Verification Vector:** `verification::proofs::verify_rounding_ties_to_even`
#[inline(always)]
pub fn round_to_even(...) -> ... { ... }
```

---

## 4. Verification Ownership Model

To avoid false abstraction boundaries while preserving testability:

* **Core-Local Proofs (`core`):**
    * Allowed only for implementation-local invariants that require private/internal visibility.
    * Must not force API expansion solely for proof convenience.
* **System/Contract Proofs (`verification`):**
    * Required for API-level properties, boundary behavior, and cross-crate invariants.
    * Must operate as witness proofs for externally visible behavior.
* **No Duplicate Intent:** The same proof intent should not be duplicated across crates unless one is explicitly internal and the other explicitly external, with separate requirement identifiers.

---

## 5. Human-In-The-Loop Workflow

Each change follows a locked sequence. No step skipping:

1. **Plan Gate:** Define one objective, one file scope, one validation set.
2. **Baseline Gate:** Run required baseline commands and record outputs.
3. **Edit Gate:** Apply minimal change set only for approved objective.
4. **Review Gate (Human):** Manually inspect all changed lines for requirement mapping, lint scope, and determinism impact.
5. **Verification Gate:** Re-run required commands and compare to baseline.
6. **Acceptance Gate (Human):** Explicit pass/fail decision with rationale.
7. **Commit Gate:** Commit only after human acceptance note is written.

For single-developer DAL-overlap rigor, the same person must still perform role-separated checks in sequence (author mode, then verifier mode), with explicit checklist completion at each gate.

---

## 6. Evidence, Provenance, and Merge Criteria

* **Immutable Provenance:** Each accepted change must reference exact commit SHA from `git rev-parse HEAD`.
* **Mandatory Verification Evidence:** Any change touching mathematical behavior requires a passing `cargo kani -p verification` run attached to the change record.
* **Merge Preconditions:**
    * Baseline and post-change command outputs recorded.
    * Traceability keys present and correct.
    * No prohibited blanket lint suppressions.
    * Human acceptance gate completed.
* **Readiness Boundary:** Satisfying this contract produces compliance-ready evidence and disciplined implementation history. It does not, by itself, close all DO-178C DAL A objectives.

---

## 7. Operational Scope for Embedded Replay Appliance

* Deterministic fixed-point math (`I64F64`) is the only accepted arithmetic substrate for safety-critical replay computations.
* Hardware-facing PRU and embedded runner components must preserve deterministic behavior and bounded failure modes under malformed input and timing stress.
* Platform validation (x86, ARM, RISC-V where applicable) must demonstrate bit-identical behavior for defined deterministic vectors.