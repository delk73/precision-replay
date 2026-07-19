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
Each retained run shall contain retained-run format identity and version, replay schema identity and version, canonical input, modeled-execution dependency bindings, retained functional reference material, and schema-required functional comparison parameters.
*Traces to: HLR-REPLAY-RUN-001*

### LLR-REPLAY-RUN-002: Pre-Replay Validation Checks
Retained-run validation shall check supported retained-run format identity and version, resolvable schema identity and version, required canonical input presence and canonical representation, every schema-declared modeled-execution dependency, absence of undeclared execution dependencies in the authoritative execution binding, retained functional reference presence and structural validity, required functional comparison parameter presence and structural validity, internal reference consistency, consistency between the computed retained-run identity and any stored or claimed retained-run identity, required immutable provenance association presence, and required immutable provenance association resolution.
*Traces to: HLR-REPLAY-RUN-002*

### LLR-REPLAY-RUN-003: Retained Content Immutability
Retained-run content shall not change after retained-run creation.
*Traces to: HLR-REPLAY-RUN-001*

### LLR-REPLAY-RUN-004: Retained Run Identity Derivation
Retained-run identity shall be calculated from one canonical representation of identity-bearing retained-run content. Map iteration, field order outside that canonical representation, platform encoding, file layout, storage path, file path, load time, validation time, and execution time shall not alter the canonical identity input. When retained-run identity is represented by a digest, collision handling shall follow the declared digest contract.
*Traces to: HLR-REPLAY-RUN-003, HLR-REPLAY-RUN-004*

### LLR-REPLAY-RUN-005: Retained Run Identity Exclusions
Upstream route or provenance, provenance associations, descriptive context, timing claims, timing observations, evidence limitations, target metadata, diagnostics, generated execution evidence, generated evaluations, and post-creation verification results shall not participate in functional retained-run identity.
*Traces to: HLR-REPLAY-RUN-004, HLR-REPLAY-RUN-005, HLR-REPLAY-RUN-006*

### LLR-REPLAY-RUN-006: Retained Execution Dependency Binding
Each retained run shall bind every modeled-execution dependency used by its declared replay schema, and the authoritative execution binding shall not contain undeclared execution dependencies.
*Traces to: HLR-REPLAY-RUN-001, HLR-REPLAY-RUN-002, HLR-REPLAY-SCHEMA-007, HLR-REPLAY-EXEC-014*

### LLR-REPLAY-RUN-007: Descriptive Information Non-Execution
Non-authoritative descriptive information, timing claims, timing observations, evidence limitations, target metadata, diagnostics, generated execution evidence, generated evaluations, and post-creation verification results shall not affect replay execution. Information required by execution shall instead be classified and bound as schema-declared modeled-execution input or configuration.
*Traces to: HLR-REPLAY-RUN-005, HLR-REPLAY-SCHEMA-008*

### LLR-REPLAY-RUN-008: Descriptive Information Non-Comparison
Non-authoritative descriptive information shall not affect functional trace equality or functional comparison. Only schema-defined functional trace behavior may participate in functional trace equality. Only schema-defined functional trace or terminal-outcome behavior may participate in functional comparison. Timing observations and timing claims shall remain separately owned by timing evidence and timing evaluation. Evidence limitations shall remain claim-bounding information. Timing information and evidence limitations shall not become functional behavior through comparison.
*Traces to: HLR-REPLAY-RUN-005, HLR-REPLAY-SCHEMA-008*

### LLR-REPLAY-RUN-009: Identity-Bearing Field Classification
Identity-bearing functional retained-run content shall consist of retained-run format identity and version, replay schema identity and version, canonical input, modeled-execution dependency binding, retained functional reference material, and functional comparison parameters.
*Traces to: HLR-REPLAY-RUN-001, HLR-REPLAY-RUN-003*

### LLR-REPLAY-RUN-010: Separate Provenance Association
Required provenance or admission evidence shall remain separately owned upstream material linked to the functional retained-run identity by a separate immutable provenance association. Descriptive context, timing claims, and evidence limitations shall be non-functional material when present. Provenance associations, referenced upstream material, descriptive context, timing claims, and evidence limitations shall not participate in functional retained-run identity.
*Traces to: HLR-REPLAY-RUN-005, HLR-REPLAY-RUN-006*

### LLR-REPLAY-RUN-011: Generated Material Exclusion
Execution records, generated traces, generated timing observations, diagnostics, comparison results, timing results, generated evaluations, and post-creation verification results shall be excluded from functional retained-run content.
*Traces to: HLR-REPLAY-RUN-004, HLR-REPLAY-RUN-005*

### LLR-REPLAY-RUN-012: Stable Validation Reasons
Invalid retained-run validation shall distinguish stable reasons for unsupported retained-run format, unknown or unsupported schema, missing canonical input, malformed canonical input representation, missing execution dependency, undeclared authoritative execution dependency, missing functional reference, malformed functional reference, missing comparison parameter, malformed comparison parameter, inconsistent stored or claimed retained-run identity relative to the computed retained-run identity, missing required provenance association, and unresolved required immutable provenance association.
*Traces to: HLR-REPLAY-RUN-002, HLR-REPLAY-RUN-007*

### LLR-REPLAY-RUN-013: Deterministic Validation Result
A retained-run validation result shall contain the computed retained-run identity when identity can be computed, validation disposition, and stable reason or deterministic ordered reason set when invalid. Validation time, checker path, storage path, and diagnostic prose shall not alter the validation result.
*Traces to: HLR-REPLAY-RUN-007*

### LLR-REPLAY-RUN-014: Validated Execution Gate
Retained-run execution shall require a retained-run validation result with disposition `valid`. Validation disposition `invalid` shall prevent execution and shall not be represented as `accepted`, `rejected`, `incomplete`, functional trace output, or terminal execution outcome.
*Traces to: HLR-REPLAY-RUN-008, HLR-REPLAY-EXEC-007, HLR-REPLAY-EXEC-009*


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
Replay execution shall not read clocks, files, environment variables, target metadata, diagnostics, post-creation verification results, or other undeclared external state to determine state evolution.
*Traces to: HLR-REPLAY-EXEC-013*

### LLR-REPLAY-EXEC-009: Validated Execution Occurrence
An execution occurrence shall be created only for an attempted execution of one retained run with validation disposition `valid`.
*Traces to: HLR-REPLAY-RUN-008, HLR-REPLAY-EXEC-015*

### LLR-REPLAY-EXEC-010: Occurrence Identity Boundary
Occurrence identity shall distinguish separate execution attempts of the same retained run. Its construction shall not alter retained-run identity or affect modeled state evolution, functional trace equality, or functional comparison. Wall-clock time shall not be required as its source.
*Traces to: HLR-REPLAY-EXEC-015*

