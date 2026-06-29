# Reproducing v0.1.0-rc1 Evidence Checks

## Purpose

This note describes the reproduction boundary for evidence package `precision-replay-v0.1.0-rc1`.

It does not widen the package claim beyond the manifest: one retained STM32F446 hardware-backed replay observation is recorded for this release candidate.

## Local Checks Without Hardware

A reviewer can inspect this package locally without STM32 hardware by checking:

- `manifest.toml` parses as TOML.
- The files listed in `manifest.toml` are present in this directory.
- `hardware_replay_artifact.md` and `hardware_replay_transcript.txt` describe the same retained payload.
- The retained transcript contains the expected `v0.1.0-rc1` `math-add-001` result line referenced by `hardware_replay_artifact.md`.
- Normative proof and traceability status are referenced through the existing repository documents, not restated here.

These local checks inspect retained evidence. They do not reproduce the hardware observation.

## Package Checks Planned Later

Later package-hardening work is expected to add machine-checkable package checks, including hashes and a checker surface.

Those checks are not part of this step. This document should not be read as defining checker-only policy or a complete verification gate.

## Retained Hardware Evidence

The retained hardware evidence is the existing STM32F446 observation recorded in:

- `hardware_replay_artifact.md`
- `hardware_replay_transcript.txt`

That evidence is retained as an artifact. It is not automatically reproduced by local commands that parse the manifest or inspect the transcript.

## Hardware Recapture Boundary

Recapturing the observation requires STM32 hardware and the capture setup described by:

- `stm32_flash_capture_procedure.md`
- `HARDWARE_SETUP.md`

A recapture is a new hardware activity. It requires an STM32F446 target, flashed `stm32-runner` firmware, ST-Link access, serial capture from `/dev/ttyACM0`, and comparison to the expected `v0.1.0-rc1` payload.

No certification, tool qualification, hardware qualification, timing behavior, board-family validation, or full arithmetic proof-closure claim is made by this package.
