use precision_replay_core::math::{round_ties_to_even, ArithmeticError, I64F64};
use proptest::prelude::*;

const SCALE_U128: u128 = 1u128 << 64;
const LOW_MASK: u128 = SCALE_U128 - 1;

fn abs_bits(value: i128) -> u128 {
    if value < 0 {
        (value as u128).wrapping_neg()
    } else {
        value as u128
    }
}

fn mul_u64_limbs(lhs: u128, rhs: u128) -> [u64; 4] {
    let lhs_limbs = [lhs as u64, (lhs >> 64) as u64];
    let rhs_limbs = [rhs as u64, (rhs >> 64) as u64];
    let mut result = [0u64; 4];

    for (i, &lhs_limb) in lhs_limbs.iter().enumerate() {
        let mut carry = 0u128;
        for (j, &rhs_limb) in rhs_limbs.iter().enumerate() {
            let index = i + j;
            let value = (lhs_limb as u128) * (rhs_limb as u128) + (result[index] as u128) + carry;
            result[index] = value as u64;
            carry = value >> 64;
        }

        let mut index = i + 2;
        while carry != 0 && index < result.len() {
            let value = (result[index] as u128) + carry;
            result[index] = value as u64;
            carry = value >> 64;
            index += 1;
        }
    }

    result
}

fn shifted_product_u128(lhs: i128, rhs: i128) -> Option<u128> {
    let product = mul_u64_limbs(abs_bits(lhs), abs_bits(rhs));
    if product[3] != 0 {
        return None;
    }

    Some((product[1] as u128) | ((product[2] as u128) << 64))
}

fn expected_mul(lhs: i128, rhs: i128) -> Result<i128, ArithmeticError> {
    if lhs == 0 || rhs == 0 {
        return Ok(0);
    }

    let lhs_abs = abs_bits(lhs);
    let rhs_abs = abs_bits(rhs);
    let lhs_hi = lhs_abs >> 64;
    let lhs_lo = lhs_abs & LOW_MASK;
    let rhs_hi = rhs_abs >> 64;
    let rhs_lo = rhs_abs & LOW_MASK;

    let hh = lhs_hi * rhs_hi;
    if hh > u64::MAX as u128 {
        return Err(ArithmeticError::MultiplicativeSaturation);
    }

    let cross_sum = (lhs_lo * rhs_hi)
        .checked_add(lhs_hi * rhs_lo)
        .ok_or(ArithmeticError::CrossTermOverflow)?;
    let scaled = (hh << 64)
        .checked_add(cross_sum)
        .and_then(|value| value.checked_add((lhs_lo * rhs_lo) >> 64))
        .ok_or(ArithmeticError::BitPoolCompositionFailure)?;

    let negative = (lhs < 0) ^ (rhs < 0);
    if scaled > i128::MAX as u128 {
        if negative && scaled == (i128::MIN as u128) {
            return Ok(i128::MIN);
        }
        return Err(ArithmeticError::CapacityBoundOverflow);
    }

    let signed = scaled as i128;
    if negative {
        signed
            .checked_neg()
            .ok_or(ArithmeticError::CapacityBoundOverflow)
    } else {
        Ok(signed)
    }
}

fn expected_div(lhs: i128, rhs: i128) -> Result<i128, ArithmeticError> {
    if rhs == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    if lhs >= I64F64::SCALE || lhs <= -I64F64::SCALE {
        return Err(ArithmeticError::Overflow);
    }

    (lhs << 64)
        .checked_div(rhs)
        .ok_or(ArithmeticError::IntegerDivisionOverflow)
}

fn expected_round(bits: i128) -> i128 {
    let integral = bits.div_euclid(I64F64::SCALE);
    let fraction = bits.rem_euclid(I64F64::SCALE) as u128;
    let increment = if fraction > SCALE_U128 / 2 {
        1
    } else if fraction == SCALE_U128 / 2 && integral & 1 != 0 {
        1
    } else {
        0
    };

    integral + increment
}

fn operator_add(lhs: i128, rhs: i128) -> Result<i128, ()> {
    std::panic::catch_unwind(|| (I64F64::from_bits(lhs) + I64F64::from_bits(rhs)).to_bits())
        .map_err(|_| ())
}

fn operator_sub(lhs: i128, rhs: i128) -> Result<i128, ()> {
    std::panic::catch_unwind(|| (I64F64::from_bits(lhs) - I64F64::from_bits(rhs)).to_bits())
        .map_err(|_| ())
}

