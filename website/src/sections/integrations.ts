const INTEGRATION_LINKS = [
  {
    name: "React",
    links: [
      { label: "Docs", href: "/docs/integrations/react" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/react" },
    ],
  },
  {
    name: "react-markdown",
    links: [
      { label: "Docs", href: "/docs/integrations/react-markdown" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/rehype-lumis" },
    ],
  },
  {
    name: "markdown-it",
    links: [
      { label: "Docs", href: "/docs/integrations/markdown-it" },
      {
        label: "Package",
        href: "https://www.npmjs.com/package/@lumis-sh/markdown-it-lumis",
      },
    ],
  },
  {
    name: "Astro",
    links: [
      { label: "Docs", href: "/docs/integrations/astro" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/rehype-lumis" },
    ],
  },
  {
    name: "Nuxt",
    links: [
      { label: "Docs", href: "/docs/integrations/nuxt" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/rehype-lumis" },
    ],
  },
  {
    name: "Ratatui",
    links: [{ label: "Docs", href: "/docs/integrations/ratatui" }],
  },
  {
    name: "Nimble Publisher",
    links: [{ label: "Docs", href: "/docs/integrations/nimble-publisher" }],
  },
  {
    name: "Tableau",
    links: [{ label: "Docs", href: "/docs/integrations/tableau" }],
  },
  {
    name: "Docusaurus",
    links: [
      { label: "Docs", href: "/docs/integrations/docusaurus" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/rehype-lumis" },
    ],
  },
  {
    name: "Vite",
    links: [
      { label: "Docs", href: "/docs/integrations/vite" },
      {
        label: "Package",
        href: "https://www.npmjs.com/package/@lumis-sh/vite",
      },
    ],
  },
  {
    name: "VitePress",
    links: [
      { label: "Docs", href: "/docs/integrations/vitepress" },
      {
        label: "Package",
        href: "https://www.npmjs.com/package/@lumis-sh/markdown-it-lumis",
      },
    ],
  },
  {
    name: "Build your own",
    links: [{ label: "Docs", href: "/docs/formatters/custom" }],
  },
] as const;

export function renderIntegrations() {
  return `
    <section id="integrations" class="py-24 sm:py-36">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#integrations" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-cyan-400">&lt;</span><span class="text-fuchsia-400">Integrations</span> <span class="text-cyan-400">/&gt;</span>
        </a>
        <h2 class="mt-8 max-w-4xl font-mono text-4xl font-bold tracking-tight text-zinc-900 dark:text-white">
          Plug the building blocks.
        </h2>

        <div class="mt-12 grid gap-px border border-zinc-200 bg-zinc-200 dark:border-zinc-800 dark:bg-zinc-800 sm:grid-cols-2 lg:grid-cols-4">
          ${INTEGRATION_LINKS.map(
            (integration) => `
              <div class="bg-white p-6 dark:bg-[#09090b]">
                <h3 class="font-mono text-sm font-bold tracking-wider text-zinc-900 uppercase dark:text-white">${integration.name}</h3>
                <div class="mt-4 flex flex-wrap gap-2">
                  ${integration.links
                    .map(
                      (link) =>
                        `<a href="${link.href}"${link.href.startsWith("http") ? ' target="_blank" rel="noreferrer"' : ""} class="font-mono text-xs text-zinc-500 underline decoration-zinc-300 underline-offset-2 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:decoration-zinc-700 dark:hover:text-white">${link.label}</a>`,
                    )
                    .join('<span class="text-zinc-300 dark:text-zinc-700">/</span>')}
                </div>
              </div>
            `,
          ).join("")}
        </div>
      </div>
    </section>`;
}
