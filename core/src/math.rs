use core::ops::{Add, Div, Mul, Sub};

/// A deterministic, platform-agnostic 128-bit fixed-point number
/// with a signed 64-bit integer part and a 64-bit fractional part.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct I64F64(pub i128);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ArithmeticError {
    AdditionOverflow,
    SubtractionOverflow,
    CrossTermOverflow,
    MultiplicativeSaturation,
    BitPoolCompositionFailure,
    CapacityBoundOverflow,
    DivisionByZero,
    DivisionNumeratorShiftOverflow,
    IntegerDivisionOverflow,
}

#[cold]
#[track_caller]
fn panic_arithmetic<T>(error: ArithmeticError) -> T {
    match error {
        ArithmeticError::AdditionOverflow => {
            panic!("CRITICAL MATH EXCEPTION: I64F64 Addition Overflow")
        }
        ArithmeticError::SubtractionOverflow => {
            panic!("CRITICAL MATH EXCEPTION: I64F64 Subtraction Overflow")
        }
        ArithmeticError::CrossTermOverflow => {
            panic!("CRITICAL MATH EXCEPTION: Cross Term Overflow")
        }
        ArithmeticError::MultiplicativeSaturation => {
            panic!("CRITICAL MATH EXCEPTION: Multiplicative Saturation")
        }
        ArithmeticError::BitPoolCompositionFailure => {
            panic!("CRITICAL MATH EXCEPTION: Bit Pool Composition Failure")
        }
        ArithmeticError::CapacityBoundOverflow => {
            panic!("CRITICAL MATH EXCEPTION: Capacity Bound Overflow")
        }
        ArithmeticError::DivisionByZero => panic!("CRITICAL MATH EXCEPTION: Division By Zero"),
        ArithmeticError::DivisionNumeratorShiftOverflow => {
            panic!("CRITICAL MATH EXCEPTION: I64F64 Division Numerator Shift Overflow")
        }
        ArithmeticError::IntegerDivisionOverflow => {
            panic!("CRITICAL MATH EXCEPTION: I64F64 Integer Division Overflow")
        }
    }
}

impl I64F64 {
    pub const BITS: u32 = 128;
    pub const FRAC_BITS: u32 = 64;
    pub const SCALE: i128 = 1i128 << Self::FRAC_BITS;

    #[inline]
    pub const fn from_bits(bits: i128) -> Self {
        Self(bits)
    }

    #[inline]
    pub const fn to_bits(self) -> i128 {
        self.0
    }

    /// Internal multiplication matrix core that handles sign isolation,
    /// primitive bypass casting, and partial product generation.
    /// Returns (out_negative, ll, cross_lo, cross_hi, hh)
    #[inline]
    fn fallible_mul_matrix(
        self,
        rhs: Self,
    ) -> Result<(bool, u128, u128, u128, u128), ArithmeticError> {
        let a = self.0;
        let b = rhs.0;

        // 1. SIGN ISOLATION
        let out_negative = (a < 0) ^ (b < 0);

        // 2. PRIMITIVE TYPE BYPASS (Branch-free absolute conversion)
        let mask_a = (a >> 127) as u128;
        let mask_b = (b >> 127) as u128;

        let unsigned_a = a as u128;
        let unsigned_b = b as u128;

        let abs_a = (unsigned_a ^ mask_a).wrapping_add(mask_a & 1);
        let abs_b = (unsigned_b ^ mask_b).wrapping_add(mask_b & 1);

        // 3. MATRIX DECONSTRUCTION (64-bit subfields)
        let a_hi = abs_a >> 64;
        let a_lo = abs_a & 0xFFFFFFFFFFFFFFFF;
        let b_hi = abs_b >> 64;
        let b_lo = abs_b & 0xFFFFFFFFFFFFFFFF;

        // 4. PARTIAL PRODUCTS
        let ll = a_lo.wrapping_mul(b_lo);
        let lh = a_lo.wrapping_mul(b_hi);
        let hl = a_hi.wrapping_mul(b_lo);
        let hh = a_hi.wrapping_mul(b_hi);

        // 5. CROSS TERM ACCUMULATION
        let cross_sum = lh
            .checked_add(hl)
            .ok_or(ArithmeticError::CrossTermOverflow)?;

        let cross_lo = cross_sum << 64;
        let cross_hi = cross_sum >> 64;

        Ok((out_negative, ll, cross_lo, cross_hi, hh))
    }

