#!/usr/bin/env python3
"""Check that the base CI workflow still matches its contract."""

from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_WORKFLOW = REPO_ROOT / ".github" / "workflows" / "ci.yml"

WORKFLOW_NAME = "CI"
REQUIRED_TRIGGER_BRANCHES = {"pull_request": ["main"], "push": ["main"]}
REQUIRED_PERMISSIONS = {"contents": "read"}
JOB_NAME = "validate"
RUNNER = "ubuntu-24.04"
REQUIRED_COMMANDS = [
    "rustup show",
    "cargo fmt --all -- --check",
    "cargo check --workspace --locked",
    "cargo test --workspace --locked",
    "cargo clippy --workspace --locked -- -D warnings",
    "make replay-check",
    "cargo check -p precision-replay-core --no-default-features --target thumbv7m-none-eabi --locked",
    "cargo check -p bsp-stm32 --no-default-features --features stm32f446 --target thumbv7m-none-eabi --locked",
    "cargo check -p bsp-pru --no-default-features --target thumbv7m-none-eabi --locked",
    "cargo check -p stm32-runner --no-default-features --target thumbv7m-none-eabi --locked",
    "cargo check -p pru-runner --no-default-features --target thumbv7m-none-eabi --locked",
]


class ContractError(Exception):
    """Raised when the CI workflow is missing, malformed, or weakened."""


@dataclass(frozen=True)
class WorkflowContract:
    name: str | None
    triggers: dict[str, list[str]]
    permissions: dict[str, str]
    validate_job_present: bool
    runner: str | None
    run_commands: list[str]


def _clean_line(raw_line: str) -> str:
    if "\t" in raw_line[: len(raw_line) - len(raw_line.lstrip())]:
        raise ContractError("workflow uses tab indentation")
    return raw_line.split("#", 1)[0].rstrip()


def _indent(line: str) -> int:
    return len(line) - len(line.lstrip(" "))


def _scalar_after(line: str, prefix: str) -> str | None:
    stripped = line.strip()
    if not stripped.startswith(prefix):
        return None
    return stripped[len(prefix) :].strip()


def _parse_workflow(path: Path) -> WorkflowContract:
    if not path.exists():
        raise ContractError(f"workflow file is missing: {path}")

    lines = [_clean_line(line) for line in path.read_text(encoding="utf-8").splitlines()]
    lines = [line for line in lines if line.strip()]

    name: str | None = None
    triggers: dict[str, list[str]] = {}
    permissions: dict[str, str] = {}
    validate_job_present = False
    runner: str | None = None
    run_commands: list[str] = []

    section: str | None = None
    trigger: str | None = None
    in_validate_job = False
    in_steps = False

    for line in lines:
        indent = _indent(line)
        stripped = line.strip()

        if indent == 0:
            section = stripped[:-1] if stripped.endswith(":") else None
            trigger = None
            in_validate_job = False
            in_steps = False

            value = _scalar_after(line, "name:")
            if value is not None:
                name = value
            continue

        if section == "on":
            if indent == 2 and stripped.endswith(":"):
                trigger = stripped[:-1]
                triggers.setdefault(trigger, [])
                continue
            if indent == 6 and stripped.startswith("- ") and trigger is not None:
                triggers[trigger].append(stripped[2:].strip())
                continue

        if section == "permissions":
            if indent == 2 and ":" in stripped:
                key, value = stripped.split(":", 1)
                permissions[key.strip()] = value.strip()
                continue

        if section == "jobs":
            if indent == 2:
                in_validate_job = stripped == f"{JOB_NAME}:"
                validate_job_present = validate_job_present or in_validate_job
                in_steps = False
                continue
            if in_validate_job and indent == 4:
                value = _scalar_after(line, "runs-on:")
                if value is not None:
                    runner = value
                    continue
                if stripped == "steps:":
                    in_steps = True
                    continue
            if in_validate_job and in_steps and indent == 8:
                value = _scalar_after(line, "run:")
                if value is not None:
                    run_commands.append(value)
                    continue

    return WorkflowContract(
        name=name,
        triggers=triggers,
        permissions=permissions,
        validate_job_present=validate_job_present,
        runner=runner,
        run_commands=run_commands,
    )


def check_workflow(path: Path = DEFAULT_WORKFLOW) -> None:
    workflow = _parse_workflow(path)

    if workflow.name != WORKFLOW_NAME:
        raise ContractError(f"workflow name must be {WORKFLOW_NAME!r}")

    for trigger, expected_branches in REQUIRED_TRIGGER_BRANCHES.items():
        branches = workflow.triggers.get(trigger)
        if branches is None:
            raise ContractError(f"missing {trigger} trigger")
        if branches != expected_branches:
            raise ContractError(f"{trigger} branches must be exactly {expected_branches!r}")

    if workflow.permissions != REQUIRED_PERMISSIONS:
        raise ContractError(f"workflow permissions must be exactly {REQUIRED_PERMISSIONS!r}")

    if not workflow.validate_job_present:
        raise ContractError(f"missing {JOB_NAME!r} job")

    if workflow.runner != RUNNER:
        raise ContractError(f"{JOB_NAME} job must run on {RUNNER!r}")

    for command in REQUIRED_COMMANDS:
        if command not in workflow.run_commands:
            raise ContractError(f"missing required CI command: {command}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "workflow",
        nargs="?",
        type=Path,
        default=DEFAULT_WORKFLOW,
        help="Path to the CI workflow to check",
    )
    args = parser.parse_args()

    try:
        check_workflow(args.workflow)
    except ContractError as exc:
        print(f"CI contract check failed: {exc}", file=sys.stderr)
        return 1

    print("CI contract check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
