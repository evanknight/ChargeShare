#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
current=$(git config --get core.hooksPath || true)
if [[ -n "$current" && "$current" != '.githooks' ]]; then
  printf '%s\n' 'An existing hooksPath is configured. Review it manually before installing these hooks.' >&2
  exit 1
fi
git config --local core.hooksPath .githooks
printf '%s\n' 'Installed repository-local pre-commit and pre-push hooks.'
