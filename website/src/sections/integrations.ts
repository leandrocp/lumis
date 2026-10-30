const INTEGRATION_LINKS = [
  {
    name: "React",
    links: [
      { label: "Docs", href: "https://docs.lumis.sh/integrations/react" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/react" },
    ],
  },
  {
    name: "react-markdown",
    links: [
      { label: "Docs", href: "https://docs.lumis.sh/integrations/react-markdown" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/rehype-lumis" },
    ],
  },
  {
    name: "markdown-it",
    links: [
      { label: "Docs", href: "https://docs.lumis.sh/integrations/markdown-it" },
      {
        label: "Package",
        href: "https://www.npmjs.com/package/@lumis-sh/markdown-it-lumis",
      },
    ],
  },
  {
    name: "Astro",
    links: [
      { label: "Docs", href: "https://docs.lumis.sh/integrations/astro" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/rehype-lumis" },
    ],
  },
  {
    name: "Nuxt",
    links: [
      { label: "Docs", href: "https://docs.lumis.sh/integrations/nuxt" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/rehype-lumis" },
    ],
  },
  {
    name: "Ratatui",
    links: [{ label: "Docs", href: "https://docs.lumis.sh/integrations/ratatui" }],
  },
  {
    name: "Nimble Publisher",
    links: [{ label: "Docs", href: "https://docs.lumis.sh/integrations/nimble-publisher" }],
  },
  {
    name: "Tableau",
    links: [{ label: "Docs", href: "https://docs.lumis.sh/integrations/tableau" }],
  },
  {
    name: "Docusaurus",
    links: [
      { label: "Docs", href: "https://docs.lumis.sh/integrations/docusaurus" },
      { label: "Package", href: "https://www.npmjs.com/package/@lumis-sh/rehype-lumis" },
    ],
  },
  {
    name: "Vite",
    links: [
      { label: "Docs", href: "https://docs.lumis.sh/integrations/vite" },
      {
        label: "Package",
        href: "https://www.npmjs.com/package/@lumis-sh/vite",
      },
    ],
  },
  {
    name: "VitePress",
    links: [
      { label: "Docs", href: "https://docs.lumis.sh/integrations/vitepress" },
      {
        label: "Package",
        href: "https://www.npmjs.com/package/@lumis-sh/markdown-it-lumis",
      },
    ],
  },
  {
    name: "Build your own",
    links: [{ label: "Docs", href: "https://docs.lumis.sh/formatters/custom" }],
  },
] as const;

export function renderIntegrations() {
  return `
    <section id="integrations" class="py-24 sm:py-36">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#integrations" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-accent">&lt;</span><span class="text-accent">Integrations</span> <span class="text-accent">/&gt;</span>
        </a>
        <h2 class="mt-8 max-w-4xl font-display text-4xl font-bold tracking-tight text-fg">
          Plug the building blocks.
        </h2>

        <div class="mt-12 grid gap-px border border-line rounded-lg bg-surface-2   sm:grid-cols-2 lg:grid-cols-4">
          ${INTEGRATION_LINKS.map(
            (integration) => `
              <div class="bg-surface p-6 ">
                <h3 class="font-mono text-sm font-bold tracking-wider text-fg uppercase ">${integration.name}</h3>
                <div class="mt-4 flex flex-wrap gap-2">
                  ${integration.links
                    .map(
                      (link) =>
                        `<a href="${link.href}"${link.href.startsWith("http") ? ' target="_blank" rel="noreferrer"' : ""} class="font-mono text-xs text-muted underline decoration-line underline-offset-2 transition-colors hover:text-fg dark:text-subtle  hover:text-accent">${link.label}</a>`,
                    )
                    .join('<span class="text-subtle">/</span>')}
                </div>
              </div>
            `,
          ).join("")}
        </div>
      </div>
    </section>`;
}
