#![forbid(unsafe_code)]

//! Local verification harnesses for deterministic replay primitives.

#[cfg(kani)]
pub mod proofs {
    use precision_replay_core::math::{round_ties_to_even, I64F64};

    const FRACTION_MASK: u128 = 0xFFFF_FFFF_FFFF_FFFF;
    const HALF_SCALE: u128 = 0x8000_0000_0000_0000;

    /// # Verification Vector: verify_i64f64_multiplication_tiny_fractional_products_truncate_to_zero
    /// Proves that bounded symbolic `i32` raw operands whose absolute magnitudes
    /// multiply below 2^64 return zero under raw `I64F64` multiplication.
    #[kani::proof]
    pub fn verify_i64f64_multiplication_tiny_fractional_products_truncate_to_zero() {
        let lhs_sample: i32 = kani::any();
        let rhs_sample: i32 = kani::any();
        let lhs_raw = lhs_sample as i128;
        let rhs_raw = rhs_sample as i128;

        let lhs_abs = if lhs_raw < 0 {
            (-lhs_raw) as u128
        } else {
            lhs_raw as u128
        };
        let rhs_abs = if rhs_raw < 0 {
            (-rhs_raw) as u128
        } else {
            rhs_raw as u128
        };

        kani::assume(lhs_abs * rhs_abs < (1u128 << I64F64::FRAC_BITS));

        let actual = I64F64::from_bits(lhs_raw) * I64F64::from_bits(rhs_raw);

        assert_eq!(actual, I64F64::from_bits(0));
    }

    /// # Verification Vector: verify_i64f64_multiplication_bounded_truncates_toward_zero
    /// Proves that raw multiplication over symbolic operands whose magnitudes
    /// are either bounded symbolic `u32` fractional raw values or the exact
    /// 1.0 raw endpoint equals sign isolation, absolute magnitude
    /// multiplication, low-64-bit truncation, and sign reapplication.
    #[kani::proof]
    pub fn verify_i64f64_multiplication_bounded_truncates_toward_zero() {
        let lhs_negative: bool = kani::any();
        let rhs_negative: bool = kani::any();
        let lhs_is_unit: bool = kani::any();
        let rhs_is_unit: bool = kani::any();
        let lhs_fractional: u32 = kani::any();
        let rhs_fractional: u32 = kani::any();
        let max_magnitude = I64F64::SCALE as u128;

        let lhs_magnitude = if lhs_is_unit {
            max_magnitude
        } else {
            lhs_fractional as u128
        };
        let rhs_magnitude = if rhs_is_unit {
            max_magnitude
        } else {
            rhs_fractional as u128
        };

        let lhs_raw = if lhs_negative && lhs_magnitude != 0 {
            -(lhs_magnitude as i128)
        } else {
            lhs_magnitude as i128
        };
        let rhs_raw = if rhs_negative && rhs_magnitude != 0 {
            -(rhs_magnitude as i128)
        } else {
            rhs_magnitude as i128
        };

        let product = lhs_magnitude.checked_mul(rhs_magnitude);
        let expected_magnitude = match product {
            Some(value) => value >> I64F64::FRAC_BITS,
            None => {
                assert_eq!(lhs_magnitude, max_magnitude);
                assert_eq!(rhs_magnitude, max_magnitude);
                max_magnitude
            }
        };
        assert!(expected_magnitude <= i128::MAX as u128);

        let expected_bits = if (lhs_raw < 0) ^ (rhs_raw < 0) {
            -(expected_magnitude as i128)
        } else {
            expected_magnitude as i128
        };

        let actual = I64F64::from_bits(lhs_raw) * I64F64::from_bits(rhs_raw);

        assert_eq!(actual, I64F64::from_bits(expected_bits));
    }

    /// # Verification Vector: verify_i64f64_addition_exact_when_in_range
    /// Proves that non-overflowing `I64F64` addition returns the exact `i128`
    /// checked-addition result bits.
    #[kani::proof]
    pub fn verify_i64f64_addition_exact_when_in_range() {
        let lhs_bits: i128 = kani::any();
        let rhs_bits: i128 = kani::any();
        let expected = lhs_bits.checked_add(rhs_bits);
        kani::assume(expected.is_some());

        let result = I64F64::from_bits(lhs_bits) + I64F64::from_bits(rhs_bits);

        assert_eq!(result.to_bits(), expected.unwrap());
    }

    /// # Verification Vector: verify_i64f64_subtraction_exact_when_in_range
    /// Proves that non-overflowing `I64F64` subtraction returns the exact `i128`
    /// checked-subtraction result bits.
    #[kani::proof]
    pub fn verify_i64f64_subtraction_exact_when_in_range() {
        let lhs_bits: i128 = kani::any();
        let rhs_bits: i128 = kani::any();
        let expected = lhs_bits.checked_sub(rhs_bits);
        kani::assume(expected.is_some());

        let result = I64F64::from_bits(lhs_bits) - I64F64::from_bits(rhs_bits);

        assert_eq!(result.to_bits(), expected.unwrap());
    }

    /// # Verification Vector: verify_i64f64_division_denominator_zero_traps
    /// Proves that raw `I64F64` division traps before division when the
    /// denominator is zero, for any symbolic numerator.
    #[kani::proof]
    #[kani::should_panic]
    pub fn verify_i64f64_division_denominator_zero_traps() {
        let numerator_bits: i128 = kani::any();

        let _ = I64F64::from_bits(numerator_bits) / I64F64::from_bits(0);
    }

