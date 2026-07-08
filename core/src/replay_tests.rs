use crate::math::I64F64;
use crate::replay::{
    execute_replay, ReplayExecutionState, ReplayFrame, ReplayRejectionReason,
};

#[test]
fn valid_math_add_replay_accepts() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE,
            rhs_bits: I64F64::SCALE,
        },
        ReplayFrame::Add,
        ReplayFrame::ExpectResultBits(I64F64::SCALE.checked_mul(2).unwrap()),
    ];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::Accepted);
    assert_eq!(
        result.result_bits,
        Some(I64F64::SCALE.checked_mul(2).unwrap())
    );
    assert_eq!(result.rejection_reason, None);
}

#[test]
fn valid_math_sub_replay_accepts() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE.checked_mul(3).unwrap(),
            rhs_bits: I64F64::SCALE,
        },
        ReplayFrame::Sub,
        ReplayFrame::ExpectResultBits(I64F64::SCALE.checked_mul(2).unwrap()),
    ];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::Accepted);
    assert_eq!(
        result.result_bits,
        Some(I64F64::SCALE.checked_mul(2).unwrap())
    );
    assert_eq!(result.rejection_reason, None);
}

#[test]
fn valid_math_mul_replay_accepts() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE.checked_mul(3).unwrap(),
            rhs_bits: I64F64::SCALE.checked_mul(2).unwrap(),
        },
        ReplayFrame::Mul,
        ReplayFrame::ExpectResultBits(I64F64::SCALE.checked_mul(6).unwrap()),
    ];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::Accepted);
    assert_eq!(
        result.result_bits,
        Some(I64F64::SCALE.checked_mul(6).unwrap())
    );
    assert_eq!(result.rejection_reason, None);
}

#[test]
fn valid_nonzero_math_div_replay_accepts() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE.checked_div(4).unwrap(),
            rhs_bits: I64F64::SCALE.checked_div(2).unwrap(),
        },
        ReplayFrame::Div,
        ReplayFrame::ExpectResultBits(I64F64::SCALE.checked_div(2).unwrap()),
    ];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::Accepted);
    assert_eq!(
        result.result_bits,
        Some(I64F64::SCALE.checked_div(2).unwrap())
    );
    assert_eq!(result.rejection_reason, None);
}

#[test]
fn math_op_before_operands_rejects() {
    let frames = [ReplayFrame::Add];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::Rejected);
    assert_eq!(result.result_bits, None);
    assert_eq!(
        result.rejection_reason,
        Some(ReplayRejectionReason::InvalidOrder)
    );
}

#[test]
fn expect_result_before_execution_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE,
            rhs_bits: I64F64::SCALE,
        },
        ReplayFrame::ExpectResultBits(I64F64::SCALE),
    ];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::Rejected);
    assert_eq!(result.result_bits, None);
    assert_eq!(
        result.rejection_reason,
        Some(ReplayRejectionReason::InvalidOrder)
    );
}

#[test]
fn expected_result_mismatch_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE,
            rhs_bits: I64F64::SCALE,
        },
        ReplayFrame::Add,
        ReplayFrame::ExpectResultBits(I64F64::SCALE),
    ];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::Rejected);
    assert_eq!(
        result.result_bits,
        Some(I64F64::SCALE.checked_mul(2).unwrap())
    );
    assert_eq!(
        result.rejection_reason,
        Some(ReplayRejectionReason::ExpectedResultMismatch {
            expected_bits: I64F64::SCALE,
            actual_bits: I64F64::SCALE.checked_mul(2).unwrap(),
        })
    );
}

#[test]
fn frame_after_acceptance_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE,
            rhs_bits: I64F64::SCALE,
        },
        ReplayFrame::Add,
        ReplayFrame::ExpectResultBits(I64F64::SCALE.checked_mul(2).unwrap()),
        ReplayFrame::Sub,
    ];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::Rejected);
    assert_eq!(
        result.result_bits,
        Some(I64F64::SCALE.checked_mul(2).unwrap())
    );
    assert_eq!(
        result.rejection_reason,
        Some(ReplayRejectionReason::InvalidOrder)
    );
}

#[test]
fn empty_frame_slice_reports_no_operands_loaded() {
    let frames = [];

    let result = execute_replay(&frames);

    assert_eq!(result.state, ReplayExecutionState::NoOperandsLoaded);
    assert_eq!(result.result_bits, None);
    assert_eq!(result.rejection_reason, None);
}

#[test]
fn same_frames_produce_same_result() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE.checked_mul(3).unwrap(),
            rhs_bits: I64F64::SCALE,
        },
        ReplayFrame::Sub,
        ReplayFrame::ExpectResultBits(I64F64::SCALE.checked_mul(2).unwrap()),
    ];

    let first = execute_replay(&frames);
    let second = execute_replay(&frames);

    assert_eq!(first, second);
}