### LLR-REPLAY-EXEC-011: One Execution Record Per Occurrence
Each execution occurrence shall produce exactly one authoritative immutable execution record after execution begins, and a storage copy or re-encoding with identical canonical record content shall not create another execution occurrence.
*Traces to: HLR-REPLAY-EXEC-016, HLR-REPLAY-EXEC-012*

### LLR-REPLAY-EXEC-012: Execution Record Content
An execution record shall contain, when applicable, execution-record format identity and version, execution occurrence identity, retained-run identity, replay schema identity and version, an immutable reference to the retained-run validation result with disposition `valid` that authorized the execution occurrence, generated functional trace, execution disposition, generated terminal outcome, stable machine-readable execution reasons when applicable, incomplete-execution evidence, authoritative execution-context facts, physical timing observations, and immutable diagnostic references.
*Traces to: HLR-REPLAY-EXEC-016*

### LLR-REPLAY-EXEC-013: Execution Record Canonical Identity
Execution-record identity shall be calculated from one canonical representation of identity-bearing execution-record content. Map iteration, field order outside that canonical representation, platform encoding, file layout, storage path, load time, and check time shall not alter the canonical execution-record identity input. When execution-record identity is represented by a digest, collision handling shall follow the declared digest contract.
*Traces to: HLR-REPLAY-EXEC-017*

### LLR-REPLAY-EXEC-014: Execution Record Identity Field Classification
Identity-bearing execution-record content shall consist of execution-record format identity and version, occurrence identity, retained-run identity, schema identity and version, the immutable valid-validation-result reference, generated trace, execution disposition, terminal outcome when present, stable machine-readable execution reasons and incomplete-execution evidence when applicable, authoritative execution-context facts, physical timing observations when present, and immutable diagnostic references. Immutable diagnostic references shall identify immutable diagnostic content independently of storage location. Referenced diagnostic content shall not participate directly in execution-record identity. Storage path, load time, check time, UI state, diagnostic paths, URLs, storage keys, mutable locators, other location-dependent values, comparison results, timing evaluation results, generated evaluations, and post-creation verification results shall be excluded from execution-record identity.
*Traces to: HLR-REPLAY-EXEC-017, HLR-REPLAY-EXEC-022*

### LLR-REPLAY-EXEC-015: Execution Disposition Meanings
Execution disposition `accepted` shall mean schema-defined successful terminal completion, `rejected` shall mean schema-defined deterministic rejection after execution begins, and `incomplete` shall mean execution began but produced no accepted or rejected terminal disposition. Validation failure shall not be represented by any execution disposition.
*Traces to: HLR-REPLAY-EXEC-009, HLR-REPLAY-EXEC-011, HLR-REPLAY-EXEC-018, HLR-REPLAY-RUN-008*

### LLR-REPLAY-EXEC-016: Incomplete Execution Evidence
An incomplete execution record shall retain, when available, generated trace prefix, stable incomplete reason or deterministic ordered reason set, terminal-outcome presence or absence, and last schema-defined execution state reached.
*Traces to: HLR-REPLAY-EXEC-019, HLR-REPLAY-TRACE-003*

### LLR-REPLAY-EXEC-017: Generated Trace and Terminal Outcome Ownership
The generated functional trace and generated terminal outcome shall belong to the execution record. The retained functional reference trace and retained reference outcome shall remain retained-run content. Terminal outcome may participate in functional comparison under schema rules but shall not be treated as functional trace behavior unless the schema explicitly defines it that way.
*Traces to: HLR-REPLAY-TRACE-001, HLR-REPLAY-TRACE-002, HLR-REPLAY-TRACE-004*

### LLR-REPLAY-EXEC-018: Execution Context Classification
Execution-context facts shall describe where and under what conditions execution occurred and shall not affect state evolution unless declared and bound as modeled-execution dependencies. Execution-context facts shall include applicable implementation identity and version, target identity and configuration, processor or accelerator configuration, peripheral and clock configuration, operating-system, runtime, and scheduler environment, resource limits and allocation, observation or trace configuration, and timing source and measurement points with timing resolution, accuracy, and uncertainty.
*Traces to: HLR-REPLAY-EXEC-013, HLR-REPLAY-EXEC-014, HLR-REPLAY-EXEC-020*

### LLR-REPLAY-EXEC-019: Physical Timing Observation Content
Physical timing observations shall be generated execution evidence separate from modeled time and shall record source, measurement points, units, resolution, accuracy, and uncertainty when applicable. Physical timing observations shall not affect functional trace equality, functional comparison, or retained-run identity. Physical timing observations shall not themselves define timing pass or fail; a separate timing evaluation may use them as evidence.
*Traces to: HLR-REPLAY-EXEC-021, HLR-REPLAY-SCHEMA-006, HLR-REPLAY-RUN-004*

### LLR-REPLAY-EXEC-020: Diagnostic Reference Boundary
Diagnostics shall remain separately owned generated artifacts. An execution record may contain immutable diagnostic references. Those references shall identify immutable diagnostic content independently of storage location and shall participate in execution-record identity. Referenced diagnostic content shall not participate directly in execution-record identity. Referenced human-readable diagnostic content shall not alter execution disposition, terminal outcome, or functional trace. Diagnostic paths, URLs, storage keys, mutable locators, and other location-dependent values shall not participate in execution-record identity. Stable machine-readable incomplete or execution reasons shall be retained in the execution record.
*Traces to: HLR-REPLAY-EXEC-022, HLR-REPLAY-EXEC-010, HLR-REPLAY-EXEC-019*

### LLR-REPLAY-EXEC-021: Retained Run Immutability During Execution
Execution occurrences, execution records, generated traces, physical timing observations, and diagnostic references shall not mutate the retained run, retained functional reference, retained-run identity, or upstream provenance.
*Traces to: HLR-REPLAY-EXEC-012, HLR-REPLAY-RUN-001, HLR-REPLAY-RUN-003, HLR-REPLAY-RUN-006*


## 7. Functional Comparison

### LLR-REPLAY-COMP-001: Comparison Inputs
Functional comparison shall consume one retained run, one execution record generated for that retained run, the retained run's replay schema identity and version, retained functional reference trace, retained reference execution disposition, retained reference terminal outcome when applicable, generated functional trace, generated execution disposition, generated terminal outcome when applicable, and retained functional comparison parameters.
*Traces to: HLR-REPLAY-COMP-001*

