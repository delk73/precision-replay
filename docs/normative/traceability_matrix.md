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

## 2. Replay Execution

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| `ReplayFrame` in `core/src/replay.rs` | **HLR-REPLAY-EXEC-001** / **HLR-REPLAY-EXEC-002** / **LLR-REPLAY-EXEC-001** | Defines the public in-memory frame surface for loading operands, executing add/sub/mul/div math operations, and expecting result bits. |
| `ReplayExecutionState`, `ReplayRejectionReason`, and `ReplayExecutionResult` in `core/src/replay.rs` | **HLR-REPLAY-EXEC-003** / **HLR-REPLAY-EXEC-004** / **HLR-REPLAY-EXEC-005** / **LLR-REPLAY-EXEC-002** / **LLR-REPLAY-EXEC-003** | Exposes deterministic execution state, optional result bits, and rejection reasons for invalid order and expected-result mismatch. |
| `execute_replay(...)` in `core/src/replay.rs` | **HLR-REPLAY-EXEC-001** / **HLR-REPLAY-EXEC-002** / **HLR-REPLAY-EXEC-003** / **HLR-REPLAY-EXEC-004** / **HLR-REPLAY-EXEC-005** / **LLR-REPLAY-EXEC-004** / **LLR-REPLAY-EXEC-005** | Executes a pure in-memory replay frame slice using existing `I64F64` add/sub/mul/div behavior, rejects invalid frame ordering and result mismatches, and returns a deterministic result. |
| `core/src/replay_tests.rs` | **HLR-REPLAY-EXEC-001** / **HLR-REPLAY-EXEC-002** / **HLR-REPLAY-EXEC-003** / **HLR-REPLAY-EXEC-004** / **HLR-REPLAY-EXEC-005** / **LLR-REPLAY-EXEC-001** / **LLR-REPLAY-EXEC-002** / **LLR-REPLAY-EXEC-003** / **LLR-REPLAY-EXEC-004** / **LLR-REPLAY-EXEC-005** | Covers valid add/sub/mul/nonzero-div acceptance, math operation before operand rejection, expected result before execution rejection, expected-result mismatch rejection, frame-after-acceptance rejection, empty frame-slice state, and repeatability for identical frame slices. |
| `ReplayInputVersion`, `ReplayInputSchema`, `ParsedReplayInput`, and `ReplayParseError` in `core/src/replay.rs` | **HLR-REPLAY-PARSE-001** / **HLR-REPLAY-PARSE-002** / **HLR-REPLAY-PARSE-003** / **HLR-REPLAY-PARSE-004** / **HLR-REPLAY-PARSE-005** / **LLR-REPLAY-PARSE-001** / **LLR-REPLAY-PARSE-003** | Defines the accepted saved-input version/schema identity and the strict parser rejection surface. |
| `parse_replay_input(...)` in `core/src/replay.rs` | **HLR-REPLAY-PARSE-001** / **HLR-REPLAY-PARSE-002** / **HLR-REPLAY-PARSE-003** / **HLR-REPLAY-PARSE-004** / **HLR-REPLAY-PARSE-005** / **HLR-REPLAY-PARSE-006** / **LLR-REPLAY-PARSE-001** / **LLR-REPLAY-PARSE-002** / **LLR-REPLAY-PARSE-003** / **LLR-REPLAY-PARSE-004** / **LLR-REPLAY-PARSE-005** | Parses `precision-replay-input v1` / `schema math-i64f64-v1` text into caller-provided replay frame storage without heap allocation, file I/O, witness output, checker commands, or replay execution. |
| `core/src/replay_tests.rs` saved-input parser coverage | **HLR-REPLAY-PARSE-001** / **HLR-REPLAY-PARSE-002** / **HLR-REPLAY-PARSE-003** / **HLR-REPLAY-PARSE-004** / **HLR-REPLAY-PARSE-005** / **HLR-REPLAY-PARSE-006** / **LLR-REPLAY-PARSE-001** / **LLR-REPLAY-PARSE-002** / **LLR-REPLAY-PARSE-003** / **LLR-REPLAY-PARSE-004** / **LLR-REPLAY-PARSE-005** | Covers valid add/sub/mul/nonzero-div saved input parsing, parse-only separation from execution, unknown version, unknown schema/lane, unknown opcode, malformed rows, missing operand fields, invalid integer fields, and frame-capacity exhaustion. |

