# MVP RC1 Evidence Package

## 1. Package Identity

- Candidate ID: `mvp-rc1`
- Branch: `release/finalize-mvp-readiness`
- Package baseline SHA: `adbd130b455ded4f97719c62c5f6a92e29c40d80`
- Repository: `delk73/precision-replay`

## 2. Purpose

This file is a reviewer-facing index of retained MVP RC1 readiness evidence for candidate `mvp-rc1`.

This is a candidate-specific evidence package. It is not a generic permanent MVP package, not a certification package, and not a release-authority claim.

## 3. Retained Hardware Evidence

- Artifact document: `docs/evidence/mvp-rc1/hardware_replay_artifact.md`
- Raw transcript: `docs/evidence/mvp-rc1/hardware_replay_transcript.txt`
- Artifact commit: `adbd130b455ded4f97719c62c5f6a92e29c40d80`
- Replay vector: `math-add-001`
- Result: `PASS`
- Transcript byte count: `96`
- Transcript SHA-256: `a2ee8d71ff72bfdb674ce6aedfac494aa21ded82be4becf5337a5aa22cd9dce2`
- Expected payload: `precision-replay mvp-rc1 vector=math-add-001 result_bits=0x00000000000000020000000000000000`

## 4. Validation Evidence

The retained hardware artifact records the following validation evidence:

- STM32 runner build command:

```sh
cargo build -p stm32-runner --no-default-features --target thumbv7m-none-eabi --locked
```

- ELF-to-BIN conversion command:

```sh
llvm-objcopy -O binary target/thumbv7m-none-eabi/debug/stm32-runner target/thumbv7m-none-eabi/debug/stm32-runner.bin
```

- ST-Link flash command:

```sh
st-flash --connect-under-reset --reset write target/thumbv7m-none-eabi/debug/stm32-runner.bin 0x08000000
```

- ST-Link reset command:

```sh
st-flash --connect-under-reset --freq=200 reset
```

- Raw `/dev/ttyACM0` transcript retention at `docs/evidence/mvp-rc1/hardware_replay_transcript.txt`
- Byte-level payload verification recorded in `docs/evidence/mvp-rc1/hardware_replay_artifact.md`

## 5. Proof and Status References

The current authoritative requirement, design, traceability, and verification status references are:

- `docs/normative/HLR_mvp.md`
- `docs/design/LLR_mvp.md`
- `docs/normative/traceability_matrix.md`
- `docs/verification/SVCP_math.md`

This package references those documents only. It does not restate or expand proof claims.

## 6. Bounded MVP RC1 Claim

MVP RC1 readiness evidence exists for the current bounded repository surface.

Hardware evidence is one retained STM32F446 replay artifact for `math-add-001`.

Math and proof coverage remain bounded by the existing verification documents.

This package indexes evidence; it does not widen claims.

## 7. Deferred Items

Known deferred surfaces remain deferred:

- Full multiplication closure
- Full unbounded symbolic limb-matrix correspondence
- Private/helper-state limb combinations not reachable from public raw operands
- Full overflow-gate correspondence
- Complete overflow/trap proof coverage
- Generalized hardware validation
- Generated traceability
- Certification evidence package
- Tool qualification
- Board qualification

## 8. Explicit Non-Claims

- No DO-178C claim
- No DAL-A compliance claim
- No certification claim
- No tool qualification claim
- No board qualification claim
- No generalized hardware validation claim
- No release-authority claim
- No generalized replay protocol claim