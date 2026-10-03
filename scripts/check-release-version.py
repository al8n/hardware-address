#!/usr/bin/env python3
"""Verify that release-facing package metadata agrees on one version."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PACKAGE_MANIFESTS = {
    ROOT / "Cargo.toml": "hardware-address",
    ROOT / "python" / "Cargo.toml": "python",
    ROOT / "wasm" / "Cargo.toml": "wasm",
}


def cargo_package_versions() -> dict[str, str]:
    result = subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version", "1"],
        check=True,
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    metadata = json.loads(result.stdout)
    manifests = {Path(package["manifest_path"]).resolve(): package for package in metadata["packages"]}

    versions = {}
    for manifest, package_name in PACKAGE_MANIFESTS.items():
        package = manifests.get(manifest.resolve())
        if package is None:
            raise RuntimeError(f"cargo metadata did not report {package_name}")
        versions[package_name] = package["version"]
    return versions


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("expected_version", nargs="?", help="expected release version, without a leading v")
    args = parser.parse_args()

    try:
        versions = cargo_package_versions()
        cargo_versions = set(versions.values())
        if len(cargo_versions) != 1:
            raise RuntimeError(f"Cargo package versions differ: {versions}")

        version = cargo_versions.pop()
        if args.expected_version is not None and version != args.expected_version:
            raise RuntimeError(f"expected {args.expected_version}, found {version}")

        package_json = json.loads((ROOT / "wasm" / "package.json").read_text())
        if package_json.get("version") != version:
            raise RuntimeError(
                f"wasm/package.json has {package_json.get('version')!r}, expected {version!r}"
            )
    except (OSError, subprocess.CalledProcessError, json.JSONDecodeError, RuntimeError) as error:
        print(f"release version check failed: {error}", file=sys.stderr)
        return 1

    print(f"release version {version} is consistent")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