---

## 3. Target Witness Runtime

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| `bsp/stm32` target feature selection for `stm32f446` | **HLR-TARGET-IO-001** | Selects the STM32F446 target path that owns the retained replay witness output surface. |
| `target::init_stlink_vcp_usart2()` | **HLR-TARGET-IO-001** / **LLR-TARGET-IO-001** | Enables GPIOA and USART2 clocks, configures PA2 for USART2 alternate-function TX, sets the retained 16 MHz / 115200 baud divisor, and enables USART2 transmit operation for ST-LINK VCP witness output. |
| `target::write_stlink_vcp_usart2(...)`<br>`write_usart2_byte(...)` | **HLR-TARGET-IO-001** / **LLR-TARGET-IO-001** | Emits bounded witness byte slices by waiting for USART2 transmit readiness before writing each byte to the data register. |
| `firmware_main()` in `runners/stm32-runner` | **HLR-TARGET-IO-001** / **LLR-RUNNER-WITNESS-001** | Initializes the STM32F446 target witness output path, allows a short post-init settle interval, then emits the retained replay result line. |
| `emit_replay_result_line()` in `runners/stm32-runner` | **HLR-TARGET-IO-001** / **LLR-RUNNER-WITNESS-001** | Produces the retained `v0.1.0-rc1` math-add witness payload over the initialized target serial path. |

---

## 4. Sensor Witness

