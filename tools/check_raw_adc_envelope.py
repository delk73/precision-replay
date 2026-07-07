#!/usr/bin/env python3
"""Judge a retained raw ADC artifact against declared envelope metadata."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

from check_raw_adc_capture import CheckResult as CaptureCheckResult
from check_raw_adc_capture import check_artifact as check_capture_artifact
from raw_adc_monitor import RAW_ADC_MAX, RawAdcSample, parse_stream


REQUIRED_METADATA_FIELDS = (
    "context_id",
    "envelope_id",
    "raw_adc_min",
    "raw_adc_max",
    "min_sample_count",
    "allow_malformed_witness_lines",
)
RESULT_VALUES = ("pass", "fail", "inconclusive", "not_applicable")
METADATA_FIELD_RE = re.compile(
    r"^\s*[-*]\s*(?P<key>[a-z0-9_]+)\s*:\s*(?P<value>.+?)\s*$"
)


@dataclass(frozen=True)
class EnvelopeMetadata:
    context_id: str | None
    envelope_id: str | None
    raw_adc_min: int | None
    raw_adc_max: int | None
    min_sample_count: int | None
    allow_malformed_witness_lines: bool | None


@dataclass(frozen=True)
class EnvelopeJudgment:
    result: str
    reason: str
    context_id: str
    envelope_id: str
    raw_adc_min: int | None
    raw_adc_max: int | None
    min_sample_count: int | None
    allow_malformed_witness_lines: bool | None
    admitted_sample_count: int
    admitted_min_raw_adc: int | None
    admitted_max_raw_adc: int | None
    malformed_witness_lines: int

    def format(self) -> str:
        return "\n".join(
            [
                f"result={self.result}",
                f"reason={self.reason}",
                f"context_id={_format_optional_text(self.context_id)}",
                f"envelope_id={_format_optional_text(self.envelope_id)}",
                f"raw_adc_min={_format_optional_raw_adc(self.raw_adc_min)}",
                f"raw_adc_max={_format_optional_raw_adc(self.raw_adc_max)}",
                f"min_sample_count={_format_optional_decimal(self.min_sample_count)}",
                "allow_malformed_witness_lines="
                f"{_format_optional_bool(self.allow_malformed_witness_lines)}",
                f"admitted_sample_count={self.admitted_sample_count}",
                f"admitted_min_raw_adc={_format_optional_raw_adc(self.admitted_min_raw_adc)}",
                f"admitted_max_raw_adc={_format_optional_raw_adc(self.admitted_max_raw_adc)}",
                f"malformed_witness_lines={self.malformed_witness_lines}",
            ]
        )


@dataclass(frozen=True)
class CheckResult:
    ok: bool
    errors: tuple[str, ...]
    judgment: EnvelopeJudgment | None = None


class EnvelopeCheckError(ValueError):
    """Raised when the retained envelope judgment cannot be checked."""


def check_artifact(artifact_dir: Path) -> CheckResult:
    if not artifact_dir.is_dir():
        return CheckResult(False, (f"artifact directory not found: {artifact_dir}",))

    judgment = judge_artifact(artifact_dir)
    expected_text = judgment.format().rstrip("\n") + "\n"

    judgment_path = artifact_dir / "judgment.txt"
    if not judgment_path.is_file():
        return CheckResult(False, ("missing required file: judgment.txt",), judgment)

    actual_text = judgment_path.read_text(encoding="utf-8")
    errors: list[str] = []
    if actual_text != expected_text:
        errors.append(
            "judgment.txt mismatch:\n"
            f"expected:\n{expected_text}"
            f"actual:\n{actual_text}"
        )
    if judgment.result != "pass":
        errors.append(f"raw ADC envelope judgment did not pass: {judgment.reason}")

    return CheckResult(ok=not errors, errors=tuple(errors), judgment=judgment)


def judge_artifact(artifact_dir: Path) -> EnvelopeJudgment:
    capture_result = check_capture_artifact(artifact_dir)
    metadata = _read_metadata(artifact_dir / "metadata.md")

    raw_witness_path = artifact_dir / "raw_witness.txt"
    samples: tuple[RawAdcSample, ...] = ()
    malformed_witness_lines = 0
    if raw_witness_path.is_file():
        parse_result = parse_stream(raw_witness_path.read_text(encoding="utf-8").splitlines())
        samples = parse_result.samples
        malformed_witness_lines = parse_result.summary.malformed_witness_lines

    admitted_sample_count = len(samples) if capture_result.ok else 0
    admitted_min_raw_adc = min((sample.raw_adc for sample in samples), default=None) if capture_result.ok else None
    admitted_max_raw_adc = max((sample.raw_adc for sample in samples), default=None) if capture_result.ok else None

    reason = _judge_reason(capture_result, metadata, samples)
    result = _result_for_reason(reason)

    return EnvelopeJudgment(
        result=result,
        reason=reason,
        context_id=metadata.context_id or "none",
        envelope_id=metadata.envelope_id or "none",
        raw_adc_min=metadata.raw_adc_min,
        raw_adc_max=metadata.raw_adc_max,
        min_sample_count=metadata.min_sample_count,
        allow_malformed_witness_lines=metadata.allow_malformed_witness_lines,
        admitted_sample_count=admitted_sample_count,
        admitted_min_raw_adc=admitted_min_raw_adc,
        admitted_max_raw_adc=admitted_max_raw_adc,
        malformed_witness_lines=malformed_witness_lines,
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifact_dir", type=Path, help="Directory containing the retained raw ADC artifact")
    parser.add_argument(
        "--write",
        action="store_true",
        help="Write the canonical retained judgment.txt before checking it",
    )
    args = parser.parse_args(argv)

    try:
        if args.write:
            judgment = judge_artifact(args.artifact_dir)
            (args.artifact_dir / "judgment.txt").write_text(judgment.format() + "\n", encoding="utf-8")
        result = check_artifact(args.artifact_dir)
    except (OSError, UnicodeDecodeError, EnvelopeCheckError) as exc:
        print(f"raw ADC envelope check failed: {exc}", file=sys.stderr)
        return 1

    if result.judgment is not None:
        print(result.judgment.format())

    if result.ok:
        return 0

    for error in result.errors:
        print(f"raw ADC envelope check failed: {error}", file=sys.stderr)
    return 1


def _judge_reason(
    capture_result: CaptureCheckResult,
    metadata: EnvelopeMetadata,
    samples: tuple[RawAdcSample, ...],
) -> str:
    if metadata.context_id is None:
        return "missing_context_id"
    missing_fields = _missing_metadata_fields(metadata)
    if missing_fields:
        return f"missing_envelope_metadata:{missing_fields[0]}"
    if not capture_result.ok:
        return "capture_artifact_not_admitted"
    if metadata.raw_adc_min is None or metadata.raw_adc_max is None:
        return "missing_raw_adc_limits"
    if metadata.raw_adc_min > metadata.raw_adc_max:
        return "invalid_raw_adc_limits"
    if any(
        sample.raw_adc < metadata.raw_adc_min or sample.raw_adc > metadata.raw_adc_max
        for sample in samples
    ):
        return "admitted_sample_outside_envelope"
    if len(samples) < metadata.min_sample_count:
        return "too_few_admitted_samples"
    if (
        capture_result.summary is not None
        and capture_result.summary.malformed_witness_lines > 0
        and metadata.allow_malformed_witness_lines is False
    ):
        return "malformed_witness_lines_not_allowed"
    return "all_admitted_observations_within_envelope"


def _result_for_reason(reason: str) -> str:
    if reason == "all_admitted_observations_within_envelope":
        return "pass"
    if reason in {"admitted_sample_outside_envelope", "invalid_raw_adc_limits"}:
        return "fail"
    if reason == "missing_context_id":
        return "not_applicable"
    return "inconclusive"


def _read_metadata(metadata_path: Path) -> EnvelopeMetadata:
    values: dict[str, str] = {}
    if metadata_path.is_file():
        for line in metadata_path.read_text(encoding="utf-8").splitlines():
            match = METADATA_FIELD_RE.fullmatch(line)
            if match is None:
                continue
            key = match.group("key")
            if key in REQUIRED_METADATA_FIELDS and key not in values:
                values[key] = _strip_markdown_value(match.group("value"))

    return EnvelopeMetadata(
        context_id=_parse_optional_text(values.get("context_id")),
        envelope_id=_parse_optional_text(values.get("envelope_id")),
        raw_adc_min=_parse_optional_raw_adc(values.get("raw_adc_min")),
        raw_adc_max=_parse_optional_raw_adc(values.get("raw_adc_max")),
        min_sample_count=_parse_optional_decimal(values.get("min_sample_count")),
        allow_malformed_witness_lines=_parse_optional_bool(
            values.get("allow_malformed_witness_lines")
        ),
    )


def _missing_metadata_fields(metadata: EnvelopeMetadata) -> tuple[str, ...]:
    missing: list[str] = []
    if metadata.envelope_id is None:
        missing.append("envelope_id")
    if metadata.raw_adc_min is None:
        missing.append("raw_adc_min")
    if metadata.raw_adc_max is None:
        missing.append("raw_adc_max")
    if metadata.min_sample_count is None:
        missing.append("min_sample_count")
    if metadata.allow_malformed_witness_lines is None:
        missing.append("allow_malformed_witness_lines")
    return tuple(missing)


def _strip_markdown_value(value: str) -> str:
    stripped = value.strip()
    if stripped.startswith("`") and stripped.endswith("`") and len(stripped) >= 2:
        return stripped[1:-1].strip()
    return stripped


def _parse_optional_text(value: str | None) -> str | None:
    if value is None or value == "" or value == "none":
        return None
    return value


def _parse_optional_decimal(value: str | None) -> int | None:
    if value is None or value == "none" or not value.isdecimal():
        return None
    return int(value, 10)


def _parse_optional_bool(value: str | None) -> bool | None:
    if value == "true":
        return True
    if value == "false":
        return False
    return None


def _parse_optional_raw_adc(value: str | None) -> int | None:
    if value is None or value == "none":
        return None
    if len(value) != 6 or not value.startswith("0x"):
        return None
    try:
        parsed = int(value[2:], 16)
    except ValueError:
        return None
    if parsed > RAW_ADC_MAX:
        return None
    return parsed


def _format_optional_text(value: str) -> str:
    return value


def _format_optional_decimal(value: int | None) -> str:
    return "none" if value is None else str(value)


def _format_optional_raw_adc(value: int | None) -> str:
    return "none" if value is None else f"0x{value:04x}"


def _format_optional_bool(value: bool | None) -> str:
    if value is None:
        return "none"
    return "true" if value else "false"


if __name__ == "__main__":
    raise SystemExit(main())
