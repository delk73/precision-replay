# High-Level Requirements - Sensor Witness (HLR-WITNESS)

## 1. Raw STM32 ADC Witness Lane (HLR-WITNESS)

### HLR-WITNESS-ADC-001: Raw ADC Acquisition
The system shall provide one raw STM32 ADC witness acquisition lane for a physical analog signal.

### HLR-WITNESS-SERIAL-001: Raw ADC Serial Emission
The witness lane shall emit raw sample-indexed witness records over an explicit serial witness stream.

### HLR-WITNESS-TIME-001: Explicit Witness Timing Claim
The witness lane shall identify its timing claim explicitly. The initial raw ADC witness implementation shall use `timing_claim=best_effort_polling_uart_stream`.

This timing claim excludes fixed-rate sampling, precise event timing, transient fidelity, replay alignment, timer-paced ADC, DMA buffering, interrupt-driven capture, and final timing authority. A future timing mode such as `timer_paced_adc` may be declared later, but it is not part of the active initial raw ADC witness claim.

### HLR-WITNESS-HOST-001: Raw ADC Host Parsing
The witness lane shall support host parsing of raw sample-indexed witness records, including deterministic summary of accepted raw ADC rows and malformed raw ADC witness rows.

### HLR-WITNESS-CAPTURE-001: Retained Raw ADC Capture Admission
A retained raw ADC capture artifact shall be admitted only when host checking accepts the required retained files, canonical deterministic capture summary, raw ADC token shape and range, retained artifact boundary metadata, and any documented non-contiguous/non-lossless limitation required by the retained sample indices.

### HLR-WITNESS-OBS-001: Raw ADC Observation Eligibility
Raw ADC witness rows shall be treated first as capture records. A raw ADC witness row may count as an observation only when it is accepted by host checking as part of an admitted raw ADC capture. Such an observation identifies the witness channel, sample index, raw ADC value, and explicit timing claim.

Observation status shall not imply calibrated voltage, magnetic-field units, stimulus response, response-envelope pass/fail, signal quality, hardware qualification, release readiness, or certification posture.

### HLR-WITNESS-CONTEXT-001: Declared Observation Context
The witness lane shall allow admitted raw ADC observations to be associated with a declared `context_id` in retained envelope metadata. Declared context shall not require or imply repository generation, verification, calibration, synchronization, qualification, or sufficiency proof for any external stimulus.

Declared context shall not imply stimulus quality, calibrated stimulus, stimulus timing, stimulus sufficiency, replay alignment, response-envelope pass/fail, or stronger timing than the observation's explicit timing claim. Timing remains governed by `HLR-WITNESS-TIME-001`.

### HLR-WITNESS-ENV-001: Raw ADC Envelope Judgment
The witness lane shall allow admitted raw ADC observations in a declared context to be judged against a raw ADC envelope. Raw ADC envelope judgment applies only to admitted observations in a declared context.

The first raw ADC envelope judgment definition shall use only `raw_adc_min`, `raw_adc_max`, `min_sample_count`, and `allow_malformed_witness_lines`. The allowed judgment results shall be `pass`, `fail`, `inconclusive`, and `not_applicable`.

`allow_malformed_witness_lines` declares whether malformed witness rows are tolerated by the judgment. When `false`, malformed witness rows shall prevent `pass`. When `true`, malformed witness rows remain outside the admitted observation set and may be tolerated only under the existing best-effort/non-lossless capture boundary. This field shall not make malformed rows valid observations and shall not claim UART losslessness, timing proof, signal quality, stimulus adequacy, or hardware correctness.

A `pass` result shall be allowed only when the declared context supplies applicable raw ADC limits and sample-count requirements, the admitted observations meet the required sample count, and every judged admitted observation is within the declared raw ADC minimum and maximum limits. Missing context, missing applicable raw ADC limits, malformed rows, or too few admitted samples shall not produce `pass`.

Raw ADC envelope judgment shall not imply calibrated measurement, timing proof, signal quality, stimulus adequacy, hardware qualification, release readiness, or certification posture.
