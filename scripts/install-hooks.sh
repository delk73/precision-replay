#!/usr/bin/env bash
set -eu

git config core.hooksPath .githooks
git config commit.template .gitmessage
chmod +x .githooks/commit-msg
echo "Configured git hooks path: .githooks"
echo "Configured commit template: .gitmessage"
