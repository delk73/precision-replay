# Low-Level Requirements - Sensor Witness (LLR-WITNESS)

## 1. Initial Raw Sensor Witness Lane (LLR-WITNESS)

### LLR-WITNESS-ADC: STM32F446 PA0 / ADC1_IN0 Raw Witness Input
The initial raw ADC witness implementation shall target STM32F446 PA0 / ADC1_IN0 as the raw analog witness input.
*Traces to: HLR-WITNESS-ADC*

### LLR-WITNESS-UART: USART2 ST-LINK VCP Raw Witness Stream
The initial raw ADC witness implementation shall emit raw witness records over USART2 ST-LINK VCP.
*Traces to: HLR-WITNESS-ADC*

### LLR-WITNESS-TIME: Best-Effort Polling UART Timing Boundary
The initial raw ADC witness implementation shall identify `timing_claim=best_effort_polling_uart_stream` as the active timing claim for raw witness records.

This timing claim excludes fixed-rate sampling, precise event timing, transient fidelity, replay alignment, timer-paced ADC, DMA buffering, interrupt-driven capture, and final timing authority. `timer_paced_adc` is deferred as a possible future timing mode and is not an active initial raw ADC witness timing claim.
*Traces to: HLR-WITNESS-TIME*

### LLR-WITNESS-HOST: Linux Raw Witness Capture Tool
The initial raw ADC witness implementation shall provide a Linux stdlib host parser/capture tool for raw witness records.
*Traces to: HLR-WITNESS-HOST*

### LLR-WITNESS-STIM: Deferred Declared External Stimulus
Declared external stimulus is deferred.
*Traces to: HLR-WITNESS-STIM*

### LLR-WITNESS-ENV: Deferred Bounded Response-Envelope Evaluation
Bounded response-envelope evaluation is deferred.
*Traces to: HLR-WITNESS-ENV*
