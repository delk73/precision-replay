# Hardware Setup Context for v0.1.0-rc1

## Purpose

This note records the hardware context needed to understand the retained `v0.1.0-rc1` observation.

It is context for review only. It is not hardware qualification evidence.

## Target Class

The retained observation is for an STM32F446 target class. The retained artifact records target identification details observed during the flash and reset flow.

This package does not claim board-family validation or generalized hardware validation.

## Flashed Runner and Firmware Relationship

The hardware observation is tied to the repo-local `stm32-runner` firmware built for the `thumbv7m-none-eabi` target.

At a high level, the runner emits the `v0.1.0-rc1` replay result payload over the target serial path after the firmware image is flashed and the target is reset.

The retained observation concerns that flashed runner payload only. It does not qualify the firmware toolchain, debugger, board, or broader BSP surface.

## Serial Transcript Role

`hardware_replay_transcript.txt` is the retained raw serial transcript for the observation.

`hardware_replay_artifact.md` records the expected payload, transcript metadata, and PASS verdict for that retained transcript.

The transcript is retained evidence of one observed serial emission. It is not a generated proof artifact and is not a substitute for arithmetic proof closure.

## Capture Boundary

The capture boundary is the serial output observed from the STM32F446 target through `/dev/ttyACM0` after flashing and reset, as described by `stm32_flash_capture_procedure.md` and recorded by `hardware_replay_artifact.md`.

The retained artifact does not claim:

- certification compliance
- tool qualification
- hardware qualification
- timing behavior
- board-family validation
- full arithmetic proof closure