    /// Internal multiplication matrix core that handles sign isolation,
    /// primitive bypass casting, and partial product generation.
    /// Returns (out_negative, ll, cross_lo, cross_hi, hh)
    #[inline]
    fn execute_mul_matrix(self, rhs: Self) -> (bool, u128, u128, u128, u128) {
        match self.fallible_mul_matrix(rhs) {
            Ok(matrix) => matrix,
            Err(error) => panic_arithmetic(error),
        }
    }

    #[inline]
    pub(crate) fn fallible_add(self, rhs: Self) -> Result<Self, ArithmeticError> {
        self.0
            .checked_add(rhs.0)
            .map(Self)
            .ok_or(ArithmeticError::AdditionOverflow)
    }

    #[inline]
    pub(crate) fn fallible_sub(self, rhs: Self) -> Result<Self, ArithmeticError> {
        self.0
            .checked_sub(rhs.0)
            .map(Self)
            .ok_or(ArithmeticError::SubtractionOverflow)
    }

    #[inline]
    pub(crate) fn fallible_mul(self, rhs: Self) -> Result<Self, ArithmeticError> {
        if self.0 == 0 || rhs.0 == 0 {
            return Ok(Self(0));
        }

        let (out_negative, ll, cross_lo, cross_hi, hh) = self.fallible_mul_matrix(rhs)?;

        // HIGH-LIMB CAPACITY GATE AFTER 64-BIT TRUNCATION
        if hh > 0xFFFF_FFFF_FFFF_FFFF {
            return Err(ArithmeticError::MultiplicativeSaturation);
        }

        // RAW TRUNCATION ALIGNMENT SHIFT
        let cross_sum = (cross_hi << 64) | (cross_lo >> 64);
        let hh_scaled = hh << 64;
        let ll_scaled = ll >> 64;
        let final_abs_bits = hh_scaled
            .checked_add(cross_sum)
            .and_then(|value| value.checked_add(ll_scaled))
            .ok_or(ArithmeticError::BitPoolCompositionFailure)?;

        // CAPACITY BOUNDARY CHECK
        if final_abs_bits > i128::MAX as u128 {
            if out_negative && final_abs_bits == (i128::MIN as u128) {
                return Ok(Self(i128::MIN));
            }
            return Err(ArithmeticError::CapacityBoundOverflow);
        }

        let final_signed =
            i128::try_from(final_abs_bits).map_err(|_| ArithmeticError::CapacityBoundOverflow)?;
        if out_negative {
            final_signed
                .checked_neg()
                .map(Self)
                .ok_or(ArithmeticError::CapacityBoundOverflow)
        } else {
            Ok(Self(final_signed))
        }
    }

    #[inline]
    pub(crate) fn fallible_div(self, rhs: Self) -> Result<Self, ArithmeticError> {
        if rhs.0 == 0 {
            return Err(ArithmeticError::DivisionByZero);
        }
        let leading_zeros = self.0.leading_zeros();
        let leading_ones = self.0.leading_ones();
        if (self.0 > 0 && leading_zeros < 64) || (self.0 < 0 && leading_ones < 64) {
            return Err(ArithmeticError::DivisionNumeratorShiftOverflow);
        }
        let shifted_numerator = self.0 << Self::FRAC_BITS;
        shifted_numerator
            .checked_div(rhs.0)
            .map(Self)
            .ok_or(ArithmeticError::IntegerDivisionOverflow)
    }

