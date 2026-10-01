export const implementations = [
  { id: "lumis-rust", label: "Lumis Rust", runner: "criterion" },
  // Both load the same WebAssembly parsers. What differs is the engine that
  // runs them and where the highlight pass happens: Wasmtime and Rust in the
  // Node addon, V8 and JavaScript in web-tree-sitter.
  { id: "lumis-js-node", label: "Lumis JavaScript (Node, Wasmtime)", runner: "mitata" },
  { id: "lumis-js-wasm", label: "Lumis JavaScript (web-tree-sitter)", runner: "mitata" },
  { id: "lumis-elixir", label: "Lumis Elixir", runner: "benchee" },
  { id: "lumis-cli", label: "Lumis CLI", runner: "hyperfine" },
  { id: "syntect", label: "syntect", runner: "criterion" },
  { id: "shiki", label: "Shiki", runner: "mitata" },
  { id: "highlight-js", label: "highlight.js", runner: "mitata" },
  // Every timed scenario highlights Rust, which TanStack Highlight does not
  // support, so it can be compared by eye but not timed.
  { id: "tanstack-highlight", label: "TanStack Highlight", benchmark: false },
  // These read Rust, but the timing suite does not run them yet, so they are
  // compared by eye only.
  { id: "sugar-high", label: "Sugar High", benchmark: false },
  { id: "prism", label: "Prism", benchmark: false },
  { id: "speed-highlight", label: "speed-highlight", benchmark: false },
  { id: "starry-night", label: "starry-night", benchmark: false },
  // bat is a syntect front-end, and the showcase now gives syntect the same
  // syntax set bat bundles, so it would render a second copy of that column.
  // It stays here because timing a CLI against another CLI, and comparing their
  // binary sizes, are still worth doing.
  { id: "bat", label: "bat", runner: "hyperfine", showcase: false },
];

/** The implementations the benchmark suite times. */
export const benchmarkImplementations = implementations.filter(
  (implementation) => implementation.benchmark !== false,
);

/**
 * The implementations the visual comparison renders, in display order: Lumis
 * first, then the rest A-Z, so a new library has an obvious place.
 */
export const showcaseImplementations = implementations
  .filter((implementation) => implementation.showcase !== false)
  .toSorted(
    (a, b) =>
      Number(b.id.startsWith("lumis-")) - Number(a.id.startsWith("lumis-")) ||
      (a.id.startsWith("lumis-") ? 0 : a.label.localeCompare(b.label, "en")),
  );

const implementationsById = new Map(
  implementations.map((implementation) => [implementation.id, implementation]),
);

export function implementationById(id) {
  const implementation = implementationsById.get(id);
  if (!implementation) throw new Error(`unknown benchmark implementation: ${id}`);
  return implementation;
}
