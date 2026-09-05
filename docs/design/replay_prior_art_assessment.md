# Replay Prior-Art Assessment: R-LAM and the Projection Boundary

## Status

This is a non-normative working note recovered from `docs/replay-system-contract`. It has not passed the `requirements-model-workflow-mcp` workflow.

Only the R-LAM paper and implementation listed below were directly reviewed during the September 4, 2026 pass.

## R-LAM Comparison

R-LAM is contemporaneous neighboring work. It overlaps with Precision Replay in structured actions, deterministic execution policies, execution traces, recorded failures, retained action results, and controlled workflow branching.

The central operational difference is:

> R-LAM replay retrieves stored workflow outputs, while Precision Replay executes canonical input again and compares the newly produced result with retained evidence.

R-LAM does not process physical acquisitions or information-losing projections. Its current implementation excludes hardware interaction. Its paper identifies hardware-in-the-loop execution and formal trace verification as future work.

## Projection Boundary

Let `S1` and `S2` be distinct admitted physical source records. Let projection `P` produce canonical replay input `C`:

```text
S1 != S2
P(S1) = C
P(S2) = C
```

Precision Replay can retain `S1` and `S2` as distinct source records, including their identities, projections, and hashes.

Results produced by re-executing `C` can be compared across executions or targets. Because those executions receive only `C`, their results cannot identify which source record produced `C`, recover differences discarded by `P`, or establish that the two physical acquisitions were equivalent.

The candidate claim examined here is:

> Deterministic re-execution can establish functional agreement for a canonical replay input under declared execution semantics. It cannot, by itself, establish equivalence of the physical acquisitions that projected to that input.

The reviewed R-LAM sources do not address projection collisions between distinct physical source records.

## Project Disposition

Existing work in Precision Replay and precision-signal provides substantial implementation and evidence foundations, including physical-signal acquisition, retained raw-ADC evidence, STM32/DRV425 work, deterministic fixed-point execution, host and embedded execution paths, functional comparison, and evidence packaging.

The broader work should proceed in a fresh `claim-preserving-reduction` project, beginning with requirements developed through `requirements-model-workflow-mcp`.

Precision Replay should then serve as one candidate implementation of those requirements. Stored-output reconstruction, deterministic re-execution, and other workflow approaches can be evaluated against the same projection-collision witness to determine which conclusions their evidence supports.

No broader Precision Replay changes are proposed by this note.

## Sources Reviewed

### R-LAM Paper

- Source: Suriya Sureshkumar, “R-LAM: Reproducibility-Constrained Large Action Models for Scientific Workflow Automation”
- Version: arXiv:2601.09749v1, submitted 2026-01-12
- Material reviewed: Abstract; Sections I, V-A through V-E, VI, VII-G, VIII-A, IX, and X
- Review date: 2026-09-04
- URL: <https://arxiv.org/abs/2601.09749>
- Finding: The paper describes immutable structured actions, deterministic execution, provenance traces, stored-output replay, controlled forking, and recorded failures. Hardware-in-the-loop execution and formal trace verification are future work.

### R-LAM Reference Implementation

- Source: `suriyasureshok/rlam`, research prototype v0.1.0
- Commit: `e7dd7171f6aaf935af734305369848db842f348a`, dated 2026-01-12
- Material reviewed: `README.md`, `src/rlam/replay.py`, `src/rlam/fork.py`, `tests/test_replay.py`, and `tests/test_invariants.py`
- Review date: 2026-09-04
- URL: <https://github.com/suriyasureshok/rlam/tree/e7dd7171f6aaf935af734305369848db842f348a>
- Finding: Replay returns a stored output without invoking the underlying function. Forking copies prior trace content into an independent trace without modifying or re-executing the original trace.
