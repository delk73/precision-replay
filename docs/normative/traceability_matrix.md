# Requirements Traceability Matrix

This document records the current traceability links and verification status between active behavioral and system requirements and their implementation blocks. It distinguishes active proofs, implementation-local tests, runtime traceability, and pending proof obligations rather than asserting that every traceability row is fully verified.

## Traceability Row-Class Policy

The traceability matrix intentionally uses more than one table shape so software implementation mappings and evidence-boundary claims remain structurally distinct before any later section reorganization.

### Implementation Traceability Rows

Use implementation traceability rows when a requirement maps primarily to software, tooling, retained software artifacts, parser behavior, executor behavior, checker behavior, or pending software implementation.

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |

Column 1 identifies the implementation block. Column 2 identifies the requirement IDs. Column 3 records the mapping, verification status, tests, proof status, retained evidence, or pending implementation status.

### Evidence-Boundary Rows

Use evidence-boundary rows when a requirement maps to observed evidence, retained hardware-adjacent evidence, admitted observations, declared context, envelope judgment, or any claim that could be overread as calibration, timing proof, signal fidelity, stimulus sufficiency, hardware qualification, release readiness, or certification posture.

| Requirement | Evidence / Implementation | Verification / Status | Boundary |
| :--- | :--- | :--- | :--- |

Column 1 identifies the requirement IDs. Column 2 identifies the evidence and/or implementation supporting the claim. Column 3 records what has actually been checked. Column 4 states the bounded claim and exclusions.

The Boundary column is not optional commentary. For evidence-boundary rows, it is part of the traceability claim and must be preserved during edits.

### Canonical Status Tokens

Each traceability row should contain an explicit status token in the verification/status cell using this form: `Status: <canonical_status>. <Plain supporting sentence.>`

Canonical statuses are `pending`, `implemented`, `tested`, `proof_partial`, `boundary_only`, and `traced`. `unknown` is reserved for parser/tool output only and must not be written as an intentional matrix status.

Use `pending` when a requirement is defined but implementation or verification remains pending. Use `implemented` when an implementation path exists without making a stronger test or proof claim through the token alone. Use `tested` when an executable test, retained checker, or retained artifact check covers the row. Use `proof_partial` when bounded or partial formal proof evidence exists. Use `boundary_only` when the row defines an evidence boundary, exclusion, or non-claim. Use `traced` when the requirement is mapped without claiming stronger implementation, test, or proof status.

Parser and browser tools must prefer explicit `Status: <token>` values over prose inference. For evidence-boundary rows, use `Status: boundary_only. <Plain supporting sentence.>` and preserve the required Boundary column.

