#!/usr/bin/env python3
"""Verify that Python wheel and source artifacts contain both license texts."""

from __future__ import annotations

import argparse
import glob
import tarfile
import zipfile
from pathlib import Path


REQUIRED_LICENSES = {"LICENSE-APACHE", "LICENSE-MIT"}


def artifact_members(path: Path) -> set[str]:
    if path.suffix == ".whl":
        with zipfile.ZipFile(path) as archive:
            return {Path(name).name for name in archive.namelist()}
    if path.name.endswith(".tar.gz"):
        with tarfile.open(path, "r:gz") as archive:
            return {Path(member.name).name for member in archive.getmembers()}
    raise ValueError(f"unsupported artifact type: {path}")


def expand_artifacts(patterns: list[Path], parser: argparse.ArgumentParser) -> list[Path]:
    artifacts: set[Path] = set()
    for pattern in patterns:
        matches = sorted(Path(path) for path in glob.glob(str(pattern)))
        if not matches:
            parser.error(f"artifact path or pattern matched no files: {pattern}")
        artifacts.update(matches)
    return sorted(artifacts, key=lambda path: str(path))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("artifacts", nargs="+", type=Path)
    args = parser.parse_args()

    for artifact in expand_artifacts(args.artifacts, parser):
        members = artifact_members(artifact)
        missing = REQUIRED_LICENSES - members
        if missing:
            parser.error(f"{artifact} is missing: {', '.join(sorted(missing))}")
        print(f"{artifact}: includes both license texts")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
