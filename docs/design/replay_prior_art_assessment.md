# Preliminary Replay Prior-Art Assessment

This document is a non-normative design and research note. The normative Replay sources remain:

- `docs/normative/HLR_replay.md`
- `docs/design/LLR_replay.md`
- `docs/normative/traceability_matrix.md`

Classification in this note does not change requirement status, implementation status, verification status, or traceability status. This note records the current assessment so novelty and overlap questions can be reviewed explicitly rather than reconstructed from conversation. It does not assert that Precision Replay is novel, patentable, fundable, or an established research contribution.

Overlap with established replay, provenance, workflow, packaging, metrology, assurance, and supply-chain systems is useful validation. Conventional or adapted classifications are not negative results; they identify places where Replay is aligned with mature practice and where the design should avoid unnecessary invention.

Representative systems and standards reviewed or identified for deeper comparison include `rr`, ReVirt and related deterministic replay systems, Workflow Run RO-Crate, Provenance Run Crate, CWL and CWLProv, W3C PROV, ReproZip, Whole Tale, Nix, Guix, in-toto, SLSA, digital calibration certificate and digital metrology work, GSN, SACM, Resolute, and Assurance 2.0.

## Assessment Status

This is an initial capability-level assessment based on the current Replay requirements and a preliminary review of neighboring system families. It is intended to guide design review and identify focused comparison work.

The assessment is not a completed literature review, standards analysis, patent search, or exhaustive product survey. A named system may have been considered at the family or documented-capability level without every relevant specification, implementation, or publication having been examined.

Classifications may change as the Replay contract develops and as stronger prior-art matches are identified.

Because the current HLR review is still in progress, some classifications may reflect requirements that are later simplified, split, removed, or relocated.

## Classification Terms

| Classification | Meaning |
| --- | --- |
| Conventional | The requirement or capability substantially reflects established engineering or research practice. |
| Adapted | The requirement or capability applies an established concept to the Replay contract or strengthens it through Replay-specific boundaries. |
| Apparently differentiating | The same explicit rule, boundary, or combination has not yet been identified in the prior systems examined to date. This classification is provisional and may reflect incomplete comparison rather than a genuine technical distinction. It is not a novelty or patentability conclusion. |
| Unresolved | The available comparison is insufficient, the Replay requirement is not yet complete enough to compare, or additional source review is required. |

### Evidence Strength

| Evidence strength | Meaning |
| --- | --- |
| Strong | The assessment is directly supported by well-established specifications, implementations, or repeatedly demonstrated practice, though specific citations may still be added. |
| Moderate | The assessment is supported by a close prior-art pattern, but detailed comparison of the exact Replay boundary remains incomplete. |
| Preliminary | The assessment is based on limited comparison and should be treated primarily as a hypothesis or search direction. |

## Prior-Art Families

### Deterministic Replay

Deterministic replay systems such as `rr`, ReVirt, and related VM or process replay systems validate the importance of repeatable execution, recorded execution context, and controlled nondeterminism. Replay's deterministic execution requirements and external-state exclusions are therefore substantially conventional in principle.

The current Replay contract appears more focused on retained-run identity, post-run comparison, bounded evaluation, and separation of generated evidence from retained functional identity than on capturing and replaying every low-level nondeterministic event. That boundary is currently assessed as adapted, with some apparently differentiating combinations where Replay requires non-mutation and non-collapsing result ownership across validation, execution, comparison, timing, target context, and evaluation.

### Workflow Reproducibility

Workflow systems and research object packaging, including Workflow Run RO-Crate, Provenance Run Crate, CWL, and CWLProv, provide strong precedents for declared workflows, inputs, outputs, software environments, run metadata, and provenance. Replay's declared modeled-execution dependencies, provenance associations, and execution records overlap usefully with this family.

Replay differs in current emphasis by treating schema-owned functional semantics, retained-run identity, generated execution occurrence evidence, and final bounded evaluation as separate objects whose source results are not rewritten by later operations. This is currently assessed as adapted to apparently differentiating depending on the specific boundary.

### Provenance Models

W3C PROV and related provenance models provide established vocabulary for entities, activities, agents, derivation, association, attribution, and usage. Replay's separate immutable provenance associations are conventional in broad provenance terms.

