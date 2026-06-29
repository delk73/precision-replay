#!/usr/bin/env python3
"""Check the v0.1.0-rc1 evidence package for internal coherence."""

from __future__ import annotations

import argparse
import hashlib
import sys
import tomllib
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PACKAGE_DIR = REPO_ROOT / "docs" / "evidence" / "v0.1.0-rc1"

EXPECTED_PACKAGE_ID = "precision-replay-v0.1.0-rc1"
EXPECTED_RELEASE_VERSION = "v0.1.0"
EXPECTED_RELEASE_CANDIDATE = "v0.1.0-rc1"
EXPECTED_REPOSITORY = "delk73/precision-replay"
EXPECTED_RC_READINESS_CHECKPOINT = "106e1b01ab5209cf74629ab6fb4fbb993278b2dd"
EXPECTED_SUPPORTED_CLAIM = (
    "one retained STM32F446 hardware-backed replay observation is recorded for this release candidate"
)
EXPECTED_RETAINED_FILES = [
    "evidence_package.md",
    "hardware_replay_artifact.md",
    "hardware_replay_transcript.txt",
    "stm32_flash_capture_procedure.md",
]
EXPECTED_CHECKSUM_FILES = [
    *EXPECTED_RETAINED_FILES,
    "manifest.toml",
    "REPRODUCING.md",
    "HARDWARE_SETUP.md",
    "REVIEW_PACKET.md",
]
REQUIRED_EXCLUDED_CLAIMS = {
    "certification compliance",
    "tool qualification",
    "hardware qualification",
    "timing behavior",
    "board-family validation",
    "full arithmetic proof closure",
}


class EvidencePackageError(Exception):
    """Raised when the evidence package is missing, malformed, or inconsistent."""


def _read_manifest(package_dir: Path) -> dict[str, object]:
    manifest_path = package_dir / "manifest.toml"
    if not manifest_path.exists():
        raise EvidencePackageError(f"manifest.toml is missing: {manifest_path}")

    try:
        with manifest_path.open("rb") as manifest_file:
            parsed = tomllib.load(manifest_file)
    except tomllib.TOMLDecodeError as exc:
        raise EvidencePackageError(f"manifest.toml does not parse as TOML: {exc}") from exc

    if not isinstance(parsed, dict):
        raise EvidencePackageError("manifest.toml did not parse to a table")
    return parsed


def _required_table(manifest: dict[str, object], name: str) -> dict[str, object]:
    table = manifest.get(name)
    if not isinstance(table, dict):
        raise EvidencePackageError(f"manifest missing [{name}] table")
    return table


def _required_string(table: dict[str, object], key: str, expected: str | None = None) -> str:
    value = table.get(key)
    if not isinstance(value, str) or not value:
        raise EvidencePackageError(f"manifest field {key!r} must be a non-empty string")
    if expected is not None and value != expected:
        raise EvidencePackageError(f"manifest field {key!r} must be {expected!r}")
    return value


def _required_string_list(table: dict[str, object], key: str) -> list[str]:
    value = table.get(key)
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        raise EvidencePackageError(f"manifest field {key!r} must be a list of strings")
    return value


def _validate_manifest(package_dir: Path, manifest: dict[str, object]) -> None:
    package = _required_table(manifest, "package")
    evidence = _required_table(manifest, "evidence")
    claims = _required_table(manifest, "claims")

    _required_string(package, "id", EXPECTED_PACKAGE_ID)
    _required_string(package, "release_version", EXPECTED_RELEASE_VERSION)
    _required_string(package, "release_candidate", EXPECTED_RELEASE_CANDIDATE)
    _required_string(package, "repository", EXPECTED_REPOSITORY)
    _required_string(package, "rc_readiness_checkpoint", EXPECTED_RC_READINESS_CHECKPOINT)

    retained_files = _required_string_list(evidence, "retained_files")
    if retained_files != EXPECTED_RETAINED_FILES:
        raise EvidencePackageError(
            f"manifest retained_files must be exactly {EXPECTED_RETAINED_FILES!r}"
        )
    for filename in retained_files:
        if not (package_dir / filename).is_file():
            raise EvidencePackageError(f"retained evidence file is missing: {filename}")

    supported_claims = _required_string_list(claims, "supported")
    if supported_claims != [EXPECTED_SUPPORTED_CLAIM]:
        raise EvidencePackageError("supported claim is missing or no longer bounded")

    excluded_claims = set(_required_string_list(claims, "excluded"))
    missing_exclusions = sorted(REQUIRED_EXCLUDED_CLAIMS - excluded_claims)
    if missing_exclusions:
        raise EvidencePackageError(f"required excluded claims are missing: {missing_exclusions}")


def _is_inside_package(package_dir: Path, candidate: Path) -> bool:
    package_root = package_dir.resolve()
    candidate_path = candidate.resolve()
    try:
        candidate_path.relative_to(package_root)
    except ValueError:
        return False
    return True


def _parse_sha256sums(package_dir: Path) -> dict[str, str]:
    sums_path = package_dir / "SHA256SUMS"
    if not sums_path.exists():
        raise EvidencePackageError(f"SHA256SUMS is missing: {sums_path}")

    entries: dict[str, str] = {}
    for line_number, raw_line in enumerate(sums_path.read_text(encoding="utf-8").splitlines(), start=1):
        if not raw_line.strip():
            continue
        parts = raw_line.split(None, 1)
        if len(parts) != 2:
            raise EvidencePackageError(f"malformed SHA256SUMS line {line_number}")
        digest, filename = parts
        if len(digest) != 64 or any(char not in "0123456789abcdef" for char in digest):
            raise EvidencePackageError(f"malformed SHA-256 digest on line {line_number}")
        if filename.startswith("*"):
            filename = filename[1:]
        if not filename or Path(filename).is_absolute():
            raise EvidencePackageError(f"checksum filename must be relative on line {line_number}")
        if "\\" in filename:
            raise EvidencePackageError(f"checksum filename must use package-relative POSIX form on line {line_number}")
        if any(part in {"", ".", ".."} for part in Path(filename).parts):
            raise EvidencePackageError(f"checksum filename escapes package on line {line_number}: {filename}")
        target = package_dir / filename
        if not _is_inside_package(package_dir, target):
            raise EvidencePackageError(f"checksum filename references outside package: {filename}")
        if filename in entries:
            raise EvidencePackageError(f"duplicate checksum entry: {filename}")
        entries[filename] = digest

    expected = set(EXPECTED_CHECKSUM_FILES)
    actual = set(entries)
    missing = sorted(expected - actual)
    extra = sorted(actual - expected)
    if missing:
        raise EvidencePackageError(f"SHA256SUMS missing entries: {missing}")
    if extra:
        raise EvidencePackageError(f"SHA256SUMS contains unexpected entries: {extra}")

    return entries


def _validate_checksums(package_dir: Path) -> None:
    entries = _parse_sha256sums(package_dir)
    for filename, expected_digest in entries.items():
        path = package_dir / filename
        if not path.is_file():
            raise EvidencePackageError(f"checksum target is missing: {filename}")
        actual_digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual_digest != expected_digest:
            raise EvidencePackageError(f"checksum mismatch for {filename}")


def check_evidence_package(package_dir: Path = DEFAULT_PACKAGE_DIR) -> None:
    if not package_dir.exists():
        raise EvidencePackageError(f"evidence package directory is missing: {package_dir}")
    if not package_dir.is_dir():
        raise EvidencePackageError(f"evidence package path is not a directory: {package_dir}")

    manifest = _read_manifest(package_dir)
    _validate_manifest(package_dir, manifest)
    _validate_checksums(package_dir)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "package_dir",
        nargs="?",
        type=Path,
        default=DEFAULT_PACKAGE_DIR,
        help="Path to the v0.1.0-rc1 evidence package directory",
    )
    args = parser.parse_args()

    try:
        check_evidence_package(args.package_dir)
    except EvidencePackageError as exc:
        print(f"Evidence package check failed: {exc}", file=sys.stderr)
        return 1

    print("Evidence package check passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
