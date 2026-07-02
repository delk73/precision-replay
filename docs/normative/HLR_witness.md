# High-Level Requirements - Sensor Witness (HLR-WITNESS)

## 1. Raw STM32 ADC Witness Lane (HLR-WITNESS)

### HLR-WITNESS-ADC: Raw Analog Witness Input Lane
The system shall provide one raw STM32 ADC witness input lane for a physical analog signal.

### HLR-WITNESS-TIME: Explicit Witness Timing Claim
The witness lane shall identify its timing claim explicitly. The initial raw ADC witness implementation shall use `timing_claim=best_effort_polling_uart_stream`.

This timing claim excludes fixed-rate sampling, precise event timing, transient fidelity, replay alignment, timer-paced ADC, DMA buffering, interrupt-driven capture, and final timing authority. A future timing mode such as `timer_paced_adc` may be declared later, but it is not part of the active initial raw ADC witness claim.

### HLR-WITNESS-HOST: Raw Sample-Indexed Host Capture
The witness lane shall support host capture of raw sample-indexed witness records.

### HLR-WITNESS-STIM: Deferred External Stimulus Boundary
The witness lane shall allow later declared external stimulus requirements without making stimulus part of the initial raw ADC witness implementation.

### HLR-WITNESS-ENV: Deferred Response-Envelope Boundary
The witness lane shall allow later bounded response-envelope requirements without making envelope evaluation part of the initial raw ADC witness implementation.
