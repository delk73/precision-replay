use precision_replay_core::replay::{
    execute_replay, parse_replay_input, ReplayExecutionState, ReplayFrame, ReplayInputSchema,
    ReplayParseError, ReplayRejectionReason,
};
use std::{env, fs, process};

const FRAME_CAPACITY: usize = 64;

fn main() {
    let mut args = env::args().skip(1);
    let Some(input_path) = args.next() else {
        eprintln!("expected exactly one replay input path");
        process::exit(2);
    };
    if args.next().is_some() {
        eprintln!("expected exactly one replay input path");
        process::exit(2);
    };

    let input = match fs::read_to_string(&input_path) {
        Ok(input) => input,
        Err(_) => {
            eprintln!("input read failed");
            process::exit(3);
        }
    };

    let mut frames = [ReplayFrame::Add; FRAME_CAPACITY];
    let parsed = match parse_replay_input(&input, &mut frames) {
        Ok(parsed) => parsed,
        Err(error) => {
            eprintln!("parse failed: {}", parse_error_identifier(error));
            process::exit(10);
        }
    };

    let schema = match parsed.schema {
        ReplayInputSchema::MathI64F64V1 => "math-i64f64-v1",
    };
    let result = execute_replay(parsed.frames);
    if result.state != ReplayExecutionState::Accepted {
        eprintln!("{}", replay_rejection_diagnostic(result.rejection_reason));
        process::exit(20);
    }

    let Some(result_bits) = result.result_bits else {
        eprintln!("replay accepted without result bits");
        process::exit(21);
    };

    println!(
        "precision-replay witness=replay-input-v1 schema={schema} state=accepted result_bits={result_bits}"
    );
}

fn parse_error_identifier(error: ReplayParseError) -> &'static str {
    match error {
        ReplayParseError::MissingVersion => "missing_version",
        ReplayParseError::UnknownVersion => "unknown_version",
        ReplayParseError::MissingSchema => "missing_schema",
        ReplayParseError::UnknownSchema => "unknown_schema",
        ReplayParseError::UnknownFrameOpcode => "unknown_frame_opcode",
        ReplayParseError::MalformedFrame => "malformed_frame",
        ReplayParseError::MissingField => "missing_field",
        ReplayParseError::InvalidInteger => "invalid_integer",
        ReplayParseError::FrameCapacityExceeded => "frame_capacity_exceeded",
    }
}

fn replay_rejection_diagnostic(reason: Option<ReplayRejectionReason>) -> String {
    match reason {
        None => "replay rejected: incomplete_replay".to_string(),
        Some(ReplayRejectionReason::InvalidOrder) => "replay rejected: invalid_order".to_string(),
        Some(ReplayRejectionReason::ArithmeticTrap) => {
            "replay rejected: arithmetic_trap".to_string()
        }
        Some(ReplayRejectionReason::ExpectedResultMismatch {
            expected_bits,
            actual_bits,
        }) => {
            format!(
                "replay rejected: expected_result_mismatch expected_bits={expected_bits} actual_bits={actual_bits}"
            )
        }
    }
}
