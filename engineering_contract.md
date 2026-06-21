# precision-replay Engineering Contract (Single-Developer, DAL A Aligned)

This contract defines how precision-replay is developed under a strict single-developer workflow aligned with DO-178C DAL A objectives. It establishes DAL-overlap rigor in day-to-day engineering so that full compliance, independent review, and certification package layering can be added later with minimal process rework.

This document is an engineering execution and evidence-readiness contract. It is intentionally structured to maximize downstream certifiability, but it is not by itself a claim of complete DO-178C DAL A compliance.

Terminology used in this contract:

* **DAL-overlap rigor:** Engineering controls and verification discipline selected to match DAL A intent where feasible in a single-developer workflow.
* **Compliance readiness:** Evidence and process structure prepared so formal compliance activities can be layered with minimal rework.
* **Compliance claim:** We are not making a DO-178C DAL A compliance claim in this contract. A future claim requires objective-by-objective closure, approved lifecycle plans, and completed independent activities with review evidence.

## 1. Deterministic Environment and Build Baseline

* **Toolchain Lock:** All builds and verification runs must use the exact toolchain specified by root `rust-toolchain.toml`.
* **Build Environment Baseline:** Active development uses the pinned Rust toolchain, locked dependencies where supported, the base CI runner, and explicit target selection for target-bound checks. A locked Nix, OCI, or equivalent content-addressed environment is a future release/certification control. It is not required for ordinary local merges until the repo provides the environment, CI path, and exception process.
* **Explicit Target Discipline:** Build and test commands that produce target artifacts must either:
    * explicitly specify `--target <approved-target-triple>`, or
    * invoke a repository-owned wrapper script that fixes the target deterministically.
  Host fallback caused by omitted target selection is prohibited for target-bound validation.
* **Deterministic Profiles:** Current `Cargo.toml` profile settings are the active best-effort build baseline for ordinary development and PR validation. They are not release-authority or certification-baseline evidence. Release, audit, and certification-readiness profiles must be explicitly named before they are used as evidence-bearing artifacts, and must document panic behavior, LTO policy, codegen units, debug-symbol policy, strip policy, and overflow-check behavior.
* **Target Evidence-Bearing Profile Considerations:**
    * `panic = "abort"` for bounded failure behavior.
    * `codegen-units = 1` for deterministic code generation.
    * LTO policy selected by documented traceability impact assessment.
    * Debug-symbol and strip policy selected according to artifact role:
        * stripped release artifact for deployment/release packaging,
        * unstripped audit artifact for WCET, coverage, and object-code review.
    * `overflow-checks = true` where applicable unless a formal verification activity intentionally analyzes the unmitigated boundary.
* **WCET Audit Artifact Note:** Worst-Case Execution Time (WCET) audits must compile a parallel, unstripped artifact under an approved audit profile so reviewers retain symbol-level visibility for timing analysis.
* **Coverage and Traceability Note:** Changes to LTO, debug-symbol policy, stripping, or codegen-units in release/certification-readiness profiles require an explicit impact assessment and updated object-code traceability strategy.
* **Optional Performance Profile:** Performance-oriented profile settings, including heavier LTO, are allowed only when separated from cert-baseline evidence or covered by a documented impact assessment.
* **Repository-Supported Validation Surface:** Active validation consists of the base CI command set, local equivalents where applicable, explicit target-bound compile checks, and Kani for math, verification, or proof-surface changes.
* **Artifact Capture:** Baseline SHA and required validation command results are recorded in the structured commit message or PR record. CI logs are authoritative only for the commands CI actually runs. Repository-defined evidence paths are required only for release or certification-readiness evidence packages once that process exists.

---

## 2. Core Language and Safety Constraints