The open question is whether Replay should define a restricted W3C PROV profile or mapping instead of inventing a separate provenance model. Current Replay requirements already distinguish functional identity from upstream provenance, but provenance interoperability remains unresolved.

### Executable Research Packaging

ReproZip, Whole Tale, and related executable research packaging systems validate packaging inputs, code, environments, data, and execution instructions so a computation can be rerun. Replay's retained-run content, dependency binding, and execution evidence overlap with this family.

Replay's current candidate contribution is not packaging by itself. The stronger distinction is the explicit separation of retained functional reference content, execution occurrence evidence, functional comparison, timing observations, timing evaluation, target profile, diagnostics, provenance, and bounded claim evaluation.

### Reproducible Builds and Declared Dependencies

Nix, Guix, and reproducible build work provide close prior-art patterns for declared dependencies, content-addressed identity, deterministic construction, environment closure, and exclusion of undeclared ambient state. Replay's schema-declared modeled-execution dependencies and deterministic identity concepts are therefore largely conventional or adapted.

Replay applies those ideas to retained functional replay objects and execution/evaluation evidence rather than only build outputs or package derivations. Digest contracts, collision handling, independent implementation criteria, and canonicalization vectors remain insufficiently specified for strong conformance claims.

### Metrology and Measurement Traceability

Digital calibration certificates, digital metrology work, and measurement traceability practice provide prior art for calibration chains, uncertainty, measurement metadata, decision rules, and traceable evidence. Replay's separation of physical timing observations from timing judgments is aligned with these practices.

Replay has not yet defined enough measurement evidence semantics to claim more than adaptation. Calibration traceability, uncertainty propagation, synchronization uncertainty, clock drift, measurement transformation, decision rules, and confidence or coverage interpretation remain unresolved.

### Assurance Cases and Evidence-Based Claim Systems

GSN, SACM, Resolute, Assurance 2.0, and related assurance-case systems provide established patterns for structured claims, evidence, argument, assumptions, context, and confidence. Replay's bounded claim evaluation and evidence limitation language overlaps with this family.

Replay's apparently differentiating element, if it survives further comparison, is not the existence of claims or evidence. It is the systematic rule that evaluation consumes stable source results and limitations without rewriting retained-run validation, execution disposition, comparison disposition, timing disposition, target-context compatibility, or source reasons.

### Attestations and Software Supply-Chain Evidence

in-toto, SLSA, and related supply-chain evidence systems provide strong precedents for signed attestations, provenance, build steps, materials, products, policy evaluation, and relying-party trust decisions. Replay's stable identities and immutable association points are compatible with this family.

Replay does not currently attempt to replace attestation or policy systems. Authenticity, signer trust, authorization, revocation, chain of custody, and relying-party acceptance are expected to remain in an external attestation or policy layer. Replay's deterministic identities and immutable association points are intended to support that external layer.

### Conformance Testing and Formal Specification

Protocol conformance suites, language specification test suites, model-based testing, differential testing, reference implementations, independent implementations, and formal executable specifications provide close patterns for schema-defined behavior, canonical serialization, cross-implementation agreement, compatibility versus divergence, stable error reasons, and implementation independence.

Replay's cross-target and cross-implementation questions should therefore not be treated only as replay or reproducibility issues. They also belong to standards-conformance practice. Current Replay requirements identify several necessary boundaries, but conformance levels, vectors, disagreement handling, and semantic closure criteria remain unresolved.

## Working Conclusion

Precision Replay is not clearly novel merely because it packages replay inputs, execution results, provenance, target context, timing evidence, and evaluations. Those broad ingredients are well represented across deterministic replay, workflow reproducibility, provenance, executable research packaging, reproducible build, metrology, assurance, and supply-chain evidence systems.

The leading current candidate contribution is the systematic, non-collapsing separation of:

- functional retained-run identity
- execution occurrence evidence
- functional comparison
- physical timing observations
- timing evaluation
- provenance
- target execution context
- diagnostics
- bounded claim evaluation

The important rule is that later operations may consume and reference earlier results but shall not rewrite, replace, or redefine those source results.

This is a working hypothesis requiring further comparison and implementation evidence. The apparently differentiating classification used below means only that the same explicit rule, boundary, or combination has not yet been identified in the prior systems examined to date.

## Capability-Level Prior-Art Matrix

| Replay capability | Closest prior-art pattern | Classification | Evidence strength | Current assessment | Follow-up needed |
| --- | --- | --- | --- | --- | --- |
| Schema-owned canonical input meaning (`HLR-REPLAY-SCHEMA-001`, `HLR-REPLAY-SYS-003`) | Workflow languages, file format schemas, CWL input schemas, RO-Crate profiles | Adapted | Moderate | Established systems bind meaning to schemas or profiles. Replay adapts this by separating common retained-run structure from schema-specific canonical input meaning. | Compare against CWL typing, RO-Crate profiles, and domain-specific workflow schema extension rules. |
| Schema-owned state evolution (`HLR-REPLAY-SCHEMA-002`, `HLR-REPLAY-EXEC-007`, `HLR-REPLAY-EXEC-008`) | Deterministic replay engines, workflow step semantics, simulation schemas | Conventional | Moderate | Assigning execution semantics to a declared engine or schema is established practice. | Confirm how replay schemas version semantic changes relative to prior workflow and simulation systems. |
| Schema-owned trace and terminal semantics (`HLR-REPLAY-SCHEMA-003`, `HLR-REPLAY-SCHEMA-004`, `HLR-REPLAY-TRACE-*`) | Event traces in deterministic replay, workflow run logs, execution provenance | Adapted | Preliminary | Trace ownership is conventional, but Replay explicitly separates trace, execution disposition, and terminal outcome. | Compare with `rr` event logs, CWLProv outputs, workflow status models, test verdict systems, protocol conformance, and state-machine specifications. |
| Declared modeled-execution dependencies (`HLR-REPLAY-SCHEMA-007`, `HLR-REPLAY-EXEC-013`, `HLR-REPLAY-EXEC-014`) | Nix, Guix, reproducible builds, workflow input declarations | Conventional | Moderate | Declared dependencies and ambient-state exclusion are well-established. | Specify conformance tests for dependency closure and undeclared dependency detection. |
| Immutable retained-run content (`HLR-REPLAY-RUN-001`, `HLR-REPLAY-EXEC-012`) | Content-addressed artifacts, research packages, provenance entities, build derivations | Conventional | Strong | Immutability is established and desirable. | Define package mutation detection and update/versioning model. |
| Canonical retained-run identity (`HLR-REPLAY-RUN-003`) | Content-addressed storage, Nix store paths, Guix derivations, in-toto subjects | Adapted | Moderate | Content addressing is conventional. Replay's exact identity domain, including retained reference behavior and comparison parameters, is Replay-specific. | Define normative serialization, canonicalization vectors, digest contracts, and collision behavior. |
| Exclusion of provenance and target context from functional identity (`HLR-REPLAY-RUN-004`, `HLR-REPLAY-RUN-005`, `HLR-REPLAY-RUN-006`) | Separation of artifact identity from metadata in build and provenance systems | Apparently differentiating | Preliminary | The broad separation is familiar. The explicit functional retained-run identity exclusion list and its use across replay comparison and evaluation has not yet been matched in the systems examined to date. | Search for equivalent exclusion rules in RO-Crate profiles, CWLProv, in-toto predicates, and research object identity guidance. |
| Separate immutable provenance associations (`HLR-REPLAY-RUN-006`) | W3C PROV associations, in-toto links, RO-Crate contextual entities | Adapted | Moderate | Separate provenance is conventional; Replay adapts it as an immutable association to functional retained-run identity that does not participate in that identity. | Decide whether to define a W3C PROV profile or mapping. |
| Deterministic pre-execution validation (`HLR-REPLAY-RUN-002`, `HLR-REPLAY-RUN-007`) | Package validation, schema validation, workflow static checks, supply-chain policy checks | Conventional | Moderate | Pre-execution validation and stable reasons are established engineering practice. | Define validation reason taxonomy, canonical ordering, and conformance expectations. |
| Validation and execution separation (`HLR-REPLAY-RUN-008`, `HLR-REPLAY-EXEC-009`) | Compiler/load validation versus execution, workflow planning versus execution | Adapted | Moderate | Separation is familiar, but Replay explicitly forbids validation failure from becoming execution rejection or producing functional trace. | Compare with workflow planning failure and deterministic replay load failure models. |
| Deterministic execution (`HLR-REPLAY-EXEC-001`, `HLR-REPLAY-EXEC-005`, `HLR-REPLAY-EXEC-008`) | `rr`, ReVirt, deterministic simulation, reproducible workflow execution | Conventional | Strong | Deterministic execution and replay of controlled behavior are established patterns. Replay uses a different abstraction boundary by executing canonical input under declared schema semantics rather than necessarily reproducing every low-level event from a previously observed process. | Clarify which forms of nondeterminism are modeled dependencies versus excluded external state. |
| Immutable execution records (`HLR-REPLAY-EXEC-015`, `HLR-REPLAY-EXEC-016`, `HLR-REPLAY-EXEC-017`) | Workflow run records, provenance activities, in-toto link metadata, execution logs | Adapted | Moderate | Execution occurrence evidence is conventional, but Replay makes occurrence identity and execution-record identity separate from retained-run identity. | Compare identity and copy semantics with workflow run crates and in-toto link metadata. |
| Execution-context recording (`HLR-REPLAY-EXEC-020`) | Workflow runtime metadata, ReproZip environment capture, supply-chain builder metadata | Conventional | Moderate | Recording implementation, target, runtime, scheduler, resource, and measurement context is established. | Define exact required fields by profile and how false or incomplete context is handled. |
| Timing observation and timing judgment separation (`HLR-REPLAY-EXEC-021`, `HLR-REPLAY-TIME-*`) | Metrology measurement records and decision rules | Adapted | Moderate | Measurement evidence separate from pass/fail judgment is established in metrology; Replay adapts it to replay execution records and profile-bound timing claims. | Specify calibration, uncertainty propagation, clock synchronization, drift, transformation, and decision rules. |
| Trace, execution disposition, and terminal outcome separation (`HLR-REPLAY-TRACE-002`, `HLR-REPLAY-TRACE-004`, `HLR-REPLAY-EXEC-018`) | State-machine theory, test models, protocol conformance, safety assurance, workflow statuses, execution logs | Apparently differentiating | Preliminary | The pieces are common, and equivalent separations may exist under different vocabulary. The current candidate distinction is Replay's explicit non-collapsing rule across trace, disposition, terminal outcome, comparison, and evaluation. | Compare with deterministic replay trace/event models, workflow status/result ontologies, state-machine specifications, protocol conformance suites, and safety assurance models. |
| Exact, diverged, and incompatible functional comparison (`HLR-REPLAY-COMP-001`, `HLR-REPLAY-COMP-002`, `HLR-REPLAY-COMP-003`) | Diff tools, conformance testing, regression replay, workflow result comparison | Adapted | Strong | Exact matching, divergence reporting, and compatibility gating are established comparison patterns. Replay combines them into one deterministic three-disposition functional comparison contract. | Define compatibility checks and schema-specific comparison conformance tests. |
| First-divergence reporting (`HLR-REPLAY-COMP-004`) | Trace diffing, deterministic replay debugging, regression test failure localization | Conventional | Strong | First-difference reporting is established. | Specify canonical first-divergence selection for unordered or partially ordered schema traces if such traces are later allowed. |
| Bounded evaluation outcomes `supported`, `not_supported`, `insufficient`, `invalid` (`HLR-REPLAY-EVAL-004`) | Assurance cases, evidence assessment, test verdict systems, metrology conformity decisions | Adapted | Moderate | The dispositions align with established evidence judgment patterns. Replay adapts them to deterministic evaluation over source results and limitations. | Develop formal claim syntax and derivation rules distinguishing unsupported, contradicted, insufficient, and structurally invalid evidence. |
| Non-mutation of source results by evaluation (`HLR-REPLAY-COMP-007`, `HLR-REPLAY-TIME-006`, `HLR-REPLAY-EVAL-006`) | Immutable provenance and audit records, assurance evidence references | Apparently differentiating | Preliminary | Immutable source evidence and derived judgments are established patterns. Replay's potentially distinguishing feature is the systematic application of non-mutation and source-result ownership across validation, execution, comparison, context compatibility, timing, and final evaluation. | Compare against SACM, Assurance 2.0, Resolute, GSN tooling, in-toto policy evaluation, and SLSA verification behavior. |
| Target execution profiles (`HLR-REPLAY-TPROF-*`) | Test environment specifications, platform profiles, supply-chain builder identity, metrology measurement conditions | Adapted | Moderate | Target profile concepts are established; Replay adapts them as optional profile-bound claim inputs separate from retained-run validation and execution disposition. | Define profile conformance levels, identity, compatibility rules, and unsupported profile-free execution handling. |
| Cross-target replay agreement (`HLR-REPLAY-TGT-001`, `HLR-REPLAY-TGT-002`) | Cross-platform reproducibility, reproducible builds, differential testing | Adapted | Preliminary | Multi-target agreement is an established goal. Replay adapts it to retained-run association, schema compatibility, functional comparison, optional timing, and non-mutation. Current requirements do not yet establish independent implementation conformance strongly enough to support a stronger claim. | Compare with Guix/Nix reproducible builds, cross-platform workflow reruns, differential testing literature, and conformance-suite practices. |
| Source-evidence versus replay-evidence separation (`HLR-REPLAY-ORIGIN-*`, `HLR-REPLAY-PROJ-*`, `HLR-REPLAY-OPS-005`) | Provenance layering, raw data admission, workflow input derivation | Adapted | Preliminary | Provenance systems commonly distinguish raw observations, transformations, workflow inputs, and derived outputs. Replay appears to adapt that layering into an explicit admission, projection, and replay-evidence boundary, but direct comparison remains incomplete. | Review CWLProv, RO-Crate derivations, digital lab notebook provenance, and metrology evidence chains. |

## Gaps Exposed by the Comparison

### Independent Implementation and Conformance

Replay needs more detail before independent implementation claims can be assessed. Open areas include normative serialization, canonicalization vectors, conformance levels, digest contracts, collision handling, independent implementation criteria, and cross-implementation disagreement handling.

### Semantic Closure Criteria

Replay uses semantic closure as a central design question, but it has not yet defined what would demonstrate closure. Open areas include what information a schema must declare, how undeclared semantic dependencies are detected, whether closure is absolute or claim-relative, whether implementation-defined behavior is permitted, what evidence demonstrates that two independent implementations interpreted the same contract, and how semantic ambiguity is distinguished from implementation defect.

### Formal Claim Derivation

Replay currently identifies bounded evaluation outcomes but does not yet define a full claim calculus. Open areas include claim syntax, evidence requirements by claim type, derivation rules, stronger and weaker claim relationships, conflicting evidence, and the distinction between unsupported, contradicted, insufficient, and structurally invalid evidence.

### Portable Package Composition

Replay has not yet specified a complete portable package model. Open areas include manifest structure, required and optional objects, embedded versus external references, offline resolution, missing-object behavior, package identity, package versioning, and partial disclosure.

### Provenance Interoperability

Replay should evaluate whether to define a restricted W3C PROV profile or mapping rather than inventing a separate provenance model. The current requirements already need immutable provenance association points, but the vocabulary and exchange model remain unresolved.

### Measurement Evidence

Replay's timing and measurement requirements need additional comparison with metrology practice. Open areas include calibration traceability, uncertainty propagation, synchronization uncertainty, clock drift, measurement transformation, decision rules, and confidence or coverage interpretation.

### Trust Boundary

Authenticity, signer trust, authorization, revocation, chain of custody, and relying-party acceptance are expected to remain in an external attestation or policy layer. Replay's deterministic identities and immutable association points are intended to support that external layer.

### Threat and Failure Model

Replay should make its threat and failure model explicit. At minimum, the model should cover accidental mutation, undeclared dependencies, referenced-object substitution, incomplete evidence resolution, false execution-context reporting, compromised implementations, and concealed nondeterminism.

## Comparison Source Record

Detailed source citations, specification versions, reviewed sections, and comparison dates remain to be added as focused comparison work is completed. Until a source is recorded here, a named system may represent a family-level comparison lead rather than a completed specification-level review.

| System or standard | Source or version | Material reviewed | Review status | Relevant Replay capabilities |
| --- | --- | --- | --- | --- |

## Current Use of This Note

This note should be treated as a living research and design-control note that records current comparisons, uncertainties, and follow-up work. It may help prioritize follow-up comparison and requirement refinement, but it does not approve a requirement, implement a feature, verify a traceability row, or establish that any apparently differentiating capability is a proven advance.
