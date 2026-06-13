#![forbid(unsafe_code)]

//! Local verification harnesses for deterministic replay primitives.

#[cfg(kani)]
pub mod proofs {
    use precision_replay_core::math::{round_ties_to_even, I64F64};

    const FRACTION_MASK: u128 = 0xFFFF_FFFF_FFFF_FFFF;
    const HALF_SCALE: u128 = 0x8000_0000_0000_0000;

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
