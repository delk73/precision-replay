# Low-Level Requirements - Replay Execution (LLR-REPLAY)

## 1. Frame Model

### LLR-REPLAY-EXEC-001: Replay Frame Variants
Replay execution shall define replay frame variants for loading operands, executing math operations, and expecting result bits.
*Traces to: HLR-REPLAY-EXEC-001, HLR-REPLAY-EXEC-002*

## 2. Execution State

### LLR-REPLAY-EXEC-002: Replay Execution States
Replay execution shall define execution states sufficient to distinguish no operands loaded, operands loaded, result produced, accepted, and rejected.
*Traces to: HLR-REPLAY-EXEC-003, HLR-REPLAY-EXEC-004, HLR-REPLAY-EXEC-005*

### LLR-REPLAY-EXEC-003: Replay Rejection Reasons
Replay execution shall define rejection reasons for invalid execution order and expected-result mismatch.
*Traces to: HLR-REPLAY-EXEC-003, HLR-REPLAY-EXEC-004*

## 3. Execution Semantics

### LLR-REPLAY-EXEC-004: Pure In-Memory Execution
Replay execution shall execute as a pure in-memory operation over a frame slice.
*Traces to: HLR-REPLAY-EXEC-001, HLR-REPLAY-EXEC-005*

### LLR-REPLAY-EXEC-005: Replay Repeatability
Running the same frame slice twice shall produce the same execution result.
*Traces to: HLR-REPLAY-EXEC-001, HLR-REPLAY-EXEC-005*


## 4. Saved Input Parsing

### LLR-REPLAY-PARSE-001: Initial Saved Input Grammar
Saved replay input parsing shall accept only the initial text grammar: first line `precision-replay-input v1`, second line `schema math-i64f64-v1`, followed by zero or more frame rows.
*Traces to: HLR-REPLAY-PARSE-001, HLR-REPLAY-PARSE-002, HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-002: Saved Input Frame Rows
Saved replay input parsing shall define frame rows `load lhs=<i128> rhs=<i128>`, `add`, `sub`, `mul`, `div`, and `expect bits=<i128>`.
*Traces to: HLR-REPLAY-PARSE-005, HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-003: Parse Rejection Reasons
Saved replay input parsing shall define rejection reasons for missing version, unknown version, missing schema, unknown schema, unknown frame opcode, malformed frame rows, missing required fields, invalid integer fields, and caller-provided frame capacity exhaustion.
*Traces to: HLR-REPLAY-PARSE-003, HLR-REPLAY-PARSE-004, HLR-REPLAY-PARSE-005*

### LLR-REPLAY-PARSE-004: Pure In-Memory Parsing
Saved replay input parsing shall be a pure in-memory operation over `&str` input and shall write parsed frames into a caller-provided output buffer.
*Traces to: HLR-REPLAY-PARSE-006*

### LLR-REPLAY-PARSE-005: Parse/Execute Separation
Saved replay input parsing shall not execute replay frames; execution remains owned by `execute_replay(&[ReplayFrame])`.
*Traces to: HLR-REPLAY-PARSE-006, HLR-REPLAY-EXEC-001*


## 5. Retained Replay Witness Checker

### LLR-REPLAY-CHECK-001: Retained Replay Artifact Layout
The retained replay artifact layout shall contain `input.txt`, `expected_witness.txt`, and `expected_result.txt` under `artifacts/replay/math-i64f64-v1/`.
*Traces to: HLR-REPLAY-CHECK-001*

### LLR-REPLAY-CHECK-002: Generated Witness Text Format
The generated replay witness text format shall be one line: `precision-replay witness=replay-input-v1 schema=math-i64f64-v1 state=accepted result_bits=<i128>`.
*Traces to: HLR-REPLAY-CHECK-004, HLR-REPLAY-CHECK-005, HLR-REPLAY-CHECK-007*

### LLR-REPLAY-CHECK-003: Checker Result Text Format
The checker result text format shall be four lines: `parse=pass`, `replay=pass`, `witness=pass`, and `result=pass`.
*Traces to: HLR-REPLAY-CHECK-006, HLR-REPLAY-CHECK-008*

### LLR-REPLAY-CHECK-004: Public Replay Checker Command
The public retained replay checker command shall be `make replay-check`.
*Traces to: HLR-REPLAY-CHECK-001*

### LLR-REPLAY-CHECK-005: Checker Stage Ordering
The retained replay checker shall run parse, replay, witness, and result stages in order.
*Traces to: HLR-REPLAY-CHECK-002, HLR-REPLAY-CHECK-003, HLR-REPLAY-CHECK-004, HLR-REPLAY-CHECK-005, HLR-REPLAY-CHECK-006*

### LLR-REPLAY-CHECK-006: Checker Failure Behavior
The retained replay checker shall exit nonzero for parse failure, replay rejection, witness mismatch, and result mismatch.
*Traces to: HLR-REPLAY-CHECK-002, HLR-REPLAY-CHECK-003, HLR-REPLAY-CHECK-007, HLR-REPLAY-CHECK-008*
