# STM32 Flash and Capture Procedure - MVP RC1

## 1. Purpose

This procedure defines the narrow STM32F446 flash and serial-capture flow for the MVP RC1 hardware artifact lane.

This procedure is not itself the retained hardware artifact. The retained artifact and transcript are captured in a later commit.

## 2. Scope

This procedure applies only to the `stm32-runner` firmware package built for `thumbv7m-none-eabi` and the MVP RC1 serial result line emitted by that firmware.

It does not define a generalized replay protocol, broad board-support procedure, hardware qualification activity, tool qualification activity, release authority, or certification evidence.

## 3. Preconditions

- STM32F446 target connected through ST-Link.
- ST-Link probe visible to the host.
- Serial device visible as `/dev/ttyACM0`.
- Serial settings: `115200 8N1`.
- Local tools available: `cargo`, `llvm-objcopy`, `st-info`, `st-flash`, and a serial capture tool such as `picocom`.

Probe check:

```sh
st-info --probe
```

The probe output shall identify an STM32F446 target before flashing.

## 4. Build Firmware

Build the firmware ELF:

```sh
cargo build -p stm32-runner --no-default-features --target thumbv7m-none-eabi --locked
```

Expected ELF path:

```text
target/thumbv7m-none-eabi/debug/stm32-runner
```

## 5. Prepare Flash Image

`st-flash write` uses a binary image for this local flow. Convert the ELF to a raw binary image:

```sh
llvm-objcopy -O binary \
  target/thumbv7m-none-eabi/debug/stm32-runner \
  target/thumbv7m-none-eabi/debug/stm32-runner.bin
```

## 6. Flash STM32F446 Target

Flash the binary image at the STM32F446 flash origin:

```sh
st-flash write target/thumbv7m-none-eabi/debug/stm32-runner.bin 0x08000000
```

## 7. Capture `/dev/ttyACM0`

Open the ST-Link virtual COM port with `115200 8N1` settings and retain the emitted line for the next commit.

One local capture command is:

```sh
picocom --baud 115200 --databits 8 --parity n --stopbits 1 --imap crcrlf --logfile docs/evidence/mvp-rc1/hardware_replay_transcript.txt /dev/ttyACM0
```

Reset the STM32F446 target after opening the serial session if the line was emitted before capture started.

Stop capture after the expected line is present in the transcript.

## 8. Expected Result

Expected emitted serial line:

```text
precision-replay mvp-rc1 vector=math-add-001 result_bits=0x00000000000000020000000000000000
```

## 9. PASS/FAIL Rule

PASS only if the retained transcript contains exactly the expected MVP RC1 result line from `/dev/ttyACM0` after flashing the `stm32-runner` image built from the recorded baseline.

FAIL if the transcript is missing, the target identity is not STM32F446, the serial device is not `/dev/ttyACM0`, the emitted vector identifier differs, or the captured `result_bits` value differs from the expected line.

## 10. Retained Artifact Path For Next Commit

Commit 5 shall retain the hardware artifact and transcript at:

- `docs/evidence/mvp-rc1/hardware_replay_artifact.md`
- `docs/evidence/mvp-rc1/hardware_replay_transcript.txt`

## 11. Explicit Non-Claims

- This is procedure only.
- No hardware artifact is retained by this commit.
- This is not a broad hardware validation claim.
- This is not board qualification evidence.
- This is not tool qualification evidence.
- This is not certification evidence.
- This is not a generalized replay protocol.