| Witness requirement | Implementation | Verification / status | Boundary |
| :--- | :--- | :--- | :--- |
| **HLR-WITNESS-ADC** / **LLR-WITNESS-ADC** | `bsp/stm32/src/lib.rs`: configures STM32F446 PA0 / ADC1_IN0 for blocking raw ADC sampling. `runners/stm32-runner/src/main.rs`: reads raw ADC samples and emits raw ADC witness records. | Implementation-traced. Build/static validation covers workspace format/check/test/clippy, STM32 BSP target check, and STM32 runner target build. Host parser validation and retained raw ADC capture artifact status are traced under **HLR-WITNESS-HOST** / **LLR-WITNESS-HOST**. | Claims only raw 12-bit ADC witness emission. No fixed-rate timing, DMA, interrupt timing, calibration, magnetic-field units, replay alignment, digest sealing, stimulus/envelope semantics, retained release evidence, or generalized board support. |
| **HLR-WITNESS-ADC** / **LLR-WITNESS-UART** | `runners/stm32-runner/src/main.rs`: emits raw sample-indexed witness records over ST-LINK VCP USART2. | Implementation-traced through the raw ADC witness lane. Host parser validation and retained raw ADC capture artifact status are traced under **HLR-WITNESS-HOST** / **LLR-WITNESS-HOST**. | Does not expand retained replay witness output claims or assert retained hardware capture evidence. |
| **HLR-WITNESS-TIME** / **LLR-WITNESS-TIME** | `runners/stm32-runner/src/main.rs`: declares `timing_claim=best_effort_polling_uart_stream` in each raw ADC witness record. | Implementation-traced. Stronger timing verification is deferred. | Excludes fixed-rate sampling, precise event timing, transient fidelity, replay alignment, timer-paced ADC, DMA buffering, interrupt-driven capture, and final timing authority. |
| **HLR-WITNESS-HOST** / **LLR-WITNESS-HOST** | `tools/raw_adc_monitor.py`: consumes `witness=raw-adc` records from stdin or optional serial input, validates exact row shape, raw ADC bounds, and timing claim, ignores non-witness noise, counts malformed raw ADC witness rows, and emits a deterministic summary. `tests/test_raw_adc_monitor.py`: covers valid rows, multiple rows in order, ignored banner/noise/math-result lines, malformed `witness=raw-adc` rows, out-of-range `raw_adc`, wrong timing claim, deterministic summary output, empty stream behavior, and CLI nonzero behavior for malformed witness rows. | Implementation-traced. Host parser and deterministic summary behavior are covered by pytest. Workspace validation passed. Manual STM32 serial smoke observation produced valid raw ADC summaries and surfaced malformed witness lines. Bounded retained raw ADC capture artifact exists at `artifacts/witness/raw-adc/20260703T221041Z/`, including retained witness rows and host monitor summary; retained release evidence remains out of scope. | Host-side raw ADC witness parsing and deterministic summary only. No calibrated voltage, calibrated magnetic-field units, fixed-rate sampling, timer-paced capture, hardware correctness, signal quality, replay alignment, digest sealing, stimulus/envelope behavior, retained release evidence, or generalized board support. |
| **HLR-WITNESS-HOST** / **LLR-WITNESS-HOST** retained raw ADC evidence admission | Retained raw ADC witness artifacts are checkable against the bounded capture claim, canonical deterministic summary text, complete raw ADC token shape/range, metadata boundary, and explicit non-lossless/non-contiguous limitations. Implementation: `tools/check_raw_adc_capture.py`. | Implemented / tested / retained artifact PASS at phash `fa7f342454ee1056cb330e05045cdab554522cee`. Verification: `tests/test_check_raw_adc_capture.py`, `tests/test_raw_adc_monitor.py`, and `python3 tools/check_raw_adc_capture.py artifacts/witness/raw-adc/20260703T221041Z`. | Host-side retained-evidence checker only. Does not claim ADC electrical correctness, UART losslessness, precise timing behavior, stimulus adequacy, board qualification, release readiness, or hardware certification. |
| **HLR-WITNESS-OBS** | Existing host raw ADC parser and retained capture checker distinguish capture rows from admitted observation use. `tools/raw_adc_monitor.py` validates raw ADC row shape, raw value bounds, and timing claim for capture summaries; `tools/check_raw_adc_capture.py` admits retained raw ADC capture artifacts against the bounded capture claim. | Requirements-defined / existing-checker-traced. This row defines observation eligibility for checked and admitted raw ADC rows and does not add runtime behavior. | Observation identifies channel, sample index, raw value, and timing claim only. It does not imply calibration, stimulus response, response-envelope pass/fail, signal quality, hardware qualification, release readiness, or certification posture. |
| **HLR-WITNESS-STIM** / **LLR-WITNESS-STIM** | Declared observation context boundary. | requirements-defined / implementation-deferred / verification-deferred. | An admitted observation may belong to a declared context, and that context may name an external stimulus. The repository does not generate or verify external stimulus. Context does not prove stimulus quality, calibration, timing, or sufficiency; timing remains best-effort; envelope pass/fail remains deferred. |
| **HLR-WITNESS-ENV** / **LLR-WITNESS-ENV** | `tools/check_raw_adc_envelope.py`: reads retained raw ADC artifacts, reuses retained capture admission, parses declared envelope metadata, judges admitted observations against `raw_adc_min`, `raw_adc_max`, `min_sample_count`, and `allow_malformed_witness_lines`, and verifies deterministic retained `judgment.txt` output. `tests/test_check_raw_adc_envelope.py`: covers pass, missing retained judgment, canonical output mismatch, missing context, missing limits, too few admitted samples, below/above envelope failures, malformed witness handling with both tolerance settings, and capture non-admission. | Implemented / tested / retained artifact PASS. Verification: `tests/test_check_raw_adc_envelope.py` and `python3 tools/check_raw_adc_envelope.py artifacts/witness/raw-adc/20260704T211403Z`. | Judgment applies only to admitted raw ADC observations in a declared context. Allowed results are `pass`, `fail`, `inconclusive`, and `not_applicable`. Tolerated malformed witness rows remain outside admitted observations. Does not implement context comparison, baseline-vs-stimulus comparison, trend judgment, delta judgment, calibrated measurement, UART losslessness, timing proof, signal quality, stimulus adequacy, hardware correctness, board qualification, release readiness, or certification posture. |

---

## 5. Verification Alignment

Rows in this matrix are requirement traceability entries with explicit verification status.

Verification status sources include:

- active symbolic proofs in `verification/src/lib.rs`
- implementation-local tests in `core/src/math.rs`
- first-class pending proof obligations tracked by the SVCP
- implementation-deferred requirement rows

STM32 target witness rows provide behavioral traceability for retained replay observation runtime code and do not expand release evidence, hardware qualification, tool qualification, certification, proof, or CI claims.

Sensor Witness rows distinguish the initial STM32F446 raw ADC witness implementation from deferred validation and later witness surfaces. The raw ADC lane is traced to implementation with build/static validation only. Retained hardware capture artifacts, host parser validation, and retained raw ADC envelope judgment are traced above. Context comparison, calibrated units, replay alignment, digest sealing, and stronger timing claims remain outside the current boundary.

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