### LLR-REPLAY-COMP-002: Retained-Run and Execution-Record Association
Functional comparison shall associate the execution record to the retained run by retained-run identity and replay schema identity/version. A missing or mismatched association shall produce comparison disposition `incompatible`.
*Traces to: HLR-REPLAY-COMP-001, HLR-REPLAY-COMP-002*

### LLR-REPLAY-COMP-003: Compatibility Checks
Functional comparison shall check that the retained run and execution record use compatible replay schema identity/version values, that the schema-defined comparison rules are available, and that required retained functional reference material and retained comparison parameters are present in comparison-usable form. Compatibility checking shall not execute or validate the retained run, shall not validate target compatibility, and shall not evaluate physical timing.
*Traces to: HLR-REPLAY-COMP-002*

### LLR-REPLAY-COMP-004: Trace Comparison
For compatible inputs, functional comparison shall compare generated functional trace against retained reference trace using the declared replay schema's trace order and trace equality rules with the retained comparison parameters.
*Traces to: HLR-REPLAY-COMP-001, HLR-REPLAY-COMP-003*

### LLR-REPLAY-COMP-005: Outcome Comparison
For compatible inputs, functional comparison shall compare generated execution disposition and generated terminal outcome against retained reference execution disposition and retained reference terminal outcome using the declared replay schema and retained comparison parameters. Outcome comparison shall not redefine the execution disposition recorded by execution.
*Traces to: HLR-REPLAY-COMP-001, HLR-REPLAY-COMP-003*

### LLR-REPLAY-COMP-006: Expected Rejection and Incomplete Exactness
A generated rejection may compare as `exact` when it matches retained reference rejection behavior under schema rules and retained comparison parameters. Generated incomplete behavior may compare as `exact` only when the replay schema permits incomplete reference behavior and the generated incomplete behavior matches the retained incomplete reference.
*Traces to: HLR-REPLAY-COMP-005, HLR-REPLAY-COMP-006*

### LLR-REPLAY-COMP-007: First-Divergence Selection
For comparison disposition `diverged`, functional comparison shall report the earliest schema-ordered mismatch. A differing trace element shall be reported at its trace position, a trace-prefix length mismatch shall be reported at the first missing trace position, and an execution disposition or terminal-outcome mismatch after equal traces shall be reported after the trace. First-divergence reporting shall belong to comparison and shall not create or change an execution rejection reason.
*Traces to: HLR-REPLAY-COMP-004*

### LLR-REPLAY-COMP-008: Mismatch Evidence
A `diverged` comparison result shall include stable mismatch evidence sufficient to identify whether the first divergence is a trace-element mismatch, trace-prefix length mismatch, execution-disposition mismatch, or terminal-outcome mismatch, including the relevant generated and retained values or stable references to them when applicable.
*Traces to: HLR-REPLAY-COMP-004*

### LLR-REPLAY-COMP-009: Comparison Result Content
A functional comparison result shall include comparison-result format identity and version, retained-run identity, execution-record identity, replay schema identity and version, comparison disposition, stable incompatibility reason or deterministic ordered reason set when disposition is `incompatible`, and first-divergence evidence when disposition is `diverged`. The comparison result shall not define the final replay evaluation.
*Traces to: HLR-REPLAY-COMP-002, HLR-REPLAY-COMP-003, HLR-REPLAY-COMP-004*

### LLR-REPLAY-COMP-010: Deterministic Non-Mutation
Functional comparison shall be deterministic over its inputs and shall not mutate replay execution, the execution record, the retained run, retained functional reference material, retained comparison parameters, retained-run identity, execution-record identity, target-profile information, physical timing observations, diagnostics, or upstream provenance.
*Traces to: HLR-REPLAY-COMP-007*


## 8. Target Execution Profile

### LLR-REPLAY-TPROF-001: Target Profile Applicability
A target execution profile shall be optional unless required by the replay schema, schema version, operation, or requested target-specific or physical-timing claim. Profile-free execution shall remain permitted only when the schema and requested claim permit execution without a target execution profile.
*Traces to: HLR-REPLAY-TPROF-001, HLR-REPLAY-TPROF-002, HLR-REPLAY-TPROF-007*

### LLR-REPLAY-TPROF-002: Target Profile Content
A target execution profile shall declare target-specific execution conditions required by the profile-bound claim and shall include a target-profile format identity and version, target-profile schema or applicability declaration, declared replay schema applicability when constrained, and the declared condition categories required by this contract.
*Traces to: HLR-REPLAY-TPROF-001, HLR-REPLAY-TPROF-003*

### LLR-REPLAY-TPROF-003: Target Profile Identity Fields
Identity-bearing target-profile content shall consist of target-profile format identity and version, target-profile schema or applicability declaration, replay schema applicability constraints when present, declared target identity and configuration, implementation and runtime identity constraints, processor, accelerator, peripheral, and clock configuration constraints, operating-system, runtime, and scheduler environment constraints when applicable, resource limits and allocation constraints, applicable timing condition constraints, timing source and measurement constraints, observation or trace configuration constraints, and target-profile context-compatibility policy fields.
*Traces to: HLR-REPLAY-TPROF-003, HLR-REPLAY-TPROF-004*

### LLR-REPLAY-TPROF-004: Target Profile Identity Exclusions
Target-profile identity shall exclude retained-run identity, execution-record identity, execution occurrence identity, generated functional trace, execution disposition, generated terminal outcome, physical timing observations, functional comparison results, timing evaluation results, replay evaluation results, diagnostics, storage path, load time, validation time, execution time, mutable locators, and post-creation verification results. Target-profile identity shall not participate in retained-run identity and shall not define execution-record identity.
*Traces to: HLR-REPLAY-TPROF-004, HLR-REPLAY-RUN-004, HLR-REPLAY-EXEC-017*

### LLR-REPLAY-TPROF-005: Target and Runtime Conditions
A target execution profile shall declare target identity and configuration, implementation identity and version, runtime identity and version when applicable, processor or accelerator configuration, peripheral configuration, clock configuration, and operating-system, runtime, or scheduler environment constraints when those conditions are applicable to the target-specific claim.
*Traces to: HLR-REPLAY-TPROF-003*

### LLR-REPLAY-TPROF-006: Resource Conditions
A target execution profile shall declare applicable resource limits and allocation constraints, including memory, stack, heap, storage, device, peripheral, accelerator, scheduling, priority, concurrency, or other resource constraints when those conditions are applicable to the target-specific claim.
*Traces to: HLR-REPLAY-TPROF-003*

