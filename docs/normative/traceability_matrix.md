# Requirements Traceability Matrix

This document records the current traceability links and verification status between active behavioral and system requirements and their implementation blocks. It distinguishes active proofs, implementation-local tests, runtime traceability, and pending proof obligations rather than asserting that every traceability row is fully verified.

---

## 1. Fixed-Point Math

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
| `impl Mul for I64F64 { ... }` | **HLR-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-002** | Maps to raw sign-isolated magnitude scaling: the absolute product magnitude is shifted/truncated by 64 bits, then the isolated sign is reapplied. `SVCP-MATH-PRO-002a` and `SVCP-MATH-PRO-002b` provide active partial proof coverage; `SVCP-MATH-PRO-002c` is active only for bounded fixed non-unit high-limb single-cross-term LH and HL correspondence, bounded two-term cross-sum composition, bounded low-limb carry contribution, bounded integrated non-overflowing matrix composition, bounded symbolic high-limb non-overflowing matrix composition, bounded high-high overflow-gate trap observation, bounded final signed-capacity overflow trap observation, public-operand cross-sum overflow unreachability for raw operands reachable through the public `I64F64` representation, signed minimum-capacity boundary allowance over the public raw multiplication path, and negative signed-capacity exceedance trap observation over the public raw multiplication path. Full multiplication closure, full unbounded symbolic limb-matrix correspondence, private/helper-state limb combinations not reachable from public raw operands, full overflow-gate correspondence, and full overflow/trap proof coverage remain pending. |
| `pub fn mul_convergent(...)` | **HLR-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-004** | Provides the drift-canceled multiplication path and applies convergent rounding behavior to discarded fractional state. |
| `if final_abs_bits > i128::MAX` | **LLR-REPLAY-MATH-OPS-002** | Maps to Step 4 capacity boundary gating, protecting signed limits while allowing the unique `i128::MIN` absolute match if `out_negative` is true. |
| `pub fn round_ties_to_even(...)` | **HLR-MATH-REP-002** / **LLR-REPLAY-MATH-OPS-004** | Implements convergent accumulator-to-integer rounding with exact half-scale ties resolved toward the even integral value. |
| `impl Add for I64F64`<br>`impl Sub for I64F64` | **HLR-MATH-OPS-001** / **LLR-REPLAY-MATH-OPS-001** | Employs `checked_add` and `checked_sub` to detect additions and subtractions crossing signed boundaries. `SVCP-MATH-PRO-001` provides active Kani coverage for non-overflowing add/sub exactness, addition overflow trap observation, and subtraction overflow trap observation. |
| `impl Div for I64F64` | **HLR-MATH-OPS-003** / **LLR-REPLAY-MATH-OPS-003** | Standardizes the fixed-point division protocol via a 64-bit numerator pre-shift with explicit leading zero/one safety bounds. `SVCP-MATH-PRO-003a` and `SVCP-MATH-PRO-003b` provide active Kani guard proof coverage for divide-by-zero and numerator shift-overflow trapping; `SVCP-MATH-PRO-003c` is active for bounded non-trapping arithmetic correspondence with symbolic `i32` raw numerators and signed power-of-two denominator family `{-8, -4, -2, -1, 1, 2, 4, 8}`. Arbitrary-denominator and full unbounded symbolic division arithmetic remain pending, and implementation-local division tests remain regression support rather than Kani proof coverage. |

---

## 2. Target Witness Runtime

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| `bsp/stm32` target feature selection for `stm32f446` | **HLR-TARGET-IO-001** | Selects the STM32F446 target path that owns the retained replay witness output surface. |
| `target::init_stlink_vcp_usart2()` | **HLR-TARGET-IO-001** / **LLR-TARGET-IO-001** | Enables GPIOA and USART2 clocks, configures PA2 for USART2 alternate-function TX, sets the retained 16 MHz / 115200 baud divisor, and enables USART2 transmit operation for ST-LINK VCP witness output. |
| `target::write_stlink_vcp_usart2(...)`<br>`write_usart2_byte(...)` | **HLR-TARGET-IO-001** / **LLR-TARGET-IO-001** | Emits bounded witness byte slices by waiting for USART2 transmit readiness before writing each byte to the data register. |
| `firmware_main()` in `runners/stm32-runner` | **HLR-TARGET-IO-001** / **LLR-RUNNER-WITNESS-001** | Initializes the STM32F446 target witness output path, allows a short post-init settle interval, then emits the retained replay result line. |
| `emit_replay_result_line()` in `runners/stm32-runner` | **HLR-TARGET-IO-001** / **LLR-RUNNER-WITNESS-001** | Produces the retained `v0.1.0-rc1` math-add witness payload over the initialized target serial path. |

