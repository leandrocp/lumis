import {
  bbcodeScoped,
  htmlInline,
  htmlLinked,
  htmlMultiThemes,
  terminal,
} from "@lumis-sh/lumis/formatters";
import type { HtmlMultiThemesOptions } from "@lumis-sh/lumis/formatters";
import type { LanguageRef } from "@lumis-sh/lumis";
import css from "@lumis-sh/lumis/langs/css";
import elixir from "@lumis-sh/lumis/langs/elixir";
import erb from "@lumis-sh/lumis/langs/erb";
import heex from "@lumis-sh/lumis/langs/heex";
import html from "@lumis-sh/lumis/langs/html";
import javascript from "@lumis-sh/lumis/langs/javascript";
import json from "@lumis-sh/lumis/langs/json";
import markdown from "@lumis-sh/lumis/langs/markdown";
import markdownInline from "@lumis-sh/lumis/langs/markdown_inline";
import ruby from "@lumis-sh/lumis/langs/ruby";
import rust from "@lumis-sh/lumis/langs/rust";
import typescript from "@lumis-sh/lumis/langs/typescript";
import catppuccinFrappe from "@lumis-sh/themes/catppuccin_frappe";
import catppuccinLatte from "@lumis-sh/themes/catppuccin_latte";
import dracula from "@lumis-sh/themes/dracula";
import githubLight from "@lumis-sh/themes/github_light";
import nord from "@lumis-sh/themes/nord";
import tokyonightMoon from "@lumis-sh/themes/tokyonight_moon";
import { highlighterFor } from "../lib/published-highlighter";
import { CHECK_SVG, COPY_SVG, escapeHtml } from "../lib/utils";

const REPO = "https://github.com/leandrocp/lumis/tree/main";
const PRE_CLASS = "m-0 overflow-x-auto p-5 font-mono text-[13px] leading-relaxed";
const ESCAPE = "\u001B";
const FENCE = "```";

/** Every demo renders in both, so a code block follows the page rather than fighting it. */
const THEMES = { light: catppuccinLatte, dark: catppuccinFrappe };

interface Demo {
  id: string;
  title: string;
  blurb: string;
  tags: string[];
  code: string;
  codeLanguage?: "html";
  source: string;
  run: () => Promise<string>;
  wire?: (output: HTMLElement) => void;
}

const TYPESCRIPT_SOURCE = `export async function search(query: string, signal?: AbortSignal) {
  const params = new URLSearchParams({ q: query, limit: "20" })
  const response = await fetch(\`/api/search?\${params}\`, { signal })

  if (!response.ok) {
    throw new SearchError(response.status, await response.text())
  }

  return (await response.json()) as SearchResult[]
}`;

const RUST_SOURCE = `impl Cache {
    pub fn get_or_insert(&mut self, key: &str, load: impl FnOnce() -> Vec<u8>) -> &[u8] {
        if !self.entries.contains_key(key) {
            let bytes = load();
            self.bytes_held += bytes.len();
            self.entries.insert(key.to_owned(), bytes);
        }

        &self.entries[key]
    }
}`;

const ELIXIR_SOURCE = `defmodule Store.CartComponent do
  @moduledoc """
  Renders the cart.

  Prices come from \`Store.Pricing\`, so **totals** are never recomputed here.
  """
  use Phoenix.Component

  attr :items, :list, required: true

  def cart(assigns) do
    ~H"""
    <ul class="cart" phx-update="stream">
      <li :for={item <- @items} id={"item-#{item.id}"} class="cart-row">
        <span class="name">{item.name}</span>
        <span class="price">{Money.to_string(item.price)}</span>
      </li>
    </ul>
    """
  end
end`;

const MARKDOWN_SOURCE = `## Install

Add it to \`Cargo.toml\`, then:

${FENCE}rust
let html = lumis::highlight(source, formatter)?;
${FENCE}

The same call in **JavaScript**:

${FENCE}js
const html = await highlight(source, htmlInline({ language, theme }))
${FENCE}
`;

const ERB_SOURCE = `<ul class="posts">
  <% @posts.each do |post| %>
    <li id="post_<%= post.id %>">
      <%= link_to post.title, post_path(post) %>
      <span class="date"><%= post.published_at.strftime("%b %d") %></span>
    </li>
  <% end %>
</ul>`;

const BRACKETS_SOURCE = `const server = createServer({
  routes: [route("/health", () => ok({ status: [200, "up"] }))],
  plugins: [metrics({ buckets: [0.1, [0.5, 1], 5] }), tracing()],
})`;

