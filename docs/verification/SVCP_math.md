# Software Verification Plan & Procedures - Math Core (SVCP-MATH)

## 1. Evaluation Methodology (SVCP-MATH-MET)

### SVCP-MATH-MET-001: Formal Symbolic Verification
Mathematical correctness and safety invariants for the stated `I64F64` verification properties shall be verified via formal symbolic execution using the Kani Rust Verifier. Kani symbolic verification is the formal proof mechanism for those properties. Implementation-local and sampled tests provide regression support, but they do not replace proof obligations or expand the active proof scope.

### SVCP-MATH-MET-002: Test Independence Override
As independent human peer review is explicitly deferred under project criteria, verification soundness is achieved through mathematical proof generation. Proof definitions must strictly test for the absence of undefined behavior, runtime panics, and un-trapped arithmetic clipping.

## 2. Verification Primitives and Proof Bounds (SVCP-MATH-PRO)

### SVCP-MATH-PRO-001: Addition & Subtraction Soundness Proof
Status: Active exactness coverage and active overflow trap observation.

The verification harnesses `verification::proofs::verify_i64f64_addition_exact_when_in_range` and `verification::proofs::verify_i64f64_subtraction_exact_when_in_range` in `verification/src/lib.rs` prove that for any two symbolic `i128` values mapped to `I64F64` structures ($A$ and $B$), non-overflowing addition and subtraction return the exact bitwise arithmetic result.

The verification harnesses `verification::proofs::verify_i64f64_addition_overflow_traps` and `verification::proofs::verify_i64f64_subtraction_overflow_traps` in `verification/src/lib.rs` observe panic/trap behavior when `i128::checked_add` or `i128::checked_sub` return `None`. These trap proofs do not claim panic message matching. Implementation-local add/sub tests remain regression support unless paired with Kani proof coverage.
*Traces to: LLR-REPLAY-MATH-OPS-001*

### SVCP-MATH-PRO-002: Multiplication Proof Slices
Status: Partial active coverage for raw multiplication; full coverage pending.

`SVCP-MATH-PRO-002a` is active. The verification harness `verification::proofs::verify_i64f64_multiplication_tiny_fractional_products_truncate_to_zero` in `verification/src/lib.rs` proves that bounded symbolic `i32` raw operands whose absolute magnitudes multiply below 2^64 return zero under raw `I64F64` multiplication. This covers positive, negative, and mixed-sign tiny fractional products and confirms truncation toward zero for this slice. This slice is paired with implementation-local regression tests for tiny raw products and fixed-point +/-1.0 multiplication.

`SVCP-MATH-PRO-002b` is active. The verification harness `verification::proofs::verify_i64f64_multiplication_bounded_truncates_toward_zero` in `verification/src/lib.rs` proves bounded raw multiplication equivalence for symbolic operands whose magnitudes are either bounded symbolic `u32` fractional raw values or the exact `I64F64::SCALE` (+/-1.0) raw endpoint. For that bounded domain, raw `I64F64` multiplication equals sign isolation, absolute magnitude multiplication, low-64-bit truncation, and sign reapplication.

`SVCP-MATH-PRO-002c` is active for bounded fixed non-unit high-limb single-cross-term raw multiplication correspondence through `verification::proofs::verify_i64f64_multiplication_bounded_lh_cross_term_correspondence` and `verification::proofs::verify_i64f64_multiplication_bounded_hl_cross_term_correspondence` in `verification/src/lib.rs`. These slices prove public `I64F64 * I64F64` behavior against independent reference models for the low-limb-by-high-limb and high-limb-by-low-limb paths with symbolic signs, one symbolic `u16` low limb, and one fixed non-unit high limb equal to 2. Full multiplication closure, full limb/cross-term matrix correspondence, two-term cross-sum composition, unbounded operand coverage, overflow-gate correspondence, and expected-panic overflow/trap observation remain pending. Convergent multiplication behavior remains outside this raw multiplication proof slice.
*Traces to: LLR-REPLAY-MATH-OPS-002*

### SVCP-MATH-PRO-003: Division Invariant Proof
Status: Partial active guard coverage and bounded signed power-of-two denominator arithmetic coverage; full arbitrary-denominator division correctness pending.

`SVCP-MATH-PRO-003a` is active. The verification harness `verification::proofs::verify_i64f64_division_denominator_zero_traps` in `verification/src/lib.rs` proves that raw `I64F64` division traps for any symbolic numerator when the denominator is zero. This is a guard-behavior proof slice only; Kani 0.58.0 observes the expected panic path but does not match the panic message.

`SVCP-MATH-PRO-003b` is active. The verification harness `verification::proofs::verify_i64f64_division_numerator_shift_overflow_traps` in `verification/src/lib.rs` proves that raw `I64F64` division traps for symbolic numerators whose sign-extension bounds show that shifting left by 64 bits would overflow the signed 128-bit representation, with the denominator constrained nonzero so the shift-overflow guard is the exercised division guard.

`SVCP-MATH-PRO-003c` is active for bounded non-trapping arithmetic correspondence with symbolic `i32` raw numerators and signed power-of-two denominator family `{-8, -4, -2, -1, 1, 2, 4, 8}`. The verification harness `verification::proofs::verify_i64f64_division_i32_unit_denominators_match_shifted_reference` in `verification/src/lib.rs` proves non-trapping division arithmetic correspondence against the shifted-numerator reference quotient for unit denominators `{-1, 1}`. The verification harness `verification::proofs::verify_i64f64_division_i32_small_denominators_match_shifted_reference` in `verification/src/lib.rs` proves the same correspondence for denominator family `{-2, -1, 1, 2}`. The verification harness `verification::proofs::verify_i64f64_division_i32_power_of_two_denominators_match_shifted_reference` in `verification/src/lib.rs` proves the same correspondence for signed power-of-two denominator family `{-8, -4, -2, -1, 1, 2, 4, 8}`. Arbitrary-denominator and full unbounded symbolic division arithmetic remain pending. Implementation-local tests in `core/src/math.rs` remain regression support and do not expand that proof scope.
*Traces to: LLR-REPLAY-MATH-OPS-003*

### SVCP-MATH-PRO-004: Convergent Integer Rounding Proof
Status: Current active verification crate coverage.

The verification harness `verification::proofs::verify_accumulator_convergent_rounding_exhaustive` in `verification/src/lib.rs` proves that accumulator-to-integer conversion rounds to the nearest integral value and resolves exact half-scale ties toward an even integral result.
*Traces to: LLR-REPLAY-MATH-OPS-004*
