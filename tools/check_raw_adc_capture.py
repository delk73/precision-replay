#!/usr/bin/env python3
"""Verify a retained raw ADC witness capture artifact."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

from raw_adc_monitor import RAW_ADC_MAX, RawAdcSample, RawAdcSummary, parse_stream


REQUIRED_FILES = ("raw_witness.txt", "capture.txt", "metadata.md")
SUMMARY_FIELDS = (
    "sample_count",
    "first_sample_index",
    "last_sample_index",
    "min_raw_adc",
    "max_raw_adc",
    "malformed_witness_lines",
)
RAW_ADC_TOKEN_RE = re.compile(r"^raw_adc=0x(?P<raw_adc>[0-9a-fA-F]{4})$")


@dataclass(frozen=True)
class CheckResult:
    ok: bool
    errors: tuple[str, ...]
    summary: RawAdcSummary | None = None


class CaptureCheckError(ValueError):
    """Raised when the retained capture artifact cannot be verified."""


def check_artifact(artifact_dir: Path) -> CheckResult:
    errors: list[str] = []

    if not artifact_dir.is_dir():
        return CheckResult(False, (f"artifact directory not found: {artifact_dir}",))

    missing = [name for name in REQUIRED_FILES if not (artifact_dir / name).is_file()]
    if missing:
        return CheckResult(
            False,
            tuple(f"missing required file: {name}" for name in missing),
        )

    raw_witness = artifact_dir / "raw_witness.txt"
    capture = artifact_dir / "capture.txt"
    metadata = artifact_dir / "metadata.md"

    raw_witness_text = raw_witness.read_text(encoding="utf-8")
    capture_text = capture.read_text(encoding="utf-8")

    parse_result = parse_stream(raw_witness_text.splitlines())
    summary = parse_result.summary
    try:
        _parse_capture_summary(capture_text)
    except CaptureCheckError as exc:
        return CheckResult(False, (str(exc),), summary)

    expected_capture = summary.format().rstrip("\n") + "\n"
    if capture_text != expected_capture:
        errors.append(
            "capture.txt summary mismatch:\n"
            f"expected:\n{expected_capture}"
            f"actual:\n{capture_text}"
        )

    raw_adc_token_errors = _find_raw_adc_token_errors(raw_witness_text.splitlines())
    for line_number, message in raw_adc_token_errors:
        errors.append(f"raw_witness.txt:{line_number}: {message}")

    metadata_text = metadata.read_text(encoding="utf-8")
    if not _preserves_artifact_boundary(metadata_text):
        errors.append("metadata.md does not preserve the retained artifact boundary")

    if not _sample_indices_are_contiguous(parse_result.samples):
        if not _documents_non_contiguous_limit(metadata_text):
            errors.append(
                "metadata.md must document non-contiguous / non-lossless capture "
                "when sample indices are not contiguous"
            )

    return CheckResult(ok=not errors, errors=tuple(errors), summary=summary)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifact_dir", type=Path, help="Directory containing the retained raw ADC artifact")
    args = parser.parse_args(argv)

    try:
        result = check_artifact(args.artifact_dir)
    except (OSError, UnicodeDecodeError, CaptureCheckError) as exc:
        print(f"raw ADC capture check failed: {exc}", file=sys.stderr)
        return 1

    if result.ok:
        if result.summary is not None:
            print(result.summary.format())
        return 0

    for error in result.errors:
        print(f"raw ADC capture check failed: {error}", file=sys.stderr)
    return 1


def _parse_capture_summary(text: str) -> RawAdcSummary:
    values: dict[str, str] = {}

    for line_number, line in enumerate(text.splitlines(), start=1):
        if not line.strip():
            continue
        key, separator, value = line.partition("=")
        if separator == "":
            raise CaptureCheckError(f"capture.txt:{line_number}: malformed summary line")
        if key not in SUMMARY_FIELDS:
            raise CaptureCheckError(f"capture.txt:{line_number}: unknown summary field: {key}")
        if key in values:
            raise CaptureCheckError(f"capture.txt:{line_number}: duplicate summary field: {key}")
        values[key] = value

    missing = [field for field in SUMMARY_FIELDS if field not in values]
    if missing:
        raise CaptureCheckError(f"capture.txt missing summary field: {missing[0]}")

    return RawAdcSummary(
        sample_count=_parse_decimal(values["sample_count"], "sample_count"),
        first_sample_index=_parse_optional_decimal(values["first_sample_index"], "first_sample_index"),
        last_sample_index=_parse_optional_decimal(values["last_sample_index"], "last_sample_index"),
        min_raw_adc=_parse_optional_raw_adc(values["min_raw_adc"], "min_raw_adc"),
        max_raw_adc=_parse_optional_raw_adc(values["max_raw_adc"], "max_raw_adc"),
        malformed_witness_lines=_parse_decimal(
            values["malformed_witness_lines"], "malformed_witness_lines"
        ),
    )


def _parse_decimal(value: str, field: str) -> int:
    if not value.isdecimal():
        raise CaptureCheckError(f"capture.txt field {field} is not a decimal integer")
    return int(value, 10)


def _parse_optional_decimal(value: str, field: str) -> int | None:
    if value == "none":
        return None
    return _parse_decimal(value, field)


def _parse_optional_raw_adc(value: str, field: str) -> int | None:
    if value == "none":
        return None
    if len(value) != 6 or not value.startswith("0x"):
        raise CaptureCheckError(f"capture.txt field {field} is not a 4-digit hex raw ADC value")
    try:
        parsed = int(value[2:], 16)
    except ValueError as exc:
        raise CaptureCheckError(f"capture.txt field {field} is not a 4-digit hex raw ADC value") from exc
    if parsed > RAW_ADC_MAX:
        raise CaptureCheckError(f"capture.txt field {field} exceeds 0x{RAW_ADC_MAX:04x}")
    return parsed


def _find_raw_adc_token_errors(lines: list[str]) -> tuple[tuple[int, str], ...]:
    errors: list[tuple[int, str]] = []

    for line_number, line in enumerate(lines, start=1):
        if "witness=raw-adc" not in line:
            continue

        token = _extract_raw_adc_token(line)
        if token is None:
            continue

        match = RAW_ADC_TOKEN_RE.fullmatch(token)
        if match is None:
            errors.append((line_number, f"malformed raw_adc token: {token}"))
            continue

        raw_adc = int(match.group("raw_adc"), 16)
        if raw_adc > RAW_ADC_MAX:
            errors.append((line_number, f"raw_adc 0x{raw_adc:04x} exceeds 0x{RAW_ADC_MAX:04x}"))

    return tuple(errors)


def _extract_raw_adc_token(line: str) -> str | None:
    for token in line.split():
        if token.startswith("raw_adc="):
            return token
    return None


def _preserves_artifact_boundary(metadata_text: str) -> bool:
    normalized = metadata_text.lower()
    return "boundary" in normalized and "retained artifact" in normalized and "claims only" in normalized


def _sample_indices_are_contiguous(samples: tuple[RawAdcSample, ...]) -> bool:
    if len(samples) < 2:
        return True
    return all(
        current.sample_index == previous.sample_index + 1
        for previous, current in zip(samples, samples[1:])
    )


def _documents_non_contiguous_limit(metadata_text: str) -> bool:
    normalized = metadata_text.lower()
    return (
        ("not claimed contiguous" in normalized or "not contiguous" in normalized or "non-contiguous" in normalized)
        and ("does not claim lossless" in normalized or "non-lossless" in normalized or "not lossless" in normalized)
    )


if __name__ == "__main__":
    raise SystemExit(main())
