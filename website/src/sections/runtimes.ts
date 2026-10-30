import { COPY_SVG } from "../lib/utils";

const RUNTIME_LINKS = [
  {
    name: "CLI",
    install: "curl -LsSf https://lumis.sh/install.sh | sh",
    links: [
      {
        label: "Docs",
        href: "https://docs.lumis.sh/usage/cli",
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
    install: `{:lumis, "~> 0.10"}`,
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
    install: "https://cdn.jsdelivr.net/npm/@lumis-sh/lumis/+esm",
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
      { label: "Guide", href: "https://docs.lumis.sh/usage/java" },
      { label: "Maven", href: "https://central.sonatype.com/search?q=io.roastedroot%3Alumis4j" },
      { label: "Source", href: "https://github.com/roastedroot/lumis4j" },
    ],
  },
  {
    name: "Python",
    install: "pip install fastpylight",
    links: [
      { label: "Guide", href: "https://docs.lumis.sh/usage/python" },
      { label: "PyPI", href: "https://pypi.org/project/fastpylight/" },
      { label: "Source", href: "https://github.com/AnswerDotAI/fastpylight" },
    ],
  },
] as const;

export function renderRuntimes() {
  return `
    <section id="runtimes" class="py-24 sm:py-28">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#runtimes" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-accent">&lt;</span><span class="text-accent">Runtimes</span> <span class="text-accent">/&gt;</span>
        </a>
        <h2 class="mt-8 font-display text-4xl font-bold tracking-tight text-fg">
          7 runtimes. Pick yours.
        </h2>
        <div class="mt-12">
          <div class="grid auto-rows-fr gap-4 sm:grid-cols-4 lg:grid-cols-8">
            ${RUNTIME_LINKS.map(
              (p) => `
              <div class="min-w-0 rounded-lg bg-surface-2 p-4 dark:bg-surface sm:col-span-2">
                <h3 class="font-mono text-sm font-bold tracking-wider text-fg uppercase  lg:min-h-10">${p.name}</h3>
                <div class="mt-4 flex min-h-16 items-center justify-between gap-1.5 rounded-md bg-canvas p-2.5">
                  <code class="min-w-0 font-mono text-[11.5px] text-muted [overflow-wrap:break-word]"><span class="mr-2 text-subtle select-none">&gt;</span>${p.install}</code>
                  <button class="copy-install shrink-0 cursor-pointer text-subtle transition-colors hover:text-fg hover:text-accent" aria-label="Copy to clipboard" data-copy="${encodeURIComponent(p.install)}">${COPY_SVG}</button>
                </div>
                <div class="mt-4 flex flex-wrap gap-x-2.5 gap-y-2">
                  ${p.links
                    .map(
                      (link) =>
                        `<a href="${link.href}" target="_blank" rel="noreferrer" class="font-mono text-xs text-accent no-underline hover:underline underline-offset-3">${link.label}</a>`,
                    )
                    .join(
                      '<span aria-hidden="true" class="size-[3px] self-center rounded-full bg-subtle"></span>',
                    )}
                </div>
              </div>
            `,
            ).join("")}
            <a href="#playground" class="flex flex-col justify-between rounded-lg bg-accent-soft p-4 no-underline sm:col-span-2">
              <div><h3 class="font-mono text-sm font-bold tracking-wider text-fg uppercase">Not sure yet?</h3>
              <p class="mt-4 font-mono text-xs text-muted">Explore code and switch between 200+ themes right in your browser.</p></div>
              <span class="mt-4 font-display text-sm font-bold text-accent">Try the playground →</span>
            </a>
          </div>
        </div>
      </div>
    </section>`;
}
