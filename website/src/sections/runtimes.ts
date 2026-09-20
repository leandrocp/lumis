import { COPY_SVG } from "../lib/utils";

const RUNTIME_LINKS = [
  {
    name: "CLI",
    install: "cargo install lumis-cli",
    links: [
      {
        label: "Docs",
        href: "https://github.com/leandrocp/lumis/blob/main/crates/lumis-cli/README.md",
      },
      { label: "Source", href: "https://github.com/leandrocp/lumis/tree/main/crates/lumis-cli" },
    ],
  },
  {
    name: "Rust",
    install: "cargo add lumis",
    links: [
      { label: "docs.rs", href: "https://docs.rs/lumis" },
      { label: "crates.io", href: "https://crates.io/crates/lumis" },
      { label: "Source", href: "https://github.com/leandrocp/lumis/tree/main/crates/lumis" },
    ],
  },
  {
    name: "Elixir",
    install: `{:lumis, "~> 0.7"}`,
    links: [
      { label: "HexDocs", href: "https://hexdocs.pm/lumis" },
      { label: "Hex", href: "https://hex.pm/packages/lumis" },
      {
        label: "Source",
        href: "https://github.com/leandrocp/lumis/tree/main/packages/elixir/lumis",
      },
    ],
  },
  {
    name: "JavaScript / TypeScript",
    install: "npm install @lumis-sh/lumis",
    links: [
      { label: "npm", href: "https://www.npmjs.com/package/@lumis-sh/lumis" },
      {
        label: "Source",
        href: "https://github.com/leandrocp/lumis/tree/main/packages/javascript/lumis",
      },
    ],
  },
  {
    name: "Browsers / CDN",
    install: "npm install @lumis-sh/lumis",
    links: [
      { label: "npm", href: "https://www.npmjs.com/package/@lumis-sh/lumis" },
      {
        label: "Source",
        href: "https://github.com/leandrocp/lumis/tree/main/packages/javascript/lumis",
      },
    ],
  },
  {
    name: "Java",
    install: "io.roastedroot:lumis4j:0.0.7",
    links: [
      { label: "Maven", href: "https://central.sonatype.com/search?q=io.roastedroot%3Alumis4j" },
      { label: "Source", href: "https://github.com/roastedroot/lumis4j" },
    ],
  },
] as const;

export function renderRuntimes() {
  return `
    <section id="runtimes" class="py-24 sm:py-36">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#runtimes" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-rose-400">&lt;</span><span class="text-indigo-400">Runtimes</span> <span class="text-rose-400">/&gt;</span>
        </a>
        <h2 class="mt-8 font-mono text-4xl font-bold tracking-tight text-zinc-900 dark:text-white">
          Same API, same themes. Pick your runtime.
        </h2>
        <div class="mt-12">
          <div class="grid gap-px border border-zinc-200 bg-zinc-200 dark:border-zinc-800 dark:bg-zinc-800 sm:grid-cols-2 lg:grid-cols-3">
            ${RUNTIME_LINKS.map(
              (p) => `
              <div class="bg-white p-6 dark:bg-[#09090b]">
                <h3 class="font-mono text-sm font-bold tracking-wider text-zinc-900 uppercase dark:text-white">${p.name}</h3>
                <div class="mt-4 flex items-center justify-between gap-2 overflow-x-auto border border-zinc-100 px-3 py-2 dark:border-zinc-800">
                  <code class="min-w-0 truncate font-mono text-xs text-zinc-600 dark:text-zinc-400"><span class="mr-2 text-zinc-300 select-none dark:text-zinc-700">&gt;</span>${p.install}</code>
                  <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="${encodeURIComponent(p.install)}">${COPY_SVG}</button>
                </div>
                <div class="mt-4 flex flex-wrap gap-2">
                  ${p.links
                    .map(
                      (link) =>
                        `<a href="${link.href}" target="_blank" rel="noreferrer" class="font-mono text-xs text-zinc-500 underline decoration-zinc-300 underline-offset-2 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:decoration-zinc-700 dark:hover:text-white">${link.label}</a>`,
                    )
                    .join('<span class="text-zinc-300 dark:text-zinc-700">/</span>')}
                </div>
              </div>
            `,
            ).join("")}
          </div>
          <p class="mt-6 max-w-3xl font-mono text-xs leading-6 text-zinc-500 dark:text-zinc-400">
            The JavaScript / TypeScript package and Elixir load exact, integrity-checked parser WASM per language.
            Applications fetch only the languages they use and persist them across restarts.
          </p>
        </div>
      </div>
    </section>`;
}
