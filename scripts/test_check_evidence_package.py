from __future__ import annotations

import hashlib
import shutil
import sys
from pathlib import Path

import pytest


sys.path.insert(0, str(Path(__file__).resolve().parent))

from check_evidence_package import EvidencePackageError, check_evidence_package


REPO_ROOT = Path(__file__).resolve().parents[1]
CURRENT_PACKAGE = REPO_ROOT / "docs" / "evidence" / "v0.1.0-rc1"


def copy_current_package(tmp_path: Path) -> Path:
    package = tmp_path / "v0.1.0-rc1"
    shutil.copytree(CURRENT_PACKAGE, package)
    return package


def replace_text(path: Path, old: str, new: str) -> None:
    contents = path.read_text(encoding="utf-8")
    assert old in contents
    path.write_text(contents.replace(old, new), encoding="utf-8")


def rewrite_sums(package: Path) -> None:
    files = [
        "evidence_package.md",
        "hardware_replay_artifact.md",
        "hardware_replay_transcript.txt",
        "stm32_flash_capture_procedure.md",
        "manifest.toml",
        "REPRODUCING.md",
        "HARDWARE_SETUP.md",
        "REVIEW_PACKET.md",
    ]
    lines = []
    for filename in files:
        digest = hashlib.sha256((package / filename).read_bytes()).hexdigest()
        lines.append(f"{digest}  {filename}\n")
    (package / "SHA256SUMS").write_text("".join(lines), encoding="utf-8")


def assert_package_fails(package: Path, expected: str) -> None:
    with pytest.raises(EvidencePackageError, match=expected):
        check_evidence_package(package)


def test_current_package_passes() -> None:
    check_evidence_package(CURRENT_PACKAGE)


def test_missing_package_directory_fails(tmp_path: Path) -> None:
    assert_package_fails(tmp_path / "missing", "evidence package directory is missing")


def test_missing_manifest_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    (package / "manifest.toml").unlink()

    assert_package_fails(package, "manifest.toml is missing")


def test_malformed_manifest_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    (package / "manifest.toml").write_text("[package\nid = 'bad'\n", encoding="utf-8")

    assert_package_fails(package, "does not parse as TOML")


def test_wrong_package_id_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    replace_text(package / "manifest.toml", "precision-replay-v0.1.0-rc1", "precision-replay-v0.1.0-rc2")
    rewrite_sums(package)

    assert_package_fails(package, "manifest field 'id'")


def test_missing_readiness_checkpoint_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    replace_text(
        package / "manifest.toml",
        'rc_readiness_checkpoint = "106e1b01ab5209cf74629ab6fb4fbb993278b2dd"\n',
        "",
    )
    rewrite_sums(package)

    assert_package_fails(package, "rc_readiness_checkpoint")


def test_missing_retained_file_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    (package / "hardware_replay_artifact.md").unlink()

    assert_package_fails(package, "retained evidence file is missing")


def test_unbounded_supported_claim_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    replace_text(
        package / "manifest.toml",
        "one retained STM32F446 hardware-backed replay observation is recorded for this release candidate",
        "generalized hardware validation is complete",
    )
    rewrite_sums(package)

    assert_package_fails(package, "supported claim")


def test_missing_excluded_claim_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    replace_text(package / "manifest.toml", '  "timing behavior",\n', "")
    rewrite_sums(package)

    assert_package_fails(package, "required excluded claims")


def test_missing_sha256sums_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    (package / "SHA256SUMS").unlink()

    assert_package_fails(package, "SHA256SUMS is missing")


def test_checksum_mismatch_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    with (package / "REVIEW_PACKET.md").open("a", encoding="utf-8") as packet:
        packet.write("\naccidental drift\n")

    assert_package_fails(package, "checksum mismatch")


def test_absolute_checksum_entry_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    replace_text(package / "SHA256SUMS", "manifest.toml", str(package / "manifest.toml"))

    assert_package_fails(package, "checksum filename must be relative")


def test_out_of_package_checksum_entry_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    replace_text(package / "SHA256SUMS", "manifest.toml", "../manifest.toml")

    assert_package_fails(package, "escapes package")


def test_unexpected_checksum_entry_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    extra = package / "extra.md"
    extra.write_text("not part of the package contract\n", encoding="utf-8")
    digest = hashlib.sha256(extra.read_bytes()).hexdigest()
    with (package / "SHA256SUMS").open("a", encoding="utf-8") as sums:
        sums.write(f"{digest}  extra.md\n")

    assert_package_fails(package, "unexpected entries")


def test_missing_checksum_entry_fails(tmp_path: Path) -> None:
    package = copy_current_package(tmp_path)
    lines = [line for line in (package / "SHA256SUMS").read_text(encoding="utf-8").splitlines() if "REPRODUCING.md" not in line]
    (package / "SHA256SUMS").write_text("\n".join(lines) + "\n", encoding="utf-8")

    assert_package_fails(package, "missing entries")
