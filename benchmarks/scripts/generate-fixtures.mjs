#!/usr/bin/env node

import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

const benchmarksDir = resolve(import.meta.dirname, "..");
const repoDir = resolve(benchmarksDir, "..");
const outputDir = resolve(repoDir, "target/benchmarks/fixtures");
const largePath = resolve(outputDir, "rust-large.rs");
const largeMinimumBytes = 5 * 1024 * 1024;

const largeParts = [
  "//! Deterministic generated 5 MiB Rust benchmark input.\n",
  "#![allow(dead_code)]\n\n",
];
let largeBytes = Buffer.byteLength(largeParts.join(""));
let moduleIndex = 0;

while (largeBytes < largeMinimumBytes) {
  const id = String(moduleIndex).padStart(5, "0");
  const factor = (moduleIndex % 97) + 3;
  const block = `pub mod block_${id} {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Record {
        pub id: u32,
        pub value: u64,
    }

    pub const LABEL: &str = "block-${id}";

    pub fn checksum(seed: u64) -> u64 {
        let record = Record { id: ${moduleIndex}, value: seed };
        record.value.wrapping_mul(${factor}).wrapping_add(record.id as u64)
    }
}

`;
  largeParts.push(block);
  largeBytes += Buffer.byteLength(block);
  moduleIndex += 1;
}

await mkdir(outputDir, { recursive: true });
await writeFile(largePath, largeParts.join(""));

const rustDir = resolve(outputDir, "rust");
await mkdir(rustDir, { recursive: true });
for (let index = 1; index <= 10; index += 1) {
  const id = String(index).padStart(2, "0");
  const source = `//! Deterministic Rust fixture ${id}.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sample${id} {
    pub id: u32,
    pub weight: u64,
}

impl Sample${id} {
    pub fn score(self) -> u64 {
        self.weight.wrapping_mul(${index + 2}).wrapping_add(self.id as u64)
    }
}

pub fn sample_${id}(seed: u64) -> u64 {
    let values = [seed, seed.rotate_left(${index}), seed ^ ${index * 17}];
    values
        .into_iter()
        .enumerate()
        .map(|(id, weight)| Sample${id} { id: id as u32, weight }.score())
        .sum()
}
`;
  await writeFile(resolve(rustDir, `file-${id}.rs`), source);
}

console.log(
  JSON.stringify({
    large: { path: largePath, bytes: largeBytes, modules: moduleIndex },
    rustFiles: 10,
  }),
);
