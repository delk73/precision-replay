use crate::math::I64F64;
use crate::replay::{
    execute_replay, parse_replay_input, ReplayExecutionState, ReplayFrame, ReplayInputSchema,
    ReplayInputVersion, ReplayParseError, ReplayRejectionReason,
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

fn assert_arithmetic_trap_rejection(frames: &[ReplayFrame]) {
    let result = execute_replay(frames);

    assert_eq!(result.state, ReplayExecutionState::Rejected);
    assert_eq!(result.result_bits, None);
    assert_eq!(
        result.rejection_reason,
        Some(ReplayRejectionReason::ArithmeticTrap)
    );
}

#[test]
fn addition_overflow_replay_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: i128::MAX,
            rhs_bits: 1,
        },
        ReplayFrame::Add,
    ];

    assert_arithmetic_trap_rejection(&frames);
}

#[test]
fn subtraction_overflow_replay_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: i128::MIN,
            rhs_bits: 1,
        },
        ReplayFrame::Sub,
    ];

    assert_arithmetic_trap_rejection(&frames);
}

#[test]
fn multiplication_trap_replay_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: i128::MAX,
            rhs_bits: i128::MAX,
        },
        ReplayFrame::Mul,
    ];

    assert_arithmetic_trap_rejection(&frames);
}

#[test]
fn division_by_zero_replay_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: I64F64::SCALE,
            rhs_bits: 0,
        },
        ReplayFrame::Div,
    ];

    assert_arithmetic_trap_rejection(&frames);
}

#[test]
fn division_numerator_shift_overflow_replay_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: i128::MAX - 1,
            rhs_bits: I64F64::SCALE,
        },
        ReplayFrame::Div,
    ];

    assert_arithmetic_trap_rejection(&frames);
}

#[test]
fn integer_division_overflow_replay_rejects() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: -(1i128 << 63),
            rhs_bits: -1,
        },
        ReplayFrame::Div,
    ];

    assert_arithmetic_trap_rejection(&frames);
}

