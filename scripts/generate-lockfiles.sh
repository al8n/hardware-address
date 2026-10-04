#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture=false
rust_185=false

for argument in "$@"; do
  case "$argument" in
    --fixture)
      fixture=true
      ;;
    --rust-1.85)
      rust_185=true
      ;;
    *)
      echo "usage: $0 [--fixture] [--rust-1.85]" >&2
      exit 2
      ;;
  esac
done

cd "$root"
export CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=fallback

cargo_with_toolchain() {
  if [[ "$rust_185" == true ]]; then
    cargo +1.85.0 "$@"
  else
    cargo "$@"
  fi
}

cargo_with_toolchain generate-lockfile
cargo_with_toolchain metadata --locked --format-version 1 >/dev/null

if [[ "$fixture" == true ]]; then
  cargo_with_toolchain generate-lockfile --manifest-path tests/fixtures/custom-address/Cargo.toml
  cargo_with_toolchain metadata --locked --format-version 1 --manifest-path tests/fixtures/custom-address/Cargo.toml >/dev/null
fi
