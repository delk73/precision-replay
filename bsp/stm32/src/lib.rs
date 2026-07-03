#![no_std]

// Enforce that exactly one concrete target feature is selected
#[cfg(not(any(feature = "stm32f446", feature = "stm32h743")))]
compile_error!("CRITICAL COMPLIANCE ERROR: You must select exactly one specific hardware target feature (e.g., --features stm32f446). Family-wide wildcards are prohibited.");

#[cfg(feature = "stm32f446")]
pub use targets::stm32f446 as target;

#[cfg(feature = "stm32h743")]
pub use targets::stm32h743 as target;

pub mod targets {
    pub mod common {}
    pub mod stm32f446 {
        const RCC_BASE: usize = 0x4002_3800;
        const GPIOA_BASE: usize = 0x4002_0000;
        const ADC1_BASE: usize = 0x4001_2000;
        const USART2_BASE: usize = 0x4000_4400;

        const RCC_AHB1ENR: *mut u32 = (RCC_BASE + 0x30) as *mut u32;
        const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x40) as *mut u32;
        const RCC_APB2ENR: *mut u32 = (RCC_BASE + 0x44) as *mut u32;
        const GPIOA_MODER: *mut u32 = GPIOA_BASE as *mut u32;
        const GPIOA_AFRL: *mut u32 = (GPIOA_BASE + 0x20) as *mut u32;
        const ADC1_SR: *const u32 = ADC1_BASE as *const u32;
        const ADC1_CR2: *mut u32 = (ADC1_BASE + 0x08) as *mut u32;
        const ADC1_SMPR2: *mut u32 = (ADC1_BASE + 0x10) as *mut u32;
        const ADC1_SQR1: *mut u32 = (ADC1_BASE + 0x2C) as *mut u32;
        const ADC1_SQR3: *mut u32 = (ADC1_BASE + 0x34) as *mut u32;
        const ADC1_DR: *const u32 = (ADC1_BASE + 0x4C) as *const u32;
        const USART2_SR: *const u32 = USART2_BASE as *const u32;
        const USART2_DR: *mut u32 = (USART2_BASE + 0x04) as *mut u32;
        const USART2_BRR: *mut u32 = (USART2_BASE + 0x08) as *mut u32;
        const USART2_CR1: *mut u32 = (USART2_BASE + 0x0C) as *mut u32;

        const RCC_AHB1ENR_GPIOAEN: u32 = 1 << 0;
        const RCC_APB1ENR_USART2EN: u32 = 1 << 17;
        const RCC_APB2ENR_ADC1EN: u32 = 1 << 8;
        const GPIO_MODER_PA0_ANALOG: u32 = 0b11;
        const GPIO_MODER_PA0_MASK: u32 = 0b11;
        const GPIO_MODER_PA2_AF: u32 = 0b10 << 4;
        const GPIO_MODER_PA2_MASK: u32 = 0b11 << 4;
        const GPIO_AFRL_PA2_AF7: u32 = 0b0111 << 8;
        const GPIO_AFRL_PA2_MASK: u32 = 0b1111 << 8;
        const ADC_SR_EOC: u32 = 1 << 1;
        const ADC_CR2_ADON: u32 = 1 << 0;
        const ADC_CR2_EOCS: u32 = 1 << 10;
        const ADC_CR2_SWSTART: u32 = 1 << 30;
        const ADC_SMPR2_CHANNEL0_MASK: u32 = 0b111;
        const ADC_SMPR2_CHANNEL0_84_CYCLES: u32 = 0b100;
        const ADC_SQR1_SEQUENCE_LENGTH_MASK: u32 = 0b1111 << 20;
        const ADC_SQR3_FIRST_CONVERSION_MASK: u32 = 0b1_1111;
        const ADC_DR_12_BIT_MASK: u32 = 0x0FFF;
        const USART_SR_TXE: u32 = 1 << 7;
        const USART_CR1_TE: u32 = 1 << 3;
        const USART_CR1_UE: u32 = 1 << 13;
        const USART2_BRR_16MHZ_115200: u32 = 0x008B;

