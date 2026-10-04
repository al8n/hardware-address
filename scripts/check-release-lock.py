#!/usr/bin/env python3
"""Validate generated Cargo.lock metadata and tag-release provenance."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
RELEASE_LOCK = ROOT / ".release" / "cargo-lock.json"
PACKAGE_MANIFESTS = {
    ROOT / "Cargo.toml": "hardware-address",
    ROOT / "python" / "Cargo.toml": "python",
    ROOT / "wasm" / "Cargo.toml": "wasm",
}


def cargo_command(toolchain: str | None, *arguments: str) -> list[str]:
    command = ["cargo"]
    if toolchain is not None:
        command.append(f"+{toolchain}")
    return [*command, *arguments]


def cargo_version(toolchain: str) -> str:
    output = subprocess.run(
        cargo_command(toolchain, "--version"),
        check=True,
        cwd=ROOT,
        capture_output=True,
        text=True,
    ).stdout
    return output.split()[1]


def package_version(toolchain: str | None) -> str:
    output = subprocess.run(
        cargo_command(toolchain, "metadata", "--locked", "--format-version", "1"),
        check=True,
        cwd=ROOT,
        capture_output=True,
        text=True,
    ).stdout
    metadata = json.loads(output)
    packages = {Path(package["manifest_path"]).resolve(): package for package in metadata["packages"]}
    versions = {
        name: packages[manifest.resolve()]["version"] for manifest, name in PACKAGE_MANIFESTS.items()
    }
    if len(set(versions.values())) != 1:
        raise RuntimeError(f"Cargo package versions differ: {versions}")
    return next(iter(versions.values()))


def lock_sha256() -> str:
    lockfile = ROOT / "Cargo.lock"
    if not lockfile.is_file():
        raise RuntimeError("Cargo.lock is missing; run scripts/generate-lockfiles.sh first")
    return hashlib.sha256(lockfile.read_bytes()).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--toolchain", default="1.85.0")
    parser.add_argument("--release", action="store_true", help="require canonical release provenance")
    parser.add_argument("--expected-version")
    parser.add_argument(
        "--artifact",
        action="store_true",
        help="validate a downloaded lock artifact with the job's Cargo toolchain",
    )
    args = parser.parse_args()

    try:
        version = package_version(None if args.artifact else args.toolchain)
        cargo = None if args.artifact else cargo_version(args.toolchain)
        sha256 = lock_sha256()
        if args.expected_version and version != args.expected_version:
            raise RuntimeError(f"expected {args.expected_version}, found {version}")

        if args.release:
            expected = json.loads(RELEASE_LOCK.read_text())
            if expected.get("version") != version:
                raise RuntimeError("release lock version differs from generated metadata")
            if cargo is not None and expected.get("cargo_version") != cargo:
                raise RuntimeError("release lock Cargo version differs from generator")
            if expected.get("sha256") != sha256:
                raise RuntimeError("release lock SHA-256 differs from generated Cargo.lock")
    except (OSError, subprocess.CalledProcessError, json.JSONDecodeError, KeyError, RuntimeError) as error:
        print(f"release lock check failed: {error}", file=sys.stderr)
        return 1

    if cargo is None:
        print(f"downloaded Cargo.lock artifact is valid for {version}")
    else:
        print(f"generated Cargo.lock is valid for {version} with Cargo {cargo}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
