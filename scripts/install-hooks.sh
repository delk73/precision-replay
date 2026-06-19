#!/usr/bin/env bash
set -eu

git config core.hooksPath .githooks
chmod +x .githooks/commit-msg
echo "Configured git hooks path: .githooks"
