# High-Level Requirements - Replay Execution (HLR-REPLAY)

## 1. System Contract

### HLR-REPLAY-SYS-001: Replay System Scope
The common Replay system shall begin with canonical input for a declared replay schema and shall define the retained-run, execution, trace, comparison, evaluation, and operation contracts that consume that canonical input.

### HLR-REPLAY-SYS-002: Non-Universal Input Representations
The common Replay system shall not define math replay frames, saved-input text, raw ADC observations, raw ADC admission, or raw-ADC-derived projection as universal replay input. Those paths may produce canonical input before common Replay begins when permitted by the applicable upstream canonicalization contract for the declared replay schema.

### HLR-REPLAY-SYS-003: Retained-Run Format Ownership
The retained-run format shall own the common replay run structure shared across replay schemas without owning schema-specific canonical input meaning, upstream canonicalization or admission behavior, execution behavior, trace semantics, or comparison rules.

### HLR-REPLAY-SCHEMA-001: Schema-Owned Canonical Input Meaning
Each replay schema shall define the meaning of canonical replay input consumed under that schema without owning the route permission, mechanics, or admission rules used to produce that input. The applicable upstream canonicalization contract for the declared replay schema shall own permitted route types.

### HLR-REPLAY-SCHEMA-002: Schema-Owned State Evolution
Each replay schema shall define its replay state evolution.

### HLR-REPLAY-SCHEMA-003: Schema-Owned Trace Semantics
Each replay schema shall define its trace elements, trace order, and trace equality rules.

### HLR-REPLAY-SCHEMA-004: Schema-Owned Terminal Behavior
Each replay schema shall define terminal acceptance, stable rejection behavior, and incomplete behavior.

### HLR-REPLAY-SCHEMA-005: Schema-Owned Outcome Comparison
Each replay schema shall define functional comparison requirements for comparing retained functional reference behavior with generated functional behavior.

### HLR-REPLAY-SCHEMA-006: Stable Schema Identity
Each replay schema identity and version shall resolve to one immutable schema contract used by retained runs, replay execution, traces, comparison, and evaluation. A new schema identity or version shall be required when canonical-input meaning, required modeled-execution dependencies, state evolution, modeled-time semantics, trace meaning, trace contents, trace ordering, trace equality, terminal behavior, functional comparison semantics, or applicable separately declared timing-evaluation semantics change. Implementation changes shall not require a new schema identity or version unless they change declared schema semantics.

### HLR-REPLAY-SCHEMA-007: Schema-Declared Execution Dependencies
Each replay schema shall declare every dependency that may affect modeled execution under that schema, including canonical replay input, initial state, module selection and module versions, execution or simulation configuration, numeric policy, transition ordering, active constraints, modeled time when applicable, or other schema-declared execution dependencies it uses.

### HLR-REPLAY-SCHEMA-008: Descriptive Context Boundary
Each replay schema shall distinguish modeled-execution input and configuration, schema-defined observable functional behavior, and non-authoritative descriptive information. Non-authoritative descriptive information shall not affect replay execution, trace equality, or functional comparison. Upstream canonicalization information shall remain governed by upstream origin, parsing, admission, and projection requirements.


## 2. Upstream Canonical Input Origins

### HLR-REPLAY-ORIGIN-001: Upstream Canonicalization Route Permission
Upstream canonicalization support shall use only input-origin routes permitted by the applicable upstream canonicalization contract for the declared replay schema.

### HLR-REPLAY-ORIGIN-002: Direct Saved Replay Input Origin
Direct saved replay input shall be a valid upstream route to canonical input when allowed by the applicable upstream canonicalization contract for the declared replay schema, as demonstrated by `math-i64f64-v1`.

### HLR-REPLAY-ORIGIN-003: Projected Source Evidence Origin
Projection from admitted source evidence shall be a valid upstream route to canonical input when allowed by the applicable upstream canonicalization contract for the declared replay schema, as intended for raw ADC.

### HLR-REPLAY-ORIGIN-004: Origin-Specific Admission Boundary
Upstream canonicalization support shall require source admission only for routes that depend on admitted source evidence.

### HLR-REPLAY-ORIGIN-005: Stable Origin-Type Contract
Canonical input origin routes shall be stable defined route types. Additional route types may be defined without changing the common retained-run model.

### HLR-REPLAY-ORIGIN-006: Single Permitted Canonicalization Route
When upstream canonicalization information records the route that produced canonical input, it shall identify exactly one stable route type, and that route shall be permitted by the applicable upstream canonicalization contract for the declared replay schema.


## 3. Retained Run