type WideSigned = (i128, u128);

fn wide_mul(lhs: i128, rhs: i128) -> WideSigned {
    let lhs_abs = abs_bits(lhs);
    let rhs_abs = abs_bits(rhs);
    let product = mul_u64_limbs(lhs_abs, rhs_abs);
    let mut low = (product[1] as u128) << 64 | product[0] as u128;
    let mut high = (product[3] as u128) << 64 | product[2] as u128;

    if (lhs < 0) ^ (rhs < 0) {
        let (negated_low, carry) = (!low).overflowing_add(1);
        low = negated_low;
        high = (!high).wrapping_add(carry as u128);
    }

    (high as i128, low)
}

fn wide_add(lhs: WideSigned, rhs: WideSigned) -> Result<WideSigned, ArithmeticError> {
    let (low, carry) = lhs.1.overflowing_add(rhs.1);
    let high = lhs
        .0
        .checked_add(rhs.0)
        .and_then(|value| value.checked_add(carry as i128))
        .ok_or(ArithmeticError::CapacityBoundOverflow)?;
    Ok((high, low))
}

fn wide_truncate(value: WideSigned) -> Result<i128, ArithmeticError> {
    let negative = value.0 < 0;
    let magnitude = if negative {
        let (low, carry) = (!value.1).overflowing_add(1);
        ((!value.0 as u128).wrapping_add(carry as u128), low)
    } else {
        (value.0 as u128, value.1)
    };
    if magnitude.0 >> 64 != 0 {
        return Err(ArithmeticError::CapacityBoundOverflow);
    }
    let shifted = (magnitude.1 >> 64) | (magnitude.0 << 64);
    if negative {
        if shifted > (1u128 << 127) {
            Err(ArithmeticError::CapacityBoundOverflow)
        } else if shifted == (1u128 << 127) {
            Ok(i128::MIN)
        } else {
            Ok(-(shifted as i128))
        }
    } else if shifted <= i128::MAX as u128 {
        Ok(shifted as i128)
    } else {
        Err(ArithmeticError::CapacityBoundOverflow)
    }
}

fn expected_dot(lhs: &[i128], rhs: &[i128]) -> Result<i128, ArithmeticError> {
    if lhs.len() != rhs.len() {
        return Err(ArithmeticError::DimensionMismatch);
    }

    let mut sum = (0, 0);
    for (&a, &b) in lhs.iter().zip(rhs) {
        sum = wide_add(sum, wide_mul(a, b))?;
    }
    wide_truncate(sum)
}

fn expected_lerp(a: i128, b: i128, t: i128) -> Result<i128, ArithmeticError> {
    if !(0..=I64F64::SCALE).contains(&t) {
        return Err(ArithmeticError::ParameterOutOfRange);
    }
    if t == 0 {
        return Ok(a);
    }
    if t == I64F64::SCALE {
        return Ok(b);
    }

    let delta = b
        .checked_sub(a)
        .ok_or(ArithmeticError::SubtractionOverflow)?;
    let scaled = wide_truncate(wide_mul(delta, t))?;
    a.checked_add(scaled)
        .ok_or(ArithmeticError::AdditionOverflow)
}

