#!/usr/bin/env python3
"""Validate the archived replay traceability matrix and emit a JSON projection."""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_HLR = REPO_ROOT / "docs" / "archive" / "wip_requirements_2026" / "HLR_replay.md"
DEFAULT_LLR = REPO_ROOT / "docs" / "archive" / "wip_requirements_2026" / "LLR_replay.md"
DEFAULT_MATRIX = (
    REPO_ROOT / "docs" / "archive" / "wip_requirements_2026" / "traceability_matrix.md"
)
DEFAULT_OUTPUT = REPO_ROOT / "trace_projection.json"

CANONICAL_STATUSES = {
    "pending",
    "implemented",
    "tested",
    "proof_partial",
    "boundary_only",
    "traced",
    "decomposed",
}
SOURCE_PREFIXES = ("core/", "verification/", "tests/", "artifacts/", "tools/", "bsp/")

REQ_ID_RE = re.compile(r"\b(?:HLR|LLR)-[A-Z0-9]+(?:-[A-Z0-9]+)*-\d{3}\b")
HEADER_ID_RE = re.compile(r"^#{1,6}\s+((?:HLR|LLR)-[A-Z0-9]+(?:-[A-Z0-9]+)*-\d{3})\b")
STATUS_RE = re.compile(r"^Status:\s+([a-z_]+)\.\s+(.+)$")
BACKTICK_RE = re.compile(r"`([^`]+)`")
PATH_RE = re.compile(
    r"(?<![A-Za-z0-9_/])(?:core|verification|tests|artifacts|tools|bsp)/[A-Za-z0-9_.\/+-]+"
)


class ValidationError(Exception):
    """Raised for deterministic command-line setup errors."""


@dataclass(frozen=True)
class Diagnostic:
    line: int
    code: str
    message: str

    def as_dict(self) -> dict[str, object]:
        return {"line": self.line, "code": self.code, "message": self.message}

    def render(self, matrix_path: Path) -> str:
        rel = matrix_path.relative_to(REPO_ROOT)
        return f"{rel}:{self.line}: {self.code}: {self.message}"


@dataclass(frozen=True)
class Requirement:
    requirement_id: str
    source: str
    line: int

    def as_dict(self) -> dict[str, object]:
        return {
            "id": self.requirement_id,
            "source": self.source,
            "line": self.line,
        }


@dataclass(frozen=True)
class TraceRow:
    row_id: str
    row_type: str
    line: int
    columns: list[str]
    requirement_ids: list[str]
    paths: list[str]
    symbols: list[str]
    status: str
    flags: list[str]

    def as_dict(self) -> dict[str, object]:
        return {
            "id": self.row_id,
            "type": self.row_type,
            "line": self.line,
            "columns": self.columns,
            "requirement_ids": self.requirement_ids,
            "paths": self.paths,
            "symbols": self.symbols,
            "status": self.status,
            "flags": self.flags,
        }


def _repo_rel(path: Path) -> str:
    return path.relative_to(REPO_ROOT).as_posix()


def _read_lines(path: Path) -> list[str]:
    if not path.exists():
        raise ValidationError(f"input file is missing: {_repo_rel(path)}")
    return path.read_text(encoding="utf-8").splitlines()


def _extract_requirements(path: Path) -> dict[str, Requirement]:
    requirements: dict[str, Requirement] = {}
    for line_no, line in enumerate(_read_lines(path), start=1):
        match = HEADER_ID_RE.match(line)
        if match:
            req_id = match.group(1)
            requirements[req_id] = Requirement(req_id, _repo_rel(path), line_no)
    return requirements


def _split_table_row(line: str) -> list[str]:
    stripped = line.strip()
    if not stripped.startswith("|") or not stripped.endswith("|"):
        return []
    return [cell.strip() for cell in stripped.strip("|").split("|")]


def _is_separator(cells: list[str]) -> bool:
    return bool(cells) and all(set(cell.replace(" ", "")) <= {":", "-"} for cell in cells)


def _row_type(cells: list[str]) -> str | None:
    if cells == [
        "Code Component / Implementation Block",
        "Requirement ID",
        "Traceability Verification",
    ]:
        return "implementation"
    if cells == [
        "Requirement",
        "Evidence / Implementation",
        "Verification / Status",
        "Boundary",
    ]:
        return "evidence_boundary"
    return None


def _extract_paths(text: str) -> list[str]:
    paths: set[str] = set()
    for match in PATH_RE.finditer(text):
        path = match.group(0).rstrip(".,);:")
        while path.endswith("..."):
            path = path[:-3]
        paths.add(path)
    return sorted(paths)


def _looks_like_symbol(value: str) -> bool:
    if "/" in value:
        return False
    if REQ_ID_RE.fullmatch(value):
        return False
    if value.startswith("Status: "):
        return False
    if "=" in value and " " not in value:
        return False
    if re.fullmatch(r"(?:make|cargo|python3?) [A-Za-z0-9_.:/+-]+", value):
        return False
    if len(value) > 120:
        return False
    if re.fullmatch(r"[A-Za-z0-9_.:+-]+", value) and "::" not in value and " " not in value:
        return False
    return bool(re.search(r"[A-Za-z_]", value))


def _extract_symbols(row_type: str, cells: list[str]) -> list[str]:
    evidence_cell = cells[0] if row_type == "implementation" else cells[1]
    symbols: set[str] = set()
    for value in BACKTICK_RE.findall(evidence_cell):
        normalized = value.strip()
        if _looks_like_symbol(normalized):
            symbols.add(normalized)
    return sorted(symbols)


def _symbol_needles(symbol: str) -> list[str]:
    needles = [symbol]
    for suffix in ("(...)", "(...);", " { ... }", "..."):
        if suffix in symbol:
            needles.append(symbol.replace(suffix, ""))
    if symbol.startswith("pub "):
        needles.append(symbol.removeprefix("pub "))
    if "::" in symbol:
        last = symbol.rsplit("::", 1)[-1]
        needles.extend([last, last.replace("(...)", "")])
    if symbol.startswith("#[") and "]" in symbol:
        needles.append(symbol)
    return sorted({needle.strip() for needle in needles if needle.strip()})


def _path_exists(repo_root: Path, path: str) -> bool:
    return (repo_root / path).exists()


def _symbol_exists(repo_root: Path, symbol: str, paths: list[str]) -> bool:
    file_paths = [repo_root / path for path in paths if (repo_root / path).is_file()]
    if not file_paths:
        return True

    needles = _symbol_needles(symbol)
    for path in file_paths:
        text = path.read_text(encoding="utf-8", errors="replace")
        for needle in needles:
            if needle in text:
                return True
    return False


def _validate_status(
    row_type: str, line: int, verification_cell: str, diagnostics: list[Diagnostic]
) -> tuple[str, list[str]]:
    flags: list[str] = []
    match = STATUS_RE.match(verification_cell)
    if not match:
        flags.append("invalid_status_shape")
        diagnostics.append(
            Diagnostic(
                line,
                "invalid-status-shape",
                "verification/status cell must start with "
                "`Status: <canonical_status>. <Plain supporting sentence.>`",
            )
        )
        #explicit_unknown = re.search(r"\bStatus:\s+unknown\b", verification_cell)
        return ("unknown", flags)

    status, sentence = match.groups()
    if status == "unknown":
        flags.append("invalid_status_token")
        diagnostics.append(
            Diagnostic(line, "invalid-status-token", "`unknown` is reserved for tool output")
        )
    elif status not in CANONICAL_STATUSES:
        flags.append("invalid_status_token")
        diagnostics.append(
            Diagnostic(line, "invalid-status-token", f"unsupported status token `{status}`")
        )
    if not sentence.strip():
        flags.append("invalid_status_shape")
        diagnostics.append(
            Diagnostic(line, "invalid-status-shape", "status sentence must be non-empty")
        )
    if row_type == "evidence_boundary" and status != "boundary_only":
        flags.append("invalid_boundary_status")
        diagnostics.append(
            Diagnostic(
                line,
                "invalid-boundary-status",
                "evidence-boundary rows must use `Status: boundary_only`",
            )
        )
    return status, flags


def _parse_matrix(
    matrix_path: Path, known_requirements: dict[str, Requirement]
) -> tuple[list[TraceRow], list[Diagnostic]]:
    diagnostics: list[Diagnostic] = []
    rows: list[TraceRow] = []
    current_type: str | None = None
    next_separator = False

    for line_no, line in enumerate(_read_lines(matrix_path), start=1):
        cells = _split_table_row(line)
        if not cells:
            current_type = None
            next_separator = False
            continue

        header_type = _row_type(cells)
        if header_type is not None:
            current_type = header_type
            next_separator = True
            continue
        if next_separator and _is_separator(cells):
            next_separator = False
            continue
        if current_type is None:
            continue

        expected_columns = 3 if current_type == "implementation" else 4
        flags: list[str] = []
        if len(cells) != expected_columns:
            flags.append("invalid_table_shape")
            diagnostics.append(
                Diagnostic(
                    line_no,
                    "invalid-table-shape",
                    f"{current_type} row has {len(cells)} columns; expected {expected_columns}",
                )
            )

        if len(cells) < expected_columns:
            cells = cells + [""] * (expected_columns - len(cells))
        elif len(cells) > expected_columns:
            cells = cells[: expected_columns - 1] + [" | ".join(cells[expected_columns - 1 :])]

        req_cell = cells[1] if current_type == "implementation" else cells[0]
        verification_cell = cells[2]
        requirement_ids = sorted(set(REQ_ID_RE.findall(req_cell)))
        paths = _extract_paths(" ".join(cells))
        symbols = _extract_symbols(current_type, cells)

        status, status_flags = _validate_status(
            current_type, line_no, verification_cell, diagnostics
        )
        flags.extend(status_flags)

        if current_type == "evidence_boundary" and not cells[3].strip():
            flags.append("missing_boundary")
            diagnostics.append(
                Diagnostic(line_no, "missing-boundary", "Boundary column must be non-empty")
            )

        for req_id in requirement_ids:
            if req_id not in known_requirements:
                flags.append("unresolved_requirement")
                diagnostics.append(
                    Diagnostic(line_no, "unresolved-requirement", f"{req_id} has no source header")
                )

        for path in paths:
            if not _path_exists(REPO_ROOT, path):
                flags.append("missing_path")
                diagnostics.append(
                    Diagnostic(line_no, "missing-path", f"{path} does not exist")
                )

        for symbol in symbols:
            if not _symbol_exists(REPO_ROOT, symbol, paths):
                flags.append("missing_symbol")
                diagnostics.append(
                    Diagnostic(
                        line_no,
                        "missing-symbol",
                        f"{symbol!r} was not found in row file targets",
                    )
                )

        rows.append(
            TraceRow(
                row_id=f"trace-row-{line_no:04d}",
                row_type=current_type,
                line=line_no,
                columns=cells,
                requirement_ids=requirement_ids,
                paths=paths,
                symbols=symbols,
                status=status,
                flags=sorted(set(flags)),
            )
        )

    diagnostics.sort(key=lambda item: (item.line, item.code, item.message))
    return rows, diagnostics


def _links(rows: list[TraceRow]) -> list[dict[str, object]]:
    links: list[dict[str, object]] = []
    for row in rows:
        for req_id in row.requirement_ids:
            links.append(
                {
                    "type": "trace_row_satisfies_requirement",
                    "from": row.row_id,
                    "to": req_id,
                    "status": row.status,
                    "line": row.line,
                }
            )
        for path in row.paths:
            links.append(
                {
                    "type": "trace_row_references_path",
                    "from": row.row_id,
                    "to": path,
                    "status": row.status,
                    "line": row.line,
                }
            )
        for symbol in row.symbols:
            links.append(
                {
                    "type": "trace_row_references_symbol",
                    "from": row.row_id,
                    "to": symbol,
                    "status": row.status,
                    "line": row.line,
                }
            )
    return sorted(links, key=lambda item: (str(item["from"]), str(item["type"]), str(item["to"])))


def _write_projection(
    output_path: Path,
    hlr_path: Path,
    llr_path: Path,
    matrix_path: Path,
    requirements: dict[str, Requirement],
    rows: list[TraceRow],
    diagnostics: list[Diagnostic],
) -> None:
    projection = {
        "schema": "precision-replay.trace_projection.v1",
        "inputs": {
            "hlr": _repo_rel(hlr_path),
            "llr": _repo_rel(llr_path),
            "matrix": _repo_rel(matrix_path),
        },
        "canonical_statuses": sorted(CANONICAL_STATUSES),
        "requirements": [
            requirements[key].as_dict() for key in sorted(requirements)
        ],
        "trace_rows": [row.as_dict() for row in rows],
        "links": _links(rows),
        "validation": {
            "ok": not diagnostics,
            "diagnostics": [diagnostic.as_dict() for diagnostic in diagnostics],
        },
    }
    output_path.write_text(
        json.dumps(projection, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def validate(
    hlr_path: Path = DEFAULT_HLR,
    llr_path: Path = DEFAULT_LLR,
    matrix_path: Path = DEFAULT_MATRIX,
    output_path: Path = DEFAULT_OUTPUT,
) -> list[Diagnostic]:
    requirements = _extract_requirements(hlr_path)
    requirements.update(_extract_requirements(llr_path))
    rows, diagnostics = _parse_matrix(matrix_path, requirements)
    _write_projection(output_path, hlr_path, llr_path, matrix_path, requirements, rows, diagnostics)
    return diagnostics


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--hlr", type=Path, default=DEFAULT_HLR)
    parser.add_argument("--llr", type=Path, default=DEFAULT_LLR)
    parser.add_argument("--matrix", type=Path, default=DEFAULT_MATRIX)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()

    try:
        diagnostics = validate(args.hlr, args.llr, args.matrix, args.output)
    except ValidationError as exc:
        print(f"trace validation setup failed: {exc}", file=sys.stderr)
        return 2

    for diagnostic in diagnostics:
        print(diagnostic.render(args.matrix), file=sys.stderr)

    rel_output = args.output.relative_to(REPO_ROOT)
    if diagnostics:
        print(
            f"trace validation failed: {len(diagnostics)} diagnostic(s); wrote {rel_output}",
            file=sys.stderr,
        )
        return 1

    print(f"trace validation passed; wrote {rel_output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