---

## 3. Sensor Witness

| Witness Surface / Requirement Boundary | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| STM32F446 PA0 / ADC1_IN0 raw witness input path | **HLR-WITNESS-ADC** / **LLR-WITNESS-ADC** | requirements-defined; implementation-deferred; verification-deferred. This row defines the raw ADC witness input requirement surface and does not imply that the ADC witness lane already exists. |
| USART2 ST-LINK VCP raw witness stream | **HLR-WITNESS-ADC** / **LLR-WITNESS-UART** | requirements-defined; implementation-deferred; verification-deferred. This row defines the raw witness stream boundary and does not expand existing retained replay witness output claims. |
| Best-effort polling UART timing boundary | **HLR-WITNESS-TIME** / **LLR-WITNESS-TIME** | requirements-defined; implementation-deferred; verification-deferred. The active timing claim is `timing_claim=best_effort_polling_uart_stream` and excludes fixed-rate sampling, precise event timing, transient fidelity, replay alignment, timer-paced ADC, DMA buffering, interrupt-driven capture, and final timing authority. |
| Linux host witness capture tool | **HLR-WITNESS-HOST** / **LLR-WITNESS-HOST** | requirements-defined; implementation-deferred; verification-deferred. This row defines the host capture requirement for raw sample-indexed witness records. |
| Declared external stimulus boundary | **HLR-WITNESS-STIM** / **LLR-WITNESS-STIM** | requirements-defined; implementation-deferred; verification-deferred. Declared external stimulus requirements are deferred beyond the initial raw ADC witness implementation. |
| Bounded response-envelope boundary | **HLR-WITNESS-ENV** / **LLR-WITNESS-ENV** | requirements-defined; implementation-deferred; verification-deferred. Bounded response-envelope evaluation requirements are deferred beyond the initial raw ADC witness implementation. |

---

## 4. Verification Alignment

Rows in this matrix are requirement traceability entries with explicit verification status.

Verification status sources include:

- active symbolic proofs in `verification/src/lib.rs`
- implementation-local tests in `core/src/math.rs`
- first-class pending proof obligations tracked by the SVCP
- implementation-deferred requirement rows

STM32 target witness rows provide behavioral traceability for retained replay observation runtime code and do not expand release evidence, hardware qualification, tool qualification, certification, proof, or CI claims.

Sensor Witness rows define the future raw ADC witness capture requirement surface and remain implementation-deferred and verification-deferred until the initial raw ADC witness implementation or later work supplies the corresponding implementation and evidence.

### Addition and Subtraction

`SVCP-MATH-PRO-001` is active for non-overflowing exactness plus overflow trap observation when `checked_add` or `checked_sub` return `None`.

It does not claim panic message matching.

### Multiplication

`SVCP-MATH-PRO-002a` is active only for tiny fractional truncation-to-zero behavior.

`SVCP-MATH-PRO-002b` is active only for bounded `u32` fractional raw operands plus the exact whole-unit raw endpoint.

`SVCP-MATH-PRO-002c` is active only for:

- bounded fixed non-unit high-limb single-cross-term LH and HL correspondence
- bounded two-term cross-sum composition
- bounded low-limb carry contribution
- bounded integrated non-overflowing matrix composition
- bounded symbolic high-limb non-overflowing matrix composition
- bounded high-high overflow-gate trap observation
- bounded final signed-capacity overflow trap observation
- public-operand cross-sum overflow unreachability for raw operands reachable through the public `I64F64` representation
- signed minimum-capacity boundary allowance over the public raw multiplication path
- negative signed-capacity exceedance trap observation over the public raw multiplication path

The following multiplication surfaces remain pending:

- full multiplication closure
- full unbounded symbolic limb-matrix correspondence
- private/helper-state limb combinations not reachable from public raw operands
- full overflow-gate correspondence
- full overflow/trap proof coverage

### Division

`SVCP-MATH-PRO-003a` and `SVCP-MATH-PRO-003b` are active only for the bounded Kani guard slice covering divide-by-zero and numerator shift-overflow traps mapped to `LLR-REPLAY-MATH-OPS-003`.

`SVCP-MATH-PRO-003c` is active for bounded non-trapping arithmetic correspondence with symbolic `i32` raw numerators and signed power-of-two denominator family:

```text
{-8, -4, -2, -1, 1, 2, 4, 8}
```

The following division surfaces remain pending:

- arbitrary-denominator arithmetic
- full unbounded symbolic division arithmetic

Implementation-local division tests do not expand that proof scope.