#[test]
fn arithmetic_trap_rejection_is_repeatable() {
    let frames = [
        ReplayFrame::LoadOperands {
            lhs_bits: i128::MAX,
            rhs_bits: 1,
        },
        ReplayFrame::Add,
    ];

    let first = execute_replay(&frames);
    let second = execute_replay(&frames);

    assert_eq!(first, second);
    assert_eq!(first.state, ReplayExecutionState::Rejected);
    assert_eq!(first.result_bits, None);
    assert_eq!(
        first.rejection_reason,
        Some(ReplayRejectionReason::ArithmeticTrap)
    );
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

fn parse_input(input: &str) -> Result<([ReplayFrame; 4], usize), ReplayParseError> {
    let mut out = [ReplayFrame::Add; 4];
    let len = {
        let parsed = parse_replay_input(input, &mut out)?;

        assert_eq!(parsed.version, ReplayInputVersion::V1);
        assert_eq!(parsed.schema, ReplayInputSchema::MathI64F64V1);

        parsed.frames.len()
    };

    Ok((out, len))
}

#[test]
fn valid_saved_add_input_parses_frames_without_executing() {
    let input =
        "precision-replay-input v1\nschema math-i64f64-v1\nload lhs=1 rhs=2\nadd\nexpect bits=99\n";
    let (out, len) = parse_input(input).unwrap();

    assert_eq!(len, 3);
    assert_eq!(
        &out[..len],
        &[
            ReplayFrame::LoadOperands {
                lhs_bits: 1,
                rhs_bits: 2,
            },
            ReplayFrame::Add,
            ReplayFrame::ExpectResultBits(99),
        ]
    );
}

#[test]
fn valid_saved_sub_input_parses_frames() {
    let input =
        "precision-replay-input v1\nschema math-i64f64-v1\nload lhs=5 rhs=3\nsub\nexpect bits=2\n";
    let (out, len) = parse_input(input).unwrap();

    assert_eq!(
        &out[..len],
        &[
            ReplayFrame::LoadOperands {
                lhs_bits: 5,
                rhs_bits: 3,
            },
            ReplayFrame::Sub,
            ReplayFrame::ExpectResultBits(2),
        ]
    );
}

#[test]
fn valid_saved_mul_input_parses_frames() {
    let input =
        "precision-replay-input v1\nschema math-i64f64-v1\nload lhs=6 rhs=7\nmul\nexpect bits=42\n";
    let (out, len) = parse_input(input).unwrap();

    assert_eq!(
        &out[..len],
        &[
            ReplayFrame::LoadOperands {
                lhs_bits: 6,
                rhs_bits: 7,
            },
            ReplayFrame::Mul,
            ReplayFrame::ExpectResultBits(42),
        ]
    );
}

#[test]
fn valid_saved_nonzero_div_input_parses_frames() {
    let input =
        "precision-replay-input v1\nschema math-i64f64-v1\nload lhs=8 rhs=2\ndiv\nexpect bits=4\n";
    let (out, len) = parse_input(input).unwrap();

    assert_eq!(
        &out[..len],
        &[
            ReplayFrame::LoadOperands {
                lhs_bits: 8,
                rhs_bits: 2,
            },
            ReplayFrame::Div,
            ReplayFrame::ExpectResultBits(4),
        ]
    );
}

#[test]
fn saved_input_unknown_version_rejects() {
    let mut out = [ReplayFrame::Add; 4];
    let input = "precision-replay-input v2\nschema math-i64f64-v1\nadd\n";

    let err = parse_replay_input(input, &mut out).unwrap_err();

    assert_eq!(err, ReplayParseError::UnknownVersion);
}

#[test]
fn saved_input_unknown_schema_rejects() {
    let mut out = [ReplayFrame::Add; 4];
    let input = "precision-replay-input v1\nschema other-lane-v1\nadd\n";

    let err = parse_replay_input(input, &mut out).unwrap_err();

    assert_eq!(err, ReplayParseError::UnknownSchema);
}

#[test]
fn saved_input_unknown_opcode_rejects() {
    let mut out = [ReplayFrame::Add; 4];
    let input = "precision-replay-input v1\nschema math-i64f64-v1\nneg\n";

    let err = parse_replay_input(input, &mut out).unwrap_err();

    assert_eq!(err, ReplayParseError::UnknownFrameOpcode);
}

#[test]
fn saved_input_malformed_frame_row_rejects() {
    let mut out = [ReplayFrame::Add; 4];
    let input = "precision-replay-input v1\nschema math-i64f64-v1\nadd extra\n";

    let err = parse_replay_input(input, &mut out).unwrap_err();

    assert_eq!(err, ReplayParseError::MalformedFrame);
}

#[test]
fn saved_input_missing_operand_field_rejects() {
    let mut out = [ReplayFrame::Add; 4];
    let input = "precision-replay-input v1\nschema math-i64f64-v1\nload lhs=1\n";

    let err = parse_replay_input(input, &mut out).unwrap_err();

    assert_eq!(err, ReplayParseError::MissingField);
}

#[test]
fn saved_input_invalid_integer_field_rejects() {
    let mut out = [ReplayFrame::Add; 4];
    let input = "precision-replay-input v1\nschema math-i64f64-v1\nexpect bits=abc\n";

    let err = parse_replay_input(input, &mut out).unwrap_err();

    assert_eq!(err, ReplayParseError::InvalidInteger);
}

#[test]
fn saved_input_frame_capacity_exceeded_rejects() {
    let mut out = [ReplayFrame::Add; 2];
    let input =
        "precision-replay-input v1\nschema math-i64f64-v1\nload lhs=1 rhs=2\nadd\nexpect bits=3\n";

    let err = parse_replay_input(input, &mut out).unwrap_err();

    assert_eq!(err, ReplayParseError::FrameCapacityExceeded);
}
