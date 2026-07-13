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
pub enum ReplayInputVersion {
    V1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReplayInputSchema {
    MathI64F64V1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ParsedReplayInput<'a> {
    pub version: ReplayInputVersion,
    pub schema: ReplayInputSchema,
    pub frames: &'a [ReplayFrame],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReplayParseError {
    MissingVersion,
    UnknownVersion,
    MissingSchema,
    UnknownSchema,
    UnknownFrameOpcode,
    MalformedFrame,
    MissingField,
    InvalidInteger,
    FrameCapacityExceeded,
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
    ArithmeticTrap,
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

pub fn parse_replay_input<'a>(
    input: &str,
    out: &'a mut [ReplayFrame],
) -> Result<ParsedReplayInput<'a>, ReplayParseError> {
    let mut lines = input.lines();

    let version = parse_version_line(lines.next())?;
    let schema = parse_schema_line(lines.next())?;
    let mut frame_count = 0usize;

    for line in lines {
        if frame_count == out.len() {
            return Err(ReplayParseError::FrameCapacityExceeded);
        }

        out[frame_count] = parse_frame_line(line)?;
        frame_count = frame_count
            .checked_add(1)
            .expect("frame count increments only after capacity check");
    }

    Ok(ParsedReplayInput {
        version,
        schema,
        frames: &out[..frame_count],
    })
}

fn parse_version_line(line: Option<&str>) -> Result<ReplayInputVersion, ReplayParseError> {
    let Some(line) = line else {
        return Err(ReplayParseError::MissingVersion);
    };

    let mut fields = line.split(' ');
    match (fields.next(), fields.next(), fields.next()) {
        (Some("precision-replay-input"), Some("v1"), None) => Ok(ReplayInputVersion::V1),
        (Some("precision-replay-input"), Some(_), None) => Err(ReplayParseError::UnknownVersion),
        _ => Err(ReplayParseError::MissingVersion),
    }
}

fn parse_schema_line(line: Option<&str>) -> Result<ReplayInputSchema, ReplayParseError> {
    let Some(line) = line else {
        return Err(ReplayParseError::MissingSchema);
    };

    let mut fields = line.split(' ');
    match (fields.next(), fields.next(), fields.next()) {
        (Some("schema"), Some("math-i64f64-v1"), None) => Ok(ReplayInputSchema::MathI64F64V1),
        (Some("schema"), Some(_), None) => Err(ReplayParseError::UnknownSchema),
        _ => Err(ReplayParseError::MissingSchema),
    }
}

fn parse_frame_line(line: &str) -> Result<ReplayFrame, ReplayParseError> {
    let mut fields = line.split(' ');
    let Some(opcode) = fields.next() else {
        return Err(ReplayParseError::MalformedFrame);
    };

    match opcode {
        "load" => parse_load_frame(fields),
        "add" => no_more_fields(fields).map(|()| ReplayFrame::Add),
        "sub" => no_more_fields(fields).map(|()| ReplayFrame::Sub),
        "mul" => no_more_fields(fields).map(|()| ReplayFrame::Mul),
        "div" => no_more_fields(fields).map(|()| ReplayFrame::Div),
        "expect" => parse_expect_frame(fields),
        "" => Err(ReplayParseError::MalformedFrame),
        _ => Err(ReplayParseError::UnknownFrameOpcode),
    }
}

fn parse_load_frame<'a>(
    mut fields: impl Iterator<Item = &'a str>,
) -> Result<ReplayFrame, ReplayParseError> {
    let lhs_bits = parse_required_i128_field(fields.next(), "lhs")?;
    let rhs_bits = parse_required_i128_field(fields.next(), "rhs")?;
    no_more_fields(fields)?;

    Ok(ReplayFrame::LoadOperands { lhs_bits, rhs_bits })
}

fn parse_expect_frame<'a>(
    mut fields: impl Iterator<Item = &'a str>,
) -> Result<ReplayFrame, ReplayParseError> {
    let bits = parse_required_i128_field(fields.next(), "bits")?;
    no_more_fields(fields)?;

    Ok(ReplayFrame::ExpectResultBits(bits))
}

fn parse_required_i128_field(field: Option<&str>, name: &str) -> Result<i128, ReplayParseError> {
    let Some(field) = field else {
        return Err(ReplayParseError::MissingField);
    };
    let Some(value) = field.strip_prefix(name) else {
        return Err(ReplayParseError::MissingField);
    };
    let Some(value) = value.strip_prefix('=') else {
        return Err(ReplayParseError::MissingField);
    };
    if value.is_empty() {
        return Err(ReplayParseError::InvalidInteger);
    }

    value
        .parse::<i128>()
        .map_err(|_| ReplayParseError::InvalidInteger)
}

fn no_more_fields<'a>(mut fields: impl Iterator<Item = &'a str>) -> Result<(), ReplayParseError> {
    if fields.next().is_some() {
        return Err(ReplayParseError::MalformedFrame);
    }

    Ok(())
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
                let Some(next_result) = lhs.checked_add(rhs) else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::ArithmeticTrap,
                        result,
                    );
                };
                result = Some(next_result);
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
                let Some(next_result) = lhs.checked_sub(rhs) else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::ArithmeticTrap,
                        result,
                    );
                };
                result = Some(next_result);
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
                let Some(next_result) = lhs.checked_mul(rhs) else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::ArithmeticTrap,
                        result,
                    );
                };
                result = Some(next_result);
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
                let Some(next_result) = lhs.checked_div(rhs) else {
                    return ReplayExecutionResult::rejected(
                        ReplayRejectionReason::ArithmeticTrap,
                        result,
                    );
                };
                result = Some(next_result);
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