Status-token normalization may be applied incrementally by section; rows without explicit status tokens remain legacy rows until normalized.

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
| `if hh > 0xFFFF_FFFF_FFFF_FFFF`<br>`.checked_add(cross_sum)` in `I64F64::fallible_mul(...)` | **LLR-REPLAY-MATH-OPS-002** | Maps to Step 4 overflow gating for the raw scaled magnitude: upper `hh` bits that cannot fit after the 64-bit truncation shift produce a typed arithmetic error, and checked composition traps bit-pool overflow through the same shared fallible multiplication implementation. |
| `I64F64::fallible_mul(...)`<br>`impl Mul for I64F64 { ... }` | **HLR-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-002** / **LLR-REPLAY-EXEC-006** | Maps to raw sign-isolated magnitude scaling: the absolute product magnitude is shifted/truncated by 64 bits, then the isolated sign is reapplied. The public multiplication operator is a thin wrapper that maps typed fallible multiplication errors to existing panic messages. Replay consumes the same fallible multiplication path and maps any arithmetic error to deterministic arithmetic-trap rejection. `SVCP-MATH-PRO-002a` and `SVCP-MATH-PRO-002b` provide active partial proof coverage; `SVCP-MATH-PRO-002c` is active only for bounded fixed non-unit high-limb single-cross-term LH and HL correspondence, bounded two-term cross-sum composition, bounded low-limb carry contribution, bounded integrated non-overflowing matrix composition, bounded symbolic high-limb non-overflowing matrix composition, bounded high-high overflow-gate trap observation, bounded final signed-capacity overflow trap observation, public-operand cross-sum overflow unreachability for raw operands reachable through the public `I64F64` representation, signed minimum-capacity boundary allowance over the public raw multiplication path, and negative signed-capacity exceedance trap observation over the public raw multiplication path. Full multiplication closure, full unbounded symbolic limb-matrix correspondence, private/helper-state limb combinations not reachable from public raw operands, full overflow-gate correspondence, and full overflow/trap proof coverage remain pending. |
| `pub fn mul_convergent(...)` | **HLR-MATH-OPS-002** / **HLR-MATH-REP-002** / **LLR-REPLAY-MATH-OPS-002** / **LLR-REPLAY-MATH-OPS-004** | Provides the drift-canceled multiplication path and applies convergent rounding behavior to discarded fractional state. |
| `if final_abs_bits > i128::MAX` | **LLR-REPLAY-MATH-OPS-002** | Maps to Step 4 capacity boundary gating, protecting signed limits while allowing the unique `i128::MIN` absolute match if `out_negative` is true. |
| `pub fn round_ties_to_even(...)` | **HLR-MATH-REP-002** / **LLR-REPLAY-MATH-OPS-004** | Implements convergent accumulator-to-integer rounding with exact half-scale ties resolved toward the even integral value. |
| `ArithmeticError`, `I64F64::fallible_add(...)`, `I64F64::fallible_sub(...)`, `I64F64::fallible_mul(...)`, and `I64F64::fallible_div(...)` | **HLR-REPLAY-EXEC-006** / **LLR-REPLAY-EXEC-006** | Provides shared crate-internal typed fallible arithmetic paths for both replay execution and public operators. The paths cover addition overflow, subtraction overflow, multiplication traps, division by zero, division numerator-shift overflow, and integer division overflow without adding a public arithmetic error API. |
| `impl Add for I64F64`<br>`impl Sub for I64F64` | **HLR-MATH-OPS-001** / **LLR-REPLAY-MATH-OPS-001** / **LLR-REPLAY-EXEC-006** | Thinly wraps `fallible_add` and `fallible_sub`, mapping typed errors back to the existing public panic messages when additions or subtractions cross signed boundaries. `SVCP-MATH-PRO-001` provides active Kani coverage for non-overflowing add/sub exactness, addition overflow trap observation, and subtraction overflow trap observation. |
| `impl Div for I64F64`<br>`I64F64::fallible_div(...)` | **HLR-MATH-OPS-003** / **LLR-REPLAY-MATH-OPS-003** / **LLR-REPLAY-EXEC-006** | Standardizes the fixed-point division protocol via a shared fallible 64-bit numerator pre-shift path with explicit leading zero/one safety bounds. The public division operator maps typed errors back to existing panic messages, while replay maps them to deterministic arithmetic-trap rejection. `SVCP-MATH-PRO-003a` and `SVCP-MATH-PRO-003b` provide active Kani guard proof coverage for divide-by-zero and numerator shift-overflow trapping; `SVCP-MATH-PRO-003c` is active for bounded non-trapping arithmetic correspondence with symbolic `i32` raw numerators and signed power-of-two denominator family `{-8, -4, -2, -1, 1, 2, 4, 8}`. Arbitrary-denominator and full unbounded symbolic division arithmetic remain pending, and implementation-local division tests remain regression support rather than Kani proof coverage. |

---

## 2. Replay Execution

