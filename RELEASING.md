# Releasing hardware-address

The Rust workspace version in the root `Cargo.toml` is the source of truth for
the Rust, Python, and WASM packages. Keep `wasm/package.json` and the release
tag (`v<version>`) identical to it; run `scripts/check-release-version.py` both
without arguments and with the tag version before publishing.

## Release candidates and stable releases

1. Set the workspace version to an RC such as `1.0.0-rc.1`, update
   `wasm/package.json`, and record the release candidate in `CHANGELOG.md`.
2. Run the full CI and packaging checks, create and push `v1.0.0-rc.1`, then
   create a GitHub prerelease from that tag. Publish and smoke-test the RC on
   crates.io, PyPI, and npm; npm prereleases publish under the `next` dist-tag.
   The Python sdist smoke must rebuild the extracted source with a locked Cargo
   build and import `MacAddr` from the resulting wheel.
3. After all three registry smoke checks succeed, replace the prerelease version
   with the stable version, update the changelog, repeat the checks, and push the
   matching stable tag.

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

## Registry smoke checks

After a registry accepts a release, install or add the published version in a
fresh consumer project and exercise Rust, Python, and WASM package imports. Check
that npm selected `next` for an RC and `latest` for a stable release.
