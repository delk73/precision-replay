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

Base CI exists under `.github/workflows/`, local hook/template setup exists through
`.githooks/` and `scripts/install-hooks.sh`, and the repository includes a small
`Makefile` for CI contract checks. There is no generated documentation index in
this snapshot.

## Public Replay Boundary

`precision-replay` defines the public replay grammar: deterministic frame semantics, artifact format, retained evidence checks, and verification-facing contracts.

In this repository, replay compatibility means that the same retained input artifact produces the same interpreted frame sequence, state transition expectations, and validation result under the public contract tests.

Operational hardening is intentionally out of scope for the public core. Downstream profiles may add private deployment policy, hardware authority models, redundancy, fault containment, telemetry policy, or other operational-envelope protections, but those profiles must not redefine replay semantics.

A downstream profile may change the operational envelope. It may not change:

- frame meaning
- deterministic state transition requirements
- retained artifact grammar
- evidence validation rules
- public replay compatibility tests

Public replay is the grammar. Hardened replay is a private operational profile. Downstream profiles consume the public contracts, continue passing the public replay contract tests, and do not backwash private deployment logic into the public architecture.

## Workspace Layout

```text
.
|-- .github/
|   `-- workflows/       # base CI validation workflow
|-- .githooks/            # repository-local git hooks
|-- core/                 # no-std replay math crate
|-- verification/         # Kani proof and verification crate
|-- bsp/
|   |-- pru/              # PRU board-support boundary
|   `-- stm32/            # STM32 board-support boundary
|-- runners/
|   |-- pru-runner/       # PRU runner binary
|   `-- stm32-runner/     # STM32 runner binary
|-- scripts/              # hook setup and CI contract checks
|-- docs/                 # requirements and verification documents
|-- engineering_contract.md
|-- Cargo.toml
|-- Makefile
`-- rust-toolchain.toml
```

## Current Validation Commands

Use the toolchain pinned by `rust-toolchain.toml`.

```sh
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --locked -- -D warnings
cargo check -p precision-replay-core --no-default-features --target thumbv7m-none-eabi --locked
cargo check -p bsp-stm32 --no-default-features --features stm32f446 --target thumbv7m-none-eabi --locked
cargo check -p bsp-pru --no-default-features --target thumbv7m-none-eabi --locked
cargo check -p stm32-runner --no-default-features --target thumbv7m-none-eabi --locked
cargo check -p pru-runner --no-default-features --target thumbv7m-none-eabi --locked
```

The root workspace currently sets `core` as the default member, so an unqualified `cargo test` exercises the core crate rather than every workspace member.

Run Kani for math, verification, proof-surface, or contract changes where
applicable:

```sh
cargo kani -p verification
```

## Deferred Surfaces

These surfaces are not yet established in this snapshot:

- repository-owned validation wrapper scripts beyond current hook/setup and CI
  contract helper scripts
- evidence artifact capture paths
- generated traceability extraction output
- docs index
- release evidence packages
