import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { afterEach, describe, expect, it } from "vitest";
import dracula from "@lumis-sh/themes/dracula";
import { packageRoots } from "../src/runtime/package-roots.js";
import { available, closest, get } from "../src/themes/node.js";

const directories: string[] = [];

afterEach(() => {
  packageRoots.clear();
  for (const directory of directories.splice(0))
    rmSync(directory, { recursive: true, force: true });
});

function install(data: unknown) {
  const root = mkdtempSync(join(tmpdir(), "lumis-installed-themes-"));
  directories.push(root);
  const pack = join(root, "node_modules/@lumis-sh/themes-copyleft");
  mkdirSync(pack, { recursive: true });
  writeFileSync(
    join(pack, "package.json"),
    JSON.stringify({
      name: "@lumis-sh/themes-copyleft",
      exports: { "./lumis-themes.json": "./lumis-themes.json" },
    }),
  );
  writeFileSync(join(pack, "lumis-themes.json"), JSON.stringify(data));
  packageRoots.add(pathToFileURL(join(root, "package.json")).href);
}

const optional = {
  name: "application_optional",
  appearance: "dark",
  highlights: {
    normal: { bg: "#000000", fg: "#ffffff" },
    keyword: { fg: "#ff0000" },
    string: { fg: "#00ff00" },
    comment: { fg: "#0000ff" },
  },
};

describe("installed optional themes", () => {
  it("keeps catalog state independent of a customized per-theme import", () => {
    const normal = dracula.highlights.normal;
    if (!normal) throw new Error("missing default theme style");
    const original = normal.fg;
    normal.fg = "#123456";
    try {
      expect(get("dracula")?.highlights.normal?.fg).toBe(original);
    } finally {
      normal.fg = original;
    }
  });

  it("discovers fixed manifests from known package roots without registration", () => {
    install({ schema_version: 1, pack: "copyleft", themes: [optional] });
    expect(get(optional.name)).toEqual(optional);
    expect(available()).toContainEqual({ name: optional.name, appearance: optional.appearance });
    expect(closest({ background: "#000000", foreground: "#ffffff" })?.name).toBe(optional.name);
    const first = get(optional.name);
    if (!first) throw new Error("missing optional theme");
    first.highlights.normal = { fg: "#123456" };
    expect(get(optional.name)?.highlights.normal?.fg).toBe("#ffffff");
  });

  it("rejects installed manifests instead of silently ignoring invalid data", () => {
    install({ schema_version: 2, pack: "copyleft", themes: [optional] });
    expect(() => available()).toThrow(TypeError);
  });

  it("rejects a pack that shadows a default theme", () => {
    install({ schema_version: 1, pack: "copyleft", themes: [{ ...optional, name: "dracula" }] });
    expect(() => get("dracula")).toThrow("Duplicate installed theme name");
  });
});
