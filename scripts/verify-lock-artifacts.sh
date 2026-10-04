#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture=false

if [[ "${1:-}" == "--fixture" ]]; then
  fixture=true
elif [[ -n "${1:-}" ]]; then
  echo "usage: $0 [--fixture]" >&2
  exit 2
fi

cd "$root"
test -f Cargo.lock
if [[ "$fixture" == true ]]; then
  test -f tests/fixtures/custom-address/Cargo.lock
fi

python_bin="$(command -v python3 || command -v python || true)"
if [[ -z "$python_bin" ]]; then
  echo "Python is required to verify Cargo.lock artifacts" >&2
  exit 1
fi

if [[ "${GITHUB_REF_TYPE:-}" == "tag" ]]; then
  if [[ "${GITHUB_REF_NAME:-}" != v* ]]; then
    echo "tag lock verification requires a v-prefixed tag" >&2
    exit 1
  fi
  "$python_bin" scripts/check-release-lock.py --artifact --release \
    --expected-version "${GITHUB_REF_NAME#v}"
else
  "$python_bin" scripts/check-release-lock.py --artifact
fi

if [[ "$fixture" == true ]]; then
  cargo metadata --locked --format-version 1 \
    --manifest-path tests/fixtures/custom-address/Cargo.toml >/dev/null
fi
