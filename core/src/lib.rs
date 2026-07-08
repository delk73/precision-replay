#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::allow_attributes)]

//! Precision Replay Core
//!
//! Strict #![no_std] execution environment containing pure logic,
//! 128-bit fixed-point (I64F64) operations, and control flow invariants.

pub mod math;
pub mod replay;

#[cfg(test)]
mod replay_tests;

pub mod validation {
    // Porting target for control flow integrity and state invariants
}