        pub fn init_stlink_vcp_usart2() {
            unsafe {
                let ahb1enr = core::ptr::read_volatile(RCC_AHB1ENR);
                core::ptr::write_volatile(RCC_AHB1ENR, ahb1enr | RCC_AHB1ENR_GPIOAEN);

                let apb1enr = core::ptr::read_volatile(RCC_APB1ENR);
                core::ptr::write_volatile(RCC_APB1ENR, apb1enr | RCC_APB1ENR_USART2EN);

                let moder = core::ptr::read_volatile(GPIOA_MODER);
                core::ptr::write_volatile(
                    GPIOA_MODER,
                    (moder & !GPIO_MODER_PA2_MASK) | GPIO_MODER_PA2_AF,
                );

                let afrl = core::ptr::read_volatile(GPIOA_AFRL);
                core::ptr::write_volatile(
                    GPIOA_AFRL,
                    (afrl & !GPIO_AFRL_PA2_MASK) | GPIO_AFRL_PA2_AF7,
                );

                core::ptr::write_volatile(USART2_BRR, USART2_BRR_16MHZ_115200);
                core::ptr::write_volatile(USART2_CR1, USART_CR1_TE | USART_CR1_UE);
            }
        }

        pub fn init_raw_adc1_pa0() {
            unsafe {
                let ahb1enr = core::ptr::read_volatile(RCC_AHB1ENR);
                core::ptr::write_volatile(RCC_AHB1ENR, ahb1enr | RCC_AHB1ENR_GPIOAEN);

                let apb2enr = core::ptr::read_volatile(RCC_APB2ENR);
                core::ptr::write_volatile(RCC_APB2ENR, apb2enr | RCC_APB2ENR_ADC1EN);

                let moder = core::ptr::read_volatile(GPIOA_MODER);
                core::ptr::write_volatile(
                    GPIOA_MODER,
                    (moder & !GPIO_MODER_PA0_MASK) | GPIO_MODER_PA0_ANALOG,
                );

                let smpr2 = core::ptr::read_volatile(ADC1_SMPR2);
                core::ptr::write_volatile(
                    ADC1_SMPR2,
                    (smpr2 & !ADC_SMPR2_CHANNEL0_MASK) | ADC_SMPR2_CHANNEL0_84_CYCLES,
                );

                let sqr1 = core::ptr::read_volatile(ADC1_SQR1);
                core::ptr::write_volatile(ADC1_SQR1, sqr1 & !ADC_SQR1_SEQUENCE_LENGTH_MASK);

                let sqr3 = core::ptr::read_volatile(ADC1_SQR3);
                core::ptr::write_volatile(ADC1_SQR3, sqr3 & !ADC_SQR3_FIRST_CONVERSION_MASK);

                core::ptr::write_volatile(ADC1_CR2, ADC_CR2_EOCS | ADC_CR2_ADON);

                for _ in 0..1024 {
                    core::hint::spin_loop();
                }
            }
        }

        pub fn read_raw_adc1_in0_blocking() -> u16 {
            unsafe {
                let cr2 = core::ptr::read_volatile(ADC1_CR2);
                core::ptr::write_volatile(ADC1_CR2, cr2 | ADC_CR2_SWSTART);

                while core::ptr::read_volatile(ADC1_SR) & ADC_SR_EOC == 0 {}

                (core::ptr::read_volatile(ADC1_DR) & ADC_DR_12_BIT_MASK) as u16
            }
        }

        pub fn write_stlink_vcp_usart2(bytes: &[u8]) {
            for byte in bytes {
                write_usart2_byte(*byte);
            }
        }

        fn write_usart2_byte(byte: u8) {
            unsafe {
                while core::ptr::read_volatile(USART2_SR) & USART_SR_TXE == 0 {}
                core::ptr::write_volatile(USART2_DR, u32::from(byte));
            }
        }
    }
    pub mod stm32h743 {}
}
