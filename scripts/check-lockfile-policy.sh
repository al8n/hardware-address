#!/usr/bin/env bash
set -euo pipefail

tracked="$(git ls-files -- Cargo.lock tests/fixtures/custom-address/Cargo.lock)"
if [[ -n "$tracked" ]]; then
  echo "Cargo lockfiles must not be tracked:" >&2
  printf '%s\n' "$tracked" >&2
  exit 1
fi