### LLR-REPLAY-TPROF-007: Timing and Measurement Conditions
A target execution profile shall declare timing deadlines, latency, interval, jitter, throughput, and synchronization tolerances when applicable, and shall declare timing source, measurement points, units, resolution, accuracy, and uncertainty required to support physical-timing claims. Declaring these conditions shall not make physical timing observations pass or fail; timing-result evaluation remains separately owned.
*Traces to: HLR-REPLAY-TPROF-003, HLR-REPLAY-TPROF-006, HLR-REPLAY-EXEC-021*

### LLR-REPLAY-TPROF-008: Observation and Trace Conditions
A target execution profile shall declare observation and trace configuration required for the target-specific claim, including probe, logging, sampling, buffering, instrumentation, trace-level, collection, or synchronization configuration when applicable. Observation configuration shall not alter schema-defined functional trace equality unless the replay schema makes that information observable.
*Traces to: HLR-REPLAY-TPROF-003, HLR-REPLAY-TGT-002*

### LLR-REPLAY-TPROF-009: Execution Context Validation Inputs
Execution-context validation shall consume the retained-run validation result with disposition `valid`, retained-run identity and schema identity, replay schema target-profile rules, declared execution-context facts, implementation support declarations, and the target execution profile when a target execution profile is required. Retained-run validation shall remain complete before execution-context validation begins.
*Traces to: HLR-REPLAY-TPROF-002, HLR-REPLAY-TPROF-005, HLR-REPLAY-RUN-008*

### LLR-REPLAY-TPROF-010: Deterministic Context Compatibility Result
Execution-context validation shall produce a deterministic result with disposition `compatible` or `incompatible`. When a target execution profile is required, the compatibility result shall include the computed target-profile identity for the exact immutable target profile used. An incompatible result shall include a stable reason or deterministic ordered reason set. Validation time, checker path, storage path, diagnostic prose, and mutable locators shall not alter the compatibility result.
*Traces to: HLR-REPLAY-TPROF-005*

### LLR-REPLAY-TPROF-011: Stable Context Incompatibility Reasons
Stable execution-context incompatibility reasons shall distinguish missing required target profile, unsupported target-profile format, unsupported target-profile applicability, retained-run/schema mismatch, implementation unsupported, target identity mismatch, target configuration mismatch, runtime or implementation mismatch, processor or accelerator mismatch, peripheral or clock mismatch, OS/runtime/scheduler mismatch, resource constraint mismatch, timing condition mismatch, measurement configuration mismatch, observation or trace configuration mismatch, and unsupported profile-free execution.
*Traces to: HLR-REPLAY-TPROF-005, HLR-REPLAY-TPROF-006*

### LLR-REPLAY-TPROF-012: Context Incompatibility Separation
A context result with disposition `incompatible` shall not change the retained-run validation result, shall not be represented as execution disposition `accepted`, `rejected`, or `incomplete`, shall not be a replay rejection reason, shall not alter functional comparison, shall not define timing pass or fail, and shall not mutate retained-run identity, target-profile identity, or execution-record identity.
*Traces to: HLR-REPLAY-TPROF-006, HLR-REPLAY-RUN-008, HLR-REPLAY-EXEC-009, HLR-REPLAY-EXEC-017*

### LLR-REPLAY-TPROF-013: Profile-Free Execution Boundary
When profile-free execution is permitted by the replay schema and requested claim, execution may proceed without target-profile compatibility validation against a target profile. The resulting execution shall not claim compatibility with a target profile and shall not make target-specific or physical-timing claims that require a compatible target execution profile.
*Traces to: HLR-REPLAY-TPROF-001, HLR-REPLAY-TPROF-007*


## 9. Physical Timing Evaluation

### LLR-REPLAY-TIME-001: Timing Evaluation Inputs
Physical timing evaluation shall consume the replay schema identity and version, the requested timing claim or schema timing requirement, one execution record, the physical timing observations from that execution record, the applicable target execution profile, and the execution-context compatibility result for that profile when profile-bound timing evaluation is required.
*Traces to: HLR-REPLAY-TIME-001, HLR-REPLAY-TIME-002*

### LLR-REPLAY-TIME-002: Target Profile and Execution Record Association
Physical timing evaluation shall associate the execution record to the applicable target execution profile by the target-profile identity reported by the compatible execution-context validation result. A missing or mismatched association shall prevent timing `pass` or `fail`.
*Traces to: HLR-REPLAY-TIME-002, HLR-REPLAY-TIME-003*

### LLR-REPLAY-TIME-003: Compatible Context Prerequisite
Profile-bound physical timing evaluation shall require an execution-context validation result with disposition `compatible` for the execution record and exact immutable target execution profile used by the timing evaluation. A missing, incompatible, or unusable context result shall produce timing disposition `insufficient`.
*Traces to: HLR-REPLAY-TIME-003, HLR-REPLAY-TIME-005*

### LLR-REPLAY-TIME-004: Measurement and Limit Comparison
For compatible profile-bound inputs, physical timing evaluation shall compare each required usable physical timing observation against the corresponding timing deadline, latency, interval, jitter, throughput, or synchronization limit declared by the applicable target execution profile. Timing evaluation shall use the units, measurement points, and timing source declared by the observation and required by the profile.
*Traces to: HLR-REPLAY-TIME-002, HLR-REPLAY-TIME-004*

### LLR-REPLAY-TIME-005: Uncertainty Treatment
Physical timing evaluation shall apply declared measurement resolution, accuracy, and uncertainty before deciding `pass` or `fail`. If uncertainty permits both a conforming and non-conforming interpretation for a required limit, the result shall be `insufficient` unless the replay schema or requested claim declares a deterministic conservative rule for that timing evaluation.
*Traces to: HLR-REPLAY-TIME-004, HLR-REPLAY-TIME-005*

### LLR-REPLAY-TIME-006: Missing or Unusable Evidence Handling
Physical timing evaluation shall produce timing disposition `insufficient` when required physical timing observations, profile timing limits, measurement metadata, timing semantics, or profile association evidence are missing, unsupported, inconsistent, or unusable for the requested timing claim.
*Traces to: HLR-REPLAY-TIME-005*

### LLR-REPLAY-TIME-007: Stable Timing Result Content
A timing evaluation result shall include timing-result format identity and version, replay schema identity and version, execution-record identity, target-profile identity when profile-bound, requested timing claim or schema timing requirement, timing disposition, stable reason or deterministic ordered reason set when disposition is `fail` or `insufficient`, and stable references to the observations and profile limits used when applicable. The timing evaluation result shall not define the final replay evaluation or claim package.
*Traces to: HLR-REPLAY-TIME-004, HLR-REPLAY-TIME-005*

