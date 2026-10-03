# Releasing hardware-address

The Rust workspace version in the root `Cargo.toml` is the source of truth for
the Rust, Python, and WASM packages. Keep `wasm/package.json` and the release
tag (`v<version>`) identical to it; run `scripts/check-release-version.py` both
without arguments and with the tag version before publishing.

## Release candidates and stable releases

1. Set the workspace version to an RC such as `1.0.0-rc.1`, update
   `wasm/package.json`, and record the release candidate in `CHANGELOG.md`.
2. Run the full CI and packaging checks, then create and push `v1.0.0-rc.1`.
   npm prereleases publish under the `next` dist-tag; PyPI handles prereleases
   according to its normal version semantics.
3. After RC smoke checks succeed, replace the prerelease version with the stable
   version, update the changelog, repeat the checks, and push the matching stable
   tag.

## crates.io trusted publishing

Before the first crates.io release, configure a trusted publisher for the
`hardware-address` crate on crates.io. Restrict it to this GitHub repository,
the `.github/workflows/crates.yml` workflow, and the GitHub environment named
`crates-io`. The workflow performs package and dry-run checks before requesting an OIDC token through
`rust-lang/crates-io-auth-action@v1`; its `crates-io` environment is the final
publication gate. Manual dispatch performs verification only; publication is
limited to pushed `v*` tags.

## Registry smoke checks

After a registry accepts a release, install or add the published version in a
fresh consumer project and exercise Rust, Python, and WASM package imports. Check
that npm selected `next` for an RC and `latest` for a stable release.
