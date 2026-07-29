---
pipeline_state:
  active_phase: "complete"
  gates:
    scope_decomposition_approved: true
    lexicon_alignment_approved: true
    hlr_baseline_approved: true
    llr_baseline_approved: true
    traceability_matrix_approved: true
---

# Replay System Story

This document is explanatory design context. Normative replay requirements remain in `docs/normative/HLR_replay.md`, `docs/design/LLR_replay.md`, and `docs/normative/traceability_matrix.md`.

Replay begins with canonical input and a declared replay schema. The schema defines what the input means, how replay executes it, what functional trace is observable, how execution can end, and how later executions are compared.

The record operation creates an immutable retained run. The retained run contains the canonical input, schema-declared execution dependencies (including the immutable Initial State Seed $S_0$ required to initialize state at cycle tick $t_0$), retained functional reference material, and functional comparison parameters. It binds the functional reference used for later comparison. Physical timing evidence and target-specific execution context are not part of that functional reference.

Replay validates the retained run before execution. Structural usability requires format identity compliance, non-empty canonical input, schema version compatibility, and a valid bound Initial State Seed $S_0$. If the retained run is not structurally usable, execution does not start. That failure is a validation failure, a pre-execution gate event, and not an execution rejection.

Replay execution of a valid retained run produces one execution record for one execution occurrence. Replay execution evaluates sequentially over discrete, monotonically increasing cycle ticks ($t_k$), completely isolated from host wall-clock time. The execution record contains the generated functional trace, execution disposition (`accepted`, `rejected`, or `incomplete`), terminal outcome when present, execution-context facts, any physical timing observations, and any applicable stable diagnostic references. 
- Execution disposition is `accepted` upon reaching a schema-defined terminal state.
- Execution disposition is `rejected` when encountering a deterministic rule violation or illegal state transition.
- Execution disposition is `incomplete` if execution is truncated, aborted, or interrupted prior to termination.
Execution does not mutate the retained run.

## Execution Trace Integrity and Tamper Detection

During cycle-tick execution, the Replay system computes incremental, deterministic digest chains across all observable state vectors from $t_0$ through $t_k$. If an execution record exhibits structural inconsistency, broken cryptographic hash continuity, or stream truncation during functional comparison, Replay assigns a specific structural invalidity fault to the comparison stage rather than classifying the execution trace as diverged. Replay guarantees that trace integrity verification depends strictly on deterministic state-vector math and does not rely on external cryptographic key availability, public key infrastructure, or network-bound attestation services.

## Comparison and Evaluation Semantics

Functional comparison compares the execution record’s generated functional behavior with the retained run’s functional reference:
- Comparison is `exact` when generated state vectors match reference state vectors at all cycle ticks ($t_k$).
- Comparison is `diverged` when traces are schema-compatible but state vectors differ at one or more cycle ticks ($t_k$).
- Comparison is `incompatible` when traces originate from non-comparable schemas or structural formats.
Comparison reports functional mismatch evidence without rewriting the execution disposition.

Physical timing evaluation applies only when the declared Replay Schema explicitly requires physical timing evidence. It uses physical timing observations from the execution record and the applicable target execution profile, after required execution-context compatibility checks. If execution-context facts are incompatible with the target profile, physical timing evaluation yields an `insufficient` disposition. Physical timing results (`pass`, `fail`, or `insufficient`) remain completely separate from functional comparison.

Replay evaluation packages the claim result. It associates the retained run, execution record, functional comparison result, optional timing result, required target-profile context, evidence limitations, and claim boundaries. Evaluation can be supported, not_supported, insufficient, or invalid. Invalid applies only when a required input or association is structurally invalid.

Replay operations keep source results distinct. Validation, execution, comparison, execution-context compatibility, timing evaluation, and replay evaluation each retain their own disposition and stable reasons. Operation-level reporting may summarize orchestration, but it must not collapse or rewrite those source results.

Replay provides deterministic identities and immutable association points suitable for external attestation and trust evaluation. Replay does not authenticate producers, establish issuer trust, manage credentials or revocation, determine who is authorized to assert or rely on a claim, establish chain of custody, or decide whether a relying party should accept an otherwise valid Replay result. An external attestation or policy layer may bind those judgments to Replay object identities, but those judgments do not alter retained-run validity, execution disposition, functional comparison, timing evaluation, or Replay evaluation.

The common Replay system does not own upstream source admission, saved-input parsing, raw ADC projection, hardware qualification, calibration, release readiness, certification claims, or relying-party acceptance policy. Those concerns may provide replay inputs, establish trust in associated evidence, or limit replay claims, but they do not change common replay semantics.
