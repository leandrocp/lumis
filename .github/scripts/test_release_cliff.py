import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
UPGRADE = "chore!: bump tree-sitter 0.27, wasmtime 48, and raise the MSRV to 1.95 (#1702)"


class ReleaseCliffTest(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="lumis-release-test-")
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        shutil.copy(ROOT / "cliff.toml", self.root / "cliff.toml")
        self.command("git", "init", "--quiet")
        self.command("git", "config", "user.name", "Release test")
        self.command("git", "config", "user.email", "release@example.invalid")
        self.command("git", "config", "commit.gpgsign", "false")
        self.commit("feat: initial version")
        for tag in ("cargo-lumis/v0.16.1", "npm-lumis/v0.9.2", "hex-lumis/v0.10.1"):
            self.command("git", "-c", "tag.gpgsign=false", "tag", tag)
        self.commit(UPGRADE)

    def command(self, *args):
        return subprocess.run(
            args, cwd=self.root, text=True, capture_output=True, check=True
        ).stdout.strip()

    def commit(self, message):
        self.command("git", "commit", "--quiet", "--allow-empty", "-m", message)

    def cliff(self, package, *args):
        return self.command(
            sys.executable, str(ROOT / ".github/scripts/release-cliff.py"),
            package, "--tag-pattern", f"{package}/v[0-9].*", *args,
        )

    def test_rust_upgrade_is_breaking_only_for_cargo(self):
        expected = {"cargo-lumis": "0.17.0", "npm-lumis": "0.9.3", "hex-lumis": "0.10.2"}
        for package, version in expected.items():
            with self.subTest(package=package):
                expected_tag = f"{package}/v{version}"
                self.assertEqual(self.cliff(package, "--bumped-version"), expected_tag)
                [release] = json.loads(self.cliff(package, "--unreleased", "--context"))
                [commit] = release["commits"]
                self.assertEqual(commit["breaking"], package.startswith("cargo-"))
                self.assertIn("(#1702)", commit["message"])
                if not package.startswith("cargo-"):
                    self.assertEqual(commit["group"], "Dependencies")
                    original = self.command(
                        "git-cliff", "--config", "cliff.toml", "--tag-pattern",
                        f"{package}/v[0-9].*", "--bumped-version",
                    )
                    self.assertNotEqual(original, expected_tag)

    def test_public_api_breaking_changes_still_bump_npm_and_hex(self):
        self.commit("feat!: remove a public API")
        for package, version in {"npm-lumis": "0.10.0", "hex-lumis": "0.11.0"}.items():
            with self.subTest(package=package):
                self.assertEqual(self.cliff(package, "--bumped-version"), f"{package}/v{version}")
                [release] = json.loads(self.cliff(package, "--unreleased", "--context"))
                breaking = [commit["message"] for commit in release["commits"] if commit["breaking"]]
                self.assertEqual(breaking, ["remove a public API"])


if __name__ == "__main__":
    unittest.main()