### LLR-REPLAY-TIME-008: Deterministic Non-Mutation
Physical timing evaluation shall be deterministic over its inputs and shall not mutate retained-run validation, replay execution, execution disposition, execution-record content, physical timing observations, functional comparison, retained-run identity, execution-record identity, target-profile identity, target-profile content, diagnostics, or upstream provenance.
*Traces to: HLR-REPLAY-TIME-006*


## 10. Generated Replay Evaluation

### LLR-REPLAY-EVAL-001: Evaluation Inputs
Replay evaluation shall consume the retained-run identity, the replay schema identity and version checked from the retained run, one execution record, one functional comparison result, the requested claim or evaluation scope, the timing result when physical timing evaluation applies, target-profile and execution-context references when required by the claim, and evidence-limit or claim-boundary inputs that are separate from retained-run functional content. Evaluation shall not independently select the replay schema identity.
*Traces to: HLR-REPLAY-EVAL-001, HLR-REPLAY-EVAL-002, HLR-REPLAY-EVAL-003*

### LLR-REPLAY-EVAL-002: Retained Run, Execution, and Comparison Association Checks
Replay evaluation shall check that the retained run, execution record, and functional comparison result identify the same retained-run identity and compatible replay schema identity/version, and that the functional comparison result identifies the execution-record identity being evaluated. A missing or mismatched retained-run, schema, execution-record, or comparison-result association shall produce evaluation disposition `invalid` with a stable reason or deterministic ordered reason set and shall not alter the referenced execution or comparison disposition.
*Traces to: HLR-REPLAY-EVAL-001, HLR-REPLAY-EVAL-004, HLR-REPLAY-EVAL-006*

### LLR-REPLAY-EVAL-003: Optional Timing Result Handling
When the replay schema or requested claim requires physical timing evaluation, replay evaluation shall include the timing-result identity or stable timing-result reference and timing disposition. When timing evaluation does not apply, replay evaluation shall omit timing disposition or record a stable not-applicable timing marker without inventing timing pass, fail, or insufficient.
*Traces to: HLR-REPLAY-EVAL-002, HLR-REPLAY-TIME-001, HLR-REPLAY-TIME-004*

### LLR-REPLAY-EVAL-004: Timing Association Checks
When a timing result is required, replay evaluation shall check that the timing result identifies the execution-record identity and replay schema identity/version being evaluated. When the timing result is profile-bound, replay evaluation shall check that the timing result identifies the applicable target-profile identity. A missing or mismatched required timing association shall produce evaluation disposition `invalid` with a stable reason or deterministic ordered reason set and shall not alter timing disposition.
*Traces to: HLR-REPLAY-EVAL-002, HLR-REPLAY-EVAL-004, HLR-REPLAY-EVAL-006, HLR-REPLAY-TIME-006*

### LLR-REPLAY-EVAL-005: Target Profile and Context References
When a target execution profile or execution-context compatibility result is required for the requested claim, replay evaluation shall include stable references to the applicable target-profile identity and execution-context compatibility result. Replay evaluation shall check that the target-profile reference identifies the applicable target-profile identity and that the execution-context compatibility result identifies the execution context being evaluated and the applicable target-profile identity. A missing or mismatched required target-profile or execution-context compatibility association shall produce evaluation disposition `invalid` with a stable reason or deterministic ordered reason set. Those references shall not copy, redefine, or validate target-profile content or execution-context compatibility.
*Traces to: HLR-REPLAY-EVAL-002, HLR-REPLAY-EVAL-004, HLR-REPLAY-EVAL-005, HLR-REPLAY-TPROF-005*

### LLR-REPLAY-EVAL-006: Evidence Limitation and Claim Boundary Representation
Replay evaluation shall represent evidence limitations and claim boundaries as stable machine-readable limitation identifiers or a deterministic ordered limitation set, with stable references to the source evidence when applicable. Limitations may identify validation-result evidence, execution, comparison, timing, target-profile context, provenance, diagnostic, or optional or non-required association evidence, but shall not convert that evidence into retained-run content or source-result content and shall not replace structural validation of required evaluation inputs.
*Traces to: HLR-REPLAY-EVAL-003, HLR-REPLAY-EVAL-004, HLR-REPLAY-EVAL-005*

### LLR-REPLAY-EVAL-007: Stable Evaluation Result Content
A replay evaluation result shall include evaluation-result format identity and version, retained-run identity, replay schema identity and version, execution-record identity, comparison-result identity or stable comparison-result reference, comparison disposition, timing-result identity or stable timing-result reference and timing disposition when timing applies, target-profile identity and context compatibility reference when applicable, requested claim or evaluation scope, evaluation disposition, stable reason or deterministic ordered reason set when applicable, evidence limitations, and claim boundaries.
*Traces to: HLR-REPLAY-EVAL-001, HLR-REPLAY-EVAL-002, HLR-REPLAY-EVAL-003, HLR-REPLAY-EVAL-004*

### LLR-REPLAY-EVAL-008: Evaluation Identity and Reference Stability
When replay evaluation identity is represented by a digest, it shall be derived from one canonical representation of stable evaluation-result content. Evaluation identity and stable references shall exclude storage path, checker text, generation time, validation time, execution time, mutable locators, diagnostic prose, and UI state.
*Traces to: HLR-REPLAY-EVAL-001, HLR-REPLAY-EVAL-005*

### LLR-REPLAY-EVAL-009: Deterministic Non-Mutation
Replay evaluation shall be deterministic over its inputs and shall not mutate retained-run validation, replay execution, execution records, functional comparison results, timing results, target execution profiles, execution-context compatibility results, diagnostics, retained-run identity, execution-record identity, comparison-result identity, timing-result identity, or target-profile identity.
*Traces to: HLR-REPLAY-EVAL-005, HLR-REPLAY-EVAL-006*

### LLR-REPLAY-EVAL-010: Checker Text Non-Authority
Checker text may report evaluation evidence or retained checker status, but shall not be the authoritative replay evaluation result and shall not define execution-record content, functional comparison content, timing-result content, target-profile context, evidence-limit semantics, or claim-boundary semantics.
*Traces to: HLR-REPLAY-EVAL-007*


### LLR-REPLAY-EVAL-011: Deterministic Evaluation Disposition
Replay evaluation shall derive exactly one evaluation disposition from the requested claim or evaluation scope, associated source dispositions and result content, required structural association checks, and recorded evidence limitations. The derivation shall be deterministic and shall not depend on storage path, checker text, generation time, mutable locators, diagnostic prose, or UI state.
*Traces to: HLR-REPLAY-EVAL-004*

