"use strict";

function linuxLibc() {
  const report = process.report?.getReport?.();
  return report?.header?.glibcVersionRuntime ? "gnu" : "musl";
}

// Kept identical to `nativeTargetFor` in src/native-binding.ts; this package
// ships alone and cannot import it. test/native-targets.test.ts pins them.
/** @returns {string | undefined} The package target, or undefined on unsupported platforms. */
function nativeTargetFor(platform, arch, libc) {
  if (platform === "darwin" && ["arm64", "x64"].includes(arch)) {
    return `darwin-${arch}`;
  }
  if (platform === "linux" && ["arm64", "x64"].includes(arch)) {
    return `linux-${arch}-${libc}`;
  }
  if (platform === "win32" && ["arm64", "x64"].includes(arch)) {
    return `win32-${arch}-msvc`;
  }
  // oxlint-disable-next-line unicorn/no-useless-undefined -- unsupported hosts keep the selector's undefined contract.
  return undefined;
}

const target = nativeTargetFor(
  process.platform,
  process.arch,
  process.platform === "linux" ? linuxLibc() : "gnu",
);

if (!target) {
  throw new Error(`The Lumis native runtime does not support ${process.platform}-${process.arch}`);
}

module.exports = require(`@lumis-sh/lumis-native-${target}`);
