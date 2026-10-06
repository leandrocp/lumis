/**
 * The addon is selected by two copies of the same function: `nativeTargetFor`
 * in src/native-binding.ts, used when `@lumis-sh/lumis` resolves the addon
 * itself, and the copy in native/npm/meta/index.js, used when an install goes
 * through the `@lumis-sh/lumis-native` selector. That package ships alone, so
 * it cannot import the first one.
 *
 * A target added to one copy and not the other, or to both but with no
 * published package, silently drops those hosts onto `web-tree-sitter`. Nothing
 * else in the suite would notice, because CI runs on hosts the addon covers.
 */
import { beforeAll, describe, expect, it } from "vitest";
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { createRequire } from "node:module";
import {
  Language as TSLanguage,
  Parser,
  Query,
  type Node as TSNode,
  type QueryMatch,
} from "web-tree-sitter";
import { nativeTargetFor } from "../src/native-binding.js";

const packageDir = resolve(import.meta.dirname, "..");
const nativeNpmDir = join(packageDir, "native", "npm");

const PLATFORMS = ["darwin", "linux", "win32", "freebsd", "openbsd", "sunos", "aix", "android"];
const ARCHS = ["arm64", "x64", "ia32", "arm", "riscv64", "ppc64", "s390x", "loong64"];
const LIBCS = ["gnu", "musl"] as const;
const EXPECTED_NATIVE_TARGETS = [
  "darwin-arm64",
  "darwin-x64",
  "linux-arm64-gnu",
  "linux-arm64-musl",
  "linux-x64-gnu",
  "linux-x64-musl",
  "win32-arm64-msvc",
  "win32-x64-msvc",
];

const SELECTOR_BRANCH_QUERY = String.raw`
(function_declaration
  name: (identifier) @function.name
  body: (statement_block
    (if_statement
      condition: (parenthesized_expression
        (binary_expression
          left: (binary_expression
            left: (identifier) @platform.variable
            operator: "==="
            right: (string (string_fragment) @platform.value))
          operator: "&&"
          right: (call_expression
            function: (member_expression
              object: (array
                (string (string_fragment) @arch.value)+)
              property: (property_identifier) @includes)
            arguments: (arguments (identifier) @arch.variable))))
      consequence: (statement_block
        (return_statement (template_string) @branch.target))) @branch)
  (#eq? @function.name "nativeTargetFor")
  (#eq? @platform.variable "platform")
  (#eq? @includes "includes")
  (#eq? @arch.variable "arch"))
`;

type SelectorVariable = "platform" | "arch" | "libc";
type TemplatePart =
  | { kind: "literal"; value: string }
  | { kind: "variable"; name: SelectorVariable };

interface SelectorRule {
  platform: string;
  arch: string;
  target: TemplatePart[];
}

function targetsFrom(fn: typeof nativeTargetFor): string[] {
  const found = new Set<string>();
  for (const platform of PLATFORMS) {
    for (const arch of ARCHS) {
      for (const libc of LIBCS) {
        const target = fn(platform, arch, libc);
        if (target) found.add(target);
      }
    }
  }
  return [...found].sort();
}

function capture(match: QueryMatch, name: string): TSNode {
  const node = match.captures.find((candidate) => candidate.name === name)?.node;
  if (!node) throw new Error(`native selector query did not capture ${name}`);
  return node;
}

function selectorVariable(node: TSNode): SelectorVariable {
  if (node.text === "platform" || node.text === "arch" || node.text === "libc") {
    return node.text;
  }
  throw new Error(`native selector target uses unsupported variable ${node.text}`);
}

function templateParts(node: TSNode): TemplatePart[] {
  return node.namedChildren.map((child): TemplatePart => {
    if (child.type === "string_fragment") return { kind: "literal", value: child.text };
    const variable = child.type === "template_substitution" ? child.firstNamedChild : null;
    if (variable?.type !== "identifier") {
      throw new Error(`native selector target contains unsupported ${child.type}`);
    }
    return { kind: "variable", name: selectorVariable(variable) };
  });
}

