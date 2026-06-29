.PHONY: ci-contract-check evidence-package-check

ci-contract-check:
	python3 scripts/check_ci_contract.py

evidence-package-check:
	python3 scripts/check_evidence_package.py docs/evidence/v0.1.0-rc1
