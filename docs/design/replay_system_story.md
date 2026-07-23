# Replay System Story

This document is explanatory design context. Normative replay requirements remain in `docs/normative/HLR_replay.md`, `docs/design/LLR_replay.md`, and `docs/normative/traceability_matrix.md`.

Replay begins with canonical input and a declared replay schema. The schema defines what the input means, how replay executes it, what functional trace is observable, how execution can end, and how later executions are compared.

The record operation creates an immutable retained run. The retained run contains the canonical input, schema-declared execution dependencies, retained functional reference material, and functional comparison parameters. It binds the functional reference used for later comparison. Physical timing evidence and target-specific execution context are not part of that functional reference.

Replay validates the retained run before execution. If the retained run is not structurally usable, execution does not start. That failure is a validation failure, not an execution rejection.

Replay execution of a valid retained run produces one execution record for one execution occurrence. The execution record contains the generated functional trace, execution disposition, terminal outcome when present, execution-context facts, any physical timing observations, and any applicable stable diagnostic references. Execution does not mutate the retained run.

Functional comparison compares the execution record’s generated functional behavior with the retained run’s functional reference. Comparison can be exact, diverged, or incompatible. It reports functional mismatch evidence without rewriting the execution disposition.

Physical timing evaluation applies only when the schema or requested claim requires timing evidence. It uses physical timing observations from the execution record and the applicable target execution profile, after any required execution-context compatibility checks. Timing can be pass, fail, or insufficient, and remains separate from functional comparison.

Replay evaluation packages the claim result. It associates the retained run, execution record, functional comparison result, optional timing result, required target-profile context, evidence limitations, and claim boundaries. Evaluation can be supported, not_supported, insufficient, or invalid. Invalid applies only when a required input or association is structurally invalid.

Replay operations keep source results distinct. Validation, execution, comparison, execution-context compatibility, timing evaluation, and replay evaluation each retain their own disposition and stable reasons. Operation-level reporting may summarize orchestration, but it must not collapse or rewrite those source results.

Replay provides deterministic identities and immutable association points suitable for external attestation and trust evaluation. Replay does not authenticate producers, establish issuer trust, manage credentials or revocation, determine who is authorized to assert or rely on a claim, establish chain of custody, or decide whether a relying party should accept an otherwise valid Replay result. An external attestation or policy layer may bind those judgments to Replay object identities, but those judgments do not alter retained-run validity, execution disposition, functional comparison, timing evaluation, or Replay evaluation.

The common Replay system does not own upstream source admission, saved-input parsing, raw ADC projection, hardware qualification, calibration, release readiness, certification claims, or relying-party acceptance policy. Those concerns may provide replay inputs, establish trust in associated evidence, or limit replay claims, but they do not change common replay semantics.
