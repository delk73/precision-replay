# Software Verification Plan & Procedures - Math Core (SVCP-MATH)

## 1. Evaluation Methodology (SVCP-MATH-MET)

### SVCP-MATH-MET-001: Bounded Formal Proof and Native Property Verification
Dual bounded formal symbolic proof and native property-based verification is required for `LLR-REPLAY-MATH-OPS-001` through `004`. Bounded Kani scalar micro-proofs, each constrained to complete in less than 1.0 second, verify branch invariants, guard conditions, and trap paths. Native `proptest` suites in `core/tests/i64f64_proptest.rs` exercise the full-width `i128` raw-bit domain against independent matrix oracles. The combined Kani and native proptest evidence is the primary verification basis for these requirements.

The native proptest matrix oracles perform bitwise equivalence checks for checked addition and subtraction, 256-bit limb multiplication and fixed-point truncation, shifted-numerator division, and convergent rounding. Kani micro-proofs and proptest cases cover complementary obligations: Kani establishes bounded branch and safety invariants, while proptest exercises broad input distributions and independently computed expected bit patterns.

### SVCP-MATH-MET-002: Test Independence Override
As independent human peer review is explicitly deferred under project criteria, verification soundness for the math operation requirements is established by dual bounded formal symbolic proof and native property-based verification. Proof definitions and native property tests must strictly test for the absence of undefined behavior, runtime panics, and un-trapped arithmetic clipping.

## 2. Verification Primitives and Proof Bounds (SVCP-MATH-PRO)

### SVCP-MATH-PRO-001: Addition & Subtraction Soundness Proof
Status: Dual Formal and Property Verified.

The verification harnesses `verification::proofs::verify_i64f64_addition_exact_when_in_range` and `verification::proofs::verify_i64f64_subtraction_exact_when_in_range` in `verification/src/lib.rs` prove that for any two symbolic `i128` values mapped to `I64F64` structures ($A$ and $B$), non-overflowing addition and subtraction return the exact bitwise arithmetic result.

The verification harnesses `verification::proofs::verify_i64f64_addition_overflow_traps` and `verification::proofs::verify_i64f64_subtraction_overflow_traps` in `verification/src/lib.rs` observe panic/trap behavior when `i128::checked_add` or `i128::checked_sub` return `None`. These trap proofs do not claim panic message matching. Implementation-local add/sub tests remain regression support unless paired with Kani proof coverage.

The native proptest case `fallible_add_and_sub_match_checked_primitives` in `core/tests/i64f64_proptest.rs` provides full-width `i128` matrix-oracle coverage for the fallible paths and public operators. It checks exact output bits for successful operations and the expected trap classification for overflowing operations. The Kani scalar proofs and native proptest bitwise checks are the primary evidence for this procedure.
*Traces to: LLR-REPLAY-MATH-OPS-001*

### SVCP-MATH-PRO-002: Multiplication Proof Slices
Status: Dual Formal and Property Verified.

`SVCP-MATH-PRO-002a` is active. The verification harness `verification::proofs::verify_i64f64_multiplication_tiny_fractional_products_truncate_to_zero` in `verification/src/lib.rs` proves that bounded symbolic `i32` raw operands whose absolute magnitudes multiply below 2^64 return zero under raw `I64F64` multiplication. This covers positive, negative, and mixed-sign tiny fractional products and confirms truncation toward zero for this slice. This slice is paired with implementation-local regression tests for tiny raw products and fixed-point +/-1.0 multiplication.

`SVCP-MATH-PRO-002b` is active. The verification harness `verification::proofs::verify_i64f64_multiplication_bounded_truncates_toward_zero` in `verification/src/lib.rs` proves bounded raw multiplication equivalence for symbolic operands whose magnitudes are either bounded symbolic `u32` fractional raw values or the exact `I64F64::SCALE` (+/-1.0) raw endpoint. For that bounded domain, raw `I64F64` multiplication equals sign isolation, absolute magnitude multiplication, low-64-bit truncation, and sign reapplication.

