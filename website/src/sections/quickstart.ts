import { LANGUAGES_BY_ID } from "../data/languages";
import { loadTheme } from "../data/themes";
import { renderHighlightMultiTheme } from "../lib/highlighter";
import { COPY_SVG, escapeHtml, setupTabs } from "../lib/utils";

const QUICKSTART_TABS = [
  {
    id: "cli",
    label: "CLI",
    install: { language: "bash", code: `cargo install lumis-cli` },
    usage: {
      language: "bash",
      code: `lumis highlight index.js --lang javascript --theme dracula`,
    },
  },
  {
    id: "rust",
    label: "Rust",
    install: { language: "bash", code: `cargo add lumis` },
    usage: {
      language: "rust",
      code: `use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};

let theme = themes::get("dracula").unwrap();

let formatter = HtmlInlineBuilder::new()
    .language(Language::Javascript)
    .theme(Some(theme))
    .build()
    .unwrap();

let html = highlight("const x = 1", formatter);`,
    },
  },
  {
    id: "javascript",
    label: "JavaScript",
    install: { language: "bash", code: `npm install @lumis-sh/lumis @lumis-sh/themes` },
    usage: {
      language: "javascript",
      code: `import { highlight } from '@lumis-sh/lumis'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import javascript from '@lumis-sh/lumis/langs/javascript'
import dracula from '@lumis-sh/themes/dracula'

const html = await highlight(
  'const x = 1',
  htmlInline({ language: javascript, theme: dracula })
)`,
    },
  },
  {
    id: "cdn",
    label: "Browsers / CDN",
    install: null,
    usage: {
      language: "javascript",
      code: `import { highlight } from 'https://esm.sh/@lumis-sh/lumis'
import { htmlInline } from 'https://esm.sh/@lumis-sh/lumis/formatters'
import javascript from 'https://esm.sh/@lumis-sh/lumis/langs/javascript'
import dracula from 'https://esm.sh/@lumis-sh/themes/dracula'

document.getElementById('output').innerHTML = await highlight(
  'const x = 1',
  htmlInline({ language: javascript, theme: dracula })
)`,
    },
  },
  {
    id: "elixir",
    label: "Elixir",
    install: { language: "elixir", code: `{:lumis, "~> 0.7"}` },
    usage: {
      language: "elixir",
      code: `Lumis.highlight!(
  "const x = 1",
  language: "javascript",
  formatter: {:html_inline, theme: "dracula"}
)`,
    },
  },
  {
    id: "java",
    label: "Java",
    install: { language: "bash", code: `io.roastedroot:lumis4j:0.0.7` },
    usage: {
      language: "java",
      code: `import io.roastedroot.lumis4j.core.Lumis;
import io.roastedroot.lumis4j.core.Lang;
import io.roastedroot.lumis4j.core.Theme;

var lumis = Lumis.builder()
    .withLang(Lang.JAVASCRIPT)
    .withTheme(Theme.DRACULA)
    .build();

var result = lumis.highlight("const x = 1");`,
    },
  },
] as const;

