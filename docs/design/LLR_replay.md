# Low-Level Requirements - Replay Execution (LLR-REPLAY)

## 1. Replay Schema Contract

### LLR-REPLAY-SCHEMA-001: Stable Schema Identity
Each replay schema shall define a stable schema identity used to associate canonical replay input, retained reference material, execution behavior, traces, and comparison requirements with that schema.
*Traces to: HLR-REPLAY-SYS-003, HLR-REPLAY-SCHEMA-001*

### LLR-REPLAY-SCHEMA-002: Permitted Input Origins
Each replay schema shall define the input origins permitted for canonical replay input under that schema, including whether direct saved replay input or projection from admitted source evidence is allowed.
*Traces to: HLR-REPLAY-SCHEMA-001*

### LLR-REPLAY-SCHEMA-003: Canonical Input Meaning
Each replay schema shall define the meaning of canonical replay input under that schema without requiring that other replay schemas share the same input representation.
*Traces to: HLR-REPLAY-SCHEMA-001*

### LLR-REPLAY-SCHEMA-004: State Evolution
Each replay schema shall define deterministic state evolution for processing canonical replay input in schema-defined order.
*Traces to: HLR-REPLAY-SCHEMA-002*

### LLR-REPLAY-SCHEMA-005: Trace Meaning and Equality
Each replay schema shall define the meaning of its trace elements, trace ordering, trace equality, and whether rejected or incomplete executions retain a deterministic trace prefix.
*Traces to: HLR-REPLAY-SCHEMA-003*

### LLR-REPLAY-SCHEMA-006: Accepted, Rejected, and Incomplete Behavior
Each replay schema shall define accepted behavior, stable rejection behavior and reasons, incomplete behavior, and the schema-defined terminal outcome associated with execution disposition.
*Traces to: HLR-REPLAY-SCHEMA-004*

### LLR-REPLAY-SCHEMA-007: Outcome Comparison Rules
Each replay schema shall define the rules for comparing retained reference trace, reference execution disposition, and reference terminal outcome against generated trace, generated execution disposition, and generated terminal outcome.
*Traces to: HLR-REPLAY-SCHEMA-005*

## 2. Replay Input Origins

### LLR-REPLAY-ORIGIN-001: Defined Input Origins
The permitted canonical replay input origins shall be direct saved replay input and input projected from admitted source evidence.
*Traces to: HLR-REPLAY-ORIGIN-001, HLR-REPLAY-ORIGIN-002, HLR-REPLAY-ORIGIN-003*

### LLR-REPLAY-ORIGIN-002: Retained Run Origin Declaration
Each retained run shall declare exactly one canonical replay input origin.
*Traces to: HLR-REPLAY-RUN-001, HLR-REPLAY-ORIGIN-001*

### LLR-REPLAY-ORIGIN-003: Schema-Permitted Origin Gate
The retained run's declared replay schema shall permit the retained run's declared canonical replay input origin.
*Traces to: HLR-REPLAY-ORIGIN-001*

### LLR-REPLAY-ORIGIN-004: Projected Evidence Admission Requirement
When the declared input origin is input projected from admitted source evidence, the retained run shall include source identity and source admission information.
*Traces to: HLR-REPLAY-ORIGIN-003, HLR-REPLAY-ORIGIN-004*

### LLR-REPLAY-ORIGIN-005: Direct Saved Input Admission Exclusion
When the declared input origin is direct saved replay input, the retained run shall not invent source identity or source admission information for that origin.
*Traces to: HLR-REPLAY-ORIGIN-002, HLR-REPLAY-ORIGIN-004*


## 3. Retained Run

### LLR-REPLAY-RUN-001: Required Retained Run Content
Each retained run shall contain a retained-run format version, replay schema identity, canonical replay input, input-origin description, source identity and admission disposition when admission is required, applicable context, timing claims, evidence limitations, reference trace, reference execution disposition and schema-defined terminal outcome, and comparison metadata required by the schema.
*Traces to: HLR-REPLAY-RUN-001*

### LLR-REPLAY-RUN-002: Pre-Replay Content Validation
Required retained-run content shall be validated before replay execution begins.
*Traces to: HLR-REPLAY-RUN-002*

