# Low-Level Requirements - Replay Execution (LLR-REPLAY)

## 1. Replay Schema Contract

### LLR-REPLAY-SCHEMA-001: Stable Schema Identity
Each replay schema identity and version shall permanently resolve to one immutable schema contract used to associate canonical replay input, retained runs, replay execution, retained reference material, traces, comparison, and evaluation with that schema. A new schema identity or version shall be required when canonical-input meaning, required modeled-execution dependencies, state evolution, modeled-time semantics, trace meaning, trace contents, trace ordering, trace equality, terminal behavior, functional comparison semantics, or applicable separately declared timing-evaluation semantics change. Implementation changes shall not require a new schema identity or version unless they change declared schema semantics.
*Traces to: HLR-REPLAY-SCHEMA-006*

### LLR-REPLAY-SCHEMA-002: Schema-Specific Canonicalization Boundary
Route permission shall belong to the applicable upstream canonicalization contract for the declared replay schema, not to the common Replay schema contract.
*Traces to: HLR-REPLAY-SCHEMA-001, HLR-REPLAY-ORIGIN-001*

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
Each replay schema shall define the rules for comparing retained functional reference trace, reference execution disposition, and reference terminal outcome against generated functional trace, generated execution disposition, and generated terminal outcome.
*Traces to: HLR-REPLAY-SCHEMA-005*

### LLR-REPLAY-SCHEMA-008: Execution Dependency Declaration
Each replay schema shall declare every dependency that may affect modeled execution, including canonical replay input, initial state, module selection and module versions, execution or simulation configuration, numeric policy, transition ordering, active constraints, modeled time when applicable, or other schema-declared execution dependencies.
*Traces to: HLR-REPLAY-SCHEMA-007*

### LLR-REPLAY-SCHEMA-009: Descriptive Context Classification
Each replay schema shall classify modeled-execution input and configuration separately from non-authoritative descriptive information.
*Traces to: HLR-REPLAY-SCHEMA-008*

### LLR-REPLAY-SCHEMA-010: Descriptive Observability Declaration
Information that participates in the schema-defined functional trace or terminal outcome shall be classified as schema-defined observable functional behavior instead of non-authoritative descriptive information. Timing observations, timing claims, and evidence limitations shall remain separately owned and shall not become functional behavior through trace equality or functional comparison.
*Traces to: HLR-REPLAY-SCHEMA-008, HLR-REPLAY-RUN-005*

## 2. Upstream Canonical Input Origins

### LLR-REPLAY-ORIGIN-001: Stable Origin Types
Upstream canonical input origin routes shall be stable defined types; the currently supported route types are direct saved replay input and input projected from admitted source evidence.
*Traces to: HLR-REPLAY-ORIGIN-002, HLR-REPLAY-ORIGIN-003, HLR-REPLAY-ORIGIN-005*

### LLR-REPLAY-ORIGIN-002: Canonicalization Route Declaration
When upstream canonicalization information records the route that produced canonical input, it shall declare exactly one stable route type.
*Traces to: HLR-REPLAY-ORIGIN-006*

### LLR-REPLAY-ORIGIN-003: Upstream Canonicalization Route Gate
The applicable upstream canonicalization contract for the declared replay schema shall permit the route that produced canonical input.
*Traces to: HLR-REPLAY-ORIGIN-001, HLR-REPLAY-ORIGIN-006*

### LLR-REPLAY-ORIGIN-004: Projected Evidence Admission Requirement
When the upstream route is input projected from admitted source evidence, upstream canonicalization information shall include source identity and source admission information.
*Traces to: HLR-REPLAY-ORIGIN-003, HLR-REPLAY-ORIGIN-004*

### LLR-REPLAY-ORIGIN-005: Direct Saved Input Admission Exclusion
When the upstream route is direct saved replay input, upstream canonicalization support shall not invent source identity or source admission information for that route.
*Traces to: HLR-REPLAY-ORIGIN-002, HLR-REPLAY-ORIGIN-004*

### LLR-REPLAY-ORIGIN-006: Extensible Origin Type Set
Additional stable route types may be defined without changing the common retained-run model.
*Traces to: HLR-REPLAY-ORIGIN-005*


## 3. Retained Run

### LLR-REPLAY-RUN-001: Required Retained Run Content
Each retained run shall contain a retained-run format version, replay schema identity, canonical replay input, schema-declared execution dependencies used by execution, any schema-required upstream canonicalization information retained for that run, descriptive context, timing claims, evidence limitations, reference trace, reference execution disposition and schema-defined terminal outcome, and comparison metadata required by the schema.
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

### LLR-REPLAY-RUN-006: Retained Execution Dependency Binding
Each retained run shall bind every execution dependency used by its declared replay schema.
*Traces to: HLR-REPLAY-RUN-001, HLR-REPLAY-SCHEMA-007, HLR-REPLAY-EXEC-014*

