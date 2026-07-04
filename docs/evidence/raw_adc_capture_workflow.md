# Retained Raw ADC Capture Workflow

This note defines the minimal operator flow for retaining a bounded raw ADC witness artifact. It is an evidence-checking workflow only.

## Retained Files

Each retained artifact directory under `artifacts/witness/raw-adc/<capture-id>/` must contain:

- `raw_witness.txt`: raw UART witness capture output.
- `capture.txt`: deterministic monitor summary; this must match the checker's canonical expected text.
- `metadata.md`: retained artifact boundary, including non-lossless/non-contiguous limitations when applicable.

## Capture And Check

Capture raw UART witness output into `raw_witness.txt`, generate `capture.txt` with the raw ADC monitor summary, then check the retained directory:

```bash
python3 tools/check_raw_adc_capture.py artifacts/witness/raw-adc/<capture-id>
```

Passing output is the canonical summary. Failure means the artifact is not admissible as retained raw ADC evidence until the retained files or boundary notes are corrected.

Counted malformed UART witness rows may pass only when reported in `capture.txt` and bounded by `metadata.md`. Malformed `raw_adc` tokens, summary mismatch, missing files, missing boundary text, or unexplained non-contiguous samples fail.

## Boundary

This workflow does not claim ADC electrical correctness, UART losslessness, precise timing behavior, stimulus adequacy, board qualification, release readiness, hardware certification, or generalized board-family support.