* **No Std Runtime:** Core logic crates must remain `#![no_std]`.
* **Memory Bounds:** Dynamic allocation through `alloc`, heap-backed collections, or custom runtime allocators is prohibited in safety-critical execution paths. Memory consumption must be statically bounded or otherwise bounded by a documented requirement and verification artifact.
* **Unsafe Prohibition:** `#![forbid(unsafe_code)]` is mandatory for core and verification crates unless a formally approved exception process is added.
* **Arithmetic Discipline:** `#![deny(clippy::arithmetic_side_effects)]` in core is mandatory.
* **Panic Elimination:** Compile-time and lint-level controls must prohibit hidden panic paths in safety-critical code. At minimum, unwrap-style failure, unchecked indexing/slicing, placeholder panics, and equivalent latent panic mechanisms must be denied unless locally justified under an approved exception.
* **No Blanket Suppressions:** File-wide `#![allow(clippy::arithmetic_side_effects)]` is prohibited.
* **Localized Exceptions Only:** Any arithmetic or panic-related lint exception must be function-scoped or expression-scoped and include a requirement reference plus rationale in adjacent comments.
* **Operator Control:** Safety-critical arithmetic must use explicitly bounded semantics such as checked, saturating, or otherwise requirement-defined operations. Reliance on default operator behavior is prohibited when overflow, underflow, divide-by-zero, or precision loss is part of the hazard analysis.

---

## 3. Requirements Traceability and Code Annotation

* **Source-of-Truth Rule:** High-level and low-level requirements are maintained in-repo and linked directly to code via doc comments.
* **Requirement Schema:** Low-level requirements must use the standardized naming form `LLR-REPLAY-[MODULE]-[UNIQUE_ID]`.
* **Traceability Key Requirement:** Every math primitive exposed across a public crate-level API boundary must include:
    * `Low-Level Requirement` identifier matching the standardized schema.
    * `Verification Vector` identifier naming the proof/test harness.
* **Traceability Review:** The active traceability control is presence of traceability keys and manual review where applicable for changes touching requirements, verification, or math behavior.
* **Traceability Extraction Target:** Automated extraction into a generated centralized JSON/CSV artifact is a target certification-readiness control, not an active local merge precondition. It becomes mandatory only after the repository owns the generator, retention path, and validation check. Once activated, a missing or malformed generated artifact is a verification failure.
* **No Untargeted Logic:** Code without requirement mapping and verification mapping is treated as non-compliant until either mapped or removed.

Example:

```rust
/// # Low-Level Requirement: LLR-REPLAY-MATH-ROUND-001
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
2. **Baseline Gate:** Record the baseline SHA and required validation command results in the structured commit message or PR record. Once CI/evidence automation exists, retained CI logs or repository-defined evidence artifacts satisfy this gate.
3. **Edit Gate:** Apply minimal change set only for approved objective.
4. **Review Gate (Verifier Mode):** Manually inspect all changed lines for requirement mapping, lint scope, determinism impact, and conformance to the `LLR-REPLAY-*` schema.
5. **Verification Gate:** Re-run required commands and record results in the structured commit message or PR record. For changes touching traceability-bearing surfaces, manually verify traceability keys are present and correct. Once the generated traceability target control is activated, regenerate and retain the centralized traceability artifact.
6. **Acceptance Gate (Verifier Mode):** Record an explicit pass/fail decision with rationale and structural review outcome.
7. **Commit Gate:** Commit only after human acceptance note is written and the change record is complete.

For single-developer DAL-overlap rigor, the same person must still perform role-separated checks in sequence (author mode, then verifier mode), with an explicit cognitive context switch and checklist completion at each gate.
Automated hooks may enforce gate formatting and other repository-owned checks, but they do not replace the human verifier-mode review obligation.

---

## 6. Evidence, Provenance, and Merge Criteria

* **Immutable Provenance:** Each accepted change must reference the exact baseline commit SHA from `git rev-parse HEAD` before the change is made.

* **Mandatory Validation Record:** Each accepted change must record the relevant validation command results in the structured commit message or PR record. At minimum, this includes applicable Cargo check/test/clippy results. Changes touching mathematical behavior, verification harnesses, or proof-surface documentation must also record a passing `cargo kani -p verification` result, or a justified non-applicability statement when Kani is not relevant.

* **Kani Authority Boundary:** The active Kani control applies to changes touching math behavior, verification harnesses, or proof-surface documentation. Other changes must record justified non-applicability when Kani is not relevant. Broader use of Kani as a release/proof authority gate is a target certification-readiness control, not an active local merge precondition. It becomes mandatory only after proof scope, runtime budget, CI/local execution path, and evidence retention are explicitly owned by the repository.

* **Commit Message Schema:** Each accepted change must use a structured commit message that captures, at minimum:
    * objective,
    * scope,
    * baseline SHA,
    * cargo test result,
    * Kani verification result or justified non-applicability,
    * verifier review note, and
    * acceptance disposition.
  Repository-local automation such as a `commit-msg` hook should reject commits that violate the schema.

* **Active Local Merge Preconditions:**
    * Baseline SHA recorded in the structured commit message.
    * Required validation command results recorded in the structured commit message or PR record.
    * `cargo kani -p verification` result recorded for mathematical behavior, verification harness, or proof-surface documentation changes, or explicitly marked non-applicable with reason.
    * Traceability keys present and correct for changes touching requirements, verification, or math behavior.
    * No prohibited blanket lint suppressions.
    * Human verifier-mode review completed and recorded.
    * Human acceptance gate completed.

* **CI Evidence Boundary:**
    * Base CI proves only the commands it actually runs:
        * `cargo fmt --all -- --check`
        * `cargo check --workspace --locked`
        * `cargo test --workspace --locked`
        * `cargo clippy --workspace --locked -- -D warnings`
        * compile-only embedded target checks currently present in `.github/workflows/ci.yml`.
    * Base CI does not prove hardware execution, replay artifact capture, release evidence authority, generated traceability, artifact bundle validity, or Kani proof authority unless Kani is explicitly run and recorded.
    * Validation evidence is recorded in the structured commit message or PR record.
    * CI command logs are authoritative retained static artifacts only for validation output from commands CI actually runs.
    * Local evidence bundles are not required for ordinary PRs unless a release or certification-readiness process explicitly requires them.
    * CI may be expanded later to enforce additional command surfaces, including Kani where applicable and commit-message/schema checks where practical, after the repository owns those automation paths.

* **Deferred Evidence Surfaces:**
    * Generated traceability extraction is deferred until the repository owns a generator, stable retention path, and validation check.
    * Release evidence packages are deferred until the repository owns a release evidence format, capture process, and bundle validator.
    * Hardware-backed replay validation artifacts are deferred until the repository owns the stable hardware capture path and evidence retention process.
    * Kani release/proof authority is deferred until the repository owns proof scope, runtime budget, CI/local execution path, and evidence retention.
    * These deferred surfaces remain intended evidence-readiness controls, but they are not active local merge preconditions in this extraction MVP snapshot.

* **Release Evidence Boundary:** The active release-evidence control is structured commit or PR evidence plus recorded validation results. A retained release evidence package is a target certification-readiness control, not an active local merge precondition. It becomes mandatory only after the repository owns the release evidence format, capture process, and bundle validator.

* **Readiness Boundary:** Satisfying this contract produces disciplined implementation history and prepares the repository for compliance-oriented evidence automation. It does not, by itself, produce DO-178C DAL A compliance or close all DO-178C DAL A objectives.

---

## 7. Operational Scope for Embedded Replay Appliance

* **Arithmetic Substrate:** Deterministic fixed-point math (`I64F64`) is the only accepted arithmetic substrate for safety-critical replay computations.
* **Precision Boundary Management:** Multiplication, division, conversion, and rounding operations must use explicit requirement-defined semantics for overflow handling, saturation, and rounding behavior. Intermediate precision loss or overflow assumptions must not be left implicit.
* **Hardware Interface Isolation:** Hardware-facing PRU logic, embedded runner components, and register- or timing-dependent behavior must be isolated behind localized hardware abstraction boundaries. The core replay math engine must remain decoupled from hardware side effects and device-specific register models.
* **Fault Containment:** Hardware-facing paths must preserve deterministic behavior and bounded failure modes under malformed input and timing stress.
* **Platform Validation Bounds:** The active hardware validation control is compile-only embedded target checking unless hardware execution is explicitly performed and recorded. Hardware-backed replay validation with retained artifacts is a target certification-readiness control, not an active local merge precondition. Once activated, hardware-backed replay validation must demonstrate bit-identical behavior for defined deterministic vectors and retain the associated artifacts.