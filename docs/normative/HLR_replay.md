# High-Level Requirements - Replay Execution (HLR-REPLAY)

## 1. System Contract

### HLR-REPLAY-SYS-001: Replay System Scope
The replay system shall include both retained integrated-math replay and raw-ADC-derived replay input projection without treating either proof path as the complete replay system.

### HLR-REPLAY-SYS-002: Non-Universal Input Representations
The replay system shall not define math replay frames or raw ADC observations as universal replay input.

### HLR-REPLAY-SYS-003: Retained-Run Format Ownership
The retained-run format shall own the common replay run structure shared across replay schemas.

### HLR-REPLAY-SCHEMA-001: Schema-Owned Canonical Input Meaning
Each replay schema shall define the meaning of its canonical replay input and its permitted input origins.

### HLR-REPLAY-SCHEMA-002: Schema-Owned State Evolution
Each replay schema shall define its replay state evolution.

### HLR-REPLAY-SCHEMA-003: Schema-Owned Trace Semantics
Each replay schema shall define its trace elements, trace order, and trace equality rules.

### HLR-REPLAY-SCHEMA-004: Schema-Owned Terminal Behavior
Each replay schema shall define terminal acceptance, stable rejection behavior, and incomplete behavior.

### HLR-REPLAY-SCHEMA-005: Schema-Owned Outcome Comparison
Each replay schema shall define reference and generated outcome comparison requirements.


## 2. Canonical Input Origins

### HLR-REPLAY-ORIGIN-001: Schema-Declared Input Origins
Each replay schema shall declare the input origins permitted for canonical replay input under that schema.

### HLR-REPLAY-ORIGIN-002: Direct Saved Replay Input Origin
The replay system shall support direct saved replay input as a permitted canonical input origin when allowed by the replay schema, as demonstrated by `math-i64f64-v1`.

### HLR-REPLAY-ORIGIN-003: Projected Source Evidence Origin
The replay system shall support projection from admitted source evidence as a permitted canonical input origin when allowed by the replay schema, as intended for raw ADC.

### HLR-REPLAY-ORIGIN-004: Origin-Specific Admission Boundary
Replay schemas shall require source admission only for input origins that depend on admitted source evidence.


## 3. Retained Run

### HLR-REPLAY-RUN-001: Retained Run Immutable Content
A retained run shall be an immutable replay object containing the retained-run format version, replay schema identity, canonical replay input, input-origin description, source identity and admission disposition when admission is required, applicable context, timing claims, evidence limitations, reference trace, reference execution disposition and schema-defined terminal outcome, and comparison metadata required by the schema.

### HLR-REPLAY-RUN-002: Retained Run Pre-Execution Validation
Required retained-run content shall be validated before replay execution begins.

### HLR-REPLAY-RUN-003: Deterministic Retained Run Identity
Retained-run identity shall be derived deterministically from the immutable retained-run content.

### HLR-REPLAY-RUN-004: Generated Evidence Identity Exclusion
Generated evaluations, diagnostics, target metadata, envelope judgments, and later verification results shall not change retained-run identity.


## 4. Parsing and Projection

### HLR-REPLAY-PARSE-001: Explicit Saved Input Version
Saved replay input shall declare a replay input format version.

### HLR-REPLAY-PARSE-002: Explicit Saved Input Schema/Lane
Saved replay input shall declare the replay schema/lane it uses.

### HLR-REPLAY-PARSE-003: Unknown Version Rejection
Saved replay input parsing shall reject unknown versions.

### HLR-REPLAY-PARSE-004: Unknown Schema/Lane Rejection
Saved replay input parsing shall reject unknown schema/lane values.

### HLR-REPLAY-PARSE-005: Malformed Frame Rejection
Saved replay input parsing shall reject malformed frame rows.

### HLR-REPLAY-PARSE-006: Deterministic Frame Production
Saved replay input parsing shall produce replay frames for deterministic execution.

### HLR-REPLAY-PROJ-001: Raw ADC Admission-Gated Projection
Canonical replay input for a raw-ADC-derived replay schema shall be created only from an admitted raw ADC capture.

