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
    pub mod stm32f446 {}
    pub mod stm32h743 {}
}
