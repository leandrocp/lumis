import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib
import unittest


ROOT = Path(__file__).resolve().parents[2]
TASKS = tomllib.loads((ROOT / "mise.toml").read_text())["tasks"]


class ReleaseOverridesTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="lumis-release-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        binary = shutil.which("git-cliff")
        self.assertIsNotNone(binary, "Run through mise run test-release-overrides")
        tools = self.root / "bin"
        tools.mkdir()
        launcher = tools / "git-cliff"
        launcher.write_text(f'#!/bin/sh\nexec "{binary}" --offline "$@"\n')
        launcher.chmod(0o755)
        self.env = dict(
            os.environ,
            MISE_TRUSTED_CONFIG_PATHS=str(self.root),
            PATH=f"{tools}:{os.environ['PATH']}",
        )
        tasks = {
            name: TASKS[name]
            for name in (
                "release-bump-overrides",
                "release-plan",
                "release-version",
                "release-prepare",
            )
        }
        tasks["release-packages"] = {
            "run": "printf 'npm-example|package\ncargo-example|crate\n'"
        }
        self.template = self.root / "notes.tera"
        self.template.write_text(
            "{{ version }}\n{% for commit in commits %}"
            "{{ commit.message }} breaking={{ commit.breaking }}"
            " group={{ commit.group }} scope={{ commit.scope }}\n{% endfor %}"
        )
        tasks["release-template"] = {"run": f"printf '%s' '{self.template}'"}
        config = "\n".join(
            f'[tasks."{name}"]\n'
            + "\n".join(
                f"{key} = {json.dumps(task[key])}"
                for key in ("run", "usage")
                if key in task
            )
            for name, task in tasks.items()
        )
        (self.root / "mise.toml").write_text(config)
        shutil.copy(ROOT / "cliff.toml", self.root / "cliff.toml")
        scripts = self.root / "packages/javascript/scripts"
        scripts.mkdir(parents=True)
        shutil.copy(ROOT / "packages/javascript/scripts/update-example-versions.mjs", scripts)
        example = self.root / "packages/javascript/example/examples"
        example.mkdir(parents=True)
        (example / "package.json").write_text(json.dumps({
            "dependencies": {"@example": "^0.4.7"},
        }))
        self.write_versions("0.4.7", "2.3.4")
        self.git("init", "-q")
        self.git("config", "user.name", "Release Test")
        self.git("config", "user.email", "release-test@example.invalid")
        self.commit("chore: initial")
        self.git("tag", "npm-example/v0.4.7")
        self.git("tag", "cargo-example/v2.3.4")

    def run_command(self, *args, check=True):
        result = subprocess.run(
            args, cwd=self.root, env=self.env, text=True,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        )
        if check:
            self.assertEqual(result.returncode, 0, result.stderr)
        return result

    def git(self, *args):
        return self.run_command("git", *args).stdout

    def commit(self, message):
        self.git("add", ".")
        self.git("commit", "-qm", message)

    def write_versions(self, npm, cargo):
        (self.root / "package").mkdir(exist_ok=True)
        (self.root / "crate").mkdir(exist_ok=True)
        (self.root / "package/package.json").write_text(json.dumps({"name": "@example", "version": npm}))
        (self.root / "crate/Cargo.toml").write_text(f'[package]\nversion = "{cargo}"\n')

    def breaking_change(self):
        for directory in ("package", "crate"):
            (self.root / directory / "api.txt").write_text("removed an argument\n")
        self.commit("feat(api)!: remove an argument\n\nBREAKING CHANGE: argument removed")

    def plan(self, overrides=None, check=True):
        args = ["mise", "run", "release-plan"]
        if overrides is not None:
            path = self.root / "overrides.json"
            path.write_text(json.dumps(overrides))
            args += ["--overrides", str(path)]
        return self.run_command(*args, check=check)

    def test_selections_are_package_scoped_and_persist_across_main_changes(self):
        self.breaking_change()
        self.assertEqual(self.plan().stdout.splitlines(), [
            "npm-example 0.5.0", "cargo-example 3.0.0",
        ])
        overrides = {"npm-example": "patch"}
        expected = ["npm-example 0.4.8", "cargo-example 3.0.0"]
        self.assertEqual(self.plan(overrides).stdout.splitlines(), expected)
        (self.root / "crate/next.txt").write_text("a later change\n")
        self.commit("fix: update Rust implementation")
        self.assertEqual(self.plan(overrides).stdout.splitlines(), expected)
        self.assertEqual(self.plan({
            "npm-example": "major", "cargo-example": "minor",
        }).stdout.splitlines(), ["npm-example 1.0.0", "cargo-example 2.4.0"])

    def test_selection_does_not_invent_a_release_or_reprepare_a_merged_one(self):
        self.assertEqual(self.plan({"npm-example": "patch"}).stdout, "")
        self.breaking_change()
        self.write_versions("0.4.8", "2.3.4")
        self.commit("chore(release): npm-example 0.4.8")
        self.assertEqual(self.plan({"npm-example": "patch"}).stdout.splitlines(), [
            "cargo-example 3.0.0",
        ])
        self.git("tag", "npm-example/v0.4.8")
        self.assertEqual(self.plan({"npm-example": "patch"}).stdout.splitlines(), [
            "cargo-example 3.0.0",
        ])

    def test_invalid_selections_fail(self):
        for overrides in ({"npm-example": "typo"}, {"npm-unknown": "patch"}):
            with self.subTest(overrides=overrides):
                self.assertNotEqual(self.plan(overrides, check=False).returncode, 0)

    def test_labels_ignore_forks_and_nonrelease_prs_and_reject_conflicts(self):
        pulls = self.root / "pulls.json"
        entries = [
            {"headRefName": "release/npm-example", "isCrossRepository": False,
             "labels": [{"name": "release:patch"}, {"name": "unrelated"}]},
            {"headRefName": "release/cargo-example", "isCrossRepository": True,
             "labels": [{"name": "release:major"}]},
            {"headRefName": "feature", "isCrossRepository": False,
             "labels": [{"name": "release:minor"}]},
        ]
        pulls.write_text(json.dumps(entries))
        result = self.run_command("mise", "run", "release-bump-overrides", str(pulls))
        self.assertEqual(json.loads(result.stdout), {"npm-example": "patch"})
        entries[0]["labels"].append({"name": "release:minor"})
        pulls.write_text(json.dumps(entries))
        result = self.run_command(
            "mise", "run", "release-bump-overrides", str(pulls), check=False,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Conflicting release bump labels", result.stderr)

    def test_patch_release_notes_do_not_claim_a_breaking_api(self):
        self.breaking_change()
        self.run_command("mise", "run", "release-prepare", "npm-example", "0.4.8", "--bump", "patch")
        notes = (self.root / "package/CHANGELOG.md").read_text()
        self.assertIn("npm-example/v0.4.8", notes)
        self.assertIn("remove an argument breaking=false group=Features scope=api", notes)
        self.assertNotIn("breaking=true", notes)
        self.run_command("mise", "run", "release-prepare", "npm-example", "0.5.0")
        notes = (self.root / "package/CHANGELOG.md").read_text()
        self.assertIn("npm-example/v0.5.0", notes)
        self.assertIn("remove an argument breaking=true group=Features scope=api", notes)