### HLR-REPLAY-PROJ-002: Raw ADC Admitted Observations Only
Raw-ADC-derived replay input projection shall include only admitted observations; rejected or malformed rows shall remain excluded.

### HLR-REPLAY-PROJ-003: Raw ADC Admitted Capture Reference
Raw-ADC-derived canonical replay input shall identify the admitted source capture without defining how that reference is represented.

### HLR-REPLAY-PROJ-004: Raw ADC Admitted Observation Summary Preservation
Raw-ADC-derived replay input projection shall preserve `sample_count`, `first_sample_index`, `last_sample_index`, `min_raw_adc`, `max_raw_adc`, and the admitted `timing_claim`.

### HLR-REPLAY-PROJ-005: Raw ADC Optional Context Preservation
Raw-ADC-derived replay input projection shall preserve `context_id` when present and shall not infer `context_id` when absent.

### HLR-REPLAY-PROJ-006: Raw ADC Deterministic Projection
Raw-ADC-derived replay input projection shall be deterministic and shall not add claims beyond the admitted source evidence.


## 5. Deterministic Execution

### HLR-REPLAY-EXEC-001: Deterministic Ordered Evaluation
Replay execution shall evaluate an ordered sequence of replay frames deterministically.

### HLR-REPLAY-EXEC-002: Initial Math Lane
Replay execution shall support an initial math lane using existing `I64F64` behavior.

### HLR-REPLAY-EXEC-003: Invalid Order Rejection
Replay execution shall reject invalid execution order.

### HLR-REPLAY-EXEC-004: Expected Result Rejection
Replay execution shall reject expected-result mismatch.

### HLR-REPLAY-EXEC-005: Deterministic Execution Result
Replay execution shall expose a deterministic execution result.

### HLR-REPLAY-EXEC-006: Arithmetic Trap Rejection
Replay execution shall reject deterministically when an initial math lane operation would trigger an existing `I64F64` arithmetic trap.

### HLR-REPLAY-EXEC-007: Schema-Declared Canonical Input Execution
Replay execution of a retained run shall consume canonical replay input under the retained run's declared schema.

### HLR-REPLAY-EXEC-008: Schema-Ordered Deterministic Processing
Replay execution of a retained run shall process canonical replay input deterministically and in the order defined by the replay schema.

### HLR-REPLAY-EXEC-009: Replay Execution Disposition
Replay execution of a retained run shall produce an execution disposition of `accepted`, `rejected`, or `incomplete`.

### HLR-REPLAY-EXEC-010: Stable Rejection Reason
A rejected replay execution outcome shall carry a stable schema-defined reason.

### HLR-REPLAY-EXEC-011: Incomplete Outcome Meaning
An incomplete replay execution outcome shall mean execution reached neither terminal acceptance nor a defined rejection.

### HLR-REPLAY-EXEC-012: Retained Run Immutability During Execution
Replay execution shall leave the retained run unchanged.


## 6. Execution Trace

### HLR-REPLAY-TRACE-001: Ordered Observable Execution Trace
Replay execution of a retained run shall produce an ordered schema-defined trace that records observable execution.

### HLR-REPLAY-TRACE-002: Trace and Disposition Separation
The execution trace shall record ordered observable execution, while the execution disposition shall record how execution ended.

### HLR-REPLAY-TRACE-003: Deterministic Prefix Trace
Rejected and incomplete execution may retain a deterministic trace prefix only when the replay schema defines that behavior.


## 7. Comparison

### HLR-REPLAY-COMP-001: Trace Comparison
Replay comparison shall compare the generated trace against the retained reference trace.

### HLR-REPLAY-COMP-002: Outcome Comparison
Replay comparison shall compare the generated execution disposition and schema-defined terminal outcome against the retained reference disposition and outcome.

### HLR-REPLAY-COMP-003: Comparison Dispositions
Replay comparison shall produce `exact` when trace and execution outcome are equal, `diverged` when comparison is compatible but trace or outcome differs, and `incompatible` when required format, schema, version, or comparison metadata is incompatible.

