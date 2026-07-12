# High-Level Requirements - Replay Execution (HLR-REPLAY)

## 1. Public Replay Path

### HLR-REPLAY-EXEC-001: Deterministic Ordered Evaluation
Replay execution shall evaluate an ordered sequence of replay frames deterministically.

### HLR-REPLAY-EXEC-002: Initial Math Lane
Replay execution shall support an initial math lane using existing `I64F64` behavior.

### HLR-REPLAY-EXEC-003: Invalid Order Rejection
Replay execution shall reject invalid execution order.

### HLR-REPLAY-EXEC-004: Expected Result Rejection
Replay execution shall reject expected-result mismatch.

### HLR-REPLAY-EXEC-005: Deterministic Execution Result
Replay execution shall expose a deterministic execution result.


## 2. Saved Replay Input Parsing

### HLR-REPLAY-PARSE-001: Explicit Saved Input Version
Saved replay input shall declare a replay input format version.

### HLR-REPLAY-PARSE-002: Explicit Saved Input Schema/Lane
Saved replay input shall declare the replay schema/lane it uses.

### HLR-REPLAY-PARSE-003: Unknown Version Rejection
Saved replay input parsing shall reject unknown versions.

### HLR-REPLAY-PARSE-004: Unknown Schema/Lane Rejection
Saved replay input parsing shall reject unknown schema/lane values.

### HLR-REPLAY-PARSE-005: Malformed Frame Rejection
Saved replay input parsing shall reject malformed frame rows.

### HLR-REPLAY-PARSE-006: Deterministic Frame Production
Saved replay input parsing shall produce replay frames for deterministic execution.


## 3. Retained Replay Witness Checking

### HLR-REPLAY-CHECK-001: Retained Saved Input Source Check
A retained saved replay input shall be checked from source files.

### HLR-REPLAY-CHECK-002: Checker Parse Stage
Replay checking shall parse the saved input.

### HLR-REPLAY-CHECK-003: Checker Replay Stage
Replay checking shall execute the parsed frames deterministically.

### HLR-REPLAY-CHECK-004: Deterministic Witness Generation
Replay checking shall generate deterministic witness output.

### HLR-REPLAY-CHECK-005: Expected Witness Comparison
Replay checking shall compare generated witness output to retained expected witness.

### HLR-REPLAY-CHECK-006: Expected Result Comparison
Replay checking shall compare generated checker result to retained expected result.

### HLR-REPLAY-CHECK-007: Witness Mismatch Failure
Replay checking shall fail when expected witness does not match generated witness.

### HLR-REPLAY-CHECK-008: Result Mismatch Failure
Replay checking shall fail when expected result does not match generated result.


## 4. Admitted Observation Projection

### HLR-REPLAY-PROJ-001: Admission-Gated Projection
Canonical replay input shall be created only from an admitted raw ADC capture.

### HLR-REPLAY-PROJ-002: Admitted Observations Only
Projection shall include only admitted observations; rejected or malformed rows shall remain excluded.

### HLR-REPLAY-PROJ-003: Admitted Capture Reference
Canonical replay input shall identify the admitted source capture without defining how that reference is represented.

### HLR-REPLAY-PROJ-004: Admitted Observation Summary Preservation
Projection shall preserve `sample_count`, `first_sample_index`, `last_sample_index`, `min_raw_adc`, `max_raw_adc`, and the admitted `timing_claim`.

### HLR-REPLAY-PROJ-005: Optional Context Preservation
Projection shall preserve `context_id` when present and shall not infer `context_id` when absent.

### HLR-REPLAY-PROJ-006: Deterministic Projection
Projection shall be deterministic and shall not add claims beyond the admitted source evidence.
