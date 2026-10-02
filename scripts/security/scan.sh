#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
cd "$root"
node scripts/security/check-paths.mjs
if [[ -x "$root/.tools/bin/gitleaks" ]]; then
  scanner="$root/.tools/bin/gitleaks"
elif command -v gitleaks >/dev/null 2>&1; then
  scanner=$(command -v gitleaks)
else
  printf '%s\n' 'Gitleaks is required. Run bash scripts/security/install-gitleaks.sh first.' >&2
  exit 1
fi
if [[ "$("$scanner" version)" != '8.30.1' ]]; then
  printf '%s\n' 'Use pinned Gitleaks 8.30.1; run bash scripts/security/install-gitleaks.sh.' >&2
  exit 1
fi
common=(--config "$root/.gitleaks.toml" --redact=100 --no-banner --no-color
  --ignore-gitleaks-allow --gitleaks-ignore-path /dev/null --max-decode-depth 5)
case "${1:-staged}" in
  staged) "$scanner" git "${common[@]}" --pre-commit --staged "$root" ;;
  history) "$scanner" git "${common[@]}" --log-opts='--all' "$root" ;;
  *) printf '%s\n' 'Usage: bash scripts/security/scan.sh [staged|history]' >&2; exit 2 ;;
esac