    /// Explicit multiplier utilizing branch-free convergent rounding (Banker's Rounding).
    /// Used internally by `ConvergentAccumulator` to eliminate long-term tracking drift.
    #[inline]
    pub fn mul_convergent(self, rhs: Self) -> Self {
        if self.0 == 0 || rhs.0 == 0 {
            return Self(0);
        }

        let (out_negative, ll, cross_lo, cross_hi, hh) = self.execute_mul_matrix(rhs);

        // EXPONENTIAL OVERFLOW TRAP
        if cross_hi != 0 || hh != 0 {
            panic!("CRITICAL MATH EXCEPTION: Multiplicative Saturation");
        }

        // TIE-BREAKING ALIGNMENT (Round-Half-to-Even)
        let discarded_fraction = ll & 0xFFFFFFFFFFFFFFFF;
        let tie_boundary = 0x8000_0000_0000_0000u128;

        let mut scaled_result = cross_lo.checked_add(ll >> 64).unwrap();

        // Branch-free evaluation selection using explicit comparisons
        let is_above_half = (discarded_fraction > tie_boundary) as u128;
        let is_exact_half = (discarded_fraction == tie_boundary) as u128;
        let is_odd = scaled_result & 1;

        let round_up = is_above_half | (is_exact_half & is_odd);
        scaled_result = scaled_result.checked_add(round_up).unwrap();

        // CAPACITY BOUNDARY CHECK
        if scaled_result > i128::MAX as u128 {
            if out_negative && scaled_result == (i128::MIN as u128) {
                return Self(i128::MIN);
            }
            panic!("CRITICAL MATH EXCEPTION: Capacity Bound Overflow");
        }

        let final_signed = i128::try_from(scaled_result).unwrap();
        if out_negative {
            Self(final_signed.checked_neg().unwrap())
        } else {
            Self(final_signed)
        }
    }
}

/// # Low-Level Requirement: LLR-REPLAY-MATH-OPS-004
/// Accumulator-to-integer conversion shall eliminate directional bias by rounding
/// to nearest and breaking exact half-scale ties toward the even integral value.
///
/// **Verification Vector:** `verification::proofs::verify_accumulator_convergent_rounding_exhaustive`
#[inline(always)]
pub fn round_ties_to_even(accum: I64F64) -> i128 {
    const FRACTION_MASK: u128 = 0xFFFF_FFFF_FFFF_FFFF;
    const HALF_SCALE: u128 = 0x8000_0000_0000_0000;

    let bits = accum.to_bits();
    let integral_part = bits >> I64F64::FRAC_BITS;
    let fractional_part = bits as u128 & FRACTION_MASK;

    let is_above_half = (fractional_part > HALF_SCALE) as i128;
    let is_exact_half = (fractional_part == HALF_SCALE) as i128;
    let is_integral_odd = (integral_part & 1) != 0;
    let increment = is_above_half | (is_exact_half & (is_integral_odd as i128));

    match integral_part.checked_add(increment) {
        Some(value) => value,
        None => panic!("CRITICAL MATH EXCEPTION: I64F64 Convergent Rounding Overflow"),
    }
}

impl Mul for I64F64 {
    type Output = Self;

    /// Default multiplication path executing standard Raw Truncation (Integer-Part Extraction).
    /// Tailored for stateless routing, FIR pipelines, and maximum execution cycle efficiency.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.fallible_mul(rhs).unwrap_or_else(panic_arithmetic)
    }
}

/// A transient context that enforces drift-canceled accumulation.
/// Instantiated within long-duration integration loops and stateful tracking states.
pub struct ConvergentAccumulator {
    state: I64F64,
}

impl ConvergentAccumulator {
    #[inline]
    pub const fn new(initial: I64F64) -> Self {
        Self { state: initial }
    }

    /// Multiplies the two inputs using convergent rounding, tracking
    /// the result within the accumulation state safely.
    #[inline]
    pub fn multiply_accumulate(&mut self, lhs: I64F64, rhs: I64F64) {
        let product = lhs.mul_convergent(rhs);
        self.state = match self.state.0.checked_add(product.0) {
            Some(val) => I64F64(val),
            None => panic!("CRITICAL MATH EXCEPTION: I64F64 Addition Overflow"),
        };
    }

    /// Resolves the accumulator context, returning the stable, unified primitive.
    #[inline]
    pub const fn unwrap(self) -> I64F64 {
        self.state
    }
}

impl Add for I64F64 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.fallible_add(rhs).unwrap_or_else(panic_arithmetic)
    }
}

impl Sub for I64F64 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.fallible_sub(rhs).unwrap_or_else(panic_arithmetic)
    }
}