export function renderQuickstart() {
  return `
    <section id="quickstart" class="py-24 sm:py-36">
      <div class="mx-auto max-w-4xl px-6">
        <a href="#quickstart" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-amber-400">&lt;</span><span class="text-violet-400">Quickstart</span> <span class="text-amber-400">/&gt;</span>
        </a>
        <h2 class="mt-8 font-mono text-4xl font-bold tracking-tight text-zinc-900 dark:text-white">
          Install, import, highlight.
        </h2>
        <div class="mt-6 flex flex-wrap items-center gap-3">
          <a href="https://docs.lumis.sh"
             class="inline-flex items-center gap-2 border border-zinc-200 px-4 py-2 font-mono text-xs tracking-wider text-zinc-700 uppercase transition-colors hover:border-zinc-900 hover:text-zinc-900 dark:border-zinc-800 dark:text-zinc-300 dark:hover:border-white dark:hover:text-white">
            docs
            <svg class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 8h10M9 4l4 4-4 4"/></svg>
          </a>
        </div>

        <div class="mt-12">
          <div class="flex gap-1 overflow-x-auto border-b border-zinc-200 dark:border-zinc-800" role="tablist">
            ${QUICKSTART_TABS.map(
              (tab, i) => `
              <button role="tab" data-tab="${tab.id}" aria-selected="${i === 0}" aria-controls="quickstart-panel-${tab.id}"
                class="quickstart-tab shrink-0 cursor-pointer border-b-2 px-4 py-2 font-mono text-xs tracking-wider uppercase transition-colors
                  ${i === 0 ? "border-zinc-900 text-zinc-900 dark:border-white dark:text-white" : "border-transparent text-zinc-500 dark:text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-300"}"
              >${tab.label}</button>
            `,
            ).join("")}
          </div>

          ${QUICKSTART_TABS.map(
            (tab, i) => `
            <div data-tab-panel="${tab.id}" id="quickstart-panel-${tab.id}" role="tabpanel" class="${i === 0 ? "" : "hidden"} mt-6 space-y-4">
              ${
                tab.install
                  ? `
              <div class="border border-zinc-200 dark:border-zinc-800">
                <div class="flex items-center justify-between border-b border-zinc-200 px-4 py-2 dark:border-zinc-800">
                  <span class="font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Install</span>
                  <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="${encodeURIComponent(tab.install.code)}">${COPY_SVG}</button>
                </div>
                <div class="quickstart-install [&_code]:font-mono" data-language="${tab.install.language}" data-code="${encodeURIComponent(tab.install.code)}">
                  <pre class="m-0 overflow-x-auto px-5 py-4 font-mono text-[13px] leading-7 text-zinc-700 dark:text-zinc-300 sm:px-6 sm:text-sm"><code>${escapeHtml(tab.install.code)}</code></pre>
                </div>
              </div>
              `
                  : ""
              }
              ${
                tab.id === "javascript"
                  ? `<p class="font-mono text-xs leading-6 text-zinc-500 dark:text-zinc-400">This install loads exact parser WASM per language and persists verified bytes, so restarts do not repeat downloads.</p>`
                  : ""
              }
              <div class="border border-zinc-200 dark:border-zinc-800">
                <div class="flex items-center justify-between border-b border-zinc-200 px-4 py-2 dark:border-zinc-800">
                  <span class="font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Usage</span>
                  <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="${encodeURIComponent(tab.usage.code)}">${COPY_SVG}</button>
                </div>
                <div class="quickstart-usage [&_code]:font-mono" data-language="${tab.usage.language}" data-code="${encodeURIComponent(tab.usage.code)}">
                  <pre class="m-0 overflow-x-auto px-5 py-4 font-mono text-[13px] leading-7 text-zinc-700 dark:text-zinc-300 sm:px-6 sm:text-sm"><code>${escapeHtml(tab.usage.code)}</code></pre>
                </div>
              </div>
            </div>
          `,
          ).join("")}
        </div>
      </div>
    </section>`;
}

export async function setupQuickstart(root: HTMLElement) {
  setupTabs(root, ".quickstart-tab", "tab", "[data-tab-panel]", "tabPanel");

  const [quickstartLightTheme, quickstartDarkTheme] = await Promise.all([
    loadTheme("catppuccin_latte"),
    loadTheme("catppuccin_frappe"),
  ]);
  root
    .querySelectorAll<HTMLDivElement>(".quickstart-install, .quickstart-usage")
    .forEach(async (el) => {
      const langId = el.dataset.language!;
      const code = decodeURIComponent(el.dataset.code!);
      const lang = LANGUAGES_BY_ID.get(langId);
      if (!lang) return;

      const highlighted = await renderHighlightMultiTheme(
        lang,
        quickstartLightTheme,
        quickstartDarkTheme,
        code,
        "m-0 overflow-x-auto p-5 font-mono text-[13px] leading-relaxed sm:p-6 sm:text-sm",
      );
      el.innerHTML = highlighted;
    });
}