const THEME_SOURCE = `defp deps do
  [{:lumis, "~> 0.7"}]
end`;

const CSS_SOURCE = `.cart-row {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 0.75rem;
}`;

const HEADER_SOURCE = `export function formatPrice(cents: number, currency = "USD") {
  return new Intl.NumberFormat("en-US", { style: "currency", currency })
    .format(cents / 100)
}`;

const TOKENS_SOURCE = `const total = items.reduce((sum, item) => sum + item.price, 0)`;

const ANSI_SOURCE = `{"id": 1, "ok": true}`;

const DEMOS: Demo[] = [
  {
    id: "dual-themes",
    title: "Dual themes",
    blurb:
      "Two themes in one render, switched by the reader's colour scheme rather than by JavaScript.",
    tags: ["htmlMultiThemes", "TypeScript"],
    source: `${REPO}/packages/javascript/lumis/examples/multi-themes.html`,
    code: `import { createHighlighter } from '@lumis-sh/lumis'
import { htmlMultiThemes } from '@lumis-sh/lumis/formatters'
import typescript from '@lumis-sh/lumis/langs/typescript'
import latte from '@lumis-sh/themes/catppuccin_latte'
import frappe from '@lumis-sh/themes/catppuccin_frappe'

const hl = await createHighlighter({ languages: [typescript] })

const html = hl.highlight(source, htmlMultiThemes({
  language: typescript,
  themes: { light: latte, dark: frappe },
  defaultTheme: 'light-dark()',
}))`,
    async run() {
      const hl = await highlighterFor(typescript);
      return hl.highlight(TYPESCRIPT_SOURCE, themed(typescript));
    },
  },
  {
    id: "injected-languages",
    title: "Injected languages",
    blurb:
      "A HEEx template in a sigil and Markdown in a doc string, each parsed by its own grammar.",
    tags: ["Elixir", "HEEx", "Markdown"],
    source: "/docs/recipes/injected-languages-html-css-js",
    code: `import elixir from '@lumis-sh/lumis/langs/elixir'
import heex from '@lumis-sh/lumis/langs/heex'
import markdown from '@lumis-sh/lumis/langs/markdown'
import markdownInline from '@lumis-sh/lumis/langs/markdown_inline'

const hl = await createHighlighter({
  languages: [elixir, heex, markdown, markdownInline],
})

const html = hl.highlight(source, htmlMultiThemes({
  language: elixir,
  themes: { light: latte, dark: frappe },
  defaultTheme: 'light-dark()',
}))`,
    async run() {
      const hl = await highlighterFor(elixir, heex, markdown, markdownInline);
      return hl.highlight(ELIXIR_SOURCE, themed(elixir));
    },
  },
  {
    id: "markdown-fences",
    title: "Markdown code fences",
    blurb: "Fenced blocks are highlighted in the language the fence names, at any depth.",
    tags: ["Markdown", "Rust", "JavaScript"],
    source: `${REPO}/packages/javascript/lumis/examples/injected-languages.html`,
    code: `const hl = await createHighlighter({
  languages: [markdown, markdownInline, rust, javascript],
})

const html = hl.highlight(readme, htmlMultiThemes({
  language: markdown,
  themes: { light: latte, dark: frappe },
  defaultTheme: 'light-dark()',
}))`,
    async run() {
      const hl = await highlighterFor(markdown, markdownInline, rust, javascript);
      return hl.highlight(MARKDOWN_SOURCE, themed(markdown));
    },
  },
  {
    id: "template-languages",
    title: "Template languages",
    blurb: "ERB, EEx, HEEx, Vue and Svelte carry two grammars at once, and both get highlighted.",
    tags: ["ERB", "Ruby", "HTML"],
    source: `${REPO}/packages/javascript/lumis/examples/erb-template.html`,
    code: `import erb from '@lumis-sh/lumis/langs/erb'
import html from '@lumis-sh/lumis/langs/html'
import ruby from '@lumis-sh/lumis/langs/ruby'

const hl = await createHighlighter({ languages: [erb, html, ruby] })

const output = hl.highlight(template, htmlMultiThemes({
  language: erb,
  themes: { light: latte, dark: frappe },
  defaultTheme: 'light-dark()',
}))`,
    async run() {
      const hl = await highlighterFor(erb, html, ruby);
      return hl.highlight(ERB_SOURCE, themed(erb));
    },
  },
  {
    id: "highlight-lines",
    title: "Highlight lines",
    blurb:
      "Mark line ranges with a CSS string, a class of your own, or the theme's highlight colour.",
    tags: ["highlightLines", "Rust"],
    source: `${REPO}/packages/javascript/lumis/examples/highlight-lines.html`,
    code: `const html = hl.highlight(source, htmlMultiThemes({
  language: rust,
  themes: { light: latte, dark: frappe },
  defaultTheme: 'light-dark()',
  highlightLines: {
    lines: [[3, 6]],
    style: 'background: light-dark(#dce0e8, #414559)',
  },
}))`,
    async run() {
      const hl = await highlighterFor(rust);
      return hl.highlight(
        RUST_SOURCE,
        themed(rust, {
          // `style: "theme"` is dropped when `defaultTheme` is `light-dark()`,
          // so the two highlight colours are named here instead.
          highlightLines: {
            lines: [[3, 6]],
            style: "background: light-dark(#dce0e8, #414559)",
          },
        }),
      );
    },
  },
  {
    id: "rainbow-brackets",
    title: "Rainbow brackets",
    blurb:
      "Bracket pairs coloured by depth, taken from each language's brackets query rather than a fixed list.",
    tags: ["rainbowBrackets", "JavaScript"],
    source: "/docs/recipes/rainbow-brackets",
    code: `const html = hl.highlight(source, htmlMultiThemes({
  language: javascript,
  themes: { light: latte, dark: frappe },
  defaultTheme: 'light-dark()',
  rainbowBrackets: true,
}))`,
    async run() {
      const hl = await highlighterFor(javascript);
      return hl.highlight(BRACKETS_SOURCE, themed(javascript, { rainbowBrackets: true }));
    },
  },
  {
    id: "themes",
    title: "250+ Neovim themes",
    blurb:
      "Each theme is generated from the colorscheme itself, so it looks like the editor it came from.",
    tags: ["htmlInline", "Elixir"],
    source: `${REPO}/packages/javascript/lumis/examples/theme-gallery.html`,
    code: `import dracula from '@lumis-sh/themes/dracula'
import nord from '@lumis-sh/themes/nord'
import githubLight from '@lumis-sh/themes/github_light'
import tokyonightMoon from '@lumis-sh/themes/tokyonight_moon'

for (const theme of [dracula, nord, githubLight, tokyonightMoon]) {
  render(hl.highlight(source, htmlInline({ language: elixir, theme })))
}`,
    async run() {
      const hl = await highlighterFor(elixir);
      const themes = [
        { label: "dracula", theme: dracula },
        { label: "nord", theme: nord },
        { label: "github_light", theme: githubLight },
        { label: "tokyonight_moon", theme: tokyonightMoon },
      ];

      return `<div class="grid gap-px bg-zinc-200 dark:bg-zinc-800 sm:grid-cols-2">${themes
        .map(({ label, theme }) => {
          const output = hl.highlight(
            THEME_SOURCE,
            htmlInline({
              language: elixir,
              theme,
              preClass: `${PRE_CLASS} text-xs`,
              italic: false,
            }),
          );
          return `<figure class="m-0">
            <figcaption class="bg-white px-5 pt-4 font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:bg-[#09090b] dark:text-zinc-400">${label}</figcaption>
            ${output}
          </figure>`;
        })
        .join("")}</div>`;
    },
  },
  {
    id: "css-classes",
    title: "CSS classes instead of inline styles",
    blurb:
      "Tokens get scope class names, and the colours come from a stylesheet the browser can cache.",
    tags: ["htmlLinked", "CSS"],
    source: "/docs/themes/css-files",
    codeLanguage: "html",
    code: `<link rel="stylesheet" media="(prefers-color-scheme: light)"
      href="https://cdn.jsdelivr.net/npm/@lumis-sh/themes/dist/css/catppuccin_latte.css">
<link rel="stylesheet" media="(prefers-color-scheme: dark)"
      href="https://cdn.jsdelivr.net/npm/@lumis-sh/themes/dist/css/catppuccin_frappe.css">

<script type="module">
  const html = hl.highlight(source, htmlLinked({ language: css }))
</script>`,
    async run() {
      const hl = await highlighterFor(css);
      linkStylesheets();
      const output = hl.highlight(CSS_SOURCE, htmlLinked({ language: css, preClass: PRE_CLASS }));
      return `<div class="grid gap-px bg-zinc-200 dark:bg-zinc-800">
        ${output}
        ${outputPanel("the markup it produced", `${output.slice(0, 240)}…`)}
      </div>`;
    },
  },
  {
    id: "file-header",
    title: "File header and copy button",
    blurb:
      "The header option wraps the output in your own markup, so a code block can carry a toolbar.",
    tags: ["header", "TypeScript"],
    source: `${REPO}/packages/javascript/lumis/examples/toolbar-copy.html`,
    code: `const html = hl.highlight(source, htmlMultiThemes({
  language: typescript,
  themes: { light: latte, dark: frappe },
  defaultTheme: 'light-dark()',
  header: {
    openTag: '<figure><figcaption>formatPrice.ts</figcaption>',
    closeTag: '</figure>',
  },
}))`,
    async run() {
      const hl = await highlighterFor(typescript);
      return hl.highlight(
        HEADER_SOURCE,
        themed(typescript, {
          header: {
            openTag:
              '<figure class="m-0">' +
              '<figcaption class="flex items-center justify-between border-b border-zinc-200 px-5 py-2 font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:border-zinc-800 dark:text-zinc-400">' +
              "formatPrice.ts" +
              '<button type="button" class="demo-copy cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard">' +
              `${COPY_SVG}</button></figcaption>`,
            closeTag: "</figure>",
          },
        }),
      );
    },
    wire(output) {
      const button = output.querySelector<HTMLButtonElement>(".demo-copy");
      button?.addEventListener("click", () => {
        void navigator.clipboard.writeText(HEADER_SOURCE);
        button.innerHTML = CHECK_SVG;
        button.classList.add("text-emerald-500");
        setTimeout(() => {
          button.innerHTML = COPY_SVG;
          button.classList.remove("text-emerald-500");
        }, 1500);
      });
    },
  },
  {
    id: "custom-formatter",
    title: "Custom formatter",
    blurb:
      "A formatter is an object with a format method, so any output format is a few lines away.",
    tags: ["Formatter", "highlightIter"],
    source: `${REPO}/packages/javascript/lumis/examples/custom-formatter.html`,
    code: `const tokenTable = {
  language: javascript,
  format(source) {
    const rows = []
    hl.highlightIter(source, javascript, latte, (text, language, range, scope) => {
      if (scope && text.trim()) rows.push(scope.padEnd(24) + text)
    })
    return rows.join('\\n')
  },
}

const table = hl.highlight(source, tokenTable)`,
    async run() {
      const hl = await highlighterFor(javascript);
      const tokenTable = {
        language: javascript,
        format(source: string) {
          const rows: string[] = [];
          hl.highlightIter(
            source,
            javascript,
            catppuccinLatte,
            (text, _language, _range, scope) => {
              if (scope && text.trim()) rows.push(`${scope.padEnd(24)}${text}`);
            },
          );
          return rows.join("\n");
        },
      };
      return outputPanel("plain text, from the same walk", hl.highlight(TOKENS_SOURCE, tokenTable));
    },
  },
  {
    id: "terminal-output",
    title: "Terminal output",
    blurb: "The same themes as ANSI escape codes, which is what the CLI prints.",
    tags: ["terminal", "JSON"],
    source: `${REPO}/packages/javascript/lumis/examples/terminal.html`,
    code: `import { terminal } from '@lumis-sh/lumis/formatters'

const ansi = hl.highlight(source, terminal({ language: json, theme: frappe }))
process.stdout.write(ansi)`,
    async run() {
      const hl = await highlighterFor(json);
      const ansi = hl.highlight(ANSI_SOURCE, terminal({ language: json, theme: catppuccinFrappe }));
      // A terminal reads U+001B; a reader has to be able to see that it is there.
      return outputPanel("the bytes it returns", ansi.replaceAll(ESCAPE, "\\e"));
    },
  },
  {
    id: "bbcode-output",
    title: "BBCode output",
    blurb: "Scope names as BBCode tags, for forums and anywhere else that is not a browser.",
    tags: ["bbcodeScoped", "JSON"],
    source: `${REPO}/packages/javascript/lumis/examples/bbcode-scoped.html`,
    code: `import { bbcodeScoped } from '@lumis-sh/lumis/formatters'

const bbcode = hl.highlight(source, bbcodeScoped({ language: json }))`,
    async run() {
      const hl = await highlighterFor(json);
      return outputPanel(
        "the bytes it returns",
        hl.highlight(ANSI_SOURCE, bbcodeScoped({ language: json })),
      );
    },
  },
];