### LLR-REPLAY-RUN-003: Retained Content Immutability
Retained-run content shall not change after retained-run creation.
*Traces to: HLR-REPLAY-RUN-001*

### LLR-REPLAY-RUN-004: Retained Run Identity Derivation
Retained-run identity shall be derived deterministically from immutable retained-run content and shall not depend on file paths or check times.
*Traces to: HLR-REPLAY-RUN-003*

### LLR-REPLAY-RUN-005: Retained Run Identity Exclusions
Generated evaluations, diagnostics, target metadata, envelope judgments, and later verification results shall not change retained-run identity.
*Traces to: HLR-REPLAY-RUN-004*


## 4. Frame Model

### LLR-REPLAY-EXEC-001: Replay Frame Variants
Replay execution shall define replay frame variants for loading operands, executing math operations, and expecting result bits.
*Traces to: HLR-REPLAY-EXEC-001, HLR-REPLAY-EXEC-002*

## 5. Execution State

### LLR-REPLAY-EXEC-002: Replay Execution States
Replay execution shall define execution states sufficient to distinguish no operands loaded, operands loaded, result produced, accepted, and rejected.
*Traces to: HLR-REPLAY-EXEC-003, HLR-REPLAY-EXEC-004, HLR-REPLAY-EXEC-005*

### LLR-REPLAY-EXEC-003: Replay Rejection Reasons
Replay execution shall define rejection reasons for invalid execution order, expected-result mismatch, and arithmetic trap rejection.
*Traces to: HLR-REPLAY-EXEC-003, HLR-REPLAY-EXEC-004, HLR-REPLAY-EXEC-006*

## 6. Execution Semantics

### LLR-REPLAY-EXEC-004: Pure In-Memory Execution
Replay execution shall execute as a pure in-memory operation over a frame slice.
*Traces to: HLR-REPLAY-EXEC-001, HLR-REPLAY-EXEC-005*

### LLR-REPLAY-EXEC-005: Replay Repeatability
Running the same frame slice twice shall produce the same execution result.
*Traces to: HLR-REPLAY-EXEC-001, HLR-REPLAY-EXEC-005*

### LLR-REPLAY-EXEC-006: Shared Fallible Arithmetic Trap Handling
Replay execution and the public `I64F64` add, sub, mul, and div operators shall share the same crate-internal fallible arithmetic paths; replay execution shall map any fallible arithmetic error to the arithmetic-trap rejection reason without producing a new result, while public operators shall map those errors to the existing trap panic messages.
*Traces to: HLR-REPLAY-EXEC-002, HLR-REPLAY-EXEC-005, HLR-REPLAY-EXEC-006*


## 7. Saved Input Parsing

### LLR-REPLAY-PARSE-001: Initial Saved Input Grammar
Saved replay input parsing shall accept only the initial text grammar: first line `precision-replay-input v1`, second line `schema math-i64f64-v1`, followed by zero or more frame rows.
*Traces to: HLR-REPLAY-PARSE-001, HLR-REPLAY-PARSE-002, HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-002: Saved Input Frame Rows
Saved replay input parsing shall define frame rows `load lhs=<i128> rhs=<i128>`, `add`, `sub`, `mul`, `div`, and `expect bits=<i128>`.
*Traces to: HLR-REPLAY-PARSE-005, HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-003: Parse Rejection Reasons
Saved replay input parsing shall define rejection reasons for missing version, unknown version, missing schema, unknown schema, unknown frame opcode, malformed frame rows, missing required fields, invalid integer fields, and caller-provided frame capacity exhaustion.
*Traces to: HLR-REPLAY-PARSE-003, HLR-REPLAY-PARSE-004, HLR-REPLAY-PARSE-005*

### LLR-REPLAY-PARSE-004: Pure In-Memory Parsing
Saved replay input parsing shall be a pure in-memory operation over `&str` input and shall write parsed frames into a caller-provided output buffer.
*Traces to: HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-005: Parse/Execute Separation
Saved replay input parsing shall not execute replay frames; execution remains owned by `execute_replay(&[ReplayFrame])`.
*Traces to: HLR-REPLAY-PARSE-006, HLR-REPLAY-EXEC-001*


## 8. Retained Replay Witness Checker

