# Hardware Replay Artifact - v0.1.0-rc1 STM32F446

## 1. Purpose

This artifact records one retained STM32F446 hardware-backed replay validation capture for `v0.1.0-rc1` readiness inspection.

## 2. Scope

This artifact covers one flashed `stm32-runner` firmware image, one STM32F446 target, one `/dev/ttyACM0` raw serial transcript, and one replay vector payload comparison for package `precision-replay-v0.1.0-rc1`.

## 3. Baseline

- Release version: `v0.1.0`
- Release candidate: `v0.1.0-rc1`
- Evidence package ID: `precision-replay-v0.1.0-rc1`
- Repository: `delk73/precision-replay`
- Branch: `release/finalize-mvp-readiness`
- Baseline SHA under test: `39273774c9a0ba0d251f274be2b3564147001150`
- Capture date: `2026-06-27`

## 4. Firmware Context

- Firmware package: `stm32-runner`
- Target triple: `thumbv7m-none-eabi`
- Target identity: STM32F446, ST-Link V2J33S25, flash `524288`, SRAM `131072`, chipid `0x421`

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

Reset output identified STM32F446 with 128 KiB SRAM and 512 KiB flash. The requested 200 kHz speed was mapped to 240 kHz by `st-flash`.

## 9. Serial Capture Context

- Serial device: `/dev/ttyACM0`
- Serial settings: `115200 8N1`
- Transcript path: `docs/evidence/v0.1.0-rc1/hardware_replay_transcript.txt`

Capture commands:

```sh
stty -F /dev/ttyACM0 115200 cs8 -cstopb -parenb -ixon -ixoff -crtscts raw -echo
timeout 30 cat /dev/ttyACM0 | tee docs/evidence/v0.1.0-rc1/hardware_replay_transcript.txt &
CAP_PID=$!

sleep 1

st-flash --connect-under-reset --freq=200 reset

wait "$CAP_PID" || true
```

For the retained transcript, serial capture was started before target reset so the boot-time payload emitted after reset was captured without trimming or normalization.

Result: PASS

The retained raw transcript was captured manually after the firmware payload label changed to `v0.1.0-rc1`.

## 10. Replay Vector

- Replay vector: `math-add-001`
- Expected payload: `precision-replay v0.1.0-rc1 vector=math-add-001 result_bits=0x00000000000000020000000000000000`

## 11. Transcript Metadata

- Transcript byte count: `99`
- Transcript SHA-256: `02467511fe2b7956fcb6efb66483bbb5b418e78e954fab395eddefefab98bd56`
- Byte-level payload occurrence count: `1`
- Line-level match count: `1`
- Hex prefix: `fe0d0a707265636973696f6e2d7265706c61792076302e312e302d7263312076`

The retained raw transcript includes leading reset/framing byte context before the expected payload. That byte context is retained as captured and was not normalized, rewritten, trimmed, or cleaned.

## 12. Verdict

PASS. The retained raw transcript contains one byte-level occurrence of the exact expected payload, and one `data.splitlines()` entry matches the expected payload exactly.

## 13. Explicit Non-Claims

- This is one retained STM32F446 hardware-backed replay artifact only.
- This is not generalized hardware validation.
- This is not board qualification evidence.
- This is not tool qualification evidence.
- This is not certification evidence.
- This is not a DAL-A compliance claim.
- This is not release authority.
- This is not a generalized replay protocol.
- This is not proof expansion.
