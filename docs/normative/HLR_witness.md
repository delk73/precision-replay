# High-Level Requirements - Sensor Witness (HLR-WITNESS)

## 1. Raw STM32 ADC Witness Lane (HLR-WITNESS)

### HLR-WITNESS-ADC: Raw Analog Witness Input Lane
The system shall provide one raw STM32 ADC witness input lane for a physical analog signal.

### HLR-WITNESS-TIME: Explicit Witness Timing Claim
The witness lane shall identify its timing claim explicitly. The initial raw ADC witness implementation shall use `timing_claim=best_effort_polling_uart_stream`.

This timing claim excludes fixed-rate sampling, precise event timing, transient fidelity, replay alignment, timer-paced ADC, DMA buffering, interrupt-driven capture, and final timing authority. A future timing mode such as `timer_paced_adc` may be declared later, but it is not part of the active initial raw ADC witness claim.

### HLR-WITNESS-HOST: Raw Sample-Indexed Host Capture
The witness lane shall support host capture of raw sample-indexed witness records.

### HLR-WITNESS-OBS: Raw ADC Capture as Observation
Raw ADC witness rows shall be treated first as capture records. A raw ADC witness row may count as an observation only when it is accepted by host checking as part of an admitted raw ADC capture. Such an observation identifies the witness channel, sample index, raw ADC value, and explicit timing claim.

Observation status shall not imply calibrated voltage, magnetic-field units, stimulus response, response-envelope pass/fail, signal quality, hardware qualification, release readiness, or certification posture.

### HLR-WITNESS-STIM: Declared Observation Context Boundary
The witness lane shall allow an admitted observation to belong to a declared context. The declared context may name an external stimulus, but the repository shall not generate, verify, calibrate, synchronize, qualify, or prove sufficiency of that stimulus as part of the initial raw ADC witness implementation.

Declared context shall not imply stimulus quality, calibrated stimulus, stimulus timing, stimulus sufficiency, replay alignment, response-envelope pass/fail, or stronger timing than the observation's explicit timing claim. Timing remains governed by HLR-WITNESS-TIME.

### HLR-WITNESS-ENV: Deferred Response-Envelope Boundary
The witness lane shall allow later bounded response-envelope requirements without making envelope evaluation part of the initial raw ADC witness implementation.
