# Software Verification Plan & Procedures - Math Core (SVCP-MATH)

## 1. Evaluation Methodology (SVCP-MATH-MET)

### SVCP-MATH-MET-001: Formal Symbolic Verification
Mathematical correctness and safety invariants for the `I64F64` arithmetic primitives shall be verified via formal symbolic execution using the Kani Rust Verifier. Standard dynamic testing is deferred in favor of exhaustive property proofs covering the entire symbolic input space.

### SVCP-MATH-MET-002: Test Independence Override
As independent human peer review is explicitly deferred under project criteria, verification soundness is achieved through mathematical proof generation. Proof definitions must strictly test for the absence of undefined behavior, runtime panics, and un-trapped arithmetic clipping.

## 2. Verification Primitives and Proof Bounds (SVCP-MATH-PRO)

### SVCP-MATH-PRO-001: Addition & Subtraction Soundness Proof
The verification harness shall prove that for any two symbolic `i128` values mapped to `I64F64` structures ($A$ and $B$), the addition and subtraction operations satisfy the following criteria:
1. If the sum ($A + B$) or difference ($A - B$) falls outside the legal bounds of a signed 128-bit integer, the operation must trigger a controlled runtime abort.
2. If the operation does not overflow, it must return the exact bitwise arithmetic result.
*Traces to: LLR-REPLAY-MATH-OPS-001*

### SVCP-MATH-PRO-002: Multiplication Bounded Proof
The verification harness shall check the multiplication kernel across the complete symbolic space of two `I64F64` values. The proof must guarantee:
1. Complete intermediate overflow tracking before truncation.
2. That any product exceeding the max/min bounds of `I64F64` forces a panic abort.
3. Success states match the canonical mathematical expectation: $\text{Result} = (A \times B) \gg 64$.
*Traces to: LLR-REPLAY-MATH-OPS-002*

### SVCP-MATH-PRO-003: Division Invariant Proof
The verification harness shall perform symbolic execution on the division kernel to prove:
1. Zero Denominator Trap: If the symbolic denominator $B == 0$, the operation aborts under all conditions.
2. Shifting Overflow Protection: If left-shifting the numerator $A$ by 64 bits overflows the signed 128-bit space, the precondition must trap the failure and abort execution before the division step occurs.
*Traces to: LLR-REPLAY-MATH-OPS-003*

### SVCP-MATH-PRO-004: Convergent Integer Rounding Proof
The verification harness in `verification/src/lib.rs` shall prove that accumulator-to-integer conversion rounds to the nearest integral value and resolves exact half-scale ties toward an even integral result.
*Traces to: LLR-REPLAY-MATH-OPS-004*