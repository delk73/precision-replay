# Low-Level Requirements - Sensor Witness (LLR-WITNESS)

## 1. Initial Raw Sensor Witness Lane (LLR-WITNESS)

### LLR-WITNESS-ADC-001: STM32F446 PA0 / ADC1_IN0 Raw ADC Acquisition
The initial raw ADC witness implementation shall target STM32F446 PA0 / ADC1_IN0 and acquire raw 12-bit ADC samples through the STM32F446 ADC path.
*Traces to: HLR-WITNESS-ADC-001*

### LLR-WITNESS-SERIAL-001: USART2 ST-LINK VCP Raw Witness Stream
The initial raw ADC witness implementation shall emit raw sample-indexed witness records over USART2 ST-LINK VCP.
*Traces to: HLR-WITNESS-SERIAL-001*

### LLR-WITNESS-TIME-001: Best-Effort Polling UART Timing Boundary
The initial raw ADC witness implementation shall identify `timing_claim=best_effort_polling_uart_stream` as the active timing claim for raw witness records.

This timing claim excludes fixed-rate sampling, precise event timing, transient fidelity, replay alignment, timer-paced ADC, DMA buffering, interrupt-driven capture, and final timing authority. `timer_paced_adc` is deferred as a possible future timing mode and is not an active initial raw ADC witness timing claim.
*Traces to: HLR-WITNESS-TIME-001*

### LLR-WITNESS-HOST-001: Linux Raw ADC Host Parser
The initial raw ADC witness implementation shall provide a Linux stdlib host parser for `witness=raw-adc` records that accepts exact raw ADC witness row shape, raw ADC range, and timing claim; ignores non-witness noise; counts malformed raw ADC witness rows; and emits deterministic summary text.
*Traces to: HLR-WITNESS-HOST-001*

### LLR-WITNESS-CAPTURE-001: Retained Raw ADC Capture Admission Checker
The retained raw ADC capture checker shall admit a retained raw ADC artifact only when required retained files exist, `capture.txt` matches the deterministic host parser summary for `raw_witness.txt`, retained raw ADC tokens use valid 4-digit raw ADC hex values within range, metadata preserves the retained artifact boundary, and non-contiguous sample indices are accompanied by a non-contiguous/non-lossless limitation.
*Traces to: HLR-WITNESS-CAPTURE-001*

### LLR-WITNESS-OBS-001: Admitted Raw ADC Observation Set
The retained raw ADC envelope checker shall treat only parsed raw ADC samples from an admitted retained raw ADC capture artifact as admitted observations. Malformed witness rows shall remain outside the admitted observation set.
*Traces to: HLR-WITNESS-OBS-001*

### LLR-WITNESS-CONTEXT-001: Retained Declared Context Metadata
The retained raw ADC envelope checker shall parse declared `context_id` from retained envelope metadata and return `not_applicable` when no context is declared. Declared context shall not generate, compare, verify, calibrate, synchronize, qualify, or prove sufficiency of external stimulus.
*Traces to: HLR-WITNESS-CONTEXT-001*

### LLR-WITNESS-ENV-001: Retained Raw ADC Envelope Judgment Checker
The retained raw ADC envelope checker shall judge admitted raw ADC observations using only `raw_adc_min`, `raw_adc_max`, `min_sample_count`, and `allow_malformed_witness_lines`, emit deterministic retained `judgment.txt` text, and restrict results to `pass`, `fail`, `inconclusive`, and `not_applicable`.
*Traces to: HLR-WITNESS-ENV-001*
