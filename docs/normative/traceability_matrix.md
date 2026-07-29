# Replay Traceability Matrix

## Story ↔ Vocabulary ↔ HLR

| Story Statement ID | Story Assertion (condensed) | Vocabulary Anchor(s) | HLR Obligation(s) |
|---|---|---|---|
| ST-01 | Replay starts from Canonical Input + declared schema semantics | `Canonical Input`, `Replay Schema` | HLR-RPL-001, 015, 016 |
| ST-02 | Record operation creates immutable retained run, binds $S_0$, reference material, comparison params | `Retained Run`, `Initial State Seed ($S_0$)` | HLR-RPL-001, 003, 009 |
| ST-03 | Pre-execution validation checks structural usability; failure blocks execution and is not rejection | `Structural Usability Validation`, `Pre-Execution Gate Event`, `Execution Disposition` | HLR-RPL-002, 003, 004, 005 |
| ST-04 | Execution of valid retained run yields one execution record per occurrence | `Execution Record` | HLR-RPL-008, 010 |
| ST-05 | Execution proceeds on monotonic cycle ticks isolated from wall-clock | `Deterministic Cycle Tick ($t_k$)` | HLR-RPL-006, 007 |
| ST-06 | Execution disposition semantics: accepted/rejected/incomplete | `Execution Disposition` | HLR-RPL-011, 012, 013, 014 |
| ST-07 | Execution does not mutate retained run | `Retained Run` | HLR-RPL-009 |
| ST-08 | Integrity/tamper detection via deterministic digest chains across state vectors | `Trace Integrity Verification`, `Digest Chain Continuity`, `State Vector ($S_k$)` | HLR-RPL-020, 021 |
| ST-09 | Structural inconsistency/hash break/truncation is structural invalidity fault, not divergence | `Structural Invalidity Fault`, `Comparison Disposition` | HLR-RPL-022, 023 |
| ST-10 | Functional comparison outcomes exact/diverged/incompatible | `Comparison Disposition` | HLR-RPL-015, 016, 017, 018, 019 |
| ST-11 | Comparison mismatch evidence does not rewrite execution disposition | `Comparison Disposition`, `Execution Disposition` | HLR-RPL-028, 033 |
| ST-12 | Timing evaluation only if schema requires timing evidence | `Replay Schema`, `Physical Timing Evaluation Disposition` | HLR-RPL-024, 027 |
| ST-13 | Timing uses execution record + target profile + compatibility checks | `Execution-Context Compatibility Check`, `Target Execution Profile` | HLR-RPL-025 |
| ST-14 | Incompatible context yields timing `insufficient`; timing remains separate | `Physical Timing Evaluation Disposition` | HLR-RPL-026, 027, 028 |
| ST-15 | Replay evaluation packages claim result from retained run, execution record, comparison, optional timing, context, limitations, boundaries | `Replay Evaluation Disposition`, `Evidence Limitation`, `Claim Boundary` | HLR-RPL-029, 030, 031 |
| ST-16 | Keep source results distinct across pipeline operations | `Execution Disposition`, `Comparison Disposition`, `Replay Evaluation Disposition` | HLR-RPL-032, 033 |
| ST-17 | Deterministic identities + immutable associations for external attestation | deterministic identity/association concept in story | HLR-RPL-034 |
| ST-18 | Replay excludes authn/authz/trust/revocation/chain-of-custody/relying-party acceptance decisions | trust-boundary semantics in story | HLR-RPL-035, 036 |
| ST-19 | External policy binding cannot alter replay semantic outcomes | `Replay Evaluation Disposition` and source disposition separation concepts | HLR-RPL-032, 036 |

## Reverse Coverage (HLR → Story)

| HLR | Covered by Story ID(s) | Coverage |
|---|---|---|
| 001 | ST-01, ST-02 | Full |
| 002–005 | ST-03 | Full |
| 006–007 | ST-05 | Full |
| 008–010 | ST-04, ST-06 | Full |
| 011–014 | ST-06 | Full |
| 015–019 | ST-10 | Full |
| 020–023 | ST-08, ST-09 | Full |
| 024–028 | ST-12, ST-13, ST-14 | Full |
| 029–031 | ST-15 | Full |
| 032–033 | ST-16 | Full |
| 034 | ST-17 | Full |
| 035–036 | ST-18, ST-19 | Full |

## Gap Analysis

- No blocking traceability gaps: all HLRs (001–036) map to at least one story assertion and at least one vocabulary anchor.
- Minor precision gap (non-blocking): deterministic identity term is asserted in story but not yet a dedicated vocabulary entry; can be added later for lexical completeness.
- Minor precision gap (non-blocking): “required input or association structurally invalid” in HLR-RPL-031 could be backed by an explicit decision table in LLR for auditability.
