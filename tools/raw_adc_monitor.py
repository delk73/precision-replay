#!/usr/bin/env python3
"""Monitor and summarize raw ADC witness rows."""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from typing import Iterable


TIMING_CLAIM = "best_effort_polling_uart_stream"
RAW_ADC_MAX = 0x0FFF
WITNESS_TOKEN = "witness=raw-adc"

RAW_ADC_WITNESS_RE = re.compile(
    r"^precision-replay v0\.1\.0-rc1 "
    r"witness=raw-adc "
    r"sample_index=(?P<sample_index>[0-9]+) "
    r"raw_adc=0x(?P<raw_adc>[0-9a-f]{4}) "
    r"timing_claim=(?P<timing_claim>best_effort_polling_uart_stream)$"
)


@dataclass(frozen=True)
class RawAdcSample:
    sample_index: int
    raw_adc: int
    timing_claim: str


@dataclass(frozen=True)
class RawAdcSummary:
    sample_count: int
    first_sample_index: int | None
    last_sample_index: int | None
    min_raw_adc: int | None
    max_raw_adc: int | None
    malformed_witness_lines: int

    def format(self) -> str:
        return "\n".join(
            [
                f"sample_count={self.sample_count}",
                f"first_sample_index={_format_optional_decimal(self.first_sample_index)}",
                f"last_sample_index={_format_optional_decimal(self.last_sample_index)}",
                f"min_raw_adc={_format_optional_raw_adc(self.min_raw_adc)}",
                f"max_raw_adc={_format_optional_raw_adc(self.max_raw_adc)}",
                f"malformed_witness_lines={self.malformed_witness_lines}",
            ]
        )


@dataclass(frozen=True)
class ParseResult:
    samples: tuple[RawAdcSample, ...]
    summary: RawAdcSummary


def parse_line(line: str) -> RawAdcSample | None:
    normalized = line.rstrip("\r\n")
    match = RAW_ADC_WITNESS_RE.fullmatch(normalized)
    if match is None:
        return None

    sample_index = int(match.group("sample_index"), 10)
    raw_adc = int(match.group("raw_adc"), 16)
    timing_claim = match.group("timing_claim")

    if raw_adc > RAW_ADC_MAX:
        return None
    if timing_claim != TIMING_CLAIM:
        return None

    return RawAdcSample(
        sample_index=sample_index,
        raw_adc=raw_adc,
        timing_claim=timing_claim,
    )


def parse_stream(lines: Iterable[str]) -> ParseResult:
    samples: list[RawAdcSample] = []
    malformed_witness_lines = 0

    for line in lines:
        sample = parse_line(line)
        if sample is not None:
            samples.append(sample)
        elif WITNESS_TOKEN in line:
            malformed_witness_lines += 1

    summary = summarize(samples, malformed_witness_lines)
    return ParseResult(samples=tuple(samples), summary=summary)


def summarize(samples: Iterable[RawAdcSample], malformed_witness_lines: int = 0) -> RawAdcSummary:
    sample_list = list(samples)
    raw_values = [sample.raw_adc for sample in sample_list]

    return RawAdcSummary(
        sample_count=len(sample_list),
        first_sample_index=sample_list[0].sample_index if sample_list else None,
        last_sample_index=sample_list[-1].sample_index if sample_list else None,
        min_raw_adc=min(raw_values) if raw_values else None,
        max_raw_adc=max(raw_values) if raw_values else None,
        malformed_witness_lines=malformed_witness_lines,
    )


def read_serial_lines(device: str, baud: int) -> Iterable[str]:
    try:
        import serial
    except ImportError as exc:
        raise RuntimeError("pyserial is required for --serial input") from exc

    with serial.Serial(device, baudrate=baud, timeout=None) as port:
        while True:
            try:
                raw_line = port.readline()
            except KeyboardInterrupt:
                break
            if raw_line == b"":
                break
            yield raw_line.decode("utf-8", errors="replace")


def run(input_lines: Iterable[str]) -> int:
    result = parse_stream(input_lines)
    print(result.summary.format())
    return 1 if result.summary.malformed_witness_lines else 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "input",
        nargs="?",
        default="-",
        choices=["-"],
        help="Use '-' for stdin",
    )
    parser.add_argument(
        "--serial",
        metavar="DEVICE",
        help="Read witness lines from a serial device instead of stdin or file input",
    )
    parser.add_argument(
        "--baud",
        type=int,
        default=115200,
        help="Serial baud rate used with --serial",
    )
    args = parser.parse_args(argv)

    if args.serial is not None:
        try:
            return run(read_serial_lines(args.serial, args.baud))
        except RuntimeError as exc:
            print(f"raw ADC monitor failed: {exc}", file=sys.stderr)
            return 1

    return run(sys.stdin)


def _format_optional_decimal(value: int | None) -> str:
    return "none" if value is None else str(value)


def _format_optional_raw_adc(value: int | None) -> str:
    return "none" if value is None else f"0x{value:04x}"


if __name__ == "__main__":
    raise SystemExit(main())