proptest! {
    #[test]
    fn fallible_add_and_sub_match_checked_primitives(
        lhs in any::<i128>(),
        rhs in any::<i128>(),
    ) {
        let expected_add = lhs.checked_add(rhs).ok_or(());
        let expected_sub = lhs.checked_sub(rhs).ok_or(());

        prop_assert_eq!(operator_add(lhs, rhs), expected_add);
        prop_assert_eq!(operator_sub(lhs, rhs), expected_sub);
    }

    #[test]
    fn public_operators_match_fallible_counterparts(
        lhs in any::<i128>(),
        rhs in any::<i128>(),
        denominator in any::<i128>(),
    ) {
        let lhs_value = I64F64::from_bits(lhs);
        let rhs_value = I64F64::from_bits(rhs);

        if let Ok(expected) = expected_mul(lhs, rhs) {
            prop_assert_eq!((lhs_value * rhs_value).to_bits(), expected);
        }

        let denominator_value = I64F64::from_bits(denominator);
        if let Ok(expected) = lhs_value.fallible_div(denominator_value) {
            prop_assert_eq!(
                (lhs_value / denominator_value).to_bits(),
                expected.to_bits()
            );
        }

        if lhs.checked_add(rhs).is_some() {
            prop_assert_eq!(
                operator_add(lhs, rhs),
                Ok((lhs_value + rhs_value).to_bits())
            );
        }
        if lhs.checked_sub(rhs).is_some() {
            prop_assert_eq!(
                operator_sub(lhs, rhs),
                Ok((lhs_value - rhs_value).to_bits())
            );
        }
    }

    #[test]
    fn algebraic_identities_and_round_tripping(
        value in any::<i128>(),
        addend in any::<i128>(),
    ) {
        let value_fixed = I64F64::from_bits(value);
        let zero = I64F64::from_bits(0);
        let one = I64F64::from_bits(I64F64::SCALE);

        prop_assert_eq!((value_fixed + zero).to_bits(), value);
        prop_assert_eq!((zero + value_fixed).to_bits(), value);
        prop_assert_eq!((value_fixed - zero).to_bits(), value);
        prop_assert_eq!((value_fixed - value_fixed).to_bits(), 0);
        prop_assert_eq!((value_fixed * zero).to_bits(), 0);
        prop_assert_eq!((zero * value_fixed).to_bits(), 0);
        prop_assert_eq!((value_fixed * one).to_bits(), value);
        prop_assert_eq!((one * value_fixed).to_bits(), value);

        if value != i128::MIN {
            let inverse = I64F64::from_bits(-value);
            prop_assert_eq!((value_fixed + inverse).to_bits(), 0);
        }

        if let Some(sum) = value.checked_add(addend) {
            let round_tripped = I64F64::from_bits(sum) - I64F64::from_bits(addend);
            prop_assert_eq!(round_tripped.to_bits(), value);
        }
    }

    #[test]
    fn fallible_div_matches_truncating_wide_reference(
        numerator in any::<i128>(),
        denominator in any::<i128>(),
    ) {
        let expected = expected_div(numerator, denominator);
        let actual = I64F64::from_bits(numerator).fallible_div(I64F64::from_bits(denominator));

        prop_assert_eq!(actual, expected.map(I64F64::from_bits));
        if let Ok(value) = expected {
            let actual_value = actual.expect("successful expected division");
            prop_assert_eq!(actual_value.to_bits() & 1, value & 1);
            prop_assert_eq!(actual_value.to_bits(), (numerator << 64) / denominator);
        }
    }

    #[test]
    fn fallible_mul_matches_256_bit_limb_oracle(
        lhs in any::<i128>(),
        rhs in any::<i128>(),
    ) {
        let expected = expected_mul(lhs, rhs);
        if let Ok(expected_bits) = expected {
            let actual = I64F64::from_bits(lhs) * I64F64::from_bits(rhs);
            prop_assert_eq!(actual.to_bits(), expected_bits);

            let wide_shifted = shifted_product_u128(lhs, rhs);
            prop_assert_eq!(wide_shifted, Some(abs_bits(expected_bits)));
        } else {
            let panicked = std::panic::catch_unwind(|| {
                let _ = I64F64::from_bits(lhs) * I64F64::from_bits(rhs);
            })
            .is_err();
            prop_assert!(panicked);
        }
    }

    #[test]
    fn convergent_rounding_matches_euclidean_tie_to_even_reference(
        bits in any::<i128>(),
    ) {
        let expected = expected_round(bits);
        let actual = round_ties_to_even(I64F64::from_bits(bits));
        prop_assert_eq!(actual, expected);
    }

    #[test]
    fn dot_product_rejects_mismatched_slices(
        lhs in prop::collection::vec(any::<i64>(), 0..5),
        rhs in prop::collection::vec(any::<i64>(), 0..5),
    ) {
        prop_assume!(lhs.len() != rhs.len());
        let lhs = lhs.into_iter().map(|value| I64F64::from_bits(value as i128)).collect::<Vec<_>>();
        let rhs = rhs.into_iter().map(|value| I64F64::from_bits(value as i128)).collect::<Vec<_>>();

        prop_assert_eq!(
            I64F64::dot_product(&lhs, &rhs),
            Err(ArithmeticError::DimensionMismatch)
        );
    }

    #[test]
    fn dot_product_is_commutative(
        values in prop::collection::vec((any::<i128>(), any::<i128>()), 0..5),
    ) {
        let lhs = values.iter().map(|&(value, _)| I64F64::from_bits(value)).collect::<Vec<_>>();
        let rhs = values.iter().map(|&(_, value)| I64F64::from_bits(value)).collect::<Vec<_>>();

        prop_assert_eq!(I64F64::dot_product(&lhs, &rhs), I64F64::dot_product(&rhs, &lhs));
    }

    #[test]
    fn dot_product_matches_independent_wide_oracle(
        values in prop::collection::vec((any::<i128>(), any::<i128>()), 0..5),
    ) {
        let lhs = values.iter().map(|&(value, _)| I64F64::from_bits(value)).collect::<Vec<_>>();
        let rhs = values.iter().map(|&(_, value)| I64F64::from_bits(value)).collect::<Vec<_>>();
        let lhs_bits = values.iter().map(|&(value, _)| value).collect::<Vec<_>>();
        let rhs_bits = values.iter().map(|&(_, value)| value).collect::<Vec<_>>();

        prop_assert_eq!(
            I64F64::dot_product(&lhs, &rhs).map(I64F64::to_bits),
            expected_dot(&lhs_bits, &rhs_bits)
        );
    }

    #[test]
    fn single_element_dot_product_matches_scalar_multiplication(
        lhs in any::<i32>(),
        rhs in any::<i32>(),
    ) {
        let lhs = I64F64::from_bits(lhs as i128);
        let rhs = I64F64::from_bits(rhs as i128);
        prop_assert_eq!(I64F64::dot_product(&[lhs], &[rhs]), Ok(lhs * rhs));
    }

    #[test]
    fn lerp_rejects_parameters_outside_unit_interval(
        t in any::<i128>(),
    ) {
        prop_assume!(t < 0 || t > I64F64::SCALE);
        let value = I64F64::from_bits(1);
        prop_assert_eq!(
            I64F64::lerp(value, value, I64F64::from_bits(t)),
            Err(ArithmeticError::ParameterOutOfRange)
        );
    }

    #[test]
    fn lerp_has_exact_endpoints(a in any::<i128>(), b in any::<i128>()) {
        prop_assert_eq!(
            I64F64::lerp(I64F64::from_bits(a), I64F64::from_bits(b), I64F64::from_bits(0)),
            Ok(I64F64::from_bits(a))
        );
        prop_assert_eq!(
            I64F64::lerp(
                I64F64::from_bits(a),
                I64F64::from_bits(b),
                I64F64::from_bits(I64F64::SCALE)
            ),
            Ok(I64F64::from_bits(b))
        );
    }

    #[test]
    fn lerp_matches_independent_wide_oracle(
        a in any::<i128>(),
        b in any::<i128>(),
        t in 0i128..=I64F64::SCALE,
    ) {
        prop_assert_eq!(
            I64F64::lerp(
                I64F64::from_bits(a),
                I64F64::from_bits(b),
                I64F64::from_bits(t)
            ).map(I64F64::to_bits),
            expected_lerp(a, b, t)
        );
    }

    #[test]
    fn lerp_is_monotonic_for_ordered_endpoints(
        a in -1_000_000i128..1_000_000i128,
        b in -1_000_000i128..1_000_000i128,
        t in 0i128..=I64F64::SCALE,
    ) {
        prop_assume!(a <= b);
        let result = I64F64::lerp(I64F64::from_bits(a), I64F64::from_bits(b), I64F64::from_bits(t));
        prop_assert!(result.is_ok());
        prop_assert!(result.unwrap().to_bits() >= a);
        prop_assert!(result.unwrap().to_bits() <= b);
    }
}

#[test]
fn multiplication_boundary_oracle_classifies_saturation() {
    assert_eq!(
        expected_mul(i128::MAX, i128::MAX),
        Err(ArithmeticError::MultiplicativeSaturation)
    );
    assert_eq!(
        expected_mul(i128::MIN, I64F64::SCALE + 1),
        Err(ArithmeticError::CapacityBoundOverflow)
    );
}

#[test]
fn dot_product_handles_large_cancellation_vectors() {
    let lhs = [I64F64::from_bits(i128::MAX), I64F64::from_bits(i128::MIN)];
    let rhs = [
        I64F64::from_bits(I64F64::SCALE),
        I64F64::from_bits(I64F64::SCALE),
    ];

    assert_eq!(I64F64::dot_product(&lhs, &rhs), Ok(I64F64::from_bits(-1)));
}