### HLR-REPLAY-RUN-001: Retained Run Functional Content
A retained run shall be an immutable functional replay object containing retained-run format identity and version, replay schema identity and version, canonical input, every schema-declared modeled-execution dependency, retained functional reference material, and schema-required functional comparison parameters. Retained functional reference material shall include the schema-defined reference material required for functional comparison, including reference functional trace, reference execution disposition, and reference terminal outcome where applicable. Physical timing reference material shall not be retained functional reference material.

### HLR-REPLAY-RUN-002: Retained Run Pre-Execution Validation
Retained-run validation shall be a deterministic pre-execution operation over retained-run content and required immutable provenance associations. Validation shall check, as applicable, supported retained-run format identity and version, resolvable schema identity and version, required canonical input presence and canonical representation, presence of every schema-declared modeled-execution dependency, absence of undeclared execution dependencies in the authoritative execution binding, presence and structural validity of retained functional reference material, presence and structural validity of required functional comparison parameters, internal reference consistency, consistency between the computed retained-run identity and any stored or claimed retained-run identity, and required immutable provenance association presence and resolution when the schema explicitly requires provenance. Retained-run validation shall not validate target compatibility or physical timing limits.

### HLR-REPLAY-RUN-003: Deterministic Retained Run Identity
Functional retained-run identity shall be derived from one canonical representation of the complete identity-bearing retained-run content. Identity-bearing content shall include retained-run format identity and version, replay schema identity and version, canonical input, modeled-execution dependency bindings, retained functional reference material, and functional comparison parameters. Two retained runs with identical canonical identity input shall have the same retained-run identity. Any change to identity-bearing content shall produce different canonical identity input. When retained-run identity is represented by a digest, the digest contract shall define deterministic collision treatment.

### HLR-REPLAY-RUN-004: Retained Run Identity Exclusions
Functional retained-run identity shall not depend on file path, load time, validation time, execution time, storage location, upstream route or provenance, descriptive context, timing claims or observations, evidence limitations, target metadata, diagnostics, generated execution evidence, generated evaluations, or post-creation verification results.

### HLR-REPLAY-RUN-005: Descriptive Retained Information Boundary
Non-authoritative descriptive context, timing claims, timing observations, evidence limitations, target metadata, diagnostics, and post-creation verification results shall not be required functional retained-run content and shall not participate in functional retained-run identity. Modeled-execution input and configuration may affect replay execution only when classified as schema-declared modeled-execution dependencies. Schema-defined functional trace behavior may participate in functional trace equality. Schema-defined functional trace and terminal-outcome behavior may participate in functional comparison. Physical timing observations and timing claims shall remain separately owned by timing evidence and timing evaluation. Evidence limitations shall bound the resulting claim and shall not become functional behavior.

### HLR-REPLAY-RUN-006: Upstream Provenance Association Boundary
Upstream route, source identity, admission result, parsing details, and projection details shall not be retained-run content. When a schema or resulting claim requires upstream provenance or admission evidence, a separate immutable provenance association shall link the functional retained-run identity to the separately owned upstream record. The provenance association and referenced upstream content shall not participate in functional retained-run identity. Validation may require and resolve the provenance association before execution when the schema explicitly requires provenance. An unresolved optional provenance association shall not affect replay execution.

### HLR-REPLAY-RUN-007: Retained Run Validation Result
Retained-run validation shall produce a deterministic validation result with disposition `valid` or `invalid`. An invalid validation result shall include one or more stable machine-readable reasons, or a deterministic ordered reason set. When retained-run identity can be computed, the validation result shall include the computed retained-run identity. Validation time, checker path, storage path, and diagnostic prose shall not alter the validation result.

### HLR-REPLAY-RUN-008: Validation and Execution Separation
Only a retained run with validation disposition `valid` may enter replay execution. Retained-run validation failure shall occur before execution, shall not be an execution disposition, and shall not be execution rejection. No functional execution trace or terminal execution outcome shall be produced when execution never begins.


## 4. Upstream Parsing and Projection

### HLR-REPLAY-PARSE-001: Explicit Saved Input Version
Upstream saved-input parsing shall require saved replay input to declare a replay input format version before producing canonical input.

### HLR-REPLAY-PARSE-002: Explicit Saved Input Schema/Lane
Upstream saved-input parsing shall require saved replay input to declare the replay schema/lane it uses before producing canonical input.

### HLR-REPLAY-PARSE-003: Unknown Version Rejection
Upstream saved-input parsing shall reject unknown versions.

### HLR-REPLAY-PARSE-004: Unknown Schema/Lane Rejection
Upstream saved-input parsing shall reject unknown schema/lane values.

