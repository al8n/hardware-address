#!/usr/bin/env bash
set -euo pipefail

tag="${1:?usage: $0 <tag>}"
workspace="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
lockfile="$workspace/Cargo.lock"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if [[ ! -f "$lockfile" ]]; then
  echo "Cargo.lock is missing" >&2
  exit 1
fi

(cd "$workspace" && sha256sum Cargo.lock > "$tmpdir/Cargo.lock.sha256")
expected_sha="$(awk '{print $1}' "$tmpdir/Cargo.lock.sha256")"
endpoint="repos/${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}/releases/tags/${tag}"

if release_json="$(gh api "$endpoint" 2>"$tmpdir/release-error")"; then
  if [[ "$(jq -r '.tag_name' <<<"$release_json")" != "$tag" ]]; then
    echo "existing release tag does not match $tag" >&2
    exit 1
  fi
  if [[ "$(jq -r '.draft' <<<"$release_json")" != "true" ]]; then
    echo "refusing to alter published release $tag" >&2
    exit 1
  fi
else
  if ! grep -q 'HTTP 404' "$tmpdir/release-error"; then
    cat "$tmpdir/release-error" >&2
    exit 1
  fi
  gh release create "$tag" --verify-tag --draft --title "$tag" --notes "Generated Cargo.lock provenance assets."
  gh release upload "$tag" "$lockfile#Cargo.lock" "$tmpdir/Cargo.lock.sha256#Cargo.lock.sha256"
  release_json="$(gh api "$endpoint")"
fi

assets="$(jq -r '.assets[].name' <<<"$release_json" | sort)"
if [[ "$assets" != $'Cargo.lock\nCargo.lock.sha256' ]]; then
  echo "draft release $tag has unexpected lock assets:" >&2
  printf '%s\n' "$assets" >&2
  exit 1
fi

mkdir -p "$tmpdir/download"
gh release download "$tag" --pattern Cargo.lock --pattern Cargo.lock.sha256 --dir "$tmpdir/download"
downloaded_sha="$(sha256sum "$tmpdir/download/Cargo.lock" | awk '{print $1}')"
recorded_sha="$(awk '{print $1}' "$tmpdir/download/Cargo.lock.sha256")"
if [[ "$downloaded_sha" != "$expected_sha" || "$recorded_sha" != "$expected_sha" ]]; then
  echo "draft release lock asset SHA does not match generated Cargo.lock" >&2
  exit 1
fi

echo "staged verified Cargo.lock assets on draft release $tag"