### 2.1 Replay System Contract and Schema Ownership

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| Replay system contract requirements in `docs/normative/HLR_replay.md` | **HLR-REPLAY-SYS-001** / **HLR-REPLAY-SYS-002** | Status: pending. HLR-defined system boundary. The integrated math path and raw ADC source-evidence path are both active proof paths, but neither is defined as the complete replay system and neither math frames nor raw ADC observations are universal replay input. LLR, implementation, and verification for the broader system contract remain pending. |
| Replay schema ownership requirements in `docs/normative/HLR_replay.md` | **HLR-REPLAY-SYS-003** / **HLR-REPLAY-SCHEMA-001** / **HLR-REPLAY-SCHEMA-002** / **HLR-REPLAY-SCHEMA-003** / **HLR-REPLAY-SCHEMA-004** / **HLR-REPLAY-SCHEMA-005** | Status: pending. HLR-defined / LLR-pending. Defines common retained-run structure ownership and schema ownership for input meaning, origins, state evolution, trace semantics, terminal behavior, and outcome comparison. No runtime implementation or verification is credited. |
| Pending schema origin declaration model | **HLR-REPLAY-ORIGIN-001** | Status: pending. HLR-defined / LLR, implementation, and verification pending. Existing `math-i64f64-v1` artifacts demonstrate direct saved replay input but do not implement a general per-schema origin declaration model. |
| Direct saved math replay input path | **HLR-REPLAY-ORIGIN-002** | Status: tested. Implemented and tested only for the initial `math-i64f64-v1` direct saved replay input lane through the parser, executor, retained artifacts, and checker rows below. |
| Pending raw-ADC replay input projection origin | **HLR-REPLAY-ORIGIN-003** / **HLR-REPLAY-ORIGIN-004** | Status: pending. HLR-defined / LLR-pending for schema-permitted projection from admitted source evidence and admission only where the input origin requires it. Existing raw ADC admission is traced under witness requirements and does not implement replay projection. |

### 2.2 Retained Run

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| Pending retained-run model | **HLR-REPLAY-RUN-001** / **HLR-REPLAY-RUN-002** | Status: pending. HLR-defined / LLR, implementation, and verification pending. Required immutable retained-run content and pre-execution validation are not implemented by the current expected witness/result files. |
| Pending retained-run identity model | **HLR-REPLAY-RUN-003** / **HLR-REPLAY-RUN-004** | Status: pending. HLR-defined / LLR, implementation, and verification pending. Retained-run identity derivation and exclusion of generated evaluations, diagnostics, target metadata, envelope judgments, and later verification results remain undefined below HLR level. |

### 2.3 Saved Input Parsing and Initial Math Execution

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| `ReplayFrame` in `core/src/replay.rs` | **HLR-REPLAY-EXEC-001** / **HLR-REPLAY-EXEC-002** / **LLR-REPLAY-EXEC-001** | Status: implemented. Defines the public in-memory frame surface for loading operands, executing add/sub/mul/div math operations, and expecting result bits. |
| `ReplayExecutionState`, `ReplayRejectionReason`, and `ReplayExecutionResult` in `core/src/replay.rs` | **HLR-REPLAY-EXEC-003** / **HLR-REPLAY-EXEC-004** / **HLR-REPLAY-EXEC-005** / **HLR-REPLAY-EXEC-006** / **LLR-REPLAY-EXEC-002** / **LLR-REPLAY-EXEC-003** | Status: implemented. Exposes deterministic execution state, optional result bits, and rejection reasons for invalid order, expected-result mismatch, and arithmetic traps. |
| `execute_replay(...)` in `core/src/replay.rs` | **HLR-REPLAY-EXEC-001** / **HLR-REPLAY-EXEC-002** / **HLR-REPLAY-EXEC-003** / **HLR-REPLAY-EXEC-004** / **HLR-REPLAY-EXEC-005** / **HLR-REPLAY-EXEC-006** / **LLR-REPLAY-EXEC-004** / **LLR-REPLAY-EXEC-005** / **LLR-REPLAY-EXEC-006** | Status: implemented. Executes a pure in-memory replay frame slice using the shared fallible `I64F64` add/sub/mul/div paths, rejects invalid frame ordering, result mismatches, and arithmetic traps, and returns a deterministic result. |
| `core/src/replay_tests.rs` | **HLR-REPLAY-EXEC-001** / **HLR-REPLAY-EXEC-002** / **HLR-REPLAY-EXEC-003** / **HLR-REPLAY-EXEC-004** / **HLR-REPLAY-EXEC-005** / **HLR-REPLAY-EXEC-006** / **LLR-REPLAY-EXEC-001** / **LLR-REPLAY-EXEC-002** / **LLR-REPLAY-EXEC-003** / **LLR-REPLAY-EXEC-004** / **LLR-REPLAY-EXEC-005** / **LLR-REPLAY-EXEC-006** | Status: tested. Covers valid add/sub/mul/nonzero-div acceptance, arithmetic-trap rejection for add overflow, sub overflow, multiplication trap, division by zero, division numerator-shift overflow, and integer division overflow, math operation before operand rejection, expected result before execution rejection, expected-result mismatch rejection, frame-after-acceptance rejection, empty frame-slice state, successful replay repeatability, and arithmetic-trap rejection repeatability. |
| `ReplayInputVersion`, `ReplayInputSchema`, `ParsedReplayInput`, and `ReplayParseError` in `core/src/replay.rs` | **HLR-REPLAY-PARSE-001** / **HLR-REPLAY-PARSE-002** / **HLR-REPLAY-PARSE-003** / **HLR-REPLAY-PARSE-004** / **HLR-REPLAY-PARSE-005** / **LLR-REPLAY-PARSE-001** / **LLR-REPLAY-PARSE-003** | Status: implemented. Defines the accepted saved-input version/schema identity and the strict parser rejection surface. |
| `parse_replay_input(...)` in `core/src/replay.rs` | **HLR-REPLAY-PARSE-001** / **HLR-REPLAY-PARSE-002** / **HLR-REPLAY-PARSE-003** / **HLR-REPLAY-PARSE-004** / **HLR-REPLAY-PARSE-005** / **HLR-REPLAY-PARSE-006** / **LLR-REPLAY-PARSE-001** / **LLR-REPLAY-PARSE-002** / **LLR-REPLAY-PARSE-003** / **LLR-REPLAY-PARSE-004** / **LLR-REPLAY-PARSE-005** | Status: implemented. Parses `precision-replay-input v1` / `schema math-i64f64-v1` text into caller-provided replay frame storage without heap allocation, file I/O, witness output, checker commands, or replay execution. |
| `core/src/replay_tests.rs` saved-input parser coverage | **HLR-REPLAY-PARSE-001** / **HLR-REPLAY-PARSE-002** / **HLR-REPLAY-PARSE-003** / **HLR-REPLAY-PARSE-004** / **HLR-REPLAY-PARSE-005** / **HLR-REPLAY-PARSE-006** / **LLR-REPLAY-PARSE-001** / **LLR-REPLAY-PARSE-002** / **LLR-REPLAY-PARSE-003** / **LLR-REPLAY-PARSE-004** / **LLR-REPLAY-PARSE-005** | Status: tested. Covers valid add/sub/mul/nonzero-div saved input parsing, parse-only separation from execution, unknown version, unknown schema/lane, unknown opcode, malformed rows, missing operand fields, invalid integer fields, and frame-capacity exhaustion. |

