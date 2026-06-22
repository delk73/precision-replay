# Requirements Traceability Matrix — Fixed-Point Math Subsystem

This document records the current traceability links and verification status between the high-level and low-level mathematical requirements and their implementation blocks within the core mathematical kernel. It distinguishes active proofs, implementation-local tests, and pending proof obligations rather than asserting that every traceability row is fully verified.

---

## 1. Traceability Mapping Matrix

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| `pub struct I64F64(pub i128);` | **HLR-MATH-REP-001** / **LLR-REPLAY-MATH-REP-001** | Maps the fixed-point storage structure to a single primitive `i128` containing a 64-bit integer part and a 64-bit fractional part. |
| `pub const FRAC_BITS: u32 = 64;`<br>`pub const SCALE: i128 = 1 << Self::FRAC_BITS;` | **HLR-MATH-REP-001** / **LLR-REPLAY-MATH-REP-002** | Defines the fractional scaling constant as $2^{64}$ and fixes the internal representation scale used by `I64F64`. |
| `#[repr(transparent)]`<br>`pub struct I64F64(pub i128);` | **HLR-MATH-REP-003** / **LLR-REPLAY-MATH-REP-001** | Establishes the current binary interoperability surface as a transparent single-field wrapper over `i128`. |
| `let out_negative = (a < 0) ^ (b < 0);` | **LLR-REPLAY-MATH-OPS-002** | Maps to Step 1 sign isolation: determine the result sign via XOR of the operand sign bits. |
| `let mask_a = (a >> 127) as u128;`<br>`let abs_a = (unsigned_a ^ mask_a)...` | **LLR-REPLAY-MATH-OPS-002** | Maps to Step 1 primitive type bypass casting, converting signed fields to absolute values branch-free and neutralizing `i128::MIN` conversion panics. |
| `let a_hi = abs_a >> 64;`<br>`let a_lo = abs_a & ...` | **HLR-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-002** | Maps to Step 2 decomposition, breaking 128-bit absolute magnitudes into 64-bit subfields to maintain the complete 256-bit intermediate cross-product precision space. |
| `let ll = a_lo * b_lo;`<br>`let lh = a_lo * b_hi; ...` | **LLR-REPLAY-MATH-OPS-002** | Maps to Step 2 partial-product generation for the complete multiplication matrix (`ll`, `lh`, `hl`, `hh`). |
| `if hh > 0xFFFF_FFFF_FFFF_FFFF`<br>`.checked_add(cross_sum)` | **LLR-REPLAY-MATH-OPS-002** | Maps to Step 4 overflow gating for the raw scaled magnitude: upper `hh` bits that cannot fit after the 64-bit truncation shift panic, and checked composition traps bit-pool overflow. |
| `impl Mul for I64F64 { ... }` | **HLR-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-002** | Maps to raw sign-isolated magnitude scaling: the absolute product magnitude is shifted/truncated by 64 bits, then the isolated sign is reapplied. `SVCP-MATH-PRO-002a` and `SVCP-MATH-PRO-002b` provide active partial proof coverage; `SVCP-MATH-PRO-002c` remains pending. |
| `pub fn mul_convergent(...)` | **HLR-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-004** | Provides the drift-canceled multiplication path and applies convergent rounding behavior to discarded fractional state. |
| `if final_abs_bits > i128::MAX` | **LLR-REPLAY-MATH-OPS-002** | Maps to Step 4 capacity boundary gating, protecting signed limits while allowing the unique `i128::MIN` absolute match if `out_negative` is true. |
| `pub fn round_ties_to_even(...)` | **HLR-MATH-REP-002** / **LLR-REPLAY-MATH-OPS-004** | Implements convergent accumulator-to-integer rounding with exact half-scale ties resolved toward the even integral value. |
| `impl Add for I64F64`<br>`impl Sub for I64F64` | **HLR-MATH-OPS-001** / **LLR-REPLAY-MATH-OPS-001** | Employs `checked_add` and `checked_sub` loops to detect and isolate additions and subtractions crossing signed boundaries. |
| `impl Div for I64F64` | **HLR-MATH-OPS-003** / **LLR-REPLAY-MATH-OPS-003** | Standardizes the fixed-point division protocol via a 64-bit numerator pre-shift with explicit leading zero/one safety bounds. `SVCP-MATH-PRO-003a` and `SVCP-MATH-PRO-003b` provide active Kani guard proof coverage for divide-by-zero and numerator shift-overflow trapping; full non-trapping division arithmetic remains pending under `SVCP-MATH-PRO-003c`. Implementation-local division tests remain regression support rather than Kani proof coverage. |

---

## 2. MVP Readiness Requirements

These rows define MVP readiness requirements for release evidence packaging, one retained hardware-backed replay validation artifact, and final MVP boundary/readiness status. They are documentation-level readiness requirements and do not claim DO-178C, DAL A, tool qualification, hardware qualification, or certification compliance.

`engineering_contract.md` remains the workflow and evidence-boundary authority. These rows define the listed items as MVP readiness requirements, but they do not implement release evidence package generation, bundle validation, hardware capture, generated traceability, certification evidence, broader Kani release/proof authority, or ordinary local merge preconditions.

| MVP Readiness Surface | Requirement ID | Traceability / Status |
| :--- | :--- | :--- |
| Retained release evidence package | **HLR-MVP-EVD-001** / **LLR-REPLAY-MVP-EVD-001** / **LLR-REPLAY-MVP-EVD-002** | MVP readiness requirement. Defines retained validation outputs, applicable proof results, provenance, requirement/verification status references, manifest contents, and bundle validation expectations; package generation and bundle validator implementation remain outside this change. |
| One retained hardware-backed replay validation artifact | **HLR-MVP-HW-001** / **LLR-REPLAY-MVP-HW-001** | MVP readiness requirement. Defines one retained deterministic replay artifact with embedded target execution, host/reference comparison, retained artifact path, and pass/fail verdict; generalized hardware replay coverage and local merge gating remain outside this row. |
| MVP boundary and readiness statement | **HLR-MVP-BND-001** / **LLR-REPLAY-MVP-BND-001** | MVP readiness requirement. Defines explicit active-covered, bounded, deferred, and non-certified surfaces, including the statement that certification compliance is not claimed. |

---

## 3. Verification Alignment

Rows in this matrix are requirement traceability entries with an explicit verification status. Some rows correspond to active symbolic proofs in `verification/src/lib.rs` or implementation-local tests in `core/src/math.rs`; other rows are first-class pending proof obligations tracked by the SVCP. For raw multiplication, `SVCP-MATH-PRO-002a` is active only for tiny fractional truncation-to-zero behavior, and `SVCP-MATH-PRO-002b` is active only for bounded `u32` fractional raw operands plus the exact whole-unit raw endpoint. Full limb/cross-term correspondence and overflow-gate proof coverage remain pending under `SVCP-MATH-PRO-002c`. For division, `SVCP-MATH-PRO-003a` and `SVCP-MATH-PRO-003b` are active only for the bounded Kani guard slice covering divide-by-zero and numerator shift-overflow traps mapped to `LLR-REPLAY-MATH-OPS-003`; full symbolic division arithmetic remains pending under `SVCP-MATH-PRO-003c`, and implementation-local division tests do not expand that proof scope.
