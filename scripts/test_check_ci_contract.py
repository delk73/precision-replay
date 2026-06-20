from __future__ import annotations

import shutil
import sys
from pathlib import Path

import pytest


sys.path.insert(0, str(Path(__file__).resolve().parent))

from check_ci_contract import ContractError, REQUIRED_COMMANDS, check_workflow


REPO_ROOT = Path(__file__).resolve().parents[1]
CURRENT_WORKFLOW = REPO_ROOT / ".github" / "workflows" / "ci.yml"


def copy_current_workflow(tmp_path: Path) -> Path:
    workflow = tmp_path / "ci.yml"
    shutil.copyfile(CURRENT_WORKFLOW, workflow)
    return workflow


def replace_text(path: Path, old: str, new: str) -> None:
    contents = path.read_text(encoding="utf-8")
    assert old in contents
    path.write_text(contents.replace(old, new), encoding="utf-8")


def remove_block(path: Path, block: str) -> None:
    replace_text(path, block, "")


def assert_contract_fails(path: Path, expected: str) -> None:
    with pytest.raises(ContractError, match=expected):
        check_workflow(path)


def test_current_workflow_passes() -> None:
    check_workflow(CURRENT_WORKFLOW)


def test_missing_workflow_file_fails(tmp_path: Path) -> None:
    assert_contract_fails(tmp_path / "missing.yml", "workflow file is missing")


def test_wrong_workflow_name_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    replace_text(workflow, "name: CI", "name: Not CI")

    assert_contract_fails(workflow, "workflow name")


def test_missing_pull_request_trigger_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    remove_block(
        workflow,
        "  pull_request:\n"
        "    branches:\n"
        "      - main\n",
    )

    assert_contract_fails(workflow, "missing pull_request trigger")


def test_missing_push_trigger_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    remove_block(
        workflow,
        "  push:\n"
        "    branches:\n"
        "      - main\n",
    )

    assert_contract_fails(workflow, "missing push trigger")


def test_wrong_branch_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    replace_text(workflow, "      - main\n", "      - develop\n")

    assert_contract_fails(workflow, "branches must be exactly")


def test_missing_contents_read_permission_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    remove_block(workflow, "permissions:\n  contents: read\n")

    assert_contract_fails(workflow, "workflow permissions")


def test_missing_validate_job_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    replace_text(workflow, "  validate:\n", "  verify:\n")

    assert_contract_fails(workflow, "missing 'validate' job")


def test_wrong_runner_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    replace_text(workflow, "runs-on: ubuntu-24.04", "runs-on: ubuntu-latest")

    assert_contract_fails(workflow, "ubuntu-24.04")


def test_missing_required_command_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    remove_block(
        workflow,
        "\n"
        "      - name: Check formatting\n"
        "        run: cargo fmt --all -- --check\n",
    )

    assert_contract_fails(workflow, "cargo fmt --all -- --check")


@pytest.mark.parametrize(
    "command",
    [command for command in REQUIRED_COMMANDS if command.startswith("cargo ") and " --locked" in command],
)
def test_missing_locked_where_required_fails(tmp_path: Path, command: str) -> None:
    workflow = copy_current_workflow(tmp_path)
    replace_text(workflow, command, command.replace(" --locked", ""))

    assert_contract_fails(workflow, command)


def test_missing_thumbv7m_target_check_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    command = "cargo check -p precision-replay-core --no-default-features --target thumbv7m-none-eabi --locked"
    replace_text(workflow, command, command.replace(" --target thumbv7m-none-eabi", ""))

    assert_contract_fails(workflow, command)


def test_missing_stm32f446_feature_check_fails(tmp_path: Path) -> None:
    workflow = copy_current_workflow(tmp_path)
    command = "cargo check -p bsp-stm32 --no-default-features --features stm32f446 --target thumbv7m-none-eabi --locked"
    replace_text(workflow, command, command.replace(" --features stm32f446", ""))

    assert_contract_fails(workflow, command)
