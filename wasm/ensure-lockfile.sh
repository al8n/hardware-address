#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ ! -f "$root/Cargo.lock" ]]; then
  "$root/scripts/generate-lockfiles.sh"
fi
