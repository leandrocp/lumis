#!/usr/bin/env python3
"""Run git-cliff with the breaking changes that apply to one release package."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import tomllib


def package_config(package):
    config = tomllib.loads(Path("cliff.toml").read_text())
    if package.startswith(("npm-", "hex-")):
        # #1702 changes Rust source requirements, not the npm or Hex APIs.
        subject = (
            r"bump tree-sitter 0\.27, wasmtime 48, and raise the MSRV to 1\.95 \(#1702\)"
        )
        config["git"]["commit_preprocessors"].insert(
            0, {"pattern": rf"^chore!: ({subject})", "replace": "chore: ${1}"}
        )
        config["git"]["commit_parsers"].insert(
            0, {"message": rf"^chore: {subject}", "group": "Dependencies"}
        )
    return config


def main():
    if len(sys.argv) < 2 or not sys.argv[1].startswith(("cargo-", "npm-", "hex-")):
        sys.exit("usage: release-cliff.py <cargo-*|npm-*|hex-*> [git-cliff arguments]")
    with tempfile.TemporaryDirectory(prefix="lumis-release-cliff-") as directory:
        config_path = Path(directory) / "cliff.yaml"
        config_path.write_text(json.dumps(package_config(sys.argv[1])))
        result = subprocess.run(
            ["git-cliff", "--config", str(config_path), *sys.argv[2:]], check=False
        )
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())