const CATALOG = [
  {
    title: "Browser and Node",
    detail:
      "17 standalone pages: every formatter, line highlighting, a custom WASM resolver, a theme gallery, a copy toolbar.",
    href: `${REPO}/packages/javascript/lumis/examples`,
  },
  {
    title: "React",
    detail: "Server-rendered React and a Next.js App Router route.",
    href: `${REPO}/packages/javascript/react/examples`,
  },
  {
    title: "Markdown pipelines",
    detail: "Astro, Docusaurus, MDX, Next.js, Nuxt, react-markdown and plain rehype.",
    href: `${REPO}/packages/javascript/rehype-lumis/examples`,
  },
  {
    title: "markdown-it and VitePress",
    detail: "Drop-in highlighting for markdown-it, and the VitePress config that uses it.",
    href: `${REPO}/packages/javascript/markdown-it-lumis/examples`,
  },
  {
    title: "Vite",
    detail: "A static HTML entry point highlighted during the Vite build.",
    href: `${REPO}/packages/javascript/vite/examples`,
  },
  {
    title: "Elixir",
    detail:
      "Six Livebooks covering light and dark, rainbow brackets and scoped CSS, plus a NimblePublisher blog.",
    href: `${REPO}/packages/elixir/lumis/examples`,
  },
  {
    title: "Rust",
    detail:
      "13 runnable examples, including three custom formatters and a Ratatui app that highlights in the terminal.",
    href: `${REPO}/crates/lumis/examples`,
  },
];

