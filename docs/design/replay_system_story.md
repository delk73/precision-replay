# Replay System Story

This document is explanatory design context. It is not a normative requirement source. Normative replay requirements remain in `docs/normative/HLR_replay.md`, `docs/design/LLR_replay.md`, and `docs/normative/traceability_matrix.md`.

Replay begins with canonical input for a declared replay schema. The schema gives that input meaning and defines modeled execution behavior, observable functional trace, terminal behavior, and functional comparison rules.

The `record` operation creates an immutable retained run from canonical input, schema-declared modeled-execution dependencies, retained functional reference material, and functional comparison parameters. The retained run binds the functional reference used for later comparison. Physical timing evidence and target-specific execution context are not retained functional reference material.

Before replay execution begins, retained-run validation checks that the retained run is structurally usable. Invalid validation stops replay before execution. Validation failure is not execution rejection because execution did not begin.

Replay execution of a valid retained run produces one execution record for one execution occurrence. The execution record contains generated functional trace, execution disposition, terminal outcome when present, execution-context facts, physical timing observations when present, and stable diagnostic references when applicable. Execution does not mutate the retained run.

Functional comparison compares generated functional behavior from the execution record with the retained functional reference from the retained run. Comparison can be `exact`, `diverged`, or `incompatible`. It reports functional mismatch evidence without rewriting the execution disposition.

Physical timing evaluation applies only when the schema or requested claim requires timing evidence. Timing evaluation uses physical timing observations from the execution record and the applicable target execution profile after required execution-context compatibility checks. Timing `pass`, `fail`, or `insufficient` remains separate from functional comparison.

Replay evaluation packages the claim result. It associates the retained run, execution record, functional comparison result, optional timing result, target-profile context when required, evidence limitations, and claim boundaries. Evaluation can be `supported`, `not_supported`, `insufficient`, or `invalid`. `invalid` is reserved for structurally invalid required inputs or associations.

Replay operations keep source results distinct. Validation, execution, comparison, execution-context compatibility, timing evaluation, and replay evaluation each retain their own disposition and stable reasons. Operation-level reporting may summarize orchestration, but it must not collapse or rewrite those source results.

The common Replay system does not own upstream source admission, saved-input parsing, raw ADC projection, hardware qualification, calibration, release readiness, or certification claims. Those may feed replay inputs or bound replay claims, but they do not redefine common replay semantics.