/** Read the shipped selector as syntax without executing its package-loading entrypoint. */
async function selectorNativeTargetFor(): Promise<typeof nativeTargetFor> {
  const source = readFileSync(join(nativeNpmDir, "meta", "index.js"), "utf8");
  await Parser.init();
  const language = await TSLanguage.load(
    new Uint8Array(
      readFileSync(
        new URL("../../../../fixtures/test-parsers/tree-sitter-javascript.wasm", import.meta.url),
      ),
    ),
  );
  const parser = new Parser();
  parser.setLanguage(language);
  const tree = parser.parse(source);
  if (!tree) throw new Error("native selector JavaScript did not parse");

  const query = new Query(language, SELECTOR_BRANCH_QUERY);
  const rules: SelectorRule[] = query.matches(tree.rootNode).map((match) => ({
    platform: capture(match, "platform.value").text,
    arch: capture(match, "arch.value").text,
    target: templateParts(capture(match, "branch.target")),
  }));
  query.delete();
  tree.delete();
  parser.delete();

  return (platform, arch, libc) => {
    const rule = rules.find(
      (candidate) => candidate.platform === platform && candidate.arch === arch,
    );
    const variables: Record<SelectorVariable, string> = { platform, arch, libc };
    return rule?.target
      .map((part) => (part.kind === "literal" ? part.value : variables[part.name]))
      .join("");
  };
}

function expectCompleteTargetCorpus(targets: readonly string[]): void {
  expect(targets).toHaveLength(EXPECTED_NATIVE_TARGETS.length);
  expect(targets).toEqual(EXPECTED_NATIVE_TARGETS);
}

const publishedTargets = readdirSync(nativeNpmDir)
  .filter((entry) => entry !== "meta")
  .sort();
let selectorTargetFor: typeof nativeTargetFor;

beforeAll(async () => {
  selectorTargetFor = await selectorNativeTargetFor();
});

describe("native target selection", () => {
  it("covers every published platform package", () => {
    expectCompleteTargetCorpus(targetsFrom(nativeTargetFor));
    expectCompleteTargetCorpus(publishedTargets);
  });

  it("agrees with the @lumis-sh/lumis-native selector", () => {
    const selectorTargets = targetsFrom(selectorTargetFor);
    expectCompleteTargetCorpus(selectorTargets);
    expect(selectorTargets).toEqual(targetsFrom(nativeTargetFor));
  });

  it("rejects an incomplete native-target corpus", () => {
    const withoutWindowsArm64 = EXPECTED_NATIVE_TARGETS.filter(
      (target) => target !== "win32-arm64-msvc",
    );
    expect(() => {
      expectCompleteTargetCorpus(withoutWindowsArm64);
    }).toThrow();
  });

  it("resolves each host to the package that declares it", () => {
    const require = createRequire(import.meta.url);
    for (const target of publishedTargets) {
      const manifest = require(join(nativeNpmDir, target, "package.json")) as {
        name: string;
        os: string[];
        cpu: string[];
        libc?: string[];
      };
      expect(manifest.name).toBe(`@lumis-sh/lumis-native-${target}`);

      const platform = manifest.os[0];
      const arch = manifest.cpu[0];
      const libc = manifest.libc?.[0] === "musl" ? "musl" : "gnu";
      expect(nativeTargetFor(platform, arch, libc), `${platform}-${arch}-${libc}`).toBe(target);
    }
  });

  it("leaves platforms with no addon on the Wasm runtime", () => {
    expect(nativeTargetFor("freebsd", "x64", "gnu")).toBeUndefined();
    expect(nativeTargetFor("android", "arm64", "gnu")).toBeUndefined();
    expect(nativeTargetFor("linux", "riscv64", "gnu")).toBeUndefined();
    expect(nativeTargetFor("win32", "ia32", "gnu")).toBeUndefined();
  });

  it("keeps glibc and musl on separate packages", () => {
    expect(nativeTargetFor("linux", "x64", "gnu")).toBe("linux-x64-gnu");
    expect(nativeTargetFor("linux", "x64", "musl")).toBe("linux-x64-musl");
    expect(nativeTargetFor("linux", "arm64", "gnu")).toBe("linux-arm64-gnu");
    expect(nativeTargetFor("linux", "arm64", "musl")).toBe("linux-arm64-musl");
  });
});