### 2.4 Retained Math Checker Path

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| `artifacts/replay/math-i64f64-v1/input.txt`, `expected_witness.txt`, and `expected_result.txt` | **HLR-REPLAY-CHECK-001** / **HLR-REPLAY-CHECK-004** / **HLR-REPLAY-CHECK-005** / **HLR-REPLAY-CHECK-006** / **LLR-REPLAY-CHECK-001** / **LLR-REPLAY-CHECK-002** / **LLR-REPLAY-CHECK-003** | Status: implemented. Retains the initial math lane saved replay input plus expected generated witness and checker result text. |
| `core/examples/replay_check.rs` | **HLR-REPLAY-CHECK-002** / **HLR-REPLAY-CHECK-003** / **HLR-REPLAY-CHECK-004** / **HLR-REPLAY-CHECK-009** / **HLR-REPLAY-CHECK-010** / **LLR-REPLAY-CHECK-002** / **LLR-REPLAY-CHECK-005** / **LLR-REPLAY-CHECK-006** / **LLR-REPLAY-CHECK-007** / **LLR-REPLAY-CHECK-008** | Status: implemented. Provides the checked-in Rust replay checker entrypoint that validates exact input-path arity, reads saved input, invokes `parse_replay_input` and `execute_replay`, emits deterministic replay witness text, and reports stable failure diagnostics and exit codes for parse rejection and replay non-acceptance, including incomplete replay. |
| `tests/test_replay_check.py` | **HLR-REPLAY-CHECK-004** / **HLR-REPLAY-CHECK-009** / **HLR-REPLAY-CHECK-010** / **LLR-REPLAY-CHECK-002** / **LLR-REPLAY-CHECK-007** / **LLR-REPLAY-CHECK-008** | Status: tested. Black-box covers the checked-in Rust replay checker command boundary for successful witness output, missing and extra arguments, stable input-read failure, parse rejection, incomplete replay, invalid-order rejection, arithmetic-trap rejection, and expected-result mismatch diagnostics. |
| `tools/check_replay.py` | **HLR-REPLAY-CHECK-001** / **HLR-REPLAY-CHECK-005** / **HLR-REPLAY-CHECK-006** / **HLR-REPLAY-CHECK-007** / **HLR-REPLAY-CHECK-008** / **LLR-REPLAY-CHECK-001** / **LLR-REPLAY-CHECK-003** / **LLR-REPLAY-CHECK-005** / **LLR-REPLAY-CHECK-006** | Status: implemented. Reads the retained replay artifact path, invokes the checked-in Rust replay checker entrypoint, compares retained expected witness/result files, and exits nonzero on parse, replay, witness, or result failure. |
| `make replay-check` | **HLR-REPLAY-CHECK-001** / **LLR-REPLAY-CHECK-004** | Status: implemented. Provides the public retained replay checker command and prints `parse=pass`, `replay=pass`, `witness=pass`, and `result=pass` on success. |