### LLR-REPLAY-CHECK-001: Retained Replay Artifact Layout
The retained replay artifact layout shall contain `input.txt`, `expected_witness.txt`, and `expected_result.txt` under `artifacts/replay/math-i64f64-v1/`.
*Traces to: HLR-REPLAY-CHECK-001*

### LLR-REPLAY-CHECK-002: Generated Witness Text Format
The generated replay witness text format shall be one line: `precision-replay witness=replay-input-v1 schema=math-i64f64-v1 state=accepted result_bits=<i128>`.
*Traces to: HLR-REPLAY-CHECK-004, HLR-REPLAY-CHECK-005, HLR-REPLAY-CHECK-007*

### LLR-REPLAY-CHECK-003: Checker Result Text Format
The checker result text format shall be four lines: `parse=pass`, `replay=pass`, `witness=pass`, and `result=pass`.
*Traces to: HLR-REPLAY-CHECK-006, HLR-REPLAY-CHECK-008*

### LLR-REPLAY-CHECK-004: Public Replay Checker Command
The public retained replay checker command shall be `make replay-check`.
*Traces to: HLR-REPLAY-CHECK-001*

### LLR-REPLAY-CHECK-005: Checker Stage Ordering
The retained replay checker shall run parse, replay, witness, and result stages in order.
*Traces to: HLR-REPLAY-CHECK-002, HLR-REPLAY-CHECK-003, HLR-REPLAY-CHECK-004, HLR-REPLAY-CHECK-005, HLR-REPLAY-CHECK-006*

### LLR-REPLAY-CHECK-006: Checker Failure Behavior
The retained replay checker shall exit nonzero for parse failure, replay rejection, witness mismatch, and result mismatch.
*Traces to: HLR-REPLAY-CHECK-002, HLR-REPLAY-CHECK-003, HLR-REPLAY-CHECK-007, HLR-REPLAY-CHECK-008*

### LLR-REPLAY-CHECK-007: Checked-In Entrypoint Argument Arity
The checked-in replay checker entrypoint shall accept exactly one replay input path and shall reject missing or extra arguments with exit code `2` and diagnostic `expected exactly one replay input path`.
*Traces to: HLR-REPLAY-CHECK-009*

### LLR-REPLAY-CHECK-008: Checked-In Entrypoint Stable Failure Diagnostics
The checked-in replay checker entrypoint shall report input read failure with exit code `3` and diagnostic `input read failed`, parse rejection with exit code `10` and diagnostic `parse failed: <stable-identifier>`, and replay non-acceptance with exit code `20` and diagnostics for incomplete replay, invalid order, arithmetic trap, or expected-result mismatch without Debug formatting or environment-dependent details.
*Traces to: HLR-REPLAY-CHECK-010*


## 9. Raw ADC Admitted Observation Projection

### LLR-REPLAY-PROJ-001: Raw ADC Admission Precondition
Raw-ADC-derived replay input projection shall require successful raw ADC capture admission before it begins.
*Traces to: HLR-REPLAY-PROJ-001*

### LLR-REPLAY-PROJ-002: Raw ADC Admitted Row Selection
Raw-ADC-derived replay input projection shall include only admitted observations and shall exclude rejected or malformed rows.
*Traces to: HLR-REPLAY-PROJ-002*

### LLR-REPLAY-PROJ-003: Raw ADC Source Reference Preservation
Raw-ADC-derived replay input projection shall preserve a reference to the admitted source capture without defining how that reference is represented.
*Traces to: HLR-REPLAY-PROJ-003*

### LLR-REPLAY-PROJ-004: Raw ADC Observation Summary Preservation
Raw-ADC-derived replay input projection shall preserve the admitted values of `sample_count`, `first_sample_index`, `last_sample_index`, `min_raw_adc`, `max_raw_adc`, and `timing_claim`.
*Traces to: HLR-REPLAY-PROJ-004*

### LLR-REPLAY-PROJ-005: Raw ADC Context Preservation
Raw-ADC-derived replay input projection shall preserve `context_id` when present and omit it when absent.
*Traces to: HLR-REPLAY-PROJ-005*

### LLR-REPLAY-PROJ-006: Raw ADC Stable Projection
The same admitted raw ADC observations and metadata shall produce the same raw-ADC-derived replay input.
*Traces to: HLR-REPLAY-PROJ-006*
