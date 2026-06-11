# precision-replay Engineering & Invariant Contract

## 1. Planning & Environment (rust-toolchain.toml & Cargo.toml)
Enforces environmental determinism and eliminates compilation drift across local machines and target runners.

* **Deterministic Toolchain Boundary:** Execution profiles are anchored exclusively to the toolchain channel specified in the root `rust-toolchain.toml`. Builds are rejected if generated under any other compiler variant.
* **Compilation and Optimization Invariants:** Release compilation must resolve under a single unified profile:
    * `panic = "abort"`: Eliminates non-deterministic stack unwinding and landing-pad code block injection.
    * `lto = "fat"` and `codegen-units = 1`: Enforces strict cross-module optimization bounds, optimizing execution paths for constant-time bit-masks.
    * `debug = true`: Mandated across release artifacts to retain precise symbol mapping for Worst-Case Execution Time (WCET) evaluation.

---

## 2. Low-Friction Software Code Standards (Enforced via core/src/lib.rs)
Replaces written prose guidelines with absolute compile-time enforcement gates.

* **Memory Management Invariants:** The compilation unit must explicitly declare `#![no_std]`. Dynamic heap allocations, use of the `alloc` crate, or reliance on runtime vector growth are prohibited. Memory mapping must rely entirely on static arrays or bounded stack layouts.
* **Safety and Pointer Restrictions:** The runtime environment declares `#![forbid(unsafe_code)]`. Memory safety must be proven entirely at compile time via the borrow checker, eliminating manual pointer arithmetic risks.
* **Arithmetic Overflow Enforcement:** The core crate enforces `#![deny(clippy::arithmetic_side_effects)]`. Native mathematical operators (`+`, `-`, `*`, `/`) are rejected at compile time unless wrapped in explicit, branch-free, saturating, or checked primitives (`I64F64`).

---

## 3. Specifications, Logic, and Bidirectional Traceability
Converts functional requirements into an inline documentation tracking schema.

* **Code-As-Source-Of-Truth Mapping:** High-Level and Low-Level Requirements are documented inside module-level documentation blocks (`//!`) in the source code files. Separate, unlinked design specifications are prohibited.
* **Traceability Keys:** Every operational logic block must include a doc attribute reference linking the implementation to its mathematical objective and its verification vector.
    * *Example Layout:*
        ```rust
        /// # Low-Level Requirement: LLR-REPLAY-MATH-ROUND
        /// Truncation logic must execute rounding to nearest, breaking ties to even using 
        /// branch-free bit selection masks to guarantee constant-time execution.
        ///
        /// **Verification Vector:** `verification::proofs::verify_rounding_ties_to_even`
        #[inline(always)]
        pub fn round_to_even(...) -> u128 { ... }
        ```
* **Dead Code Elimination:** Any functional block or instruction that lacks an explicit tracing key to a functional requirement is classified as untargeted code and must be stripped from the codebase immediately.

---

## 4. Verification and Mathematical Contracts (crates/verification)
Maps testing and structural coverage requirements directly to formal methods.

* **Formal Model Checking Invariants:** Structural code coverage metrics (Statement, Branch, and MC/DC equivalents) are satisfied mathematically rather than through manual test matrices. Property-based verification harnesses inside the `verification` crate must analyze the `core` mathematical primitives across all possible input bit-vectors (`kani::any()`).
* **Absence of Runtime Failures:** The formal verification harnesses must prove that the arithmetic modules are mathematically incapable of triggering runtime panics, division-by-zero, out-of-bounds array indexing, or arithmetic wrapping errors under any valid operational range.
* **Robustness Testing Boundary:** Physical integration testing on the target hardware (STM32F446/BeagleBone) must validate compliance against external signal anomalies. The verification suite must inject malformed RPL0 binary frames and maximum-frequency clock jitter to verify that peripheral registers handle exceptions without dropping lines or entering non-deterministic loop states.

---

## 5. Artifact Provenance and Configuration Integrity
Translates configuration tracking into Git workflow automation.

* **Immutable Tracking Keys:** Every functional release configuration is assigned a unique cryptographic identifier based on the exact `git rev-parse HEAD` SHA-256 baseline. 
* **Repeatability Validation:** Any change to the mathematical logic requires a successful local run of the formal methods pipeline (`cargo kani -p verification`) prior to merging the commit hash into production baselines.