### 2.5 Raw ADC Replay Input Projection

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| Pending raw ADC projection implementation | **HLR-REPLAY-PROJ-001** / **LLR-REPLAY-PROJ-001** | Status: pending. Requires canonical replay input for a raw-ADC-derived replay schema to be created only from an admitted raw ADC capture. Implementation and verification are pending. |
| Pending raw ADC projection implementation | **HLR-REPLAY-PROJ-002** / **LLR-REPLAY-PROJ-002** | Status: pending. Restricts raw-ADC-derived replay input projection to admitted observations and excludes rejected or malformed rows. Implementation and verification are pending. |
| Pending raw ADC projection implementation | **HLR-REPLAY-PROJ-003** / **LLR-REPLAY-PROJ-003** | Status: pending. Requires raw-ADC-derived canonical replay input to identify the admitted source capture. Source-reference representation, implementation, and verification are pending. |
| Pending raw ADC projection implementation | **HLR-REPLAY-PROJ-004** / **LLR-REPLAY-PROJ-004** | Status: pending. Requires raw-ADC-derived replay input projection to preserve `sample_count`, `first_sample_index`, `last_sample_index`, `min_raw_adc`, `max_raw_adc`, and the admitted `timing_claim`. Implementation and verification are pending. |
| Pending raw ADC projection implementation | **HLR-REPLAY-PROJ-005** / **LLR-REPLAY-PROJ-005** | Status: pending. Requires raw-ADC-derived replay input projection to preserve `context_id` when present and omit it when absent. Implementation and verification are pending. |
| Pending raw ADC projection implementation | **HLR-REPLAY-PROJ-006** / **LLR-REPLAY-PROJ-006** | Status: pending. Requires deterministic raw-ADC-derived replay input projection without adding claims beyond the admitted source evidence. Implementation and verification are pending. |

### 2.6 General Retained-Run Execution, Trace, and Comparison

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| Pending retained-run execution model | **HLR-REPLAY-EXEC-007** / **HLR-REPLAY-EXEC-008** / **HLR-REPLAY-EXEC-009** / **HLR-REPLAY-EXEC-010** / **HLR-REPLAY-EXEC-011** / **HLR-REPLAY-EXEC-012** | Status: pending. HLR-defined / LLR, implementation, and verification pending. Existing `execute_replay(...)` satisfies only the initial frame-based math execution HLRs and is not credited with the broader retained-run execution model. |
| Pending retained-run trace model | **HLR-REPLAY-TRACE-001** / **HLR-REPLAY-TRACE-002** / **HLR-REPLAY-TRACE-003** | Status: pending. HLR-defined / LLR, implementation, and verification pending. Existing witness text is not a schema-defined reference trace model. |
| Pending retained-run comparison model | **HLR-REPLAY-COMP-001** / **HLR-REPLAY-COMP-002** / **HLR-REPLAY-COMP-003** / **HLR-REPLAY-COMP-004** / **HLR-REPLAY-COMP-005** / **HLR-REPLAY-COMP-006** / **HLR-REPLAY-COMP-007** | Status: pending. HLR-defined / LLR, implementation, and verification pending. Existing checker witness/result comparison remains the initial proof path and does not implement trace/outcome comparison, comparison dispositions, or first-divergence reporting. |

