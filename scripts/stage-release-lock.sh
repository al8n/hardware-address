#!/usr/bin/env bash
set -euo pipefail

tag="${1:?usage: $0 <tag>}"
workspace="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
lockfile="$workspace/Cargo.lock"
gh_bin="${GH_BIN:-gh}"
jq_bin="${JQ_BIN:-jq}"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if [[ ! -f "$lockfile" ]]; then
  echo "Cargo.lock is missing" >&2
  exit 1
fi

(cd "$workspace" && sha256sum Cargo.lock > "$tmpdir/Cargo.lock.sha256")
endpoint="repos/${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}/releases/tags/${tag}"
expected_assets=("Cargo.lock" "Cargo.lock.sha256")
download_dir="$tmpdir/download"
expected_prerelease=false
if [[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+-.+$ ]]; then
  expected_prerelease=true
fi
mkdir -p "$download_dir"

release_assets() {
  "$jq_bin" -r '.assets[].name' <<<"$release_json" | LC_ALL=C sort
}

validate_draft_release() {
  if [[ "$("$jq_bin" -r '.tag_name' <<<"$release_json")" != "$tag" ]]; then
    echo "existing release tag does not match $tag" >&2
    exit 1
  fi
  if [[ "$("$jq_bin" -r '.draft' <<<"$release_json")" != "true" ]]; then
    echo "refusing to alter published release $tag" >&2
    exit 1
  fi
  if [[ "$("$jq_bin" -r '.prerelease' <<<"$release_json")" != "$expected_prerelease" ]]; then
    echo "draft release $tag has an unexpected prerelease state" >&2
    exit 1
  fi
}

is_expected_asset() {
  [[ "$1" == "Cargo.lock" || "$1" == "Cargo.lock.sha256" ]]
}

validate_asset_subset() {
  local asset
  while IFS= read -r asset; do
    [[ -z "$asset" ]] && continue
    if ! is_expected_asset "$asset"; then
      echo "draft release $tag has unexpected lock asset $asset" >&2
      exit 1
    fi
  done < <(release_assets)
}

has_asset() {
  release_assets | grep -Fxq "$1"
}

asset_source() {
  case "$1" in
    Cargo.lock)
      printf '%s\n' "$lockfile"
      ;;
    Cargo.lock.sha256)
      printf '%s\n' "$tmpdir/Cargo.lock.sha256"
      ;;
  esac
}

verify_downloaded_asset() {
  local asset="$1"
  local source
  source="$(asset_source "$asset")"
  rm -f "$download_dir/$asset"
  "$gh_bin" release download "$tag" --pattern "$asset" --dir "$download_dir"
  if [[ ! -f "$download_dir/$asset" ]] || ! cmp -s "$source" "$download_dir/$asset"; then
    echo "draft release $tag has a mismatched $asset" >&2
    exit 1
  fi
}

if release_json="$("$gh_bin" api "$endpoint" 2>"$tmpdir/release-error")"; then
  validate_draft_release
else
  if ! grep -q 'HTTP 404' "$tmpdir/release-error"; then
    cat "$tmpdir/release-error" >&2
    exit 1
  fi
  if [[ "$expected_prerelease" == true ]]; then
    "$gh_bin" release create "$tag" --verify-tag --draft --prerelease --latest=false \
      --title "$tag" --notes "Generated Cargo.lock provenance assets."
  else
    "$gh_bin" release create "$tag" --verify-tag --draft --title "$tag" \
      --notes "Generated Cargo.lock provenance assets."
  fi
  release_json="$("$gh_bin" api "$endpoint")"
  validate_draft_release
fi

validate_asset_subset

for asset in "${expected_assets[@]}"; do
  if has_asset "$asset"; then
    verify_downloaded_asset "$asset"
  else
    source="$(asset_source "$asset")"
    "$gh_bin" release upload "$tag" "$source#$asset"
  fi
done

release_json="$("$gh_bin" api "$endpoint")"
validate_draft_release
assets="$(release_assets)"
if [[ "$assets" != $'Cargo.lock\nCargo.lock.sha256' ]]; then
  echo "draft release $tag has unexpected lock assets after reconciliation:" >&2
  printf '%s\n' "$assets" >&2
  exit 1
fi

for asset in "${expected_assets[@]}"; do
  verify_downloaded_asset "$asset"
done
(
  cd "$download_dir"
  sha256sum -c Cargo.lock.sha256
)

echo "staged verified Cargo.lock assets on draft release $tag"
