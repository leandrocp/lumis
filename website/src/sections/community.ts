const PROJECTS = [
  {
    name: "Hex.pm",
    href: "https://hex.pm",
    description: "Highlights package source.",
    runtime: "Elixir",
  },
  {
    name: "Tuist",
    href: "https://tuist.dev",
    description: "Highlights Markdown through MDEx.",
    runtime: "Elixir",
  },
  {
    name: "Oban Pro",
    href: "https://oban.pro",
    description: "Highlights website code examples through MDEx.",
    runtime: "Elixir",
  },
  {
    name: "Petal Components",
    href: "https://petal.build",
    description: "Highlights HEEx component examples through MDEx.",
    runtime: "Elixir",
  },
  {
    name: "SocratiCode",
    href: "https://github.com/giancarloerra/SocratiCode",
    description: "Uses Lumis parsers to analyze HEEx and EEx templates.",
    runtime: "TypeScript",
  },
  {
    name: "see",
    href: "https://github.com/guilhermeprokisch/see",
    description: "Highlights code and Markdown in the terminal.",
    runtime: "Rust",
  },
] as const;

const MENTIONS = [
  {
    title: "Lumis: Syntax Highlighter powered by Tree-sitter",
    href: "https://blog.master.dev/lumis-syntax-highlighter-powered-by-tree-sitter/",
    source: "Chris Coyier · Master.dev",
    description: "An introduction to Lumis and its runtimes.",
  },
  {
    title: "Syntax highlighting in Java, without the pain",
    href: "https://chicory.dev/blog/syntax-highlight/",
    source: "Andrea Peruffo · Chicory",
    description: "How Lumis4J brings Lumis to the JVM through WebAssembly.",
  },
  {
    title: "Leandro Pereira on MDEx",
    href: "https://www.youtube.com/watch?v=IyDNtqlClhU",
    source: "Elixir Mentor · Interview",
    description: "A conversation about MDEx, Lumis, open source, and AI.",
  },
] as const;

export function renderCommunity() {
  return `
    <section id="community" class="py-24 sm:py-28">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#community" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-accent">&lt;</span><span class="text-accent">Community</span> <span class="text-accent">/&gt;</span>
        </a>
        <h2 class="mt-8 font-display text-4xl font-bold tracking-tight text-fg">Lumis in the wild.</h2>
        <p class="mt-6 max-w-3xl font-mono text-sm leading-7 text-muted">See how other projects use Lumis.</p>

        <div id="community-runtimes" class="mt-12 scroll-mt-24">
          <h3 class="font-display text-xl font-bold text-fg">Community runtimes</h3>
          <p class="mt-4 max-w-3xl font-mono text-sm leading-7 text-muted">
            roastedroot maintains <a href="https://github.com/roastedroot/lumis4j" target="_blank" rel="noreferrer" class="underline underline-offset-4 hover:text-fg hover:text-accent">Lumis4J</a> for Java.
            Answer.AI maintains <a href="https://github.com/AnswerDotAI/fastpylight" target="_blank" rel="noreferrer" class="underline underline-offset-4 hover:text-fg hover:text-accent">fastpylight</a> for Python.
            Both packages have their own APIs and releases. Check the guides for supported features and examples.
          </p>
          <a href="https://docs.lumis.sh/community#community-runtimes" class="mt-4 inline-block font-mono text-sm text-muted underline underline-offset-4 hover:text-fg dark:text-subtle hover:text-accent">Community runtime guides</a>
        </div>

        <div id="used-by" class="mt-12 scroll-mt-24">
          <h3 class="font-display text-xl font-bold text-fg">Used by</h3>
          <ul class="mt-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          ${PROJECTS.map(
            (project) => `
            <li class="rounded-lg bg-surface-2 p-6 dark:bg-surface">
              <p class="font-mono text-xs text-muted">${project.runtime}</p>
              <h4 class="mt-3 font-display text-lg font-bold text-fg"><a href="${project.href}" target="_blank" rel="noreferrer" class="text-accent no-underline hover:underline underline-offset-3">${project.name}</a></h4>
              <p class="mt-3 font-mono text-sm leading-6 text-muted">${project.description}</p>
            </li>
          `,
          ).join("")}
          </ul>
          <a href="https://docs.lumis.sh/community#used-by" class="mt-6 inline-block font-mono text-sm text-muted underline underline-offset-4 hover:text-fg dark:text-subtle hover:text-accent">Full project list and integration links</a>
        </div>

        <div id="mentions" class="mt-12 scroll-mt-24">
          <h3 class="font-display text-xl font-bold text-fg">Mentions</h3>
          <ul class="mt-6 grid gap-4 lg:grid-cols-3">
            ${MENTIONS.map(
              (mention) => `
              <li class="rounded-lg bg-surface-2 p-6 dark:bg-surface">
                <p class="font-mono text-xs text-muted">${mention.source}</p>
                <h4 class="mt-3 font-display text-lg font-bold text-fg"><a href="${mention.href}" target="_blank" rel="noreferrer" class="text-accent no-underline hover:underline underline-offset-3">${mention.title}</a></h4>
                <p class="mt-3 font-mono text-sm leading-6 text-muted">${mention.description}</p>
              </li>
            `,
            ).join("")}
          </ul>
          <a href="https://docs.lumis.sh/community#mentions" class="mt-6 inline-block font-mono text-sm text-muted underline underline-offset-4 hover:text-fg dark:text-subtle hover:text-accent">All mentions</a>
        </div>

        <p class="mt-12 font-mono text-sm leading-7 text-muted">
          To add your project or a mention, <a href="https://github.com/leandrocp/lumis/blob/main/docs/content/community.md" class="underline underline-offset-4 hover:text-fg hover:text-accent">send a PR</a>.
        </p>
      </div>
    </section>`;
}
