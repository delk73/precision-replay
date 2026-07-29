# Replay Low-Level Requirements (LLR)

## A. Retained Run Construction and Immutability

- **LLR-RET-001**: The system shall serialize Canonical Input in a deterministic canonicalization format before Retained Run persistence.
- **LLR-RET-002**: The system shall bind exactly one Replay Schema identifier and version to each Retained Run.
- **LLR-RET-003**: The system shall validate presence and schema-conformance of bound Initial State Seed $S_0$ at Retained Run creation.
- **LLR-RET-004**: The system shall compute and store deterministic identity material for Retained Run association points.
- **LLR-RET-005**: Post-creation mutation attempts on Retained Run fields shall be rejected and auditable.

## B. Structural Usability Validation

- **LLR-VAL-001**: Validation shall execute before any replay cycle tick is evaluated.
- **LLR-VAL-002**: Validation shall emit distinct reason codes for each failed predicate: format identity, empty Canonical Input, schema incompatibility, invalid/missing $S_0$.
- **LLR-VAL-003**: On validation failure, execution shall remain unstarted and no Execution Record shall be emitted.
- **LLR-VAL-004**: Validation outcomes shall be persisted as pre-execution gate results with stable diagnostic references.
- **LLR-VAL-005**: Validation success shall produce an explicit “structurally usable” status token consumed by execution start logic.

## C. Deterministic Tick Execution

- **LLR-EXE-001**: Execution shall iterate on integer cycle ticks $t_0, t_1, \dots, t_n$ with strict monotonic increment by 1.
- **LLR-EXE-002**: Execution state transition functions shall be deterministic for identical input tuple (schema, $S_0$, canonical input, dependencies).
- **LLR-EXE-003**: Execution shall not query host wall-clock to influence functional state transitions.
- **LLR-EXE-004**: Terminal-state detection shall map to `accepted`; deterministic rule violation/illegal transition to `rejected`; interruption/truncation to `incomplete`.
- **LLR-EXE-005**: Exactly one Execution Record shall be finalized per execution occurrence, with immutable final disposition.

## D. Execution Record Schema Guarantees

- **LLR-REC-001**: Execution Record shall include: execution ID, retained-run ID, schema ID/version, tick range, functional trace payload, disposition, terminal outcome (optional), execution-context facts, timing observations (optional), diagnostic refs (optional).
- **LLR-REC-002**: Functional trace entries shall include tick index and deterministic state-vector encoding.
- **LLR-REC-003**: Disposition field shall be enum-constrained to `accepted|rejected|incomplete`.
- **LLR-REC-004**: Missing optional sections shall be represented explicitly (e.g., null/absent by schema contract), not inferred.
- **LLR-REC-005**: Record serialization shall be stable under repeated encoding of identical data.

## E. Trace Integrity and Structural Invalidity

- **LLR-INT-001**: The system shall compute incremental digest chain elements $D_k = f(D_{k-1}, S_k)$ from $t_0$ through final tick.
- **LLR-INT-002**: Comparison-stage integrity verification shall validate digest continuity and structure completeness before divergence assessment.
- **LLR-INT-003**: Integrity failures shall emit structural invalidity reason codes (malformed structure, chain break, truncation).
- **LLR-INT-004**: If structural invalidity is present, comparison result shall not be `diverged`.
- **LLR-INT-005**: Integrity verification shall require no external key/PKI/network dependency to complete.

## F. Functional Comparison Mechanics

- **LLR-CMP-001**: Comparison shall require schema/format comparability check before per-tick state-vector comparison.
- **LLR-CMP-002**: For comparable traces, equality shall be evaluated at each corresponding tick over full state-vector domain.
- **LLR-CMP-003**: First mismatch and cumulative mismatch summary shall be captured as evidence payload.
- **LLR-CMP-004**: Result shall be `exact` iff all comparable ticks match and no structural invalidity exists.
- **LLR-CMP-005**: Result shall be `incompatible` when comparability preconditions fail.
- **LLR-CMP-006**: Comparison result shall not modify execution disposition.

## G. Execution-Context Compatibility and Timing Evaluation

- **LLR-TIM-001**: Timing evaluation shall run only when Replay Schema marks timing evidence as required.
- **LLR-TIM-002**: Compatibility checks shall evaluate execution-context facts against required Target Execution Profile predicates.
- **LLR-TIM-003**: Failed compatibility predicates shall force timing result `insufficient` with explicit failed-predicate evidence.
- **LLR-TIM-004**: On compatible context, timing metrics shall be evaluated against profile thresholds to yield `pass` or `fail`.
- **LLR-TIM-005**: Timing result storage shall be isolated from functional comparison and execution disposition fields.

## H. Replay Evaluation Decision Logic

- **LLR-EVL-001**: Replay evaluation shall assemble required associations: retained run, execution record, comparison result, optional timing result, context, limitations, boundaries.
- **LLR-EVL-002**: Missing/structurally invalid required association shall yield final `invalid`.
- **LLR-EVL-003**: Decision logic for `supported|not_supported|insufficient|invalid` shall be table-driven and deterministic.
- **LLR-EVL-004**: Evaluation output shall include explicit claim boundary and evidence limitation sections.
- **LLR-EVL-005**: Evaluation shall preserve referenced source-stage dispositions without transformation.

## I. Reporting and Source-Result Separation

- **LLR-REP-001**: Stage outputs shall be stored with separate namespaces/fields for validation, execution, comparison, compatibility, timing, evaluation.
- **LLR-REP-002**: Summary reporting may aggregate but shall include canonical links back to each source-stage disposition and reason.
- **LLR-REP-003**: No stage processor shall overwrite another stage’s persisted disposition/reason fields.
- **LLR-REP-004**: Stable diagnostic references shall be immutable once emitted.
- **LLR-REP-005**: Audit logs shall capture transitions between stage completion states.

## J. External Trust Boundary Enforcement

- **LLR-TRU-001**: Replay APIs shall expose deterministic artifact identities for external attestation binding.
- **LLR-TRU-002**: Replay core shall not require identity provider integration for functional validity determination.
- **LLR-TRU-003**: External trust decisions ingested as metadata shall be non-authoritative over replay technical outcomes.
- **LLR-TRU-004**: Claim acceptance policy fields, if present, shall be informational and outside replay disposition logic.
