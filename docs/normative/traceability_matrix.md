# Requirements Traceability Matrix — Fixed-Point Math Subsystem

This document establishes the verified traceability link between the high-level and low-level mathematical requirements and their implementation blocks within the core mathematical kernel.

---

## 1. Traceability Mapping Matrix

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| `pub struct I64F64(pub i128);` | **HLR-MATH-REP-001** / **LLR-MATH-REP-001** | Maps the fixed-point storage structure to a single primitive `i128` containing a 64-bit integer part and a 64-bit fractional part. |
| `let out_negative = (a < 0) ^ (b < 0);` | **LLR-MATH-OPS-002 (Step 1)** | Maps to the sign-isolation pass: "Determine the sign of the result via XOR of the sign bits of the operands, tracking this context." |
| `let mask_a = (a >> 127) as u128;`<br>`let abs_a = (unsigned_a ^ mask_a)...` | **LLR-MATH-OPS-002 (Step 1)** | Implements the primitive type bypass casting to convert signed fields to absolute values branch-free, neutralizing `i128::MIN` conversion panics. |
| `let a_hi = abs_a >> 64;`<br>`let a_lo = abs_a & ...` | **HLR-MATH-OPS-002** / **LLR-MATH-OPS-002 (Step 2)** | Breaks the 128-bit absolute magnitudes down into 64-bit subfields to maintain the complete 256-bit intermediate cross-product precision space. |
| `let ll = a_lo * b_lo;`<br>`let lh = a_lo * b_hi; ...` | **LLR-MATH-OPS-002 (Step 2)** | Computes the four 128-bit partial products (`ll`, `lh`, `hl`, `hh`) representing the complete multiplication matrix. |
| `if cross_hi != 0 \|\| hh != 0` | **LLR-MATH-OPS-002 (Step 3)** | Maps to the exponential overflow gate: "If the upper blocks contain any bits, a saturation exception is guaranteed and must panic." |
| `impl Mul for I64F64 { ... }` | **HLR-MATH-OPS-002** / **LLR-MATH-OPS-002 (Step 2)** | Implements raw truncation via arithmetic right-shifting by 64 bits (`ll >> 64`) to restore the scaling alignment default. |
| `pub fn mul_convergent(...)` | **HLR-MATH-OPS-002** | Provides the drift-canceled path utilizing branch-free masks to evaluate tie-breaking conditions on the discarded fraction (`ll & 0xFFFFFFFFFFFFFFFF`). |
| `if final_abs_bits > i128::MAX` | **LLR-MATH-OPS-002 (Step 3)** | Evaluates the capacity boundary gate, protecting signed limits while allowing the unique `i128::MIN` absolute match if `out_negative` is true. |
| `pub struct ConvergentAccumulator` | **HLR-MATH-REP-002** | Implements the isolated contextual wrapper required to guarantee cross-platform tracking parity in long-duration integration loops. |
| `impl Add for I64F64`<br>`impl Sub for I64F64` | **HLR-MATH-OPS-001** / **LLR-MATH-OPS-001** | Employs `checked_add` and `checked_sub` loops to detect and isolate additions and subtractions crossing signed boundaries. |
| `impl Div for I64F64` | **HLR-MATH-OPS-003** / **LLR-MATH-OPS-003** | Standardizes the fixed-point division protocol via a 64-bit numerator pre-shift with explicit leading zero/one safety bounds. |

---

## 2. Verification Alignment

Every row specified in this matrix corresponds to a targeted symbolic verification block inside the inline test module wrapper (`core/src/math.rs`). This structure ensures that any compilation run executing `cargo kani` validates the specific code constraints and their underlying documentation assertions concurrently.