### LLR-REPLAY-EVAL-012: Disposition Distinction
Evaluation disposition `invalid` shall be used only for structurally invalid required evaluation inputs or associations. Evaluation disposition `insufficient` shall be used when structurally valid usable evidence cannot determine whether the requested claim is supported. Evaluation disposition `not_supported` shall be used when structurally valid evidence determines that the requested claim is not supported within the recorded boundaries. Evaluation disposition `supported` shall be used when structurally valid evidence supports the requested claim within the recorded boundaries.
*Traces to: HLR-REPLAY-EVAL-004, HLR-REPLAY-EVAL-003*

### LLR-REPLAY-EVAL-013: Source Disposition Non-Mutation
An evaluation disposition of `supported`, `not_supported`, `insufficient`, or `invalid` shall not rewrite retained-run validation disposition, execution disposition, functional comparison disposition, timing disposition, execution-context compatibility disposition, or any stable source reason associated with those dispositions.
*Traces to: HLR-REPLAY-EVAL-004, HLR-REPLAY-EVAL-006*



## 11. Replay Operations and Trace Envelope

### LLR-REPLAY-OPS-001: Record Operation Inputs
The `record` operation shall consume replay schema identity and version, canonical input produced through a route permitted by the applicable upstream canonicalization contract, modeled-execution dependency bindings for every dependency declared by the replay schema, retained functional reference material, schema-required functional comparison parameters, and any required immutable provenance association reference separately owned by the upstream canonicalization contract.
*Traces to: HLR-REPLAY-OPS-001, HLR-REPLAY-RUN-001, HLR-REPLAY-ORIGIN-001*

### LLR-REPLAY-OPS-002: Record Schema Identity and Version
The `record` operation shall bind the created retained run to the declared replay schema identity and version, and that schema identity and version shall resolve to the immutable schema contract used for canonical input meaning, modeled-execution dependencies, retained functional reference material, and functional comparison parameters.
*Traces to: HLR-REPLAY-OPS-001, HLR-REPLAY-SCHEMA-006*

### LLR-REPLAY-OPS-003: Record Canonical Input Route Gate
The `record` operation shall require evidence that the canonical input was produced through a route permitted by the applicable upstream canonicalization contract for the declared replay schema. The operation shall not admit upstream source material, parse raw source material, or project source evidence into canonical input.
*Traces to: HLR-REPLAY-OPS-001, HLR-REPLAY-SYS-002, HLR-REPLAY-ORIGIN-001*

### LLR-REPLAY-OPS-004: Record Retained Functional Content
The `record` operation shall construct retained-run content from canonical input, modeled-execution dependency bindings, retained functional reference material, and functional comparison parameters. Retained functional reference material shall remain separate from physical timing reference material, generated execution evidence, generated evaluations, and upstream source admission evidence.
*Traces to: HLR-REPLAY-OPS-001, HLR-REPLAY-RUN-001, HLR-REPLAY-RUN-005*

### LLR-REPLAY-OPS-005: Deterministic Retained-Run Construction
Given the same retained-run format identity and version, replay schema identity and version, canonical input, modeled-execution dependency bindings, retained functional reference material, and functional comparison parameters, the `record` operation shall construct the same identity-bearing retained-run content and retained-run identity. Retained-run identity generation shall use the retained-run identity contract.
*Traces to: HLR-REPLAY-OPS-001, HLR-REPLAY-RUN-003*

### LLR-REPLAY-OPS-006: Record Operation Output
The `record` operation shall output the created retained run and its retained-run identity or stable retained-run identity reference. The `record` operation output shall not include a retained-run validation result, execution record, functional comparison result, timing result, execution-context compatibility result, generated replay evaluation, replay-trace envelope judgment, or retained-run diff result.
*Traces to: HLR-REPLAY-OPS-001, HLR-REPLAY-RUN-001*

### LLR-REPLAY-OPS-007: Record Operation Boundaries
The `record` operation shall not perform retained-run validation, replay execution, functional comparison, execution-context compatibility validation, physical timing evaluation, generated replay evaluation, replay-trace envelope judgment, retained-run diff, upstream parsing, upstream source admission, or upstream projection.
*Traces to: HLR-REPLAY-OPS-001, HLR-REPLAY-RUN-002, HLR-REPLAY-EVAL-001*

### LLR-REPLAY-OPS-008: Record Non-Mutation
The `record` operation shall be deterministic over its inputs and shall not mutate canonical input, modeled-execution dependency inputs, retained functional reference inputs, functional comparison parameter inputs, separately owned upstream evidence, source admission records, provenance associations, or upstream canonicalization records.
*Traces to: HLR-REPLAY-OPS-001, HLR-REPLAY-RUN-006*

### LLR-REPLAY-OPS-009: Replay Operation Inputs
The `replay` operation shall consume one retained run, the requested claim or replay evaluation scope, implementation support declarations required for execution and context validation, and any required immutable provenance association references. When the schema or requested claim requires target-specific or physical-timing evidence, it shall also consume the applicable target execution profile, required execution-context compatibility inputs, and timing-claim parameters; physical timing observations shall be consumed from the execution record after execution begins.
*Traces to: HLR-REPLAY-OPS-002, HLR-REPLAY-TPROF-001, HLR-REPLAY-TIME-001*

### LLR-REPLAY-OPS-010: Replay Orchestration Order
The `replay` operation shall orchestrate source operations in this order: retained-run validation; termination before execution when validation disposition is `invalid`; execution when validation disposition is `valid`; execution-record production; functional comparison; execution-context compatibility validation and physical timing evaluation when required by the schema or requested claim; and generated replay evaluation.
*Traces to: HLR-REPLAY-OPS-002, HLR-REPLAY-RUN-008, HLR-REPLAY-EVAL-001*

### LLR-REPLAY-OPS-011: Replay Invalid Validation Stop
When retained-run validation produces disposition `invalid`, the `replay` operation shall stop before execution and shall not produce an execution record, functional comparison result, timing result, or generated replay evaluation that represents execution as having begun. The validation result shall retain its own disposition and stable reasons.
*Traces to: HLR-REPLAY-OPS-002, HLR-REPLAY-RUN-007, HLR-REPLAY-RUN-008*

### LLR-REPLAY-OPS-012: Replay Stable Output References
The `replay` operation output shall contain stable references to each source result it produced or consumed, including retained-run validation result, execution record when execution began, functional comparison result when comparison ran, execution-context compatibility result when required, timing result when required, and generated replay evaluation when produced. Each source result shall preserve its own disposition and stable reasons.
*Traces to: HLR-REPLAY-OPS-002, HLR-REPLAY-EVAL-001, HLR-REPLAY-EVAL-004*

