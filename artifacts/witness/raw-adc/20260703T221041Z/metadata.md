# Raw ADC Witness Capture Metadata

## Capture Identity

- Repository: `delk73/precision-replay`
- Branch: `evidence/raw-adc-retained-capture`
- HEAD SHA used for capture: `675ac68979c32f5e9e6f7dd37011627cc540985a`
- Capture date/time: `2026-07-03T22:10:41Z`
- Operator note: Bounded raw witness rows retained; `malformed_witness_lines=1` was observed in the retained slice and preserved in `capture.txt`.

## Capture Setup

- Host monitor command: `python3 tools/raw_adc_monitor.py --serial /dev/ttyACM0 --baud 115200`
- Serial device: `/dev/ttyACM0`
- Baud: `115200`
- Target board: STM32F446 target board observed through ST-LINK VCP
- ADC path: STM32F446 PA0 / ADC1_IN0
- Witness format: `precision-replay v0.1.0-rc1 witness=raw-adc sample_index=<n> raw_adc=0x<hhhh> timing_claim=best_effort_polling_uart_stream`
- Timing claim: `best_effort_polling_uart_stream`

## Retained Files

- `raw_witness.txt`: bounded retained raw `witness=raw-adc` rows from the STM32 serial stream.
- `capture.txt`: deterministic host monitor summary generated from `raw_witness.txt`.

## Boundary

This retained artifact claims only bounded raw ADC witness capture, host monitor parsing and deterministic summary, and observed STM32 serial smoke capture.

## Known Limitations

This artifact does not claim calibrated voltage, calibrated magnetic-field units, fixed-rate sampling, timer-paced capture, timing authority, hardware qualification, tool qualification, certification readiness, hardware correctness, signal quality, replay alignment, digest sealing, stimulus/envelope behavior, or generalized board support.

Sample indices in the retained summary are not claimed contiguous. Sample index span exceeds retained sample count; this artifact does not claim lossless serial capture.
