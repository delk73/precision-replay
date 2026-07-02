# Low-Level Requirements - Target Witness I/O (LLR-TARGET-IO)

## 1. STM32F446 Witness Output Path (LLR-TARGET-IO)

### LLR-TARGET-IO-001: ST-LINK VCP USART2 TX Configuration
The STM32F446 BSP shall configure the ST-LINK VCP USART2 TX path needed to emit replay witness bytes from the target.
*Traces to: HLR-TARGET-IO-001*

### LLR-RUNNER-WITNESS-001: Retained Replay Payload Emission Ordering
The STM32 runner shall initialize the target witness output path before emitting the replay result payload.
*Traces to: HLR-TARGET-IO-001*
