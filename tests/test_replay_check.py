from __future__ import annotations

import subprocess
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
REPLAY_CHECK = [
    "cargo",
    "run",
    "--quiet",
    "--offline",
    "-p",
    "precision-replay-core",
    "--example",
    "replay_check",
    "--",
]


def run_replay_check(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [*REPLAY_CHECK, *args],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )


def write_replay_input(tmp_path: Path, text: str) -> Path:
    replay_input = tmp_path / "input.txt"
    replay_input.write_text(text, encoding="utf-8")
    return replay_input


def replay_input(*frames: str) -> str:
    return "\n".join(
        [
            "precision-replay-input v1",
            "schema math-i64f64-v1",
            *frames,
            "",
        ]
    )


def test_successful_replay_witness_output(tmp_path: Path) -> None:
    input_path = write_replay_input(
        tmp_path,
        replay_input(
            "load lhs=55340232221128654848 rhs=18446744073709551616",
            "sub",
            "expect bits=36893488147419103232",
        ),
    )

    result = run_replay_check(str(input_path))

    assert result.returncode == 0
    assert result.stdout == (
        "precision-replay witness=replay-input-v1 schema=math-i64f64-v1 "
        "state=accepted result_bits=36893488147419103232\n"
    )
    assert result.stderr == ""


def test_missing_input_argument_fails() -> None:
    result = run_replay_check()

    assert result.returncode == 2
    assert result.stdout == ""
    assert result.stderr == "expected exactly one replay input path\n"


def test_extra_argument_fails(tmp_path: Path) -> None:
    input_path = write_replay_input(tmp_path, replay_input("add"))

    result = run_replay_check(str(input_path), "extra")

    assert result.returncode == 2
    assert result.stdout == ""
    assert result.stderr == "expected exactly one replay input path\n"


def test_input_read_failure_is_stable(tmp_path: Path) -> None:
    result = run_replay_check(str(tmp_path / "missing.txt"))

    assert result.returncode == 3
    assert result.stdout == ""
    assert result.stderr == "input read failed\n"


def test_parse_rejection_reports_stable_identifier(tmp_path: Path) -> None:
    input_path = write_replay_input(tmp_path, replay_input("unknown"))

    result = run_replay_check(str(input_path))

    assert result.returncode == 10
    assert result.stdout == ""
    assert result.stderr == "parse failed: unknown_frame_opcode\n"


def test_invalid_order_rejection_reports_stable_identifier(tmp_path: Path) -> None:
    input_path = write_replay_input(tmp_path, replay_input("add"))

    result = run_replay_check(str(input_path))

    assert result.returncode == 20
    assert result.stdout == ""
    assert result.stderr == "replay rejected: invalid_order\n"


def test_incomplete_replay_reports_stable_identifier(tmp_path: Path) -> None:
    input_path = write_replay_input(tmp_path, replay_input())

    result = run_replay_check(str(input_path))

    assert result.returncode == 20
    assert result.stdout == ""
    assert result.stderr == "replay rejected: incomplete_replay\n"


def test_arithmetic_trap_rejection_reports_stable_identifier(tmp_path: Path) -> None:
    input_path = write_replay_input(
        tmp_path,
        replay_input(
            "load lhs=170141183460469231731687303715884105727 rhs=1",
            "add",
        ),
    )

    result = run_replay_check(str(input_path))

    assert result.returncode == 20
    assert result.stdout == ""
    assert result.stderr == "replay rejected: arithmetic_trap\n"


def test_expected_result_mismatch_reports_deterministic_values(tmp_path: Path) -> None:
    input_path = write_replay_input(
        tmp_path,
        replay_input(
            "load lhs=1 rhs=2",
            "add",
            "expect bits=4",
        ),
    )

    result = run_replay_check(str(input_path))

    assert result.returncode == 20
    assert result.stdout == ""
    assert result.stderr == (
        "replay rejected: expected_result_mismatch expected_bits=4 actual_bits=3\n"
    )