type ThemedOptions = Omit<
  HtmlMultiThemesOptions,
  "language" | "themes" | "defaultTheme" | "preClass" | "italic"
>;

function themed(language: LanguageRef, options: ThemedOptions = {}) {
  return htmlMultiThemes({
    language,
    themes: THEMES,
    defaultTheme: "light-dark()",
    preClass: PRE_CLASS,
    italic: false,
    ...options,
  });
}

// The linked formatter names scopes and leaves the colours to a stylesheet, so
// the page carries one per scheme, from the same CDN the snippet names. A CDN
// serves files rather than the package's export map, so the path is the one on
// disk, `dist/css`, not the `css/` subpath an import would use.
const THEME_CSS = "https://cdn.jsdelivr.net/npm/@lumis-sh/themes/dist/css";
let stylesheetsLinked = false;
function linkStylesheets() {
  if (stylesheetsLinked) return;
  stylesheetsLinked = true;

  for (const [theme, media] of [
    ["catppuccin_latte", "(prefers-color-scheme: light)"],
    ["catppuccin_frappe", "(prefers-color-scheme: dark)"],
  ]) {
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = `${THEME_CSS}/${theme}.css`;
    link.media = media;
    document.head.append(link);
  }
}

function outputPanel(label: string, content: string) {
  return `<div class="bg-white dark:bg-[#09090b]">
    <p class="px-5 pt-4 font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">${label}</p>
    <pre class="m-0 max-h-44 overflow-auto p-5 font-mono text-[13px] leading-relaxed break-all whitespace-pre-wrap text-zinc-700 dark:text-zinc-300">${escapeHtml(content)}</pre>
  </div>`;
}