### LLR-REPLAY-RUN-007: Descriptive Information Non-Execution
Information retained as descriptive context, timing claims, or evidence limitations shall not affect replay execution. Information required by execution shall instead be classified and bound as schema-declared execution input or configuration.
*Traces to: HLR-REPLAY-RUN-005, HLR-REPLAY-SCHEMA-008*

### LLR-REPLAY-RUN-008: Descriptive Information Non-Comparison
Non-authoritative descriptive information shall not affect functional trace equality or functional comparison. Only schema-defined functional trace or terminal-outcome behavior may participate in functional comparison. Timing observations and timing claims shall remain separately owned by timing evidence and timing evaluation. Evidence limitations shall remain claim-bounding information. Timing information and evidence limitations shall not become functional behavior through comparison.
*Traces to: HLR-REPLAY-RUN-005, HLR-REPLAY-SCHEMA-008*


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

### LLR-REPLAY-EXEC-007: Retained Run Execution Input Closure
Replay execution of a retained run shall use the declared replay schema to interpret only the execution-dependency values bound by that retained run.
*Traces to: HLR-REPLAY-EXEC-013, HLR-REPLAY-EXEC-014*

### LLR-REPLAY-EXEC-008: External State Exclusion
Replay execution shall not read clocks, files, environment variables, target metadata, diagnostics, later verification results, or other undeclared external state to determine state evolution.
*Traces to: HLR-REPLAY-EXEC-013*


## 7. Upstream Saved Input Parsing

### LLR-REPLAY-PARSE-001: Initial Saved Input Grammar
Upstream saved-input parsing shall accept only the initial text grammar before producing canonical input: first line `precision-replay-input v1`, second line `schema math-i64f64-v1`, followed by zero or more frame rows.
*Traces to: HLR-REPLAY-PARSE-001, HLR-REPLAY-PARSE-002, HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-002: Saved Input Frame Rows
Upstream saved-input parsing shall define frame rows `load lhs=<i128> rhs=<i128>`, `add`, `sub`, `mul`, `div`, and `expect bits=<i128>`.
*Traces to: HLR-REPLAY-PARSE-005, HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-003: Parse Rejection Reasons
Upstream saved-input parsing shall define rejection reasons for missing version, unknown version, missing schema, unknown schema, unknown frame opcode, malformed frame rows, missing required fields, invalid integer fields, and caller-provided frame capacity exhaustion.
*Traces to: HLR-REPLAY-PARSE-003, HLR-REPLAY-PARSE-004, HLR-REPLAY-PARSE-005*

### LLR-REPLAY-PARSE-004: Pure In-Memory Parsing
Upstream saved-input parsing shall be a pure in-memory operation over `&str` input and shall write parsed frames into a caller-provided output buffer.
*Traces to: HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-005: Parse/Execute Separation
Upstream saved-input parsing shall not execute replay frames; execution remains owned by `execute_replay(&[ReplayFrame])` after canonical input exists.
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


## 9. Upstream Raw ADC Admitted Observation Projection

### LLR-REPLAY-PROJ-001: Raw ADC Admission Precondition
Upstream raw-ADC-derived replay input projection shall require successful raw ADC capture admission before it begins.
*Traces to: HLR-REPLAY-PROJ-001*

### LLR-REPLAY-PROJ-002: Raw ADC Admitted Row Selection
Upstream raw-ADC-derived replay input projection shall include only admitted observations and shall exclude rejected or malformed rows.
*Traces to: HLR-REPLAY-PROJ-002*

### LLR-REPLAY-PROJ-003: Raw ADC Source Reference Preservation
Upstream raw-ADC-derived replay input projection shall preserve a reference to the admitted source capture without defining how that reference is represented.
*Traces to: HLR-REPLAY-PROJ-003*

### LLR-REPLAY-PROJ-004: Raw ADC Observation Summary Preservation
Upstream raw-ADC-derived replay input projection shall preserve the admitted values of `sample_count`, `first_sample_index`, `last_sample_index`, `min_raw_adc`, `max_raw_adc`, and `timing_claim`.
*Traces to: HLR-REPLAY-PROJ-004*

### LLR-REPLAY-PROJ-005: Raw ADC Context Preservation
Upstream raw-ADC-derived replay input projection shall preserve `context_id` when present and omit it when absent.
*Traces to: HLR-REPLAY-PROJ-005*

### LLR-REPLAY-PROJ-006: Raw ADC Stable Projection
The same admitted raw ADC observations and metadata shall produce the same upstream raw-ADC-derived replay input.
*Traces to: HLR-REPLAY-PROJ-006*
