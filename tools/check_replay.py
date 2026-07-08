#!/usr/bin/env python3
"""Check the retained public replay artifact against generated replay output."""

from __future__ import annotations

import argparse
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

REQUIRED_FILES = ("input.txt", "expected_witness.txt", "expected_result.txt")
DEFAULT_ARTIFACT_DIR = Path("artifacts/replay/math-i64f64-v1")


@dataclass(frozen=True)
class GeneratedReplay:
    witness: str


class ReplayCheckError(ValueError):
    """Raised when the retained replay artifact cannot be checked."""


def check_replay(artifact_dir: Path, repo_root: Path) -> tuple[str, ...]:
    missing = [name for name in REQUIRED_FILES if not (artifact_dir / name).is_file()]
    if missing:
        raise ReplayCheckError(f"missing required file: {missing[0]}")

    expected_witness = (artifact_dir / "expected_witness.txt").read_text(encoding="utf-8")
    expected_result = (artifact_dir / "expected_result.txt").read_text(encoding="utf-8")

    try:
        generated = _run_core_replay(repo_root, artifact_dir / "input.txt")
    except ReplayCheckError as exc:
        raise ReplayCheckError(f"parse/replay stage failed: {exc}") from exc

    if generated.witness != expected_witness:
        raise ReplayCheckError(
            "witness mismatch:\n"
            f"expected:\n{expected_witness}"
            f"actual:\n{generated.witness}"
        )

    generated_result = "parse=pass\nreplay=pass\nwitness=pass\nresult=pass\n"
    if generated_result != expected_result:
        raise ReplayCheckError(
            "result mismatch:\n"
            f"expected:\n{expected_result}"
            f"actual:\n{generated_result}"
        )

    return tuple(generated_result.splitlines())


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "artifact_dir",
        nargs="?",
        type=Path,
        default=DEFAULT_ARTIFACT_DIR,
        help="Directory containing retained replay input and expected outputs",
    )
    args = parser.parse_args(argv)

    repo_root = Path(__file__).resolve().parents[1]
    artifact_dir = args.artifact_dir
    if not artifact_dir.is_absolute():
        artifact_dir = repo_root / artifact_dir

    try:
        pass_lines = check_replay(artifact_dir, repo_root)
    except (OSError, UnicodeDecodeError, ReplayCheckError) as exc:
        print(f"replay check failed: {exc}", file=sys.stderr)
        return 1

    for line in pass_lines:
        print(line)
    return 0


def _run_core_replay(repo_root: Path, input_path: Path) -> GeneratedReplay:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "--quiet",
            "--offline",
            "-p",
            "precision-replay-core",
            "--example",
            "replay_check",
            "--",
            str(input_path),
        ],
        cwd=repo_root,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    if completed.returncode != 0:
        detail = completed.stderr.strip() or completed.stdout.strip() or f"exit {completed.returncode}"
        raise ReplayCheckError(detail)

    return GeneratedReplay(witness=_ensure_trailing_newline(completed.stdout))


def _ensure_trailing_newline(text: str) -> str:
    if text.endswith("\n"):
        return text
    return text + "\n"


if __name__ == "__main__":
    raise SystemExit(main())