export function renderShowcase() {
  return `
    <section id="showcase" class="pt-32 pb-24 sm:pt-40 sm:pb-36">
      <div class="mx-auto max-w-6xl px-6">
        <h1 class="inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider">
          <span class="text-pink-400">&lt;</span><span class="text-purple-400">Showcase</span> <span class="text-pink-400">/&gt;</span>
        </h1>
        <h2 class="mt-8 max-w-3xl font-mono text-4xl font-bold leading-tight tracking-tight text-zinc-900 dark:text-white">
          What Lumis can do.
        </h2>
        <p class="mt-4 max-w-2xl text-sm leading-relaxed text-zinc-600 dark:text-zinc-400">
          From built-in languages and themes to custom formatters and framework integrations, on
          every runtime.
        </p>

        <div class="mt-16 space-y-12">
          ${DEMOS.map(
            (demo) => `
            <article id="${demo.id}" class="border border-zinc-200 dark:border-zinc-800">
              <header class="border-b border-zinc-200 px-5 py-4 dark:border-zinc-800">
                <div class="flex flex-wrap items-baseline justify-between gap-3">
                  <h3 class="font-mono text-lg font-bold tracking-tight text-zinc-900 dark:text-white">${demo.title}</h3>
                  <div class="flex flex-wrap gap-2">
                    ${demo.tags
                      .map(
                        (tag) =>
                          `<span class="border border-zinc-200 px-2 py-0.5 font-mono text-[10px] tracking-wider text-zinc-500 uppercase dark:border-zinc-800 dark:text-zinc-400">${tag}</span>`,
                      )
                      .join("")}
                  </div>
                </div>
                <p class="mt-2 max-w-3xl text-sm text-zinc-600 dark:text-zinc-400">${demo.blurb}</p>
              </header>
              <div class="grid lg:grid-cols-2">
                <div class="demo-code grid grid-rows-[auto_1fr] border-b border-zinc-200 dark:border-zinc-800 lg:border-r lg:border-b-0" data-demo="${demo.id}">
                  <div class="flex items-center justify-between border-b border-zinc-200 px-5 py-2 dark:border-zinc-800">
                    <span class="font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">The code</span>
                    <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="${encodeURIComponent(demo.code)}">${COPY_SVG}</button>
                  </div>
                  <pre class="${PRE_CLASS} text-zinc-700 dark:text-zinc-300"><code>${escapeHtml(demo.code)}</code></pre>
                </div>
                <div class="demo-output grid" data-demo="${demo.id}">
                  <p class="px-5 py-12 text-center font-mono text-xs text-zinc-400">Fetching parser…</p>
                </div>
              </div>
              <footer class="border-t border-zinc-200 px-5 py-3 dark:border-zinc-800">
                <a href="${demo.source}" ${demo.source.startsWith("http") ? 'target="_blank" rel="noreferrer"' : ""}
                   class="inline-flex items-center gap-1.5 font-mono text-xs tracking-wider text-zinc-500 uppercase transition-colors hover:text-zinc-900 dark:hover:text-white">
                  Full example
                  <svg class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 8h10M9 4l4 4-4 4"/></svg>
                </a>
              </footer>
            </article>`,
          ).join("")}
        </div>

        <h2 class="mt-24 font-mono text-4xl font-bold tracking-tight text-zinc-900 dark:text-white">
          Examples you can clone and run.
        </h2>
        <p class="mt-4 max-w-2xl text-sm leading-relaxed text-zinc-600 dark:text-zinc-400">
          Frameworks, editors and terminals cannot be embedded in a page. Each of these is a
          directory in the repository.
        </p>
        <div class="mt-8 grid gap-px border border-zinc-200 bg-zinc-200 dark:border-zinc-800 dark:bg-zinc-800 sm:grid-cols-2 lg:grid-cols-3">
          ${CATALOG.map(
            (entry) => `
            <a href="${entry.href}" target="_blank" rel="noreferrer"
               class="group bg-white p-6 transition-colors hover:bg-zinc-50 dark:bg-[#09090b] dark:hover:bg-zinc-950">
              <h3 class="flex items-center gap-1.5 font-mono text-sm font-bold tracking-wider text-zinc-900 dark:text-white">
                ${entry.title}
                <svg class="h-3.5 w-3.5 text-zinc-400 transition-transform group-hover:translate-x-0.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 8h10M9 4l4 4-4 4"/></svg>
              </h3>
              <p class="mt-2 font-mono text-xs leading-relaxed text-zinc-500">${entry.detail}</p>
            </a>`,
          ).join("")}
        </div>
      </div>
    </section>`;
}