`SVCP-MATH-PRO-002c` is active for bounded fixed non-unit high-limb raw multiplication correspondence, bounded low-limb carry contribution, bounded integrated non-overflowing matrix composition, bounded symbolic high-limb non-overflowing matrix composition, bounded high-high overflow-gate trap observation, bounded final signed-capacity overflow trap observation, public-operand cross-sum overflow unreachability for raw operands reachable through the public `I64F64` representation, signed minimum-capacity boundary allowance, and negative signed-capacity exceedance trap observation. Active harnesses are `verification::proofs::verify_i64f64_multiplication_bounded_lh_cross_term_correspondence`, `verification::proofs::verify_i64f64_multiplication_bounded_hl_cross_term_correspondence`, `verification::proofs::verify_i64f64_multiplication_bounded_cross_sum_composition`, `verification::proofs::verify_i64f64_multiplication_bounded_ll_carry_contribution`, `verification::proofs::verify_i64f64_multiplication_bounded_matrix_composition`, `verification::proofs::verify_i64f64_multiplication_bounded_symbolic_matrix_composition`, `verification::proofs::verify_i64f64_multiplication_hh_overflow_gate_traps`, `verification::proofs::verify_i64f64_multiplication_signed_capacity_overflow_traps`, `verification::proofs::verify_i64f64_multiplication_signed_min_capacity_boundary_allowed`, `verification::proofs::verify_i64f64_multiplication_negative_signed_capacity_exceedance_traps`, `verification::proofs::verify_i64f64_multiplication_negative_signed_capacity_exceedance_traps_commuted`, and `verification::proofs::verify_i64f64_multiplication_cross_sum_overflow_unreachable_for_public_operands` in `verification/src/lib.rs`. The native proptest case `fallible_mul_matches_256_bit_limb_oracle` in `core/tests/i64f64_proptest.rs` independently computes the full-width `i128` result through a 256-bit limb matrix, fixed-point truncation, sign restoration, and capacity classification, then performs bitwise equivalence checks or verifies the expected trap. The bounded Kani slices and this full-width proptest oracle together provide the primary evidence for raw multiplication; convergent multiplication remains covered by the rounding procedure.
*Traces to: LLR-REPLAY-MATH-OPS-002*

### SVCP-MATH-PRO-003: Division Invariant Proof
Status: Dual Formal and Property Verified.

`SVCP-MATH-PRO-003a` is active. The verification harness `verification::proofs::verify_i64f64_division_denominator_zero_traps` in `verification/src/lib.rs` proves that raw `I64F64` division traps for any symbolic numerator when the denominator is zero. This is a guard-behavior proof slice only; Kani 0.58.0 observes the expected panic path but does not match the panic message.

`SVCP-MATH-PRO-003b` is active. The verification harness `verification::proofs::verify_i64f64_division_numerator_shift_overflow_traps` in `verification/src/lib.rs` proves that raw `I64F64` division traps for symbolic numerators whose sign-extension bounds show that shifting left by 64 bits would overflow the signed 128-bit representation, with the denominator constrained nonzero so the shift-overflow guard is the exercised division guard.

`SVCP-MATH-PRO-003c` is active for bounded non-trapping arithmetic correspondence with symbolic `i32` raw numerators and signed power-of-two denominator family `{-8, -4, -2, -1, 1, 2, 4, 8}`. The verification harness `verification::proofs::verify_i64f64_division_i32_unit_denominators_match_shifted_reference` in `verification/src/lib.rs` proves non-trapping division arithmetic correspondence against the shifted-numerator reference quotient for unit denominators `{-1, 1}`. The verification harness `verification::proofs::verify_i64f64_division_i32_small_denominators_match_shifted_reference` in `verification/src/lib.rs` proves the same correspondence for denominator family `{-2, -1, 1, 2}`. The verification harness `verification::proofs::verify_i64f64_division_i32_power_of_two_denominators_match_shifted_reference` in `verification/src/lib.rs` proves the same correspondence for signed power-of-two denominator family `{-8, -4, -2, -1, 1, 2, 4, 8}`. The native proptest case `fallible_div_matches_truncating_wide_reference` in `core/tests/i64f64_proptest.rs` independently computes the shifted-numerator quotient and guard classification across the full-width `i128` raw-bit domain, then checks exact result bits or the expected arithmetic error. The bounded Kani guard proofs and native proptest bitwise oracle are the primary evidence for this procedure.
*Traces to: LLR-REPLAY-MATH-OPS-003*

### SVCP-MATH-PRO-004: Convergent Integer Rounding Proof
Status: Dual Formal and Property Verified.

The verification harness `verification::proofs::verify_accumulator_convergent_rounding_exhaustive` in `verification/src/lib.rs` proves that accumulator-to-integer conversion rounds to the nearest integral value and resolves exact half-scale ties toward an even integral result.

The native proptest case `convergent_rounding_matches_euclidean_tie_to_even_reference` in `core/tests/i64f64_proptest.rs` independently computes Euclidean quotient, remainder, and tie-to-even selection for every generated full-width `i128` raw value, then checks exact integer equivalence. The Kani exhaustive proof and native proptest bitwise/reference checks are the primary evidence for this procedure.
*Traces to: LLR-REPLAY-MATH-OPS-004*
