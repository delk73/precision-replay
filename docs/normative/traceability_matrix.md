# Replay Layered Traceability Matrix

This matrix is the sole normative authority for layered bi-directional mappings:
$Story \leftrightarrow Vocabulary \leftrightarrow HLR \leftrightarrow LLR$.

## Matrix

| Story Statement ID | Vocabulary Terms | HLR Obligations | LLR Obligations | Coverage / Gap Notes |
|---|---|---|---|---|
| **ST-001** Replay starts from canonical input + declared schema | Canonical Input, Replay Schema | HLR-RET-001, HLR-VAL-002 | LLR-RET-001, LLR-RET-002, LLR-VAL-002 | Covered |
| **ST-002** Record operation creates immutable retained run with bound dependencies and $S_0$ | Retained Run, Initial State Seed ($S_0$), Immutable Association Point | HLR-RET-001, HLR-RET-002, HLR-RET-003 | LLR-RET-002, LLR-RET-003, LLR-RET-005 | Covered |
| **ST-003** Structural validation gates execution start | Structural Usability Validation, Pre-Execution Gate Event | HLR-VAL-001..003 | LLR-VAL-001..005 | Covered |
| **ST-004** One execution occurrence yields one execution record | Execution Record, Execution Disposition | HLR-EXE-002, HLR-EXE-003, HLR-EXE-004 | LLR-EXE-005, LLR-REC-001..003 | Covered |
| **ST-005** Replay advances over discrete monotonic ticks independent of wall-clock | Deterministic Cycle Tick ($t_k$), State Vector ($S_k$) | HLR-EXE-001 | LLR-EXE-001, LLR-EXE-003, LLR-REC-002 | Covered |
| **ST-006** Execution disposition semantics (`accepted`,`rejected`,`incomplete`) | Execution Disposition | HLR-EXE-004 | LLR-EXE-004, LLR-REC-003 | Covered |
| **ST-007** Execution does not mutate retained run | Retained Run | HLR-RET-003 | LLR-RET-005 | Covered |
| **ST-008** Trace integrity via deterministic digest chain across ticks | Trace Integrity Verification, Digest Chain Continuity | HLR-INT-001 | LLR-INT-001, LLR-INT-002 | Covered |
| **ST-009** Structural inconsistency / broken continuity / truncation => structural invalidity fault (not divergence) | Structural Invalidity Fault | HLR-INT-002, HLR-INT-003 | LLR-INT-003, LLR-INT-004 | Covered |
| **ST-010** Integrity verification independent of PKI/network | Trace Integrity Verification | HLR-INT-001 (implicit), HLR-INT-003 (classification separation) | LLR-INT-005 | Partially covered at HLR level (LLR complete) |
| **ST-011** Functional comparison results `exact\|diverged\|incompatible` | Comparison Disposition | HLR-CMP-001, HLR-CMP-002 | LLR-CMP-001, LLR-CMP-004, LLR-CMP-005 | Covered |
| **ST-012** Comparison mismatch evidence must not rewrite execution disposition | Comparison Disposition, Execution Disposition | HLR-CMP-003 | LLR-CMP-003, LLR-CMP-006 | Covered |
| **ST-013** Timing evaluation only if schema requires it | Physical Timing Evaluation Disposition, Target Execution Profile | HLR-TIM-001 | LLR-TIM-001 | Covered |
| **ST-014** Timing requires execution-context compatibility; else `insufficient` | Execution-Context Compatibility Check | HLR-TIM-002, HLR-TIM-003 | LLR-TIM-002, LLR-TIM-003 | Covered |
| **ST-015** Timing result remains separate from functional comparison | Physical Timing Evaluation Disposition | HLR-TIM-004 | LLR-TIM-005 | Covered |
| **ST-016** Replay evaluation packaging and dispositions | Replay Evaluation Disposition, Evidence Limitation, Claim Boundary | HLR-EVL-001..003 | LLR-EVL-001..005 | Covered |
| **ST-017** Preserve source-stage dispositions; no collapsing/rewriting | Stable Diagnostic Reference | HLR-REP-001, HLR-REP-002 | LLR-REP-001..005 | Covered |
| **ST-018** Deterministic identities + immutable association points for external trust | Deterministic Identity, Immutable Association Point | HLR-TRU-001 | LLR-RET-004, LLR-TRU-001 | Covered |
| **ST-019** Replay excludes auth/trust/policy acceptance responsibilities | Claim Boundary | HLR-TRU-002, HLR-TRU-003 | LLR-TRU-002, LLR-TRU-003, LLR-TRU-004 | Covered |
| **ST-020** Common replay excludes upstream admission/parsing/hardware qualification concerns | Claim Boundary, Evidence Limitation | HLR-TRU-002, HLR-EVL-001 | LLR-EVL-004, LLR-TRU-004 | Partially covered (explicit exclusion list could be sharpened in HLR/LLR) |

## Gap Notes

1. Add explicit HLR statement for integrity verification independence from external key infrastructure/network dependencies.
2. Add explicit enumerated exclusions for upstream source admission/parsing/raw-ADC projection/hardware qualification at HLR/LLR boundary clauses.
3. Add a concrete normative decision-table artifact referenced by LLR-EVL-003.
