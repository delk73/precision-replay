---
title: Replay System Story
active_phase: "domain_boundary_analysis"
gates:
  narrative_ingested: true
  domain_boundary_approved: false
  lexicon_alignment_approved: false
  hlr_baseline_approved: false
  llr_baseline_approved: false
  traceability_matrix_approved: false
---

# Replay System Story

The replay system ingests recorded events and replays them deterministically
for analysis. It preserves event ordering, reports malformed input, and keeps
the source recording distinct from derived replay outputs.