### LLR-REPLAY-OPS-013: Replay Association Checks
Before using a source result in later orchestration steps, the `replay` operation shall check required structural associations among retained-run identity, replay schema identity and version, validation-result reference, execution-record identity, comparison-result identity, target-profile identity, compatibility-result identity or stable reference, timing-result identity, and evaluation-result identity as applicable. Missing or mismatched required associations shall be represented by the replay-operation report, or by the generated replay evaluation when evaluation is reached and owns that association, as structural invalidity with stable reasons, not only as evidence limitations.
*Traces to: HLR-REPLAY-OPS-002, HLR-REPLAY-EVAL-004, HLR-REPLAY-EVAL-005*

### LLR-REPLAY-OPS-014: Replay Disposition Separation
The `replay` operation shall not collapse retained-run validation disposition, execution disposition, functional comparison disposition, execution-context compatibility disposition, timing disposition, or generated replay evaluation disposition into one shared disposition. Operation-level reporting may summarize orchestration using a separate replay-operation status and stable references to source results, but shall not rewrite any source disposition.
*Traces to: HLR-REPLAY-OPS-002, HLR-REPLAY-EVAL-004, LLR-REPLAY-EVAL-013*

### LLR-REPLAY-OPS-015: Replay Operation Report
A replay-operation report shall include replay-operation report format identity and version, retained-run identity when available, replay schema identity and version when available, requested claim or replay evaluation scope, replay-operation status `completed` or `blocked`, stable references to each source result produced or consumed, the orchestration step at which processing stopped when blocked, stable orchestration reasons or a deterministic ordered reason set when blocked, and report identity or stable report reference derived from canonical report content. Replay-operation status shall describe orchestration completion only and shall not replace retained-run validation disposition, execution disposition, functional comparison disposition, execution-context compatibility disposition, timing disposition, or generated replay evaluation disposition.
*Traces to: HLR-REPLAY-OPS-002, HLR-REPLAY-EVAL-004, LLR-REPLAY-EVAL-013*

### LLR-REPLAY-OPS-016: Replay Non-Mutation
The `replay` operation shall be deterministic over its inputs and shall not mutate the retained run, validation result, execution record, functional comparison result, timing result, target execution profile, execution-context compatibility result, generated replay evaluation inputs, diagnostics, provenance associations, retained functional reference material, or upstream evidence.
*Traces to: HLR-REPLAY-OPS-002, HLR-REPLAY-EVAL-006*

### LLR-REPLAY-OPS-017: Diff Operation Inputs
The `diff` operation shall consume exactly two retained runs and any retained-run validation results required to establish that both retained runs are structurally valid before compatible retained-run comparison proceeds.
*Traces to: HLR-REPLAY-OPS-003, HLR-REPLAY-RUN-002*

### LLR-REPLAY-OPS-018: Diff Validation Prerequisite
The `diff` operation shall require both retained runs to have validation disposition `valid` before producing retained-run diff disposition `exact` or `diverged`. If either retained run is invalid or cannot be validated as required, the diff disposition shall be `incompatible` with stable validation or association reasons.
*Traces to: HLR-REPLAY-OPS-003, HLR-REPLAY-RUN-007*

### LLR-REPLAY-OPS-019: Diff Compatibility Checks
The `diff` operation shall check retained-run format compatibility, replay schema identity and version compatibility, canonical input comparability, modeled-execution dependency binding comparability, retained functional reference material comparability, and functional comparison parameter comparability before deciding `exact` or `diverged`.
*Traces to: HLR-REPLAY-OPS-003, HLR-REPLAY-SCHEMA-006*

### LLR-REPLAY-OPS-020: Diff Dispositions
The `diff` operation shall produce exactly one retained-run diff disposition: `exact`, `diverged`, or `incompatible`. `exact` shall mean the two retained runs match under the retained-run diff contract. `diverged` shall mean the two retained runs are compatible for diff but differ in identity-bearing retained-run content. `incompatible` shall mean they cannot be compared under the retained-run diff contract.
*Traces to: HLR-REPLAY-OPS-003*

### LLR-REPLAY-OPS-021: Diff Evidence and Symmetry
For disposition `diverged`, the `diff` operation shall report deterministic first-difference evidence. For disposition `incompatible`, it shall report stable incompatibility reasons or a deterministic ordered reason set. Swapping the two retained-run inputs shall not change the disposition; first-difference or incompatibility evidence shall be selected by canonical retained-run content ordering defined by the retained-run format and replay schema as applicable rather than caller input position.
*Traces to: HLR-REPLAY-OPS-003*

### LLR-REPLAY-OPS-022: Diff Authority Boundary
The `diff` operation shall not treat either retained run as authoritative and shall not redefine ordinary replay functional comparison of generated execution behavior against a retained functional reference. Diff evidence shall remain separate from replay execution, functional comparison, physical timing evaluation, generated replay evaluation, and replay-trace envelope judgment.
*Traces to: HLR-REPLAY-OPS-003, HLR-REPLAY-COMP-001*

### LLR-REPLAY-OPS-023: Diff Non-Mutation
The `diff` operation shall be deterministic over its inputs and shall not mutate either retained run, retained-run validation result, retained functional reference material, modeled-execution dependency binding, functional comparison parameter, provenance association, upstream evidence, execution record, comparison result, timing result, target profile, or generated replay evaluation.
*Traces to: HLR-REPLAY-OPS-003*

### LLR-REPLAY-ENV-001: Replay-Trace Envelope Inputs
A replay-trace envelope judgment shall consume the applicable retained-run identity, replay schema identity and version checked from the retained run, a trace or immutable trace reference, trace origin, named deterministic envelope rule identity and version, rule parameters, and rule context. When the trace is a generated functional trace, the inputs shall also include the applicable execution-record identity or stable reference. The judgment shall not independently select the replay schema identity.
*Traces to: HLR-REPLAY-OPS-004, HLR-REPLAY-TRACE-001*

### LLR-REPLAY-ENV-002: Trace Origin Classification
Replay-trace envelope input shall identify whether the trace origin is retained functional reference material, generated functional trace from an execution record, or another schema-defined replay trace origin. Trace origin shall be stable result content and shall not be inferred from storage path or diagnostic prose.
*Traces to: HLR-REPLAY-OPS-004, HLR-REPLAY-TRACE-002*

### LLR-REPLAY-ENV-003: Envelope Association Validation
A replay-trace envelope judgment shall validate required associations among the mandatory retained-run identity, trace identity or immutable trace reference, trace origin, execution-record identity when the trace is a generated functional trace, replay schema identity and version, rule identity and version, rule parameters, and rule context. Missing or mismatched required associations shall produce judgment disposition `invalid` with stable reasons and shall not be represented only as evidence limitations.
*Traces to: HLR-REPLAY-OPS-004, HLR-REPLAY-EVAL-004*

