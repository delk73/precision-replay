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
- Local tools available: `cargo`, `llvm-objcopy`, `st-info`, `st-flash`, `stty`, `timeout`, `cat`, and `tee`.

Probe check:

```sh
st-info --probe
```

The probe output should identify an STM32F446 target before ordinary flashing. If probe or ordinary flash fails with SWD attach/connect errors, the reset-under-flash path may be used. In that case, the `st-flash --connect-under-reset --reset write ...` output must identify STM32F446 before the flash result is accepted.

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

If `st-info --probe` or ordinary `st-flash write ...` fails with SWD attach or connect errors, use reset-under-flash with the same repo-local `stm32-runner` binary image:

```sh
st-flash --connect-under-reset --reset write target/thumbv7m-none-eabi/debug/stm32-runner.bin 0x08000000
```

Do not use the historical `precision-signal` binary path:

```text
target/thumbv7em-none-eabihf/debug/replay-fw-f446.bin
```

Successful target identification or flash does not itself create the retained hardware artifact. UART capture begins only after firmware flash succeeds.

## 7. Capture `/dev/ttyACM0`

Configure the ST-Link virtual COM port for `115200 8N1` and retain the emitted line for the later artifact-retention commit.

One local capture flow is:

```sh
stty -F /dev/ttyACM0 115200 cs8 -cstopb -parenb raw -echo
timeout 10 cat /dev/ttyACM0 | tee docs/evidence/mvp-rc1/hardware_replay_transcript.txt
```

Start the capture command before resetting the STM32F446 target.

Reset the STM32F446 target after the capture command is running if the line was emitted before capture started.

Stop capture after the expected line is present in the transcript.

The retained transcript must contain target output captured from `/dev/ttyACM0`; do not manually synthesize or edit the expected result into the transcript.

## 8. Expected Result

Expected emitted serial line:

```text
precision-replay mvp-rc1 vector=math-add-001 result_bits=0x00000000000000020000000000000000
```

## 9. PASS/FAIL Rule

PASS only if the retained transcript contains exactly the expected MVP RC1 result line from `/dev/ttyACM0` after flashing the `stm32-runner` image built from the recorded baseline.

FAIL if the transcript is missing, the target identity is not STM32F446, the serial device is not `/dev/ttyACM0`, the emitted vector identifier differs, or the captured `result_bits` value differs from the expected line.

## 10. Retained Artifact Path For Next Commit

The later artifact-retention commit shall retain the hardware artifact and transcript at:

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