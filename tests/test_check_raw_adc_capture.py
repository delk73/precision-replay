from __future__ import annotations

import subprocess
import sys
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))

from check_raw_adc_capture import check_artifact  # noqa: E402


REPO_ROOT = Path(__file__).resolve().parents[1]
CHECKER = REPO_ROOT / "tools" / "check_raw_adc_capture.py"


def valid_row(sample_index: int, raw_adc: str) -> str:
    return (
        "precision-replay v0.1.0-rc1 witness=raw-adc "
        f"sample_index={sample_index} raw_adc={raw_adc} "
        "timing_claim=best_effort_polling_uart_stream"
    )


def metadata(limitations: str = "") -> str:
    return (
        "# Raw ADC Witness Capture Metadata\n\n"
        "## Boundary\n\n"
        "This retained artifact claims only bounded raw ADC witness capture, "
        "host monitor parsing, and deterministic summary.\n\n"
        "## Known Limitations\n\n"
        f"{limitations}\n"
    )


def summary(
    *,
    sample_count: int | str = 2,
    first_sample_index: str = "10",
    last_sample_index: str = "11",
    min_raw_adc: str = "0x0002",
    max_raw_adc: str = "0x0007",
    malformed_witness_lines: int | str = 0,
) -> str:
    return "\n".join(
        [
            f"sample_count={sample_count}",
            f"first_sample_index={first_sample_index}",
            f"last_sample_index={last_sample_index}",
            f"min_raw_adc={min_raw_adc}",
            f"max_raw_adc={max_raw_adc}",
            f"malformed_witness_lines={malformed_witness_lines}",
            "",
        ]
    )


def write_artifact(
    artifact_dir: Path,
    *,
    raw_witness: str | None = None,
    capture: str | None = None,
    metadata_text: str | None = None,
) -> None:
    artifact_dir.mkdir()
    (artifact_dir / "raw_witness.txt").write_text(
        raw_witness
        if raw_witness is not None
        else "\n".join([valid_row(10, "0x0007"), valid_row(11, "0x0002"), ""]),
        encoding="utf-8",
    )
    (artifact_dir / "capture.txt").write_text(
        capture if capture is not None else summary(),
        encoding="utf-8",
    )
    (artifact_dir / "metadata.md").write_text(
        metadata_text if metadata_text is not None else metadata(),
        encoding="utf-8",
    )


def test_valid_artifact_passes(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir)

    result = check_artifact(artifact_dir)

    assert result.ok
    assert result.errors == ()


def test_missing_files_fail(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    artifact_dir.mkdir()

    result = check_artifact(artifact_dir)

    assert not result.ok
    assert result.errors == (
        "missing required file: raw_witness.txt",
        "missing required file: capture.txt",
        "missing required file: metadata.md",
    )


def test_malformed_witness_row_is_counted(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(
        artifact_dir,
        raw_witness="\n".join(
            [
                valid_row(10, "0x0007"),
                "precision-replay v0.1.0-rc1 witness=raw-adc sample_index=11 raw_adc=0x0002",
                valid_row(12, "0x0003"),
                "",
            ]
        ),
        capture=summary(
            sample_count=2,
            first_sample_index="10",
            last_sample_index="12",
            min_raw_adc="0x0003",
            max_raw_adc="0x0007",
            malformed_witness_lines=1,
        ),
        metadata_text=metadata(
            "Sample indices are not claimed contiguous. "
            "This artifact does not claim lossless serial capture."
        ),
    )

    result = check_artifact(artifact_dir)

    assert result.ok
    assert result.summary is not None
    assert result.summary.malformed_witness_lines == 1


def test_summary_mismatch_fails(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, capture=summary(sample_count=3))

    result = check_artifact(artifact_dir)

    assert not result.ok
    assert "capture.txt summary mismatch" in result.errors[0]


def test_capture_summary_must_be_canonical(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, capture=summary(sample_count="02"))

    result = check_artifact(artifact_dir)

    assert not result.ok
    assert "capture.txt summary mismatch" in result.errors[0]


def test_capture_summary_field_order_must_be_canonical(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(
        artifact_dir,
        capture="\n".join(
            [
                "first_sample_index=10",
                "sample_count=2",
                "last_sample_index=11",
                "min_raw_adc=0x0002",
                "max_raw_adc=0x0007",
                "malformed_witness_lines=0",
                "",
            ]
        ),
    )

    result = check_artifact(artifact_dir)

    assert not result.ok
    assert "capture.txt summary mismatch" in result.errors[0]


def test_malformed_summary_fails(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, capture="sample_count two\n")

    result = run_cli(artifact_dir)

    assert result.returncode == 1
    assert "malformed summary line" in result.stderr


def test_malformed_raw_adc_tokens_are_not_silently_truncated(tmp_path: Path) -> None:
    malformed_tokens = ("0x0fffBAD", "0x1000BAD", "0x123", "0xzzzz")

    for token in malformed_tokens:
        artifact_dir = tmp_path / token.replace("0x", "raw_")
        write_artifact(
            artifact_dir,
            raw_witness=valid_row(10, token) + "\n",
            capture=summary(
                sample_count=0,
                first_sample_index="none",
                last_sample_index="none",
                min_raw_adc="none",
                max_raw_adc="none",
                malformed_witness_lines=1,
            ),
        )

        result = check_artifact(artifact_dir)

        assert not result.ok
        assert f"malformed raw_adc token: raw_adc={token}" in result.errors[0]


def test_out_of_range_raw_adc_fails(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(
        artifact_dir,
        raw_witness=valid_row(10, "0x1000") + "\n",
        capture=summary(
            sample_count=0,
            first_sample_index="none",
            last_sample_index="none",
            min_raw_adc="none",
            max_raw_adc="none",
            malformed_witness_lines=1,
        ),
    )

    result = check_artifact(artifact_dir)

    assert not result.ok
    assert "raw_adc 0x1000 exceeds 0x0fff" in result.errors[0]


def test_missing_metadata_boundary_fails(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, metadata_text="# Metadata\n\nNo boundary statement.\n")

    result = check_artifact(artifact_dir)

    assert not result.ok
    assert result.errors == ("metadata.md does not preserve the retained artifact boundary",)


def test_non_contiguous_samples_require_explicit_limitation(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(
        artifact_dir,
        raw_witness="\n".join([valid_row(10, "0x0007"), valid_row(12, "0x0002"), ""]),
        capture=summary(last_sample_index="12"),
    )

    result = check_artifact(artifact_dir)

    assert not result.ok
    assert result.errors == (
        "metadata.md must document non-contiguous / non-lossless capture "
        "when sample indices are not contiguous",
    )


def test_cli_pass_and_fail_exit_codes(tmp_path: Path) -> None:
    passing_artifact = tmp_path / "passing"
    write_artifact(passing_artifact)

    failing_artifact = tmp_path / "failing"
    write_artifact(failing_artifact, capture=summary(sample_count=3))

    passing = run_cli(passing_artifact)
    failing = run_cli(failing_artifact)

    assert passing.returncode == 0
    assert "sample_count=2" in passing.stdout
    assert passing.stderr == ""
    assert failing.returncode == 1
    assert "capture.txt summary mismatch" in failing.stderr


def run_cli(artifact_dir: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(CHECKER), str(artifact_dir)],
        text=True,
        capture_output=True,
        check=False,
    )
