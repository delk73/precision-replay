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
3. The shift operation must be guarded; if left-shifting $A$ would overflow a signed 128-bit boundary, the operation must abort.
4. The division must use checked integer division primitives. Any division overflow (e.g., `MIN_VALUE / -1`) shall trigger an immediate panic abort.
*Traces to: HLR-MATH-OPS-001, HLR-MATH-OPS-003*

### LLR-REPLAY-MATH-OPS-004: Convergent Integer Rounding
Accumulator-to-integer conversion shall eliminate directional bias by rounding to nearest and breaking exact half-scale ties toward the even integral value.
*Traces to: HLR-MATH-REP-002*