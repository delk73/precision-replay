use core::ops::{Add, Sub, Mul, Div};

/// A deterministic, platform-agnostic 128-bit fixed-point number
/// with a signed 64-bit integer part and a 64-bit fractional part.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct I64F64(pub i128);

impl I64F64 {
    pub const BITS: u32 = 128;
    pub const FRAC_BITS: u32 = 64;
    pub const SCALE: i128 = 1 << Self::FRAC_BITS;

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
    fn execute_mul_matrix(self, rhs: Self) -> (bool, u128, u128, u128, u128) {
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
        let ll = a_lo * b_lo;
        let lh = a_lo * b_hi;
        let hl = a_hi * b_lo;
        let hh = a_hi * b_hi;

        // 5. CROSS TERM ACCUMULATION
        let cross_sum = match lh.checked_add(hl) {
            Some(val) => val,
            None => panic!("CRITICAL MATH EXCEPTION: Cross Term Overflow"),
        };

        let cross_lo = cross_sum << 64;
        let cross_hi = cross_sum >> 64;

        (out_negative, ll, cross_lo, cross_hi, hh)
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
        let tie_boundary = 0x8000000000000000u64;
        
        let mut scaled_result = cross_lo.checked_add(ll >> 64).unwrap();

        // Branch-free evaluation selection masks
        let is_above_half = ((tie_boundary as i64 - discarded_fraction as i64) >> 63) as u128;
        let is_exact_half = (((discarded_fraction ^ tie_boundary) as i128).wrapping_sub(1) >> 127) as u128;
        let is_odd = (scaled_result & 1) as u128;

        let round_up = is_above_half | (is_exact_half & is_odd);
        scaled_result = scaled_result.checked_add(round_up).unwrap();

        // CAPACITY BOUNDARY CHECK
        if scaled_result > i128::MAX as u128 {
            if out_negative && scaled_result == (i128::MIN as u128) {
                return Self(i128::MIN);
            }
            panic!("CRITICAL MATH EXCEPTION: Capacity Bound Overflow");
        }

        let final_signed = scaled_result as i128;
        if out_negative { Self(-final_signed) } else { Self(final_signed) }
    }
}

impl Mul for I64F64 {
    type Output = Self;

    /// Default multiplication path executing standard Raw Truncation (Integer-Part Extraction).
    /// Tailored for stateless routing, FIR pipelines, and maximum execution cycle efficiency.
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        if self.0 == 0 || rhs.0 == 0 {
            return Self(0);
        }

        let (out_negative, ll, cross_lo, cross_hi, hh) = self.execute_mul_matrix(rhs);

        // EXPONENTIAL OVERFLOW TRAP
        if cross_hi != 0 || hh != 0 {
            panic!("CRITICAL MATH EXCEPTION: Multiplicative Saturation");
        }

        // RAW TRUNCATION ALIGNMENT SHIFT
        let ll_scaled = ll >> 64;
        let final_abs_bits = match cross_lo.checked_add(ll_scaled) {
            Some(val) => val,
            None => panic!("CRITICAL MATH EXCEPTION: Bit Pool Composition Failure"),
        };

        // CAPACITY BOUNDARY CHECK
        if final_abs_bits > i128::MAX as u128 {
            if out_negative && final_abs_bits == (i128::MIN as u128) {
                return Self(i128::MIN);
            }
            panic!("CRITICAL MATH EXCEPTION: Capacity Bound Overflow");
        }

        let final_signed = final_abs_bits as i128;
        if out_negative { Self(-final_signed) } else { Self(final_signed) }
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
        self.state = self.state + product;
    }

    /// Resolves the accumulator context, returning the stable, unified primitive.
    #[inline]
    pub const fn unwrap(self) -> I64F64 {
        self.state
    }
}

// Standard structural implementations for Add, Sub, and Div remain unchanged...
impl Add for I64F64 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        match self.0.checked_add(rhs.0) {
            Some(val) => Self(val),
            None => panic!("CRITICAL MATH EXCEPTION: I64F64 Addition Overflow"),
        }
    }
}

impl Sub for I64F64 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        match self.0.checked_sub(rhs.0) {
            Some(val) => Self(val),
            None => panic!("CRITICAL MATH EXCEPTION: I64F64 Subtraction Overflow"),
        }
    }
}

impl Div for I64F64 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 { panic!("CRITICAL MATH EXCEPTION: Division By Zero"); }
        let leading_zeros = self.0.leading_zeros();
        let leading_ones = self.0.leading_ones();
        if (self.0 > 0 && leading_zeros < 64) || (self.0 < 0 && leading_ones < 64) {
            panic!("CRITICAL MATH EXCEPTION: I64F64 Division Numerator Shift Overflow");
        }
        let shifted_numerator = self.0 << Self::FRAC_BITS;
        match shifted_numerator.checked_div(rhs.0) {
            Some(val) => Self(val),
            None => panic!("CRITICAL MATH EXCEPTION: I64F64 Integer Division Overflow"),
        }
    }
}