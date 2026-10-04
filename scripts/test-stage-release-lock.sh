#!/usr/bin/env bash
set -euo pipefail

mock_gh() {
  local command="${1:?}"
  shift
  printf '%q ' "$command" "$@" >> "$MOCK_GH_LOG"
  printf '\n' >> "$MOCK_GH_LOG"

  case "$command" in
    api)
      if [[ ! -f "$MOCK_GH_DIR/exists" ]]; then
        echo "HTTP 404" >&2
        exit 1
      fi
      local first=true
      local asset
      printf '{"tag_name":"%s","draft":%s,"prerelease":%s,"assets":[' \
        "$(<"$MOCK_GH_DIR/tag")" "$(<"$MOCK_GH_DIR/draft")" \
        "$(<"$MOCK_GH_DIR/prerelease")"
      shopt -s nullglob
      for asset in "$MOCK_GH_DIR/assets"/*; do
        if [[ "$first" == false ]]; then
          printf ','
        fi
        first=false
        printf '{"name":"%s"}' "$(basename "$asset")"
      done
      printf ']}\n'
      ;;
    release)
      local action="${1:?}"
      shift
      case "$action" in
        create)
          local prerelease=false
          local argument
          for argument in "$@"; do
            if [[ "$argument" == "--prerelease" ]]; then
              prerelease=true
            fi
          done
          printf '%s\n' "${1:?}" > "$MOCK_GH_DIR/tag"
          printf 'true\n' > "$MOCK_GH_DIR/draft"
          printf '%s\n' "$prerelease" > "$MOCK_GH_DIR/prerelease"
          touch "$MOCK_GH_DIR/exists"
          ;;
        upload)
          shift
          local spec source destination
          for spec in "$@"; do
            source="${spec%%#*}"
            destination="${spec#*#}"
            cp "$source" "$MOCK_GH_DIR/assets/$destination"
          done
          ;;
        download)
          shift
          local destination=""
          local pattern
          local -a patterns=()
          while (($#)); do
            case "$1" in
              --pattern)
                patterns+=("${2:?}")
                shift 2
                ;;
              --dir)
                destination="${2:?}"
                shift 2
                ;;
              *)
                echo "unexpected mock gh download argument $1" >&2
                exit 2
                ;;
            esac
          done
          mkdir -p "$destination"
          for pattern in "${patterns[@]}"; do
            cp "$MOCK_GH_DIR/assets/$pattern" "$destination/$pattern"
          done
          ;;
        *)
          echo "unexpected mock gh release action $action" >&2
          exit 2
          ;;
      esac
      ;;
    *)
      echo "unexpected mock gh command $command" >&2
      exit 2
      ;;
  esac
}

if [[ "$(basename "$0")" == "gh" ]]; then
  mock_gh "$@"
  exit
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
stage_script="$root/scripts/stage-release-lock.sh"
test_script="$root/scripts/test-stage-release-lock.sh"
test_root="$(mktemp -d "${TMPDIR:-/tmp}/stage-release-lock-test.XXXXXX")"
trap 'rm -rf "$test_root"' EXIT

fail() {
  echo "stage-release-lock test failed: $*" >&2
  exit 1
}

setup_case() {
  local name="$1"
  local tag="$2"
  local draft="$3"
  local prerelease="$4"
  local initial_assets="$5"
  local case_dir="$test_root/$name"

  mkdir -p "$case_dir/bin" "$case_dir/mock/assets" "$case_dir/expected"
  ln -s "$test_script" "$case_dir/bin/gh"
  printf '%s\n' "$tag" > "$case_dir/mock/tag"
  if [[ "$draft" != "missing" ]]; then
    printf '%s\n' "$draft" > "$case_dir/mock/draft"
    printf '%s\n' "$prerelease" > "$case_dir/mock/prerelease"
    touch "$case_dir/mock/exists"
  fi
  (cd "$root" && sha256sum Cargo.lock > "$case_dir/expected/Cargo.lock.sha256")
  cp "$root/Cargo.lock" "$case_dir/expected/Cargo.lock"

  case "$initial_assets" in
    none)
      ;;
    both)
      cp "$case_dir/expected/Cargo.lock" "$case_dir/mock/assets/Cargo.lock"
      cp "$case_dir/expected/Cargo.lock.sha256" "$case_dir/mock/assets/Cargo.lock.sha256"
      ;;
    lock)
      cp "$case_dir/expected/Cargo.lock" "$case_dir/mock/assets/Cargo.lock"
      ;;
    corrupt-lock)
      printf 'corrupt lock\n' > "$case_dir/mock/assets/Cargo.lock"
      ;;
    *)
      fail "unknown initial asset set $initial_assets"
      ;;
  esac

  printf '%s\n' "$case_dir"
}

run_case() {
  local case_dir="$1"
  local tag="$2"
  PATH="$case_dir/bin:$PATH" \
    MOCK_GH_DIR="$case_dir/mock" \
    MOCK_GH_LOG="$case_dir/gh.log" \
    GITHUB_REPOSITORY="al8n/hardware-address" \
    "$stage_script" "$tag" >"$case_dir/output" 2>&1
}

assert_exact_assets() {
  local case_dir="$1"
  local assets
  assets="$(find "$case_dir/mock/assets" -maxdepth 1 -type f -exec basename {} \; | LC_ALL=C sort)"
  [[ "$assets" == $'Cargo.lock\nCargo.lock.sha256' ]] || fail "unexpected assets: $assets"
  cmp -s "$case_dir/expected/Cargo.lock" "$case_dir/mock/assets/Cargo.lock" \
    || fail "Cargo.lock asset differs"
  cmp -s "$case_dir/expected/Cargo.lock.sha256" "$case_dir/mock/assets/Cargo.lock.sha256" \
    || fail "checksum asset differs"
  (cd "$case_dir/mock/assets" && sha256sum -c Cargo.lock.sha256 >/dev/null) \
    || fail "uploaded checksum cannot verify Cargo.lock"
}

assert_no_release_writes() {
  local case_dir="$1"
  if grep -Eq 'release (create|upload)' "$case_dir/gh.log"; then
    fail "draft validation wrote release state"
  fi
}

stable_case="$(setup_case stable v1.0.0 missing false none)"
run_case "$stable_case" v1.0.0
grep -Fq 'release create v1.0.0 --verify-tag --draft' "$stable_case/gh.log" \
  || fail "stable release did not create a draft"
if grep -Eq -- '--prerelease|--latest=false' "$stable_case/gh.log"; then
  fail "stable release was marked as prerelease"
fi
assert_exact_assets "$stable_case"

rc_case="$(setup_case rc v1.1.0-rc.1 missing true none)"
run_case "$rc_case" v1.1.0-rc.1
grep -Fq 'release create v1.1.0-rc.1 --verify-tag --draft --prerelease --latest=false' \
  "$rc_case/gh.log" || fail "RC release flags are incomplete"
assert_exact_assets "$rc_case"

reuse_case="$(setup_case reuse v1.0.0 true false both)"
run_case "$reuse_case" v1.0.0
assert_no_release_writes "$reuse_case"
assert_exact_assets "$reuse_case"

rc_reuse_case="$(setup_case rc-reuse v1.1.0-rc.1 true true both)"
run_case "$rc_reuse_case" v1.1.0-rc.1
assert_no_release_writes "$rc_reuse_case"
assert_exact_assets "$rc_reuse_case"

partial_case="$(setup_case partial v1.0.0 true false lock)"
run_case "$partial_case" v1.0.0
grep -Fq '#Cargo.lock.sha256' "$partial_case/gh.log" \
  || fail "partial draft did not upload the missing checksum"
if grep -Fq 'Cargo.lock#Cargo.lock' "$partial_case/gh.log"; then
  fail "partial draft overwrote the existing lock"
fi
assert_exact_assets "$partial_case"

mismatch_case="$(setup_case mismatch v1.0.0 true false corrupt-lock)"
if run_case "$mismatch_case" v1.0.0; then
  fail "mismatched draft asset was accepted"
fi
assert_no_release_writes "$mismatch_case"

rc_state_mismatch_case="$(setup_case rc-state-mismatch v1.1.0-rc.1 true false both)"
if run_case "$rc_state_mismatch_case" v1.1.0-rc.1; then
  fail "RC draft with a stable prerelease state was accepted"
fi
assert_no_release_writes "$rc_state_mismatch_case"

stable_state_mismatch_case="$(setup_case stable-state-mismatch v1.0.0 true true both)"
if run_case "$stable_state_mismatch_case" v1.0.0; then
  fail "stable draft with a prerelease state was accepted"
fi
assert_no_release_writes "$stable_state_mismatch_case"

published_case="$(setup_case published v1.0.0 false false both)"
if run_case "$published_case" v1.0.0; then
  fail "published release was accepted"
fi
assert_no_release_writes "$published_case"

echo "stage-release-lock mock tests passed"