### HLR-REPLAY-PARSE-005: Malformed Frame Rejection
Upstream saved-input parsing shall reject malformed frame rows.

### HLR-REPLAY-PARSE-006: Deterministic Frame Production
Upstream saved-input parsing shall deterministically produce the canonical math replay frames consumed by the declared schema before common Replay execution begins.

### HLR-REPLAY-PROJ-001: Raw ADC Admission-Gated Projection
Upstream projection support shall create canonical replay input for a raw-ADC-derived replay schema only from an admitted raw ADC capture.

### HLR-REPLAY-PROJ-002: Raw ADC Admitted Observations Only
Upstream raw-ADC-derived replay input projection shall include only admitted observations; rejected or malformed rows shall remain excluded.

### HLR-REPLAY-PROJ-003: Raw ADC Admitted Capture Reference
Upstream raw-ADC-derived canonical replay input projection shall identify the admitted source capture without defining how that reference is represented.

### HLR-REPLAY-PROJ-004: Raw ADC Admitted Observation Summary Preservation
Upstream raw-ADC-derived replay input projection shall preserve `sample_count`, `first_sample_index`, `last_sample_index`, `min_raw_adc`, `max_raw_adc`, and the admitted `timing_claim`.

### HLR-REPLAY-PROJ-005: Raw ADC Optional Context Preservation
Upstream raw-ADC-derived replay input projection shall preserve `context_id` when present and shall not infer `context_id` when absent.

### HLR-REPLAY-PROJ-006: Raw ADC Deterministic Projection
Upstream raw-ADC-derived replay input projection shall be deterministic and shall not add claims beyond the admitted source evidence.


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

Once execution of a valid retained run begins, the execution shall produce exactly one disposition: `accepted`, `rejected`, or `incomplete`.

A retained run that fails validation shall not receive an execution disposition because execution did not begin.

### HLR-REPLAY-EXEC-010: Stable Rejection Reason

A replay execution with disposition `rejected` shall carry a stable schema-defined reason.

### HLR-REPLAY-EXEC-011: Incomplete Disposition Meaning

A replay execution with disposition `incomplete` shall mean execution began but reached neither schema-defined successful terminal completion nor a defined deterministic rejection.

### HLR-REPLAY-EXEC-012: Retained Run Immutability During Execution

Replay execution and generated execution evidence shall leave the retained run, retained functional reference, retained-run identity, and upstream provenance unchanged.

### HLR-REPLAY-EXEC-013: No Undeclared External Execution State

Replay execution shall not depend on undeclared external state for modeled state evolution.

### HLR-REPLAY-EXEC-014: Bound Execution Dependency Use
Replay execution of a retained run shall use the declared replay schema semantics and only execution-dependency values bound by the retained run.

### HLR-REPLAY-EXEC-015: Execution Occurrence Identity
One execution occurrence shall be one attempted execution of one retained run with validation disposition `valid`. Each execution occurrence shall have an occurrence identity that distinguishes it from other attempts of the same retained run without requiring wall-clock time as the identity source. Occurrence identity shall not affect modeled state evolution, retained-run identity, functional trace equality, or functional comparison.

### HLR-REPLAY-EXEC-016: Execution Record Content
Each execution occurrence shall produce one authoritative immutable execution record after execution begins. The execution record shall contain, as applicable, execution-record format identity and version, execution occurrence identity, retained-run identity, replay schema identity and version, an immutable reference to the retained-run validation result with disposition `valid` that authorized the execution occurrence, generated functional trace, execution disposition, generated terminal outcome, stable machine-readable execution reasons when applicable, incomplete-execution evidence, authoritative execution-context facts, physical timing observations, and immutable references to separately owned diagnostics. A storage copy or re-encoding with identical canonical record content shall not represent a new execution occurrence.

### HLR-REPLAY-EXEC-017: Execution Record Identity
Execution-record identity shall be derived from one canonical representation of identity-bearing execution-record content. Identity-bearing content shall include execution-record format identity and version, occurrence identity, retained-run identity, schema identity and version, the immutable valid-validation-result reference, generated functional trace, execution disposition, terminal outcome when present, stable machine-readable execution reasons and incomplete-execution evidence when applicable, authoritative execution-context facts, physical timing observations when present, and immutable diagnostic references. Immutable diagnostic references shall identify immutable diagnostic content independently of storage location. Referenced diagnostic content shall not participate directly in execution-record identity. Storage path, load time, check time, UI state, diagnostic paths, URLs, storage keys, mutable locators, other location-dependent values, comparison results, timing evaluation results, generated evaluations, and post-creation verification results shall not participate in execution-record identity. When execution-record identity is represented by a digest, collision handling shall follow the declared digest contract.

