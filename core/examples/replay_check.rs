use precision_replay_core::replay::{
    execute_replay, parse_replay_input, ReplayExecutionState, ReplayFrame, ReplayInputSchema,
};
use std::{env, fs, process};

const FRAME_CAPACITY: usize = 64;

fn main() {
    let Some(input_path) = env::args().nth(1) else {
        eprintln!("missing retained input path");
        process::exit(2);
    };

    let input = match fs::read_to_string(&input_path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("input read failed: {error}");
            process::exit(3);
        }
    };

    let mut frames = [ReplayFrame::Add; FRAME_CAPACITY];
    let parsed = match parse_replay_input(&input, &mut frames) {
        Ok(parsed) => parsed,
        Err(error) => {
            eprintln!("parse failed: {error:?}");
            process::exit(10);
        }
    };

    let schema = match parsed.schema {
        ReplayInputSchema::MathI64F64V1 => "math-i64f64-v1",
    };
    let result = execute_replay(parsed.frames);
    if result.state != ReplayExecutionState::Accepted {
        eprintln!("replay rejected: {:?}", result.rejection_reason);
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