### LLR-REPLAY-ENV-004: Envelope Rule Application
For structurally valid inputs, replay-trace envelope judgment shall apply exactly the named deterministic envelope rule version with the declared rule parameters and context to the trace content or immutable trace reference. Rule identity, rule version, parameters, and context shall be stable machine-readable result content.
*Traces to: HLR-REPLAY-OPS-004*

### LLR-REPLAY-ENV-005: Envelope Judgment Dispositions
A replay-trace envelope judgment shall produce exactly one stable judgment disposition: `pass`, `fail`, `inconclusive`, `not_applicable`, or `invalid`. `fail` shall include first violation evidence when applicable. `inconclusive`, `not_applicable`, and `invalid` shall include stable reasons or a deterministic ordered reason set.
*Traces to: HLR-REPLAY-OPS-004*

### LLR-REPLAY-ENV-006: Envelope Result Content and Identity
A replay-trace envelope result shall include envelope-result format identity and version, replay schema identity and version, mandatory retained-run identity, trace identity or immutable trace reference, trace origin, execution-record identity when the trace is a generated functional trace, rule identity and version, rule parameters, rule context, judgment disposition, stable reasons or first violation evidence when applicable, and envelope-result identity or stable result reference derived from canonical result content.
*Traces to: HLR-REPLAY-OPS-004*

### LLR-REPLAY-ENV-007: Replay-Trace Envelope Boundary
Replay-trace envelope judgment shall not redefine retained-run validation, replay execution, functional comparison, physical timing evaluation, generated replay evaluation, retained-run diff, or upstream source admission. The existing raw ADC witness envelope remains a source-evidence judgment over admitted observations and shall not be treated as a replay-trace envelope.
*Traces to: HLR-REPLAY-OPS-004, HLR-REPLAY-OPS-005*

### LLR-REPLAY-ENV-008: Envelope Non-Mutation
Replay-trace envelope judgment shall be deterministic over its inputs and shall not mutate the trace, immutable trace reference, retained run, execution record, comparison result, timing result, target profile, generated replay evaluation, rule parameters, rule context, upstream evidence, or raw ADC source-evidence envelope material.
*Traces to: HLR-REPLAY-OPS-004, HLR-REPLAY-OPS-005*

## 12. Upstream Saved Input Parsing

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


## 13. Retained Replay Witness Checker

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


## 14. Upstream Raw ADC Admitted Observation Projection

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

## 15. Target Agreement

### LLR-REPLAY-TGT-001: Multi-Target Agreement Scope
Multi-target agreement shall compare executions of the same retained run only when those executions identify the same retained-run identity, compatible replay schema identity and version, and valid execution-record associations. When target-specific claims are involved, each execution shall identify the applicable target-profile identity and shall have a compatible execution-context result for that exact target profile before participating in profile-bound multi-target agreement.
*Traces to: HLR-REPLAY-TGT-001, HLR-REPLAY-TPROF-005*

### LLR-REPLAY-TGT-002: Target Metadata Equality Boundary
Target-specific diagnostic metadata, target execution profile metadata, and context compatibility results shall not participate in replay equality unless the replay schema makes that information observable. Multi-target agreement shall not validate target profiles, shall not evaluate physical timing pass or fail unless timing agreement is explicitly required, and shall not redefine retained-run validation, execution disposition, functional comparison, target-profile compatibility, timing evaluation, or generated replay evaluation.
*Traces to: HLR-REPLAY-TGT-002, HLR-REPLAY-TPROF-006*

### LLR-REPLAY-TGT-003: Agreement Inputs
Multi-target agreement shall consume only two or more execution records for the same retained run, their functional comparison results, replay schema identity and version, applicable target-profile identities when profile-bound claims are involved, execution-context compatibility results required for those profiles, and timing results only when timing agreement is explicitly required.
*Traces to: HLR-REPLAY-TGT-001, HLR-REPLAY-TGT-002*

### LLR-REPLAY-TGT-004: Agreement Compatibility Checks
Multi-target agreement shall check retained-run identity equality, compatible replay schema identity and version, execution-record association integrity, comparison-result association integrity, applicable target-profile identity association, compatible execution-context result association for profile-bound claims, and timing-result association only when timing agreement is explicitly required. Missing or mismatched required associations shall produce agreement disposition `incompatible` with stable reasons.
*Traces to: HLR-REPLAY-TGT-001, HLR-REPLAY-TGT-002*

### LLR-REPLAY-TGT-005: Functional and Timing Agreement Scope
For compatible inputs, multi-target agreement shall compare schema-defined functional trace, execution disposition, terminal outcome when applicable, functional comparison disposition, and first-divergence evidence when applicable. Timing disposition and timing-result evidence shall participate only when the replay schema or requested claim explicitly requires timing agreement.
*Traces to: HLR-REPLAY-TGT-001*

### LLR-REPLAY-TGT-006: Agreement Dispositions and Reasons
A multi-target agreement result shall produce exactly one disposition: `agree`, `mismatch`, or `incompatible`. `agree` shall mean all compatible participating executions agree within the applicable functional and explicit timing scope. `mismatch` shall mean compatible inputs differ within that scope. `incompatible` shall mean the inputs cannot be compared for multi-target agreement. Mismatch and incompatible results shall include stable reasons or deterministic ordered reason sets, with first mismatch evidence when applicable.
*Traces to: HLR-REPLAY-TGT-001, HLR-REPLAY-TGT-002*

### LLR-REPLAY-TGT-007: Agreement Result Content and Identity
A multi-target agreement result shall include agreement-result format identity and version, retained-run identity, replay schema identity and version, participating execution-record identities, comparison-result identities or stable references, applicable target-profile identities, required compatibility-result references, timing-result identities or stable references when timing agreement applies, agreement disposition, stable mismatch or incompatibility reasons when applicable, first mismatch evidence when applicable, and agreement-result identity or stable result reference derived from canonical result content.
*Traces to: HLR-REPLAY-TGT-001, HLR-REPLAY-TGT-002*

### LLR-REPLAY-TGT-008: Deterministic Ordering and Non-Mutation
Multi-target agreement shall treat input ordering deterministically: reordering the same participating execution-record, target-profile, compatibility-result, comparison-result, and timing-result inputs shall not change the agreement disposition or result identity. Multi-target agreement shall not mutate execution records, target profiles, compatibility results, comparison results, timing results, retained runs, retained-run validation results, diagnostics, or upstream evidence.
*Traces to: HLR-REPLAY-TGT-002*