    /// # Verification Vector: verify_i64f64_division_numerator_shift_overflow_traps
    /// Proves that raw `I64F64` division traps before the division step when a
    /// symbolic numerator cannot be shifted left by 64 bits without overflowing
    /// the signed 128-bit representation.
    #[kani::proof]
    #[kani::should_panic]
    pub fn verify_i64f64_division_numerator_shift_overflow_traps() {
        let numerator_bits: i128 = kani::any();
        let denominator_bits: i128 = kani::any();
        let positive_shift_overflow = numerator_bits > 0 && numerator_bits.leading_zeros() < 64;
        let negative_shift_overflow = numerator_bits < 0 && numerator_bits.leading_ones() < 64;

        kani::assume(denominator_bits != 0);
        kani::assume(positive_shift_overflow || negative_shift_overflow);

        let _ = I64F64::from_bits(numerator_bits) / I64F64::from_bits(denominator_bits);
    }

    /// # Verification Vector: verify_i64f64_division_i32_unit_denominators_match_shifted_reference
    /// Proves that bounded symbolic `i32` raw numerators with unit
    /// denominators match the explicit shifted-numerator reference quotient.
    #[kani::proof]
    pub fn verify_i64f64_division_i32_unit_denominators_match_shifted_reference() {
        let numerator_sample: i32 = kani::any();
        let denominator_negative: bool = kani::any();
        let numerator_bits = numerator_sample as i128;
        let denominator_bits = if denominator_negative { -1 } else { 1 };
        let positive_shift_overflow = numerator_bits > 0 && numerator_bits.leading_zeros() < 64;
        let negative_shift_overflow = numerator_bits < 0 && numerator_bits.leading_ones() < 64;

        kani::assume(denominator_bits != 0);
        kani::assume(!positive_shift_overflow);
        kani::assume(!negative_shift_overflow);

        let shifted_numerator = numerator_bits << I64F64::FRAC_BITS;
        let expected = shifted_numerator.checked_div(denominator_bits);
        kani::assume(expected.is_some());

        let actual = I64F64::from_bits(numerator_bits) / I64F64::from_bits(denominator_bits);

        assert_eq!(actual.to_bits(), expected.unwrap());
    }

    /// # Verification Vector: verify_i64f64_division_i32_small_denominators_match_shifted_reference
    /// Proves that bounded symbolic `i32` raw numerators with denominator
    /// family {-2, -1, 1, 2} match the explicit shifted-numerator reference
    /// quotient.
    #[kani::proof]
    pub fn verify_i64f64_division_i32_small_denominators_match_shifted_reference() {
        let numerator_sample: i32 = kani::any();
        let denominator_selector: u8 = kani::any();
        let numerator_bits = numerator_sample as i128;
        kani::assume(denominator_selector < 4);
        let denominator_bits = match denominator_selector {
            0 => -2,
            1 => -1,
            2 => 1,
            _ => 2,
        };
        let positive_shift_overflow = numerator_bits > 0 && numerator_bits.leading_zeros() < 64;
        let negative_shift_overflow = numerator_bits < 0 && numerator_bits.leading_ones() < 64;

        kani::assume(denominator_bits != 0);
        kani::assume(!positive_shift_overflow);
        kani::assume(!negative_shift_overflow);

        let shifted_numerator = numerator_bits << I64F64::FRAC_BITS;
        let expected = shifted_numerator.checked_div(denominator_bits);
        kani::assume(expected.is_some());

        let actual = I64F64::from_bits(numerator_bits) / I64F64::from_bits(denominator_bits);

        assert_eq!(actual.to_bits(), expected.unwrap());
    }

    /// # Verification Vector: verify_accumulator_convergent_rounding_exhaustive
    /// Proves nearest-integer mapping across all fractional intervals and verifies
    /// that exact half-scale ties resolve toward the nearest even integer.
    #[kani::proof]
    pub fn verify_accumulator_convergent_rounding_exhaustive() {
        let raw_accum: i128 = kani::any();
        kani::assume(raw_accum > i128::MIN && raw_accum < i128::MAX);

        let rounded = round_ties_to_even(I64F64::from_bits(raw_accum));
        let expected_base = raw_accum >> I64F64::FRAC_BITS;
        let fractional_part = raw_accum as u128 & FRACTION_MASK;
        let expected_successor = expected_base.checked_add(1);

        if fractional_part < HALF_SCALE {
            assert_eq!(rounded, expected_base);
        } else if fractional_part > HALF_SCALE {
            match expected_successor {
                Some(successor) => assert_eq!(rounded, successor),
                None => assert_eq!(rounded, expected_base),
            }
        } else {
            assert_eq!(rounded & 1, 0);
            match expected_successor {
                Some(successor) => assert!(rounded == expected_base || rounded == successor),
                None => assert_eq!(rounded, expected_base),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use precision_replay_core::math::{round_ties_to_even, I64F64};

    #[test]
    fn sampled_accumulator_convergent_rounding_stays_within_successor_bound() {
        let samples = [
            i128::MIN,
            i128::MIN + I64F64::SCALE,
            -(I64F64::SCALE + (I64F64::SCALE >> 1)),
            -I64F64::SCALE,
            -(I64F64::SCALE >> 1),
            -1,
            0,
            I64F64::SCALE >> 1,
            I64F64::SCALE,
            I64F64::SCALE + (I64F64::SCALE >> 1),
            i128::MAX - I64F64::SCALE,
            i128::MAX,
        ];

        for raw_accum in samples {
            let rounded = round_ties_to_even(I64F64::from_bits(raw_accum));
            let expected_base = raw_accum >> I64F64::FRAC_BITS;
            let successor = expected_base.checked_add(1);

            match successor {
                Some(value) => assert!(rounded == expected_base || rounded == value),
                None => assert_eq!(rounded, expected_base),
            }
        }
    }
}