### 2.7 Replay Evaluation, Operations, Envelope, and Target Agreement

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| Pending replay evaluation model | **HLR-REPLAY-EVAL-001** / **HLR-REPLAY-EVAL-002** / **HLR-REPLAY-EVAL-003** | Status: pending. HLR-defined / LLR, implementation, and verification pending. Existing checker output remains retained math checker evidence and is not credited with the broader replay-evaluation model. |
| Pending replay run operations | **HLR-REPLAY-OPS-001** / **HLR-REPLAY-OPS-002** / **HLR-REPLAY-OPS-003** | Status: pending. HLR-defined / LLR, implementation, and verification pending for `record`, `replay`, and `diff` operations. No CLI command or runtime behavior is implemented by this traceability row. |
| Pending replay-trace envelope operation | **HLR-REPLAY-OPS-004** | Status: pending. HLR-defined / LLR, implementation, and verification pending for applying a named deterministic rule to a replay trace. |
| Existing raw ADC source-evidence envelope boundary | **HLR-REPLAY-OPS-005** | Status: boundary_only. HLR-defined boundary tying replay requirements to the existing witness-envelope separation. The implemented raw ADC envelope remains traced under **HLR-WITNESS-ENV-001** / **LLR-WITNESS-ENV-001** below and is not a replay-trace envelope. |
| Pending multi-target replay agreement | **HLR-REPLAY-TGT-001** / **HLR-REPLAY-TGT-002** | Status: pending. HLR-defined / LLR, implementation, and verification pending. No target agreement implementation or verification is claimed. |

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

### 4.1 Sensor Witness Implementation Traceability

| Code Component / Implementation Block | Requirement ID | Traceability Verification |
| :--- | :--- | :--- |
| `bsp/stm32/src/lib.rs`: configures STM32F446 PA0 / ADC1_IN0 for blocking raw ADC sampling. `runners/stm32-runner/src/main.rs`: reads raw ADC samples. | **HLR-WITNESS-ADC-001** / **LLR-WITNESS-ADC-001** | Implementation-traced. Build/static validation covers workspace format/check/test/clippy, STM32 BSP target check, and STM32 runner target build. Host parser validation and retained raw ADC capture artifact status are traced under their own numbered witness rows. Boundary: Claims only raw 12-bit ADC acquisition for the initial STM32F446 PA0 / ADC1_IN0 lane. No fixed-rate timing, DMA, interrupt timing, calibration, magnetic-field units, replay alignment, digest sealing, stimulus/envelope semantics, retained release evidence, or generalized board support. |
| `runners/stm32-runner/src/main.rs`: emits raw sample-indexed witness records over ST-LINK VCP USART2. | **HLR-WITNESS-SERIAL-001** / **LLR-WITNESS-SERIAL-001** | Implementation-traced through the raw ADC witness lane. Host parser validation and retained raw ADC capture artifact status are traced under their own numbered witness rows. Boundary: Does not expand retained replay witness output claims or assert retained hardware capture evidence. |
| `runners/stm32-runner/src/main.rs`: declares `timing_claim=best_effort_polling_uart_stream` in each raw ADC witness record. | **HLR-WITNESS-TIME-001** / **LLR-WITNESS-TIME-001** | Implementation-traced. Stronger timing verification is deferred. Boundary: Excludes fixed-rate sampling, precise event timing, transient fidelity, replay alignment, timer-paced ADC, DMA buffering, interrupt-driven capture, and final timing authority. |
| `tools/raw_adc_monitor.py`: consumes `witness=raw-adc` records from stdin or optional serial input, validates exact row shape, raw ADC bounds, and timing claim, ignores non-witness noise, counts malformed raw ADC witness rows, and emits a deterministic summary. `tests/test_raw_adc_monitor.py`: covers valid rows, multiple rows in order, ignored banner/noise/math-result lines, malformed `witness=raw-adc` rows, out-of-range `raw_adc`, wrong timing claim, deterministic summary output, empty stream behavior, and CLI nonzero behavior for malformed witness rows. | **HLR-WITNESS-HOST-001** / **LLR-WITNESS-HOST-001** | Implementation-traced. Host parser and deterministic summary behavior are covered by pytest. Workspace validation passed. Manual STM32 serial smoke observation produced valid raw ADC summaries and surfaced malformed witness lines. Boundary: Host-side raw ADC witness parsing and deterministic summary only. No calibrated voltage, calibrated magnetic-field units, fixed-rate sampling, timer-paced capture, hardware correctness, signal quality, replay alignment, digest sealing, stimulus/envelope behavior, retained release evidence, or generalized board support. |

