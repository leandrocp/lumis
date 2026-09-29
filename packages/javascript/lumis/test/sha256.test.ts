/**
 * Pins the pure-JS SHA-256 to `crypto.subtle`, the implementation it stands in
 * for when a browser withholds `crypto.subtle` from a non-secure origin.
 */
import { sha256 } from "@noble/hashes/sha2.js";
import { describe, expect, it } from "vitest";

const hex = (bytes: Uint8Array) =>
  Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");

const subtleHex = async (data: Uint8Array) =>
  hex(new Uint8Array(await crypto.subtle.digest("SHA-256", data.slice().buffer)));

describe("sha256", () => {
  it("matches the published test vectors", () => {
    expect(hex(sha256(new Uint8Array()))).toBe(
      "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    );
    expect(hex(sha256(new TextEncoder().encode("abc")))).toBe(
      "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    );
  });

  it("matches crypto.subtle across every padding boundary", async () => {
    // 55/56 and 63/64 are where the length field forces an extra block.
    for (const length of [0, 1, 54, 55, 56, 63, 64, 65, 119, 120, 127, 128, 1000]) {
      const data = new Uint8Array(length);
      for (let i = 0; i < length; i++) data[i] = (i * 31 + 7) % 256;
      expect(hex(sha256(data)), `length ${length}`).toBe(await subtleHex(data));
    }
  });

  it("matches crypto.subtle on random input", async () => {
    for (let round = 0; round < 50; round++) {
      const data = crypto.getRandomValues(new Uint8Array(Math.floor(Math.random() * 4096)));
      expect(hex(sha256(data))).toBe(await subtleHex(data));
    }
  });
});

describe("verifyWasm without crypto.subtle", () => {
  it("verifies rather than throwing on a non-secure origin", async () => {
    const original = globalThis.crypto;
    // A non-secure origin exposes crypto but not crypto.subtle.
    Object.defineProperty(globalThis, "crypto", {
      value: { getRandomValues: original.getRandomValues.bind(original) },
      configurable: true,
    });
    try {
      const { verifyWasm } = await import("../src/core/languages.js");
      const data = new TextEncoder().encode("abc");
      const ref = {
        packageName: "@lumis-sh/wasm-x",
        name: "tree-sitter-x",
        version: "1.0.0",
        sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
      };
      await expect(verifyWasm(ref, data)).resolves.toBe(data);
      await expect(verifyWasm({ ...ref, sha256: "00" }, data)).rejects.toThrow("integrity");
    } finally {
      Object.defineProperty(globalThis, "crypto", { value: original, configurable: true });
    }
  });
});
