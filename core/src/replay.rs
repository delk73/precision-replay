use core::ops::{Add, Div, Mul, Sub};

use crate::math::I64F64;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReplayFrame {
    LoadOperands { lhs_bits: i128, rhs_bits: i128 },
    Add,
    Sub,
    Mul,
    Div,
    ExpectResultBits(i128),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReplayExecutionState {
    NoOperandsLoaded,
    OperandsLoaded,
    ResultProduced,
    Accepted,
    Rejected,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReplayRejectionReason {
    InvalidOrder,
    ExpectedResultMismatch {
        expected_bits: i128,
        actual_bits: i128,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ReplayExecutionResult {
    pub state: ReplayExecutionState,
    pub result_bits: Option<i128>,
    pub rejection_reason: Option<ReplayRejectionReason>,
}

impl ReplayExecutionResult {
    #[inline]
    const fn in_progress(state: ReplayExecutionState, result: Option<I64F64>) -> Self {
        Self {
            state,
            result_bits: result_bits(result),
            rejection_reason: None,
        }
    }

    #[inline]
    const fn rejected(reason: ReplayRejectionReason, result: Option<I64F64>) -> Self {
        Self {
            state: ReplayExecutionState::Rejected,
            result_bits: result_bits(result),
            rejection_reason: Some(reason),
        }
    }
}

#[inline]
const fn result_bits(result: Option<I64F64>) -> Option<i128> {
    match result {
        Some(value) => Some(value.to_bits()),
        None => None,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ExecutionPhase {
    NoOperandsLoaded,
    OperandsLoaded,
    ResultProduced,
    Accepted,
}

pub fn execute_replay(frames: &[ReplayFrame]) -> ReplayExecutionResult {
    let mut phase = ExecutionPhase::NoOperandsLoaded;
    let mut operands: Option<(I64F64, I64F64)> = None;
    let mut result: Option<I64F64> = None;

    for frame in frames {
        match *frame {
            ReplayFrame::LoadOperands { lhs_bits, rhs_bits } => {
                if phase != ExecutionPhase::NoOperandsLoaded {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                }
                operands = Some((I64F64::from_bits(lhs_bits), I64F64::from_bits(rhs_bits)));
                phase = ExecutionPhase::OperandsLoaded;
            }
            ReplayFrame::Add => {
                let Some((lhs, rhs)) = operands else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                };
                if phase != ExecutionPhase::OperandsLoaded {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                }
                result = Some(Add::add(lhs, rhs));
                phase = ExecutionPhase::ResultProduced;
            }
            ReplayFrame::Sub => {
                let Some((lhs, rhs)) = operands else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                };
                if phase != ExecutionPhase::OperandsLoaded {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                }
                result = Some(Sub::sub(lhs, rhs));
                phase = ExecutionPhase::ResultProduced;
            }
            ReplayFrame::Mul => {
                let Some((lhs, rhs)) = operands else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                };
                if phase != ExecutionPhase::OperandsLoaded {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                }
                result = Some(Mul::mul(lhs, rhs));
                phase = ExecutionPhase::ResultProduced;
            }
            ReplayFrame::Div => {
                let Some((lhs, rhs)) = operands else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                };
                if phase != ExecutionPhase::OperandsLoaded {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                }
                result = Some(Div::div(lhs, rhs));
                phase = ExecutionPhase::ResultProduced;
            }
            ReplayFrame::ExpectResultBits(expected_bits) => {
                if phase != ExecutionPhase::ResultProduced {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                }
                let Some(actual) = result else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::InvalidOrder,
                        result,
                    );
                };
                let actual_bits = actual.to_bits();
                if actual_bits != expected_bits {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::ExpectedResultMismatch {
                            expected_bits,
                            actual_bits,
                        },
                        result,
                    );
                }
                phase = ExecutionPhase::Accepted;
            }
        }
    }

    match phase {
        ExecutionPhase::NoOperandsLoaded => {
            ReplayExecutionResult::in_progress(ReplayExecutionState::NoOperandsLoaded, result)
        }
        ExecutionPhase::OperandsLoaded => {
            ReplayExecutionResult::in_progress(ReplayExecutionState::OperandsLoaded, result)
        }
        ExecutionPhase::ResultProduced => {
            ReplayExecutionResult::in_progress(ReplayExecutionState::ResultProduced, result)
        }
        ExecutionPhase::Accepted => {
            ReplayExecutionResult::in_progress(ReplayExecutionState::Accepted, result)
        }
    }
}
