# precision-replay Engineering Contract (Single-Developer, DAL A Aligned)

This contract defines how precision-replay is developed under a strict single-developer workflow aligned with DO-178C DAL A objectives. It establishes DAL-overlap rigor in day-to-day engineering so that full compliance, independent review, and certification package layering can be added later with minimal process rework.

This document is an engineering execution and evidence-readiness contract. It is intentionally structured to maximize downstream certifiability, but it is not by itself a claim of complete DO-178C DAL A compliance.

Terminology used in this contract:

* **DAL-overlap rigor:** Engineering controls and verification discipline selected to match DAL A intent where feasible in a single-developer workflow.
* **Compliance readiness:** Evidence and process structure prepared so formal compliance activities can be layered with minimal rework.
* **Compliance claim:** We are not making a DO-178C DAL A compliance claim in this contract. A future claim requires objective-by-objective closure, approved lifecycle plans, and completed independent activities with review evidence.

## 1. Deterministic Environment and Build Baseline

* **Toolchain Lock:** All builds and verification runs must use the exact toolchain specified by root `rust-toolchain.toml`.
* **Build Environment Isolation:** Tool execution must occur inside a cryptographically pinned, content-addressed environment baseline such as a locked Nix flake or pinned OCI image. Ad hoc host-environment execution is non-compliant unless an explicit exception record is attached to the change.
* **Explicit Target Discipline:** Build and test commands that produce target artifacts must either:
    * explicitly specify `--target <approved-target-triple>`, or
    * invoke a repository-owned wrapper script that fixes the target deterministically.
  Host fallback caused by omitted target selection is prohibited for target-bound validation.
* **Deterministic Profiles:** To guarantee reproducible builds and straightforward structural coverage analysis, use these profile settings:
    * `panic = "abort"`: Halts the system immediately on a critical error instead of attempting an unpredictable unwinding sequence.
    * `lto = false` and `codegen-units = 1`: Disables parallel compilation and global optimization. This ensures identical builds every time and keeps the final assembly cleanly mapped to the original Rust source code.
    * `debug = 0` and `strip = "symbols"` (primary release profile): Produces the cert-baseline release artifact without embedded debug symbols.
    * `overflow-checks = true`: Forces the compiler to catch mathematical boundary violations during testing. The only exception is if a formal verification tool needs to analyze the unmitigated boundary directly.
* **WCET Audit Artifact Note:** Worst-Case Execution Time (WCET) audits must compile a parallel, unstripped artifact under an approved audit profile so reviewers retain symbol-level visibility for timing analysis.
* **Coverage and Traceability Note:** With `lto = false`, source-to-object traceability is intentionally simplified for structural coverage analysis; any future change to enable LTO requires an explicit impact assessment and updated object-code traceability strategy.
* **Optional Performance Profile:** `lto = "thin"` is allowed only under a formally documented impact assessment approved within the change evidence package.
* **Baseline Commands (Required Before and After Change):**
    * `cargo test --workspace` using the repository-approved deterministic target selection method.
    * `cargo kani -p verification`, or the repository-approved wrapper for Kani when target or model configuration must differ from final machine-code compilation.
* **Artifact Capture:** Baseline and post-change command output must be preserved as static, machine-readable artifacts under a repository-defined evidence path.

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
* **Traceability Extraction:** Verification runs must execute an automated extraction step that compiles code-level requirement markers into a centralized traceability artifact such as JSON or CSV. A missing or malformed artifact is a verification failure.
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
2. **Baseline Gate:** Run required baseline commands and capture outputs as machine-readable artifacts.
3. **Edit Gate:** Apply minimal change set only for approved objective.
4. **Review Gate (Verifier Mode):** Manually inspect all changed lines for requirement mapping, lint scope, determinism impact, and conformance to the `LLR-REPLAY-*` schema.
5. **Verification Gate:** Re-run required commands, regenerate the centralized traceability artifact, and compare results to baseline artifacts.
6. **Acceptance Gate (Verifier Mode):** Record an explicit pass/fail decision with rationale and structural review outcome.
7. **Commit Gate:** Commit only after human acceptance note is written and the change record is complete.

For single-developer DAL-overlap rigor, the same person must still perform role-separated checks in sequence (author mode, then verifier mode), with an explicit cognitive context switch and checklist completion at each gate.
Automated hooks may enforce gate formatting and artifact presence, but they do not replace the human verifier-mode review obligation.

---

## 6. Evidence, Provenance, and Merge Criteria

* **Immutable Provenance:** Each accepted change must reference exact commit SHA from `git rev-parse HEAD`.
* **Mandatory Verification Evidence:** Any change touching mathematical behavior requires a passing `cargo kani -p verification` run attached to the change record.
* **Commit Message Schema:** Each accepted change must use a structured commit message that captures, at minimum:
    * objective,
    * scope,
    * baseline SHA,
    * cargo test result,
    * Kani verification result or justified non-applicability,
    * verifier review note, and
    * acceptance disposition.
  Repository-local automation such as a `commit-msg` hook should reject commits that violate the schema.
* **Merge Preconditions:**
    * Baseline and post-change command outputs recorded as static artifacts.
    * Traceability keys present and correct.
    * Centralized traceability artifact generated successfully and retained with the change evidence.
    * No prohibited blanket lint suppressions.
    * Human verifier-mode review completed and recorded.
    * Human acceptance gate completed.
* **Readiness Boundary:** Satisfying this contract produces compliance-ready evidence and disciplined implementation history. It does not, by itself, close all DO-178C DAL A objectives.

---

## 7. Operational Scope for Embedded Replay Appliance

* **Arithmetic Substrate:** Deterministic fixed-point math (`I64F64`) is the only accepted arithmetic substrate for safety-critical replay computations.
* **Precision Boundary Management:** Multiplication, division, conversion, and rounding operations must use explicit requirement-defined semantics for overflow handling, saturation, and rounding behavior. Intermediate precision loss or overflow assumptions must not be left implicit.
* **Hardware Interface Isolation:** Hardware-facing PRU logic, embedded runner components, and register- or timing-dependent behavior must be isolated behind localized hardware abstraction boundaries. The core replay math engine must remain decoupled from hardware side effects and device-specific register models.
* **Fault Containment:** Hardware-facing paths must preserve deterministic behavior and bounded failure modes under malformed input and timing stress.
* **Platform Validation Bounds:** Platform validation on approved architectures must demonstrate bit-identical behavior for defined deterministic vectors. Where hardware execution is not available in the development loop, the surrogate execution environment and its limitations must be explicitly recorded.