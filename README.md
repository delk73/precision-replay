# precision-replay

precision-replay is an early extraction MVP for deterministic replay foundations and embedded-platform integration experiments. The repository is being shaped for evidence-ready engineering discipline, but it does not currently claim DO-178C DAL A compliance.

## Current Status

The current implemented surface is intentionally small:

- `core` contains the no-std replay math substrate and is the default workspace member.
- `verification` contains formal verification harnesses for externally visible behavior.
- `bsp` contains board-support-package crates for target-specific boundaries.
- `runners` contains platform runner binaries for embedded execution paths.
- `docs` contains requirements, criteria, traceability, and verification planning material.
- `engineering_contract.md` defines the repository engineering controls and evidence-readiness expectations.

There is no CI pipeline, Makefile, or generated documentation index in this snapshot.

## Workspace Layout

```text
.
|-- core/                 # no-std replay math crate
|-- verification/         # Kani proof and verification crate
|-- bsp/
|   |-- pru/              # PRU board-support boundary
|   `-- stm32/            # STM32 board-support boundary
|-- runners/
|   |-- pru-runner/       # PRU runner binary
|   `-- stm32-runner/     # STM32 runner binary
|-- docs/                 # requirements and verification documents
|-- engineering_contract.md
|-- Cargo.toml
`-- rust-toolchain.toml
```

## Current Validation Commands

Use the toolchain pinned by `rust-toolchain.toml`.

```sh
cargo check --workspace
cargo test --workspace
cargo kani -p verification
```

The root workspace currently sets `core` as the default member, so an unqualified `cargo test` exercises the core crate rather than every workspace member.

## Deferred Surfaces

These surfaces are not yet established in this snapshot:

- CI orchestration
- repository-owned validation wrapper scripts
- evidence artifact capture paths
- generated traceability extraction output
- docs index
- release evidence packages
