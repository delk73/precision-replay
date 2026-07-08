.PHONY: ci-contract-check evidence-package-check replay-check

ci-contract-check:
	python3 scripts/check_ci_contract.py

evidence-package-check:
	python3 scripts/check_evidence_package.py docs/evidence/v0.1.0-rc1

replay-check:
	python3 tools/check_replay.py artifacts/replay/math-i64f64-v1
