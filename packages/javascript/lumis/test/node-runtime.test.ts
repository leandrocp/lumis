import { describe, expect, it } from "vitest";

import { browserRuntime } from "../src/runtime/browser.js";
import { isUrlString } from "../src/runtime/node-cache.js";
import { wasmCacheFilename } from "../src/runtime/node.js";

describe("Node runtime", () => {
  it("keeps custom WASM cache keys inside the cache directory", () => {
    const filename = wasmCacheFilename("../../outside\\parser");

    expect(filename).not.toMatch(/[\\/]/u);
    expect(filename).toBe("..%2F..%2Foutside%5Cparser.wasm");
  });

  it("distinguishes URLs from POSIX and Windows paths on Node 18", () => {
    expect(isUrlString("https://cdn.example/parser.wasm")).toBe(true);
    expect(isUrlString("file:///tmp/parser.wasm")).toBe(true);
    expect(isUrlString("/tmp/parser.wasm")).toBe(false);
    expect(isUrlString("C:\\parsers\\tree-sitter-rust.wasm")).toBe(false);
    expect(isUrlString("C:tree-sitter-rust.wasm")).toBe(false);
  });

  it("serializes browser cache operations for the same parser", async () => {
    const calls: string[] = [];
    let releaseFirst!: () => void;
    let markFirstStarted!: () => void;
    const firstGate = new Promise<void>((resolve) => {
      releaseFirst = resolve;
    });
    const firstStarted = new Promise<void>((resolve) => {
      markFirstStarted = resolve;
    });

    const first = browserRuntime.withFsCacheLock("same-parser", async () => {
      calls.push("first:start");
      markFirstStarted();
      await firstGate;
      calls.push("first:end");
    });
    await firstStarted;

    const second = browserRuntime.withFsCacheLock("same-parser", async () => {
      calls.push("second");
    });
    await Promise.resolve();
    expect(calls).toEqual(["first:start"]);

    releaseFirst();
    await Promise.all([first, second]);
    expect(calls).toEqual(["first:start", "first:end", "second"]);
  });
});