### HLR-REPLAY-EXEC-018: Execution Disposition Meanings
An `accepted` execution disposition shall mean schema-defined successful terminal completion. A `rejected` execution disposition shall mean schema-defined deterministic rejection after execution begins. An `incomplete` execution disposition shall mean execution began but produced no accepted or rejected terminal disposition.

### HLR-REPLAY-EXEC-019: Incomplete Execution Evidence
Incomplete execution shall retain, when available, the generated trace prefix, stable incomplete reason or deterministic ordered reason set, terminal-outcome presence or absence, and last schema-defined execution state reached.

### HLR-REPLAY-EXEC-020: Execution Context Boundary
Modeled-execution dependencies bound by the retained run may affect state evolution. Execution-context facts shall describe where and under what conditions execution occurred and shall not affect state evolution unless already declared and bound as modeled-execution dependencies. Execution records shall record applicable implementation identity and version, target identity and configuration, processor or accelerator configuration, peripheral and clock configuration, operating-system, runtime, and scheduler environment, resource limits and allocation, observation or trace configuration, and timing source and measurement points with timing resolution, accuracy, and uncertainty. Execution-context recording shall not evaluate target compatibility.

### HLR-REPLAY-EXEC-021: Physical Timing Observation Evidence
Physical timing observations shall be generated execution evidence, shall remain separate from modeled time, and shall identify source, measurement points, units, resolution, accuracy, and uncertainty when applicable. Physical timing observations shall not affect functional trace equality, functional comparison, or retained-run identity, and shall not define timing pass or fail.

### HLR-REPLAY-EXEC-022: Diagnostics Reference Boundary
Diagnostics shall remain separately owned generated artifacts. An execution record may contain immutable diagnostic references. Those references shall identify immutable diagnostic content independently of storage location and shall participate in execution-record identity. Referenced diagnostic content shall not participate directly in execution-record identity. Referenced human-readable diagnostic content shall not alter execution disposition, terminal outcome, or functional trace. Diagnostic paths, URLs, storage keys, mutable locators, and other location-dependent values shall not participate in execution-record identity. Stable machine-readable incomplete or execution reasons shall belong in the execution record.


## 6. Execution Trace

### HLR-REPLAY-TRACE-001: Ordered Observable Execution Trace
Replay execution of a retained run shall produce an ordered schema-defined generated functional trace that records observable execution and belongs to the execution record.

### HLR-REPLAY-TRACE-002: Trace, Disposition, and Terminal Outcome Separation
The generated functional trace shall record ordered observable execution, the execution disposition shall record how execution ended, and the generated terminal outcome shall belong to the execution record when present. The retained functional reference trace and retained reference outcome shall remain retained-run content.

### HLR-REPLAY-TRACE-003: Deterministic Prefix Trace
Rejected and incomplete execution may retain a deterministic generated trace prefix only when the replay schema defines that behavior.

### HLR-REPLAY-TRACE-004: Terminal Outcome Non-Trace Boundary
Terminal outcome may participate in functional comparison under schema rules, but shall not become functional trace behavior unless the schema explicitly defines it that way.


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
The `record` operation shall create a retained run from canonical input produced through a route permitted by the applicable upstream canonicalization contract for the declared replay schema.

### HLR-REPLAY-OPS-002: Replay Operation
The `replay` operation shall validate a retained run, execute it, compare generated behavior with its reference, and produce a replay evaluation.

### HLR-REPLAY-OPS-003: Diff Operation
The `diff` operation shall compare two retained runs without treating either retained run as authoritative; swapping inputs shall not change `exact`, `diverged`, or `incompatible`.

### HLR-REPLAY-OPS-004: Replay Trace Envelope Operation
The `envelope` operation shall apply a named deterministic rule to a trace and produce a judgment associated with the retained-run identity, trace origin, context, and rule.

### HLR-REPLAY-OPS-005: Raw ADC Source-Evidence Envelope Boundary
The existing raw ADC witness envelope shall remain a source-evidence judgment over admitted observations and shall not be redefined as a replay-trace envelope without an explicit requirement.


## 10. Target Agreement

### HLR-REPLAY-TGT-001: Multi-Target Replay Agreement
For a replay schema supported on multiple targets, the same retained run shall produce the same schema-defined trace, execution outcome, comparison disposition, and first divergence.

### HLR-REPLAY-TGT-002: Target Diagnostic Metadata Boundary
Target-specific diagnostic metadata may differ and shall not participate in replay equality unless the replay schema makes that metadata observable.
