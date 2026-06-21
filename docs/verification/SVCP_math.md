# Software Verification Plan & Procedures - Math Core (SVCP-MATH)

## 1. Evaluation Methodology (SVCP-MATH-MET)

### SVCP-MATH-MET-001: Formal Symbolic Verification
Mathematical correctness and safety invariants for the stated `I64F64` verification properties shall be verified via formal symbolic execution using the Kani Rust Verifier. Kani symbolic verification is the formal proof mechanism for those properties. Implementation-local and sampled tests provide regression support, but they do not replace proof obligations or expand the active proof scope.

### SVCP-MATH-MET-002: Test Independence Override
As independent human peer review is explicitly deferred under project criteria, verification soundness is achieved through mathematical proof generation. Proof definitions must strictly test for the absence of undefined behavior, runtime panics, and un-trapped arithmetic clipping.

## 2. Verification Primitives and Proof Bounds (SVCP-MATH-PRO)

### SVCP-MATH-PRO-001: Addition & Subtraction Soundness Proof
Status: Current active exactness coverage; overflow trap observation deferred.

The verification harnesses `verification::proofs::verify_i64f64_addition_exact_when_in_range` and `verification::proofs::verify_i64f64_subtraction_exact_when_in_range` in `verification/src/lib.rs` prove that for any two symbolic `i128` values mapped to `I64F64` structures ($A$ and $B$), non-overflowing addition and subtraction return the exact bitwise arithmetic result.

Overflow trap behavior remains implemented by the core operators through `i128::checked_add` / `i128::checked_sub` returning `None`, but end-to-end panic/trap verification is deferred.
*Traces to: LLR-REPLAY-MATH-OPS-001*

### SVCP-MATH-PRO-002: Multiplication Proof Slices
Status: Partial active coverage for raw multiplication; full coverage pending.

`SVCP-MATH-PRO-002a` is active. The verification harness `verification::proofs::verify_i64f64_multiplication_tiny_fractional_products_truncate_to_zero` in `verification/src/lib.rs` proves that bounded symbolic `i32` raw operands whose absolute magnitudes multiply below 2^64 return zero under raw `I64F64` multiplication. This covers positive, negative, and mixed-sign tiny fractional products and confirms truncation toward zero for this slice. This slice is paired with implementation-local regression tests for tiny raw products and fixed-point +/-1.0 multiplication.

`SVCP-MATH-PRO-002b` is active. The verification harness `verification::proofs::verify_i64f64_multiplication_bounded_truncates_toward_zero` in `verification/src/lib.rs` proves bounded raw multiplication equivalence for symbolic operands whose magnitudes are either bounded symbolic `u32` fractional raw values or the exact `I64F64::SCALE` (+/-1.0) raw endpoint. For that bounded domain, raw `I64F64` multiplication equals sign isolation, absolute magnitude multiplication, low-64-bit truncation, and sign reapplication.

`SVCP-MATH-PRO-002c` remains pending. It shall verify full multiplication matrix, limb, and cross-term correspondence, including overflow-gate correspondence and panic/trap observation. Convergent multiplication behavior remains outside this raw multiplication proof slice.
*Traces to: LLR-REPLAY-MATH-OPS-002*

### SVCP-MATH-PRO-003: Division Invariant Proof
Status: Planned/deferred verification obligation.

The verification harness shall perform symbolic execution on the division kernel to prove:
1. Zero Denominator Trap: If the symbolic denominator $B == 0$, the operation aborts under all conditions.
2. Shifting Overflow Protection: If left-shifting the numerator $A$ by 64 bits overflows the signed 128-bit space, the precondition must trap the failure and abort execution before the division step occurs.
*Traces to: LLR-REPLAY-MATH-OPS-003*

### SVCP-MATH-PRO-004: Convergent Integer Rounding Proof
Status: Current active verification crate coverage.

The verification harness `verification::proofs::verify_accumulator_convergent_rounding_exhaustive` in `verification/src/lib.rs` proves that accumulator-to-integer conversion rounds to the nearest integral value and resolves exact half-scale ties toward an even integral result.
*Traces to: LLR-REPLAY-MATH-OPS-004*
