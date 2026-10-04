# Releasing hardware-address

The Rust workspace version in the root `Cargo.toml` is the source of truth for
the Rust, Python, and WASM packages. Keep `wasm/package.json` and the release
tag (`v<version>`) identical to it; run `scripts/check-release-version.py` both
without arguments and with the tag version before publishing. Cargo lockfiles are
generated in CI and intentionally are not tracked by Git.

## Release candidates and stable releases

1. Set the workspace version to an RC such as `1.0.0-rc.1`, update
   `wasm/package.json`, and record the release candidate in `CHANGELOG.md`.
2. Run the full CI and packaging checks, create and push `v1.0.0-rc.1`. The
   crates workflow stages a draft GitHub release containing verified lock assets.
   Publish and smoke-test the RC on crates.io, PyPI, and npm; npm prereleases
   publish under the `next` dist-tag.
   The Python sdist smoke must rebuild the extracted source with a locked Cargo
   build and import `MacAddr` from the resulting wheel.
3. After all three registry smoke checks succeed, replace the prerelease version
   with the stable version, update the changelog, repeat the checks, and push the
   matching stable tag. Finalize the existing draft release after all tag
   workflows and registry smoke checks succeed; do not create a replacement
   release with `gh release create`.

## Trusted publishing

Protect the GitHub environments `crates-io`, `npm`, and `pypi` with `al8n`
approval. Configure each registry publisher to match this repository and its
release workflow: crates.io uses `.github/workflows/crates.yml` with environment
`crates-io`; npm uses `.github/workflows/wasm.yml` with environment `npm`; and
PyPI uses `.github/workflows/python.yml` with environment `pypi`. All three
registry workflows publish through GitHub OIDC.

The crates.io workflow performs package and dry-run checks before requesting its
temporary token through `rust-lang/crates-io-auth-action@v1`. Manual crates.io
dispatch performs verification only; publication is limited to pushed `v*` tags.
The npm release job publishes the Node 24-tested artifact with npm 11.15.0 or
later. Old `NPM_TOKEN` and `PYPI_API_TOKEN` secrets may be removed only after the
first successful RC publication through trusted publishing.

## Cargo.lock provenance

CI invokes `scripts/generate-lockfiles.sh --rust-1.85` with Cargo 1.85.0 and
the resolver fallback policy. Local helpers use the caller's Cargo when they
need to regenerate an ignored lock. `.release/cargo-lock.json` records the
exact Cargo.lock asset of the current published release; its SHA-256 is not a
development-lock checksum. A release-preparation PR must update its version and
SHA-256 from the newly generated candidate lock before a matching tag is pushed.
Tag workflows compare that candidate against the provenance file, then stage
`Cargo.lock` and `Cargo.lock.sha256` on the draft release. crates.io still
includes Cargo's automatically generated minimized lockfile in the published
`.crate`; it does not come from repository tracking.

## Registry smoke checks

After a registry accepts a release, install or add the published version in a
fresh consumer project and exercise Rust, Python, and WASM package imports. Check
that npm selected `next` for an RC and `latest` for a stable release.
