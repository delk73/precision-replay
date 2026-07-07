from __future__ import annotations

import subprocess
import sys
from pathlib import Path


sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "tools"))

from check_raw_adc_envelope import judge_artifact  # noqa: E402


REPO_ROOT = Path(__file__).resolve().parents[1]
CHECKER = REPO_ROOT / "tools" / "check_raw_adc_envelope.py"


def valid_row(sample_index: int, raw_adc: str) -> str:
    return (
        "precision-replay v0.1.0-rc1 witness=raw-adc "
        f"sample_index={sample_index} raw_adc={raw_adc} "
        "timing_claim=best_effort_polling_uart_stream"
    )


def metadata(
    *,
    context_id: str = "bench-context-raw-adc-retained-001",
    envelope_id: str = "raw-adc-envelope-retained-001",
    raw_adc_min: str = "0x0002",
    raw_adc_max: str = "0x0007",
    min_sample_count: str = "2",
    allow_malformed_witness_lines: str = "false",
    limitations: str = "",
) -> str:
    return (
        "# Raw ADC Witness Capture Metadata\n\n"
        "## Envelope Judgment Metadata\n\n"
        f"- context_id: `{context_id}`\n"
        f"- envelope_id: `{envelope_id}`\n"
        f"- raw_adc_min: `{raw_adc_min}`\n"
        f"- raw_adc_max: `{raw_adc_max}`\n"
        f"- min_sample_count: `{min_sample_count}`\n"
        f"- allow_malformed_witness_lines: `{allow_malformed_witness_lines}`\n\n"
        "## Boundary\n\n"
        "This retained artifact claims only bounded raw ADC witness capture, "
        "host monitor parsing, deterministic summary, and raw ADC envelope judgment.\n\n"
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
    write_judgment: bool = True,
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
    if write_judgment:
        judgment = judge_artifact(artifact_dir)
        (artifact_dir / "judgment.txt").write_text(judgment.format() + "\n", encoding="utf-8")


def test_valid_envelope_artifact_passes(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir)

    result = run_cli(artifact_dir)

    assert result.returncode == 0
    assert "result=pass\n" in result.stdout
    assert "reason=all_admitted_observations_within_envelope" in result.stdout
    assert result.stderr == ""


def test_missing_judgment_file_fails_retained_check(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, write_judgment=False)

    result = run_cli(artifact_dir)

    assert result.returncode == 1
    assert "missing required file: judgment.txt" in result.stderr


def test_judgment_output_must_be_canonical(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir)
    (artifact_dir / "judgment.txt").write_text("result=pass\n", encoding="utf-8")

    result = run_cli(artifact_dir)

    assert result.returncode == 1
    assert "judgment.txt mismatch" in result.stderr


def test_missing_context_is_not_applicable_and_does_not_pass(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, metadata_text=metadata(context_id="none"))

    judgment = judge_artifact(artifact_dir)

    assert judgment.result == "not_applicable"
    assert judgment.reason == "missing_context_id"


def test_missing_limits_are_inconclusive_and_do_not_pass(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, metadata_text=metadata(raw_adc_min="none"))

    judgment = judge_artifact(artifact_dir)

    assert judgment.result == "inconclusive"
    assert judgment.reason == "missing_envelope_metadata:raw_adc_min"


def test_too_few_admitted_samples_are_inconclusive(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, metadata_text=metadata(min_sample_count="3"))

    judgment = judge_artifact(artifact_dir)

    assert judgment.result == "inconclusive"
    assert judgment.reason == "too_few_admitted_samples"


def test_admitted_sample_below_minimum_fails(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, metadata_text=metadata(raw_adc_min="0x0003"))

    judgment = judge_artifact(artifact_dir)

    assert judgment.result == "fail"
    assert judgment.reason == "admitted_sample_outside_envelope"


def test_admitted_sample_above_maximum_fails(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, metadata_text=metadata(raw_adc_max="0x0006"))

    judgment = judge_artifact(artifact_dir)

    assert judgment.result == "fail"
    assert judgment.reason == "admitted_sample_outside_envelope"


def test_malformed_rows_prevent_pass_when_not_allowed(tmp_path: Path) -> None:
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
            first_sample_index="10",
            last_sample_index="12",
            min_raw_adc="0x0003",
            malformed_witness_lines=1,
        ),
        metadata_text=metadata(
            limitations=(
                "Sample indices are not claimed contiguous. "
                "This artifact does not claim lossless serial capture."
            )
        ),
    )

    judgment = judge_artifact(artifact_dir)

    assert judgment.result == "inconclusive"
    assert judgment.reason == "malformed_witness_lines_not_allowed"


def test_tolerated_malformed_rows_remain_outside_admitted_observations(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(
        artifact_dir,
        raw_witness="\n".join(
            [
                valid_row(10, "0x0007"),
                "precision-replay v0.1.0-rc1 witness=raw-adc sample_index=11 raw_adc=0x0fff",
                valid_row(12, "0x0003"),
                "",
            ]
        ),
        capture=summary(
            first_sample_index="10",
            last_sample_index="12",
            min_raw_adc="0x0003",
            malformed_witness_lines=1,
        ),
        metadata_text=metadata(
            raw_adc_max="0x0007",
            allow_malformed_witness_lines="true",
            limitations=(
                "Sample indices are not claimed contiguous. "
                "This artifact does not claim lossless serial capture."
            ),
        ),
    )

    judgment = judge_artifact(artifact_dir)

    assert judgment.result == "pass"
    assert judgment.admitted_sample_count == 2
    assert judgment.admitted_max_raw_adc == 0x0007
    assert judgment.malformed_witness_lines == 1


def test_invalid_capture_is_inconclusive_not_admitted(tmp_path: Path) -> None:
    artifact_dir = tmp_path / "artifact"
    write_artifact(artifact_dir, capture=summary(sample_count=3))

    judgment = judge_artifact(artifact_dir)

    assert judgment.result == "inconclusive"
    assert judgment.reason == "capture_artifact_not_admitted"
    assert judgment.admitted_sample_count == 0


def run_cli(artifact_dir: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(CHECKER), str(artifact_dir)],
        check=False,
        text=True,
        capture_output=True,
    )
