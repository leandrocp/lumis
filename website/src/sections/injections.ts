import { loadTheme } from "../data/themes";
import { LANGUAGES_BY_ID } from "../data/languages";
import { loadLanguages, renderHighlightMultiTheme } from "../lib/highlighter";

const HTML_CSS_JS_SOURCE = `<section class="greeting-card">
  <style>
    .greeting-card {
      border: 1px solid currentColor;
      padding: 1rem;
    }
  </style>

  <h1>Hello, Maya</h1>

  <script>
    const heading = document.querySelector('.greeting-card h1')
    console.log(heading.textContent)
  </script>
</section>`;

export function renderInjections() {
  return `
    <section id="injections" class="py-24 sm:py-28">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#injections" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-accent">&lt;</span><span class="text-accent">Injections</span> <span class="text-accent">/&gt;</span>
        </a>
        <h2 class="mt-8 font-display text-4xl font-bold leading-tight tracking-tight text-fg">
          Highlight nested injected languages.
        </h2>
        <div class="mt-12">
          <div class="mb-3 flex items-center justify-between">
            <span class="font-mono text-[11px] tracking-wider text-muted uppercase dark:text-subtle">3 languages</span>
            <div class="flex gap-3">
              <span class="inline-flex items-center gap-1.5 font-mono text-[10px] text-subtle">
                <span class="inline-block h-2 w-2 rounded-full bg-accent"></span>CSS
              </span>
              <span class="inline-flex items-center gap-1.5 font-mono text-[10px] text-subtle">
                <span class="inline-block h-2 w-2 rounded-full bg-accent"></span>HTML
              </span>
              <span class="inline-flex items-center gap-1.5 font-mono text-[10px] text-subtle">
                <span class="inline-block h-2 w-2 rounded-full bg-accent"></span>JavaScript
              </span>
            </div>
          </div>
          <div class="overflow-hidden rounded-lg bg-surface">
            <div class="injection-preview-html-nested [&_code]:font-mono"></div>
          </div>
        </div>

        <dl class="mt-12 grid gap-8 sm:grid-cols-3">
          <div class="py-1">
            <dt class="font-mono text-sm font-medium text-fg">Multi-level nesting</dt>
            <dd class="mt-1 font-mono text-xs leading-relaxed text-muted">Each injected language gets its own parser, loaded during the pass that finds it. No depth limits, no second pass.</dd>
          </div>
          <div class="py-1">
            <dt class="font-mono text-sm font-medium text-fg">Language-aware scopes</dt>
            <dd class="mt-1 font-mono text-xs leading-relaxed text-muted">Tokens know which language they belong to. Themes apply the right colors automatically.</dd>
          </div>
          <div class="py-1">
            <dt class="font-mono text-sm font-medium text-fg">Real embedded syntax</dt>
            <dd class="mt-1 font-mono text-xs leading-relaxed text-muted">Injected blocks are not plain text, so strings, keywords, and punctuation keep the right scopes.</dd>
          </div>
        </dl>
      </div>
    </section>`;
}

export async function setupInjections(_root: HTMLElement) {
  await loadLanguages(["html", "css", "javascript"]);

  const [lightTheme, darkTheme] = await Promise.all([
    loadTheme("catppuccin_latte"),
    loadTheme("catppuccin_frappe"),
  ]);

  const preClass =
    "m-0 overflow-x-auto p-5 font-mono text-[13px] leading-relaxed sm:p-6 sm:text-sm";

  const phpPreview = document.querySelector<HTMLDivElement>(".injection-preview-html-nested");
  const htmlCssJsLang = LANGUAGES_BY_ID.get("html");
  if (phpPreview && htmlCssJsLang) {
    const highlighted = await renderHighlightMultiTheme(
      htmlCssJsLang,
      lightTheme,
      darkTheme,
      HTML_CSS_JS_SOURCE,
      preClass,
    );
    phpPreview.innerHTML = highlighted;
  }
}