export function setupShowcase(root: HTMLElement) {
  for (const demo of DEMOS) {
    const output = root.querySelector<HTMLDivElement>(`.demo-output[data-demo="${demo.id}"]`)!;
    const code = root.querySelector<HTMLDivElement>(`.demo-code[data-demo="${demo.id}"] pre`)!;

    const observer = new IntersectionObserver(
      (entries) => {
        if (!entries[0].isIntersecting) return;
        observer.disconnect();
        void mount(demo, output, code);
      },
      { rootMargin: "300px" },
    );
    observer.observe(output);
  }
}

// A demo that cannot fetch its parser reports that in its own panel. One failure
// is one panel, on the page as much as in the library.
async function mount(demo: Demo, output: HTMLElement, code: HTMLElement) {
  try {
    output.innerHTML = await demo.run();
    demo.wire?.(output);
  } catch (error) {
    output.innerHTML = `<p class="px-5 py-12 text-center font-mono text-xs text-red-500">${escapeHtml(
      `${demo.title} did not render: ${String(error)}`,
    )}</p>`;
  }

  try {
    const language = demo.codeLanguage === "html" ? html : javascript;
    const hl = await highlighterFor(language);
    code.outerHTML = hl.highlight(demo.code, themed(language));
  } catch {
    // The snippet is already readable as plain text, so leave it alone.
  }
}
