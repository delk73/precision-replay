# Raw ADC Witness Capture Metadata

## Capture Identity

- Repository: `delk73/precision-replay`
- Branch: `evidence/add-raw-adc-envelope-judgment`
- Capture ID: `20260704T211403Z`
- Capture date/time: `2026-07-04T21:14:03Z`
- Operator note: Bounded raw witness rows retained through the checked raw ADC capture workflow; `malformed_witness_lines=0` was observed in the retained slice and preserved in `capture.txt`.

## Capture Setup

- Raw UART capture source: `/dev/ttyACM0`
- Baud: `115200`
- Raw witness retention command: bounded line-aligned UART read from `/dev/ttyACM0` into `raw_witness.txt`
- Host monitor summary command: `python3 tools/raw_adc_monitor.py - < raw_witness.txt > capture.txt`
- Target board: STM32F446 target board observed through ST-LINK VCP
- ADC path: STM32F446 PA0 / ADC1_IN0
- Witness format: `precision-replay v0.1.0-rc1 witness=raw-adc sample_index=<n> raw_adc=0x<hhhh> timing_claim=best_effort_polling_uart_stream`
- Timing claim: `best_effort_polling_uart_stream`

## Retained Files

- `raw_witness.txt`: bounded retained raw `witness=raw-adc` rows from the STM32 serial stream.
- `capture.txt`: deterministic host monitor summary generated from `raw_witness.txt`.
- `metadata.md`: retained artifact boundary, limitations, and declared raw ADC envelope metadata.
- `judgment.txt`: deterministic retained raw ADC envelope judgment output.

## Envelope Judgment Metadata

- context_id: `bench-context-raw-adc-retained-20260704T211403Z`
- envelope_id: `raw-adc-envelope-retained-20260704T211403Z`
- raw_adc_min: `0x0830`
- raw_adc_max: `0x0870`
- min_sample_count: `128`
- allow_malformed_witness_lines: `false`

## Boundary

This retained artifact claims only bounded raw ADC witness capture, host monitor parsing, deterministic summary, and raw ADC envelope judgment.

## Known Limitations

This artifact does not claim calibrated voltage, calibrated magnetic-field units, fixed-rate sampling, timer-paced capture, timing authority, hardware qualification, tool qualification, certification readiness, hardware correctness, signal quality, replay alignment, digest sealing, baseline-vs-stimulus comparison, trend judgment, delta judgment, context comparison, stimulus generation, stimulus adequacy, or generalized board support.

The retained summary reports `malformed_witness_lines=0`, and the retained sample indices are contiguous from `626883` through `627138`.
