# Replay High-Level Requirements (HLR)

## A. Retained Run and Validation

- **HLR-RPL-001**: The Replay system shall create a `Retained Run` that immutably binds `Canonical Input`, `Replay Schema`, schema-declared execution dependencies, and `Initial State Seed ($S_0$)`.
- **HLR-RPL-002**: The Replay system shall perform `Structural Usability Validation` before execution.
- **HLR-RPL-003**: `Structural Usability Validation` shall verify format identity compliance, non-empty `Canonical Input`, `Replay Schema` version compatibility, and presence/validity of bound `Initial State Seed ($S_0$)`.
- **HLR-RPL-004**: On `Structural Usability Validation` failure, the Replay system shall raise a `Pre-Execution Gate Event` and shall not start execution.
- **HLR-RPL-005**: A validation-stage failure shall not be represented as `Execution Disposition = rejected`.

## B. Deterministic Execution Semantics

- **HLR-RPL-006**: Replay execution shall evaluate over `Deterministic Cycle Tick ($t_k$)` values that are discrete and monotonically increasing.
- **HLR-RPL-007**: Replay execution semantics shall be isolated from host wall-clock time.
- **HLR-RPL-008**: One execution occurrence of a `Retained Run` shall produce exactly one `Execution Record`.
- **HLR-RPL-009**: Replay execution shall not mutate the `Retained Run`.

## C. Execution Record and Dispositions

- **HLR-RPL-010**: Each `Execution Record` shall include generated functional trace, `Execution Disposition`, terminal outcome when present, execution-context facts, physical timing observations when present, and stable diagnostic references when present.
- **HLR-RPL-011**: `Execution Disposition` shall be one of `accepted`, `rejected`, or `incomplete`.
- **HLR-RPL-012**: The Replay system shall assign `accepted` only when a schema-defined terminal state is reached.
- **HLR-RPL-013**: The Replay system shall assign `rejected` on deterministic rule violation or illegal state transition.
- **HLR-RPL-014**: The Replay system shall assign `incomplete` on truncation, abort, or interruption before termination.

## D. Functional Comparison and Integrity

- **HLR-RPL-015**: The Replay system shall compare `Execution Record` functional behavior to retained functional reference using `Comparison Disposition`.
- **HLR-RPL-016**: `Comparison Disposition` shall be one of `exact`, `diverged`, or `incompatible`.
- **HLR-RPL-017**: The Replay system shall assign `exact` only when generated and reference state vectors match at all cycle ticks.
- **HLR-RPL-018**: The Replay system shall assign `diverged` only when traces are schema-compatible and differ at one or more cycle ticks.
- **HLR-RPL-019**: The Replay system shall assign `incompatible` when traces are non-comparable by schema or structural format.
- **HLR-RPL-020**: The Replay system shall perform `Trace Integrity Verification` during comparison.
- **HLR-RPL-021**: `Trace Integrity Verification` shall verify `Digest Chain Continuity` across observable state vectors from $t_0$ through $t_k$.
- **HLR-RPL-022**: If structural inconsistency, digest-chain discontinuity, or truncation is detected, the Replay system shall assign a `Structural Invalidity Fault` at comparison stage.
- **HLR-RPL-023**: A `Structural Invalidity Fault` shall not be classified as functional divergence.

## E. Timing Evaluation and Context Compatibility

- **HLR-RPL-024**: Physical timing evaluation shall execute only when the `Replay Schema` explicitly requires physical timing evidence.
- **HLR-RPL-025**: Before timing evaluation, the Replay system shall perform an `Execution-Context Compatibility Check` against the `Target Execution Profile`.
- **HLR-RPL-026**: If compatibility prerequisites are not met, physical timing evaluation shall return `Physical Timing Evaluation Disposition = insufficient`.
- **HLR-RPL-027**: `Physical Timing Evaluation Disposition` shall be one of `pass`, `fail`, or `insufficient`.
- **HLR-RPL-028**: Physical timing evaluation outcomes shall remain separate from `Execution Disposition` and `Comparison Disposition`.

## F. Replay Evaluation and Result Separation

- **HLR-RPL-029**: The Replay system shall compute a `Replay Evaluation Disposition` by packaging retained run association, execution record, functional comparison result, optional timing result, required target profile context, `Evidence Limitation`, and `Claim Boundary`.
- **HLR-RPL-030**: `Replay Evaluation Disposition` shall be one of `supported`, `not_supported`, `insufficient`, or `invalid`.
- **HLR-RPL-031**: `invalid` shall apply only when a required input or association is structurally invalid.
- **HLR-RPL-032**: Replay operations shall preserve distinct source results for validation, execution, comparison, execution-context compatibility, timing evaluation, and replay evaluation.
- **HLR-RPL-033**: Operation-level summaries shall not collapse or rewrite source dispositions or reasons.

## G. Trust Boundary Constraints

- **HLR-RPL-034**: The Replay system shall provide deterministic identities and immutable association points suitable for external attestation and trust evaluation.
- **HLR-RPL-035**: The Replay system shall not perform producer authentication, issuer trust establishment, credential or revocation management, authorization determination for claim assertion/reliance, chain-of-custody adjudication, or relying-party acceptance decisions.
- **HLR-RPL-036**: External attestation/policy bindings to Replay identities shall not alter retained-run validity, execution disposition, functional comparison, timing evaluation, or replay evaluation semantics.
