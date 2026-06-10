# Low-Level Requirements - Fixed-Point Math Core (LLR-MATH)

## 1. Structural Layout (LLR-MATH-SXL)

### LLR-MATH-SXL-001: Fixed-Point Data Type Definition
The core mathematical unit shall be implemented as a public tuple struct named `I64F64` wrapping a primitive signed 128-bit integer (`i128`).
*Traces to: HLR-MATH-REP-001*

### LLR-MATH-SXL-002: Fractional Scaling Constant
The fractional scaling factor shall be defined as a compile-time constant equal to $2^{64}$. The fixed-point representation value is calculated as $\text{Internal Value} = \text{Real Number} \times 2^{64}$.
*Traces to: HLR-MATH-REP-001*

## 2. Operational Invariants (LLR-MATH-OPS)

### LLR-MATH-OPS-001: Addition & Subtraction Mechanics
Addition and subtraction operations shall be executed using the native wrapping primitives of the underlying `i128` type. The implementation must check for arithmetic overflow/underflow using standard checked boundaries. If an overflow or underflow condition occurs, execution shall abort immediately.
*Traces to: HLR-MATH-OPS-001*

### LLR-MATH-OPS-002: Multiplication Scaling and Widening
Multiplication of two `I64F64` values ($A \times B$) must execute via the following deterministic sequence:
1. Widen the operands or calculate intermediate signs to ensure multiplication does not clip. Since native `i256` is not available as a standard primitive, intermediate calculation overflow boundaries must be checked prior to bit-shifting.
2. The raw product of the two `i128` inner values must be arithmetically right-shifted by 64 bits to restore the fractional alignment.
3. If the intermediate product cannot fit within the boundaries of a signed 128-bit integer before or after shifting, the operation shall trigger an immediate panic abort.
*Traces to: HLR-MATH-OPS-001, HLR-MATH-OPS-002*

### LLR-MATH-OPS-003: Division Scaling and Guardrails
Division of two `I64F64` values ($A \div B$) must execute via the following deterministic sequence:
1. The denominator $B$ must be checked against zero. If $B == 0$, execution shall abort immediately.
2. The numerator $A$ must be arithmetically left-shifted by 64 bits before the division occurs to preserve the fractional resolution of the quotient.
3. The shift operation must be guarded; if left-shifting $A$ would overflow a signed 128-bit boundary, the operation must abort.
4. The division must use checked integer division primitives. Any division overflow (e.g., `MIN_VALUE / -1`) shall trigger an immediate panic abort.
*Traces to: HLR-MATH-OPS-001, HLR-MATH-OPS-003*