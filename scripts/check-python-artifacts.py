#!/usr/bin/env python3
"""Verify that Python wheel and source artifacts contain both license texts."""

from __future__ import annotations

import argparse
import glob
import tarfile
import zipfile
from pathlib import Path, PurePosixPath


REQUIRED_LICENSES = {"LICENSE-APACHE", "LICENSE-MIT"}
REQUIRED_SDIST_FILES = {
    "Cargo.toml",
    "Cargo.lock",
    "src/lib.rs",
    "python/Cargo.toml",
    "python/src/lib.rs",
    "pyproject.toml",
    *REQUIRED_LICENSES,
}


def wheel_members(path: Path) -> set[str]:
    with zipfile.ZipFile(path) as archive:
        return {Path(name).name for name in archive.namelist()}


def sdist_members(path: Path) -> set[str]:
    with tarfile.open(path, "r:gz") as archive:
        members = [PurePosixPath(member.name) for member in archive.getmembers() if member.isfile()]
    roots = {member.parts[0] for member in members if member.parts}
    if len(roots) != 1:
        raise ValueError(f"source distribution must have one top-level directory: {path}")
    return {str(PurePosixPath(*member.parts[1:])) for member in members if len(member.parts) > 1}


def validate_artifact(path: Path) -> None:
    if path.suffix == ".whl":
        missing = REQUIRED_LICENSES - wheel_members(path)
        if missing:
            raise ValueError(f"{path} is missing: {', '.join(sorted(missing))}")
        return
    if path.name.endswith(".tar.gz"):
        missing = REQUIRED_SDIST_FILES - sdist_members(path)
        if missing:
            raise ValueError(f"{path} is missing: {', '.join(sorted(missing))}")
        return
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
        try:
            validate_artifact(artifact)
        except (OSError, tarfile.TarError, ValueError, zipfile.BadZipFile) as error:
            parser.error(str(error))
        print(f"{artifact}: layout and license texts are valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