### 4.2 Sensor Witness Evidence-Boundary Traceability

| Requirement | Evidence / Implementation | Verification / Status | Boundary |
| :--- | :--- | :--- | :--- |
| **HLR-WITNESS-CAPTURE-001** / **LLR-WITNESS-CAPTURE-001** | Retained raw ADC witness artifacts are checkable against the bounded capture claim, canonical deterministic summary text, complete raw ADC token shape/range, metadata boundary, and explicit non-lossless/non-contiguous limitations. Implementation: `tools/check_raw_adc_capture.py`. | Implemented / tested / retained artifact PASS at phash `fa7f342454ee1056cb330e05045cdab554522cee`. Verification: `tests/test_check_raw_adc_capture.py`, `tests/test_raw_adc_monitor.py`, and `python3 tools/check_raw_adc_capture.py artifacts/witness/raw-adc/20260703T221041Z`. | Host-side retained capture admission checker only. Does not claim ADC electrical correctness, UART losslessness, precise timing behavior, stimulus adequacy, board qualification, release readiness, or hardware certification. |
| **HLR-WITNESS-OBS-001** / **LLR-WITNESS-OBS-001** | `tools/raw_adc_monitor.py` validates raw ADC row shape, raw value bounds, and timing claim for capture summaries. `tools/check_raw_adc_capture.py` admits retained raw ADC capture artifacts against the bounded capture claim. `tools/check_raw_adc_envelope.py` uses only parsed samples from admitted retained captures as admitted observations. | Requirements-defined / existing-checker-traced. This row defines observation eligibility for checked and admitted raw ADC rows and does not add runtime behavior. | Observation identifies channel, sample index, raw value, and timing claim only. It does not imply calibration, stimulus response, response-envelope pass/fail, signal quality, hardware qualification, release readiness, or certification posture. |
| **HLR-WITNESS-CONTEXT-001** / **LLR-WITNESS-CONTEXT-001** | `tools/check_raw_adc_envelope.py`: parses declared `context_id` from retained envelope metadata and returns `not_applicable` when context is missing. | Implemented / tested for retained envelope metadata context handling. Verification: `tests/test_check_raw_adc_envelope.py`. | Declared context is retained metadata only. The repository does not generate, compare, verify, calibrate, synchronize, qualify, or prove sufficiency of external stimulus. Context does not prove stimulus quality, calibration, timing, or sufficiency; timing remains best-effort. |
| **HLR-WITNESS-ENV-001** / **LLR-WITNESS-ENV-001** | `tools/check_raw_adc_envelope.py`: reads retained raw ADC artifacts, reuses retained capture admission, parses declared envelope metadata, judges admitted observations against `raw_adc_min`, `raw_adc_max`, `min_sample_count`, and `allow_malformed_witness_lines`, and verifies deterministic retained `judgment.txt` output. `tests/test_check_raw_adc_envelope.py`: covers pass, missing retained judgment, canonical output mismatch, missing context, missing limits, too few admitted samples, below/above envelope failures, malformed witness handling with both tolerance settings, and capture non-admission. | Implemented / tested / retained artifact PASS. Verification: `tests/test_check_raw_adc_envelope.py` and `python3 tools/check_raw_adc_envelope.py artifacts/witness/raw-adc/20260704T211403Z`. | Judgment applies only to admitted raw ADC observations in a declared context. Allowed results are `pass`, `fail`, `inconclusive`, and `not_applicable`. Tolerated malformed witness rows remain outside admitted observations. Does not implement context comparison, baseline-vs-stimulus comparison, trend judgment, delta judgment, calibrated measurement, UART losslessness, timing proof, signal quality, stimulus adequacy, hardware correctness, board qualification, release readiness, or certification posture. |

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

`SVCP-MATH-PRO-001` is active for non-overflowing exactness plus overflow trap observation when the shared fallible add or sub path returns an arithmetic error.

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
