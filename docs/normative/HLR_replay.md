# Replay High-Level Requirements (HLR)

## A. Retained Run and Recording

- **HLR-RET-001**: The system shall create a **Retained Run** as an immutable artifact binding Canonical Input, Replay Schema, schema-declared execution dependencies, and functional comparison parameters.
- **HLR-RET-002**: The Retained Run shall include a valid, bound **Initial State Seed ($S_0$)** for initialization at $t_0$.
- **HLR-RET-003**: Replay execution shall not mutate any Retained Run content after record creation.

## B. Pre-Execution Structural Validation

- **HLR-VAL-001**: The system shall perform **Structural Usability Validation** before execution starts.
- **HLR-VAL-002**: Structural usability shall require: format identity compliance, non-empty Canonical Input, Replay Schema version compatibility, and valid bound $S_0$.
- **HLR-VAL-003**: If structural usability fails, execution shall not start and the outcome shall be classified as a validation-stage **Pre-Execution Gate Event**, not execution rejection.

## C. Deterministic Replay Execution

- **HLR-EXE-001**: Replay execution shall progress deterministically over discrete, monotonically increasing cycle ticks $t_k$, independent of host wall-clock time.
- **HLR-EXE-002**: One valid replay execution occurrence shall produce exactly one **Execution Record**.
- **HLR-EXE-003**: The Execution Record shall include generated functional trace, execution disposition, terminal outcome when present, execution-context facts, physical timing observations (when available), and stable diagnostic references (when applicable).
- **HLR-EXE-004**: Execution disposition shall be constrained to `accepted`, `rejected`, or `incomplete` with semantics defined in vocabulary.

## D. Trace Integrity and Structural Faulting

- **HLR-INT-001**: The system shall verify deterministic digest-chain continuity across observable state vectors from $t_0$ through the terminal/truncation tick.
- **HLR-INT-002**: Structural inconsistency, digest-chain discontinuity, or stream truncation detected during comparison shall be classified as **Structural Invalidity Fault**.
- **HLR-INT-003**: Structural invalidity shall not be classified as functional divergence.

## E. Functional Comparison

- **HLR-CMP-001**: The system shall compare generated functional behavior against the retained functional reference.
- **HLR-CMP-002**: Functional comparison disposition shall be constrained to `exact`, `diverged`, or `incompatible`.
- **HLR-CMP-003**: Functional mismatch evidence shall be reported without rewriting execution disposition.

## F. Execution-Context Compatibility and Physical Timing

- **HLR-TIM-001**: Physical timing evaluation shall execute only when required by the declared Replay Schema.
- **HLR-TIM-002**: Timing evaluation shall require execution-context compatibility against the applicable Target Execution Profile.
- **HLR-TIM-003**: If compatibility prerequisites are not met, timing evaluation shall yield `insufficient`.
- **HLR-TIM-004**: Timing outcomes (`pass`, `fail`, `insufficient`) shall remain separate from functional comparison and execution disposition.

## G. Replay Evaluation Packaging

- **HLR-EVL-001**: The system shall package replay evaluation by associating Retained Run, Execution Record, functional comparison result, optional timing result, required target-profile context, Evidence Limitations, and Claim Boundaries.
- **HLR-EVL-002**: Replay evaluation disposition shall be constrained to `supported`, `not_supported`, `insufficient`, or `invalid`.
- **HLR-EVL-003**: `invalid` shall apply only when a required input or association is structurally invalid.

## H. Source-Result Separation and Reporting Integrity

- **HLR-REP-001**: Validation, execution, comparison, execution-context compatibility, timing evaluation, and replay evaluation shall each retain independent dispositions and stable reasons.
- **HLR-REP-002**: Operation-level summaries shall not collapse, overwrite, or reinterpret source-stage results.

## I. External Trust Boundary

- **HLR-TRU-001**: Replay shall provide deterministic identities and immutable association points suitable for external attestation binding.
- **HLR-TRU-002**: Replay shall not itself perform producer authentication, issuer-trust establishment, credential/revocation management, chain-of-custody adjudication, authorization policy decisions, or relying-party acceptance decisions.
- **HLR-TRU-003**: External attestation/policy outcomes shall not alter replay-stage technical outcomes (validation, execution, comparison, timing, replay evaluation).
