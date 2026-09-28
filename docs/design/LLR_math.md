# Low-Level Requirements - Fixed-Point Math Core (LLR-MATH)

## 1. Representation Layout (LLR-REPLAY-MATH-REP)

### LLR-REPLAY-MATH-REP-001: Fixed-Point Data Type Definition
The core mathematical unit shall be implemented as a public tuple struct named `I64F64` wrapping a primitive signed 128-bit integer (`i128`).
*Traces to: HLR-MATH-REP-001*

### LLR-REPLAY-MATH-REP-002: Fractional Scaling Constant
The fractional scaling factor shall be defined as a compile-time constant equal to $2^{64}$. The fixed-point representation value is calculated as $\text{Internal Value} = \text{Real Number} \times 2^{64}$.
*Traces to: HLR-MATH-REP-001*

## 2. Operational Invariants (LLR-REPLAY-MATH-OPS)

### LLR-REPLAY-MATH-OPS-001: Addition & Subtraction Mechanics
Addition and subtraction operations shall be executed using the native wrapping primitives of the underlying `i128` type. The implementation must check for arithmetic overflow/underflow using standard checked boundaries. If an overflow or underflow condition occurs, execution shall abort immediately.
*Traces to: HLR-MATH-OPS-001*

### LLR-REPLAY-MATH-OPS-002: Multiplication Scaling and Widening
Multiplication of two `I64F64` values ($A \times B$) must execute via the following deterministic sequence:
1. Isolate the output sign from the operand signs and convert each operand to an unsigned absolute magnitude before partial-product generation. Since native `i256` is not available as a standard primitive, intermediate multiplication must be represented through checked limb decomposition.
2. Decompose the absolute magnitudes into 64-bit limbs, multiply the limbs, compose the scaled absolute product, and discard the low 64 fractional bits to restore the fixed-point alignment.
3. Reapply the isolated sign after magnitude scaling. This raw multiplication path truncates toward zero for negative products with discarded fractional magnitude.
4. If the partial-product composition proves that the scaled absolute result cannot fit within the signed 128-bit output range after fixed-point realignment, the operation shall trigger an immediate panic abort.
*Traces to: HLR-MATH-OPS-001, HLR-MATH-OPS-002*

### LLR-REPLAY-MATH-OPS-003: Division Scaling and Guardrails
Division of two `I64F64` values ($A \div B$) must execute via the following deterministic sequence:
1. The denominator $B$ must be checked against zero. If $B == 0$, execution shall abort immediately.
2. The numerator $A$ must be arithmetically left-shifted by 64 bits before the division occurs to preserve the fractional resolution of the quotient.
3. The shift operation must be guarded using the signed numerator boundary: if $A \ge 2^{64}$ or $A \le -2^{64}$, the numerator is outside the permitted shift range and the operation must return a fallible `ArithmeticError` or abort before performing the left shift, preventing signed integer overflow.
4. The division must use checked integer division primitives. Any division overflow (e.g., `MIN_VALUE / -1`) shall trigger an immediate panic abort.
*Traces to: HLR-MATH-OPS-001, HLR-MATH-OPS-003*

### LLR-REPLAY-MATH-OPS-004: Convergent Integer Rounding
Accumulator-to-integer conversion shall eliminate directional bias by rounding to nearest and breaking exact half-scale ties toward the even integral value.
*Traces to: HLR-MATH-REP-002*

## 3. CORDIC Trigonometric Requirements (LLR-REPLAY-MATH-CORDIC)

### LLR-REPLAY-MATH-CORDIC-001: Quadrant Reduction
CORDIC trigonometric inputs shall be reduced and mapped into Quadrant I before iteration. After the iteration completes, the implementation shall restore the result to the full $[-\pi, \pi]$ range using the recorded quadrant mapping.
*Traces to: HLR-MATH-CORDIC-001*

### LLR-REPLAY-MATH-CORDIC-002: Fixed Iteration Depth and Lookup Table
CORDIC iteration shall execute exactly 64 iterations in Q64.64 fixed-point representation using a static lookup table of arctangent angles. The implementation shall not perform floating-point conversions.
*Traces to: HLR-MATH-CORDIC-002*

### LLR-REPLAY-MATH-CORDIC-003: Vectoring Gain Scaling
After the CORDIC loop, vectoring results shall be scaled using fixed-point arithmetic by the reciprocal vectoring gain $K_n^{-1} \approx 0.6072529350088812561694$.
*Traces to: HLR-MATH-CORDIC-003*

## 4. S3 Unit Quaternion & Hopf Geometry Requirements (LLR-REPLAY-MATH-S3)

### LLR-REPLAY-MATH-S3-001: Unit Quaternion Memory Layout
A unit quaternion shall be represented as a 64-byte struct containing four `I64F64` values and annotated with `#[repr(C)]` to guarantee its memory layout.
*Traces to: HLR-MATH-S3-001*

### LLR-REPLAY-MATH-S3-002: Deterministic Hopf Projection
The Hopf projection ($\eta: S^3 \to (S^2, S^1)$) shall deterministically map each $S^3$ element to $S^2$ base and $S^1$ fiber coordinates using fixed-point multiplication and CORDIC `atan2`, without floating-point operations.
*Traces to: HLR-MATH-S3-002*