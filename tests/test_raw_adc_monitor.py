from __future__ import annotations

import subprocess
import sys
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))

from raw_adc_monitor import (  # noqa: E402
    TIMING_CLAIM,
    RawAdcSample,
    parse_line,
    parse_stream,
)


REPO_ROOT = Path(__file__).resolve().parents[1]
MONITOR = REPO_ROOT / "tools" / "raw_adc_monitor.py"


def valid_row(sample_index: int, raw_adc: str) -> str:
    return (
        "precision-replay v0.1.0-rc1 witness=raw-adc "
        f"sample_index={sample_index} raw_adc={raw_adc} "
        "timing_claim=best_effort_polling_uart_stream"
    )


def test_parses_one_valid_witness_row() -> None:
    sample = parse_line(valid_row(7, "0x000a"))

    assert sample == RawAdcSample(
        sample_index=7,
        raw_adc=0x000A,
        timing_claim=TIMING_CLAIM,
    )


def test_parses_multiple_valid_witness_rows_in_order() -> None:
    result = parse_stream(
        [
            valid_row(0, "0x0001"),
            valid_row(1, "0x0004"),
            valid_row(2, "0x0002"),
        ]
    )

    assert [sample.sample_index for sample in result.samples] == [0, 1, 2]
    assert [sample.raw_adc for sample in result.samples] == [1, 4, 2]
    assert result.summary.first_sample_index == 0
    assert result.summary.last_sample_index == 2


def test_ignores_non_witness_banner_noise_and_math_result_lines() -> None:
    result = parse_stream(
        [
            "",
            "boot banner",
            "precision-replay v0.1.0-rc1 vector=math-add-001 result_bits=0x00000000000000020000000000000000",
            valid_row(3, "0x0003"),
        ]
    )

    assert len(result.samples) == 1
    assert result.summary.malformed_witness_lines == 0


def test_rejects_malformed_witness_rows() -> None:
    result = parse_stream(
        [
            "precision-replay v0.1.0-rc1 witness=raw-adc sample_index=4 raw_adc=0x0004",
            valid_row(5, "0x0005"),
        ]
    )

    assert [sample.sample_index for sample in result.samples] == [5]
    assert result.summary.malformed_witness_lines == 1


def test_rejects_out_of_range_raw_adc_values() -> None:
    result = parse_stream([valid_row(0, "0x1000")])

    assert result.samples == ()
    assert result.summary.malformed_witness_lines == 1


def test_rejects_wrong_timing_claim() -> None:
    result = parse_stream(
        [
            "precision-replay v0.1.0-rc1 witness=raw-adc "
            "sample_index=0 raw_adc=0x0001 timing_claim=timer_paced_adc"
        ]
    )

    assert result.samples == ()
    assert result.summary.malformed_witness_lines == 1


def test_produces_correct_summary_for_short_captured_stream() -> None:
    result = parse_stream(
        [
            "boot banner",
            valid_row(10, "0x0007"),
            valid_row(11, "0x0002"),
            valid_row(12, "0x000f"),
        ]
    )

    assert result.summary.format() == "\n".join(
        [
            "sample_count=3",
            "first_sample_index=10",
            "last_sample_index=12",
            "min_raw_adc=0x0002",
            "max_raw_adc=0x000f",
            "malformed_witness_lines=0",
        ]
    )


def test_handles_empty_stream_deterministically() -> None:
    result = parse_stream([])

    assert result.summary.format() == "\n".join(
        [
            "sample_count=0",
            "first_sample_index=none",
            "last_sample_index=none",
            "min_raw_adc=none",
            "max_raw_adc=none",
            "malformed_witness_lines=0",
        ]
    )


def test_cli_exits_nonzero_after_summary_for_malformed_witness_rows() -> None:
    completed = subprocess.run(
        [sys.executable, str(MONITOR), "-"],
        input=valid_row(1, "0x1000") + "\n",
        text=True,
        capture_output=True,
        check=False,
    )

    assert completed.returncode == 1
    assert completed.stdout == "\n".join(
        [
            "sample_count=0",
            "first_sample_index=none",
            "last_sample_index=none",
            "min_raw_adc=none",
            "max_raw_adc=none",
            "malformed_witness_lines=1",
            "",
        ]
    )
