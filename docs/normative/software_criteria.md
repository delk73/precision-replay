# Software Development & Verification Criteria (Level A Adapted)

## 1. Process Classification and Deferrals
This project targets the technical execution rigor of DO-178C Level A within a single-developer workspace. 

* **Current Status:** Active enforcement includes architectural isolation, language subset restrictions, bounded-resource coding constraints, repository-supported CI validation, and formal verification within the currently defined Kani proof surfaces. Completion of full-scope arithmetic proof closure, full decision-coverage evidence, including any required structural object-code analysis, remains a future-state verification objective.
* **Deferred Objectives:** Independent peer review, formal change control boards, tool qualification, full-scope arithmetic proof closure, full decision-coverage evidence, and structural object-code analysis are deferred indefinitely due to organizational single-entity constraints and current repository proof-surface bounds.

## 2. Coding Standards & Invariants (CS-INV)

### CS-INV-001: Memory Allocation
The entire workspace shall compile under strict `#![no_std]` constraints. Dynamic memory allocation via a heap or the `alloc` crate is strictly prohibited. All memory layouts must be static and bounded.

### CS-INV-002: Error Handling and Termination
Stack unwinding is prohibited. The virtual workspace shall force `panic = "abort"` globally. Any runtime exception or failed invariant must result in immediate hardware termination.

### CS-INV-003: Language Subset
The code shall use safe Rust primitives. The `unsafe` keyword is restricted exclusively to hardware peripheral register mappings inside the BSP crates and must be documented with explicit safety justifications.

## 3. Requirements & Design Control
No source code shall be written unless it satisfies an explicit Low-Level Requirement that traces directly back to a High-Level Requirement. Code lacking bidirectional traceability is classified as extraneous and shall be removed.