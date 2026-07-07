# Retained Raw ADC Capture Workflow

This note defines the minimal operator flow for retaining a bounded raw ADC witness artifact. It is an evidence-checking workflow only.

## Retained Files

Each retained artifact directory under `artifacts/witness/raw-adc/<capture-id>/` must contain:

- `raw_witness.txt`: raw UART witness capture output.
- `capture.txt`: deterministic monitor summary; this must match the checker's canonical expected text.
- `metadata.md`: retained artifact boundary, including non-lossless/non-contiguous limitations when applicable, plus raw ADC envelope metadata when judgment is retained.
- `judgment.txt`: deterministic raw ADC envelope judgment output when a retained envelope judgment is claimed.

## Capture And Check

Capture raw UART witness output into `raw_witness.txt`, generate `capture.txt` with the raw ADC monitor summary, then check the retained directory:

```bash
python3 tools/check_raw_adc_capture.py artifacts/witness/raw-adc/<capture-id>
```

Passing output is the canonical summary. Failure means the artifact is not admissible as retained raw ADC evidence until the retained files or boundary notes are corrected.

Counted malformed UART witness rows may pass only when reported in `capture.txt` and bounded by `metadata.md`. Malformed `raw_adc` tokens, summary mismatch, missing files, missing boundary text, or unexplained non-contiguous samples fail.

## Envelope Judgment

To retain a raw ADC envelope judgment, declare the following metadata fields in `metadata.md`:

- `context_id`
- `envelope_id`
- `raw_adc_min`
- `raw_adc_max`
- `min_sample_count`
- `allow_malformed_witness_lines`

Generate and retain the canonical judgment output, then check it:

```bash
python3 tools/check_raw_adc_envelope.py --write artifacts/witness/raw-adc/<capture-id>
python3 tools/check_raw_adc_envelope.py artifacts/witness/raw-adc/<capture-id>
```

Envelope judgment applies only to raw ADC observations admitted by the retained capture checker in the declared context. Missing context produces `not_applicable`; missing limits, too few admitted samples, capture non-admission, or disallowed malformed witness rows produce `inconclusive`; any admitted sample outside the declared raw ADC minimum or maximum produces `fail`; and only admitted samples that meet the declared count and limits produce `pass`.

Tolerated malformed witness rows remain outside admitted observations. The retained `judgment.txt` format is deterministic key/value text for later declared-context comparison work, but this workflow does not implement that comparison.

## Boundary

This workflow does not claim ADC electrical correctness, UART losslessness, precise timing behavior, stimulus adequacy, baseline-vs-stimulus comparison, trend judgment, delta judgment, context comparison, board qualification, release readiness, hardware certification, or generalized board-family support.