impl Div for I64F64 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.fallible_div(rhs).unwrap_or_else(panic_arithmetic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // REQ TRACE: LLR-REPLAY-MATH-OPS-001 - ADDITION & SUBTRACTION OVERFLOW
    // =========================================================================
    #[test]
    #[should_panic(expected = "CRITICAL MATH EXCEPTION: I64F64 Addition Overflow")]
    fn test_addition_overflow_gate() {
        let max = I64F64::from_bits(i128::MAX);
        let one = I64F64::from_bits(1);
        let _ = max + one;
    }

    #[test]
    fn test_fallible_add_matches_public_addition_when_successful() {
        let lhs = I64F64::from_bits(I64F64::SCALE * 2);
        let rhs = I64F64::from_bits(I64F64::SCALE * 3);

        assert_eq!(lhs.fallible_add(rhs).unwrap(), lhs + rhs);
    }

    #[test]
    #[should_panic(expected = "CRITICAL MATH EXCEPTION: I64F64 Subtraction Overflow")]
    fn test_subtraction_overflow_gate() {
        let min = I64F64::from_bits(i128::MIN);
        let one = I64F64::from_bits(1);
        let _ = min - one;
    }

    #[test]
    fn test_fallible_sub_matches_public_subtraction_when_successful() {
        let lhs = I64F64::from_bits(I64F64::SCALE * 5);
        let rhs = I64F64::from_bits(I64F64::SCALE * 3);

        assert_eq!(lhs.fallible_sub(rhs).unwrap(), lhs - rhs);
    }

    // =========================================================================
    // REQ TRACE: LLR-REPLAY-MATH-OPS-003 - DIVISION BY ZERO & SHIFT OVERFLOW
    // =========================================================================
    #[test]
    #[should_panic(expected = "CRITICAL MATH EXCEPTION: Division By Zero")]
    fn test_div_by_zero_gate() {
        let a = I64F64::from_bits(I64F64::SCALE);
        let zero = I64F64::from_bits(0);
        let _ = a / zero;
    }

    #[test]
    #[should_panic(expected = "CRITICAL MATH EXCEPTION: I64F64 Division Numerator Shift Overflow")]
    fn test_div_numerator_shift_overflow() {
        let huge = I64F64::from_bits(i128::MAX - 1);
        let one = I64F64::from_bits(I64F64::SCALE);
        let _ = huge / one;
    }

    #[test]
    #[should_panic(expected = "CRITICAL MATH EXCEPTION: I64F64 Integer Division Overflow")]
    fn test_div_integer_overflow() {
        let min_shifted_numerator = I64F64::from_bits(i128::MIN >> I64F64::FRAC_BITS);
        let negative_raw_one = I64F64::from_bits(-1);
        let _ = min_shifted_numerator / negative_raw_one;
    }

    #[test]
    fn test_fallible_div_matches_public_division_when_successful() {
        let lhs = I64F64::from_bits(I64F64::SCALE / 4);
        let rhs = I64F64::from_bits(I64F64::SCALE / 2);

        assert_eq!(lhs.fallible_div(rhs).unwrap(), lhs / rhs);
    }

    // =========================================================================
    // REQ TRACE: LLR-REPLAY-MATH-OPS-004 - INTEGER TIES-TO-EVEN ROUNDING
    // =========================================================================
    #[test]
    fn test_round_ties_to_even_positive_values() {
        let one_quarter = I64F64::from_bits(I64F64::SCALE + (I64F64::SCALE >> 2));
        let three_quarters = I64F64::from_bits(I64F64::SCALE + ((I64F64::SCALE >> 2) * 3));
        let one_and_half = I64F64::from_bits(I64F64::SCALE + (I64F64::SCALE >> 1));
        let two_and_half = I64F64::from_bits((I64F64::SCALE * 2) + (I64F64::SCALE >> 1));

        assert_eq!(round_ties_to_even(one_quarter), 1);
        assert_eq!(round_ties_to_even(three_quarters), 2);
        assert_eq!(round_ties_to_even(one_and_half), 2);
        assert_eq!(round_ties_to_even(two_and_half), 2);
    }

    #[test]
    fn test_round_ties_to_even_negative_values() {
        let minus_one_quarter = I64F64::from_bits(-(I64F64::SCALE + (I64F64::SCALE >> 2)));
        let minus_three_quarters = I64F64::from_bits(-(I64F64::SCALE + ((I64F64::SCALE >> 2) * 3)));
        let minus_one_and_half = I64F64::from_bits(-(I64F64::SCALE + (I64F64::SCALE >> 1)));
        let minus_two_and_half = I64F64::from_bits(-((I64F64::SCALE * 2) + (I64F64::SCALE >> 1)));

        assert_eq!(round_ties_to_even(minus_one_quarter), -1);
        assert_eq!(round_ties_to_even(minus_three_quarters), -2);
        assert_eq!(round_ties_to_even(minus_one_and_half), -2);
        assert_eq!(round_ties_to_even(minus_two_and_half), -2);
    }

    // =========================================================================
    // REQ TRACE: LLR-REPLAY-MATH-OPS-002 - RAW MULTIPLICATION TRUNCATION
    // =========================================================================
    #[test]
    fn test_raw_mul_tiny_fractional_products_truncate_to_zero() {
        let tiny_positive = I64F64::from_bits(1);
        let tiny_negative = I64F64::from_bits(-1);
        let zero = I64F64::from_bits(0);

        assert_eq!(tiny_positive * tiny_positive, zero);
        assert_eq!(tiny_negative * tiny_positive, zero);
        assert_eq!(tiny_positive * tiny_negative, zero);
        assert_eq!(tiny_negative * tiny_negative, zero);
    }

    #[test]
    fn test_raw_mul_fixed_point_one_signs() {
        let one = I64F64::from_bits(I64F64::SCALE);
        let negative_one = I64F64::from_bits(-I64F64::SCALE);

        assert_eq!(one * one, one);
        assert_eq!(negative_one * one, negative_one);
        assert_eq!(one * negative_one, negative_one);
        assert_eq!(negative_one * negative_one, one);
    }

    #[test]
    fn test_fallible_mul_matches_public_multiplication_when_successful() {
        let lhs = I64F64::from_bits(I64F64::SCALE * 3);
        let rhs = I64F64::from_bits(I64F64::SCALE * 2);

        assert_eq!(lhs.fallible_mul(rhs).unwrap(), lhs * rhs);
    }

    #[test]
    #[should_panic(expected = "CRITICAL MATH EXCEPTION: Multiplicative Saturation")]
    fn test_raw_mul_multiplicative_saturation_message() {
        let lhs = I64F64::from_bits(i128::MAX);
        let rhs = I64F64::from_bits(i128::MAX);
        let _ = lhs * rhs;
    }

    // =========================================================================
    // REQ TRACE: LLR-REPLAY-MATH-OPS-002 (Step 1) - SIGN ISOLATION & PRIMITIVE BYPASS
    // =========================================================================
    #[cfg(kani)]
    #[kani::proof]
    #[kani::unwind(2)]
    fn verify_sign_isolation_and_bypass() {
        let bit_a: i128 = kani::any();
        let bit_b: i128 = kani::any();

        kani::assume(bit_a > -1_000_000_000 && bit_a < 1_000_000_000);
        kani::assume(bit_b > -1_000_000_000 && bit_b < 1_000_000_000);

        let a = I64F64::from_bits(bit_a);
        let b = I64F64::from_bits(bit_b);

        let expected_negative = (bit_a < 0) ^ (bit_b < 0);
        let result = a * b;

        if result.to_bits() != 0 {
            assert_eq!(result.to_bits() < 0, expected_negative);
        }
    }

    // =========================================================================
    // REQ TRACE: LLR-REPLAY-MATH-OPS-002 (Step 4) - EXPONENTIAL OVERFLOW GATES
    // =========================================================================
    #[cfg(kani)]
    #[kani::proof]
    #[kani::unwind(2)]
    fn verify_exponential_overflow_traps() {
        let bit_a: i128 = kani::any();
        let bit_b: i128 = kani::any();

        kani::assume(bit_a > i128::MAX - 10_000 || bit_a < i128::MIN + 10_000);
        kani::assume(bit_b > i128::MAX - 10_000 || bit_b < i128::MIN + 10_000);

        let a = I64F64::from_bits(bit_a);
        let b = I64F64::from_bits(bit_b);

        assert_eq!(
            a.fallible_mul(b),
            Err(ArithmeticError::MultiplicativeSaturation)
        );
    }

    // =========================================================================
    // REQ TRACE: HLR-MATH-OPS-002 / LLR-REPLAY-MATH-OPS-004 - CONVERGENT TIE-BREAK
    // =========================================================================
    #[cfg(kani)]
    #[kani::proof]
    #[kani::unwind(2)]
    fn verify_convergent_rounding_ties() {
        let bit_a: i128 = kani::any();

        kani::assume(bit_a > -10_000_000 && bit_a < 1_000_000_000);
        kani::assume((bit_a & 1) != 0);
        let a = I64F64::from_bits(bit_a);

        let half = I64F64::from_bits(I64F64::SCALE >> 1);
        let result = a.mul_convergent(half);

        assert_eq!(result.to_bits() & 1, 0);
    }
}
