/**
 * A JavaScript project declares the languages it uses by depending on
 * `@lumis-sh/wasm-*` packages, and that declaration is the whole set — the same
 * way `Cargo.toml` features are in Rust and `lumis-lock.toml` is in Elixir.
 *
 * Before this, installing parsers pinned their versions but did not close the
 * set: a language an injection named and the project never installed was still
 * fetched. So JavaScript had an open set even when it looked closed, and Rust
 * did not.
 *
 * `declaresLanguages` is what decides, so these pin it directly: whether a
 * manifest is found, and what is treated as a declaration.
 */
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";

const { declaresLanguages, __resetDeclaredLanguages } =
  await import("../src/runtime/node-cache.js");

function project(manifest: unknown, nested = false): string {
  const root = mkdtempSync(join(tmpdir(), "lumis-declared-"));
  if (manifest !== undefined) {
    writeFileSync(join(root, "package.json"), JSON.stringify(manifest));
  }
  if (!nested) return root;

  const inner = join(root, "apps", "web");
  mkdirSync(inner, { recursive: true });
  return inner;
}

function inDirectory(directory: string): void {
  process.chdir(directory);
  __resetDeclaredLanguages();
}

const origin = process.cwd();

afterEach(() => {
  process.chdir(origin);
  __resetDeclaredLanguages();
});

describe("declaresLanguages", () => {
  it("is false when nothing depends on a parser", async () => {
    inDirectory(project({ name: "app", dependencies: { "@lumis-sh/lumis": "^1" } }));

    await expect(declaresLanguages()).resolves.toBe(false);
  });

  it("is true for a dependency", async () => {
    inDirectory(project({ name: "app", dependencies: { "@lumis-sh/wasm-rust": "^0.26" } }));

    await expect(declaresLanguages()).resolves.toBe(true);
  });

  // A bundle is a parser dependency like any other, and is how most projects
  // declare a set rather than listing a dozen packages.
  it("is true for a bundle", async () => {
    inDirectory(project({ dependencies: { "@lumis-sh/wasm-bundle-web": "^0.26" } }));

    await expect(declaresLanguages()).resolves.toBe(true);
  });

  it("counts optionalDependencies, which are shipped", async () => {
    inDirectory(project({ optionalDependencies: { "@lumis-sh/wasm-css": "^0.26" } }));

    await expect(declaresLanguages()).resolves.toBe(true);
  });

  // A library that tests against a parser has not told its consumers anything
  // about what they may load. This package is one: its own devDependencies list
  // two parsers, and counting them would close the set for its whole suite.
  it("ignores devDependencies, which are not shipped", async () => {
    inDirectory(project({ devDependencies: { "@lumis-sh/wasm-css": "^0.26" } }));

    await expect(declaresLanguages()).resolves.toBe(false);
  });

  // Searched upward like a lock is, so a command run inside a workspace package
  // still finds the manifest that installed the parsers.
  it("searches upward from the working directory", async () => {
    inDirectory(project({ dependencies: { "@lumis-sh/wasm-json": "^0.26" } }, true));

    await expect(declaresLanguages()).resolves.toBe(true);
  });

  it("is false when there is no manifest at all", async () => {
    inDirectory(project(undefined));

    await expect(declaresLanguages()).resolves.toBe(false);
  });

  // A manifest that is not readable JSON is not a declaration, and must not
  // stop the walk either: a broken file somewhere above should not decide this.
  it("ignores a manifest it cannot parse", async () => {
    const root = mkdtempSync(join(tmpdir(), "lumis-declared-"));
    writeFileSync(join(root, "package.json"), "{ not json");
    inDirectory(root);

    await expect(declaresLanguages()).resolves.toBe(false);
  });
});