### HLR-REPLAY-COMP-004: First Divergence Reporting
Replay comparison shall report the first differing trace position; a prefix-length difference shall diverge at the first missing position, and matching traces with different terminal outcomes shall diverge after the trace.

### HLR-REPLAY-COMP-005: Expected Rejection Exactness
Matching expected rejection may compare as `exact`.

### HLR-REPLAY-COMP-006: Incomplete Reference Exactness Boundary
Incomplete outcomes may compare as `exact` only when the replay schema permits incomplete reference outcomes.

### HLR-REPLAY-COMP-007: Comparison-Owned Divergence
First divergence shall belong to comparison, not execution rejection.


## 8. Replay Evaluation and Witness

### HLR-REPLAY-EVAL-001: Generated Replay Evaluation Evidence
A replay evaluation shall be generated evidence associated with a retained-run identity.

The associated retained run shall determine the replay schema identity for the evaluation.

The replay evaluation shall contain the generated trace, generated execution disposition and outcome, comparison disposition, first divergence when present, and target identity when applicable.

### HLR-REPLAY-EVAL-002: Evaluation Non-Mutation
A replay evaluation shall not mutate the retained run.

### HLR-REPLAY-EVAL-003: Checker Text Evidence Boundary
Checker text shall represent evaluation evidence and shall not define execution or comparison semantics.

### HLR-REPLAY-CHECK-001: Retained Saved Input Source Check
A retained saved replay input shall be checked from source files.

### HLR-REPLAY-CHECK-002: Checker Parse Stage
Replay checking shall parse the saved input.

### HLR-REPLAY-CHECK-003: Checker Replay Stage
Replay checking shall execute the parsed frames deterministically.

### HLR-REPLAY-CHECK-004: Deterministic Witness Generation
Replay checking shall generate deterministic witness output.

### HLR-REPLAY-CHECK-005: Expected Witness Comparison
Replay checking shall compare generated witness output to retained expected witness.

### HLR-REPLAY-CHECK-006: Expected Result Comparison
Replay checking shall compare generated checker result to retained expected result.

### HLR-REPLAY-CHECK-007: Witness Mismatch Failure
Replay checking shall fail when expected witness does not match generated witness.

### HLR-REPLAY-CHECK-008: Result Mismatch Failure
Replay checking shall fail when generated result does not match retained expected result.

### HLR-REPLAY-CHECK-009: Exact Checker Input Path Validation
The checked-in replay checker entrypoint shall require exactly one replay input path.

### HLR-REPLAY-CHECK-010: Deterministic Checker Failure Diagnostics
The checked-in replay checker entrypoint shall report invalid invocation, input read failure, parse rejection, and replay rejection with deterministic diagnostics and exit codes.


## 9. Run Operations

### HLR-REPLAY-OPS-001: Record Operation
The `record` operation shall create a retained run from direct canonical input or admitted and projected source evidence, as permitted by the replay schema.

### HLR-REPLAY-OPS-002: Replay Operation
The `replay` operation shall validate a retained run, execute it, compare generated behavior with its reference, and produce a replay evaluation.

### HLR-REPLAY-OPS-003: Diff Operation
The `diff` operation shall compare two retained runs without treating either retained run as authoritative; swapping inputs shall not change `exact`, `diverged`, or `incompatible`.

### HLR-REPLAY-OPS-004: Replay Trace Envelope Operation
The `envelope` operation shall apply a named deterministic rule to a trace and produce a judgment associated with the retained-run identity, trace origin, context, and rule.

### HLR-REPLAY-OPS-005: Raw ADC Source-Evidence Envelope Boundary
The existing raw ADC witness envelope shall remain a source-evidence judgment over admitted observations and shall not be redefined as a replay-trace envelope without an explicit future requirement.


## 10. Target Agreement

### HLR-REPLAY-TGT-001: Multi-Target Replay Agreement
For a replay schema supported on multiple targets, the same retained run shall produce the same schema-defined trace, execution outcome, comparison disposition, and first divergence.

### HLR-REPLAY-TGT-002: Target Diagnostic Metadata Boundary
Target-specific diagnostic metadata may differ and shall not participate in replay equality unless the replay schema makes that metadata observable.
