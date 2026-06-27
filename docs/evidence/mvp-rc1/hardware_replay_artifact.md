# Hardware Replay Artifact - MVP RC1 STM32F446

## 1. Purpose

This artifact records one retained STM32F446 hardware-backed replay validation capture for MVP RC1 readiness inspection.

## 2. Scope

This artifact covers one flashed `stm32-runner` firmware image, one STM32F446 target, one `/dev/ttyACM0` raw serial transcript, and one replay vector payload comparison.

## 3. Baseline

- Baseline commit SHA before artifact capture: `ad7de9112e2fa6668f30880da2b32f47a1da5251`
- Capture timestamp: `2026-06-26T23:59:36Z`

## 4. Firmware Context

- Firmware package: `stm32-runner`
- Target triple: `thumbv7m-none-eabi`

## 5. Build Result

Command:

```sh
cargo build -p stm32-runner --no-default-features --target thumbv7m-none-eabi --locked
```

Result: PASS

## 6. ELF-to-BIN Conversion Result

Command:

```sh
llvm-objcopy -O binary target/thumbv7m-none-eabi/debug/stm32-runner target/thumbv7m-none-eabi/debug/stm32-runner.bin
```

Result: PASS

## 7. Flash Result

Command:

```sh
st-flash --connect-under-reset --reset write target/thumbv7m-none-eabi/debug/stm32-runner.bin 0x08000000
```

Result: PASS

Flash output identified STM32F446 with 128 KiB SRAM and 512 KiB flash, wrote 4296 bytes, and reported flash written and verified.

## 8. Reset Result

Command:

```sh
st-flash --connect-under-reset --freq=200 reset
```

Result: PASS

Reset output identified STM32F446 with 128 KiB SRAM and 512 KiB flash.

## 9. Serial Capture Context

- Serial device: `/dev/ttyACM0`
- Serial settings: `115200 8N1`
- Transcript path: `docs/evidence/mvp-rc1/hardware_replay_transcript.txt`

Capture commands:

```sh
stty -F /dev/ttyACM0 115200 cs8 -cstopb -parenb -ixon -ixoff -crtscts raw -echo
timeout 30 cat /dev/ttyACM0 | tee docs/evidence/mvp-rc1/hardware_replay_transcript.txt
```

Result: PASS

The capture command ended by timeout after retaining target output.

## 10. Replay Vector

- Replay vector: `math-add-001`
- Expected payload: `precision-replay mvp-rc1 vector=math-add-001 result_bits=0x00000000000000020000000000000000`

## 11. Transcript Metadata

- Transcript byte count: `96`
- Transcript SHA-256: `a2ee8d71ff72bfdb674ce6aedfac494aa21ded82be4becf5337a5aa22cd9dce2`
- Byte-level payload occurrence count: `1`
- Line-level match count: `1`

The retained raw transcript includes reset/framing byte context before the expected payload. That byte context is retained as captured and was not normalized or edited.

## 12. Verdict

PASS. The retained raw transcript contains one byte-level occurrence of the exact expected payload, and one `data.splitlines()` entry matches the expected payload exactly.

## 13. Explicit Non-Claims

- This is one retained STM32F446 hardware-backed replay artifact only.
- This is not generalized hardware validation.
- This is not board qualification evidence.
- This is not tool qualification evidence.
- This is not a certification claim.
- This is not a DAL-A compliance claim.
- This is not a release-authority claim.
- This is not a generalized replay protocol.
- This is not proof expansion.
