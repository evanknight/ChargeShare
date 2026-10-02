#!/usr/bin/env bash
set -euo pipefail

# Checksums are pinned from the official v8.30.1 release's checksums file:
# https://github.com/gitleaks/gitleaks/releases/tag/v8.30.1
version=8.30.1
root=$(git rev-parse --show-toplevel)
case "$(uname -s):$(uname -m)" in
  Linux:x86_64) platform=linux_x64; digest=551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb ;;
  Linux:aarch64|Linux:arm64) platform=linux_arm64; digest=e4a487ee7ccd7d3a7f7ec08657610aa3606637dab924210b3aee62570fb4b080 ;;
  Darwin:x86_64) platform=darwin_x64; digest=dfe101a4db2255fc85120ac7f3d25e4342c3c20cf749f2c20a18081af1952709 ;;
  Darwin:arm64) platform=darwin_arm64; digest=b40ab0ae55c505963e365f271a8d3846efbc170aa17f2607f13df610a9aeb6a5 ;;
  *) printf '%s\n' 'Unsupported platform. Install official Gitleaks 8.30.1 manually.' >&2; exit 1 ;;
esac
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
archive="gitleaks_${version}_${platform}.tar.gz"
curl --fail --silent --show-error --location --proto '=https' --tlsv1.2 \
  --retry 2 --connect-timeout 15 --max-time 120 \
  "https://github.com/gitleaks/gitleaks/releases/download/v${version}/${archive}" \
  --output "$work/$archive"
if command -v sha256sum >/dev/null 2>&1; then
  actual=$(sha256sum "$work/$archive" | cut -d ' ' -f 1)
else
  actual=$(shasum -a 256 "$work/$archive" | cut -d ' ' -f 1)
fi
if [[ "$actual" != "$digest" ]]; then
  printf '%s\n' 'Gitleaks checksum mismatch; refusing to install.' >&2
  exit 1
fi
tar -xzf "$work/$archive" -C "$work" gitleaks
mkdir -p "$root/.tools/bin"
install -m 0755 "$work/gitleaks" "$root/.tools/bin/gitleaks"
"$root/.tools/bin/gitleaks" version
