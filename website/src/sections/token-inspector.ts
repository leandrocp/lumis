import { loadTheme } from "../data/themes";

export function renderTokenInspector() {
  return `
    <section id="inspector" class="py-24 sm:py-36">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#inspector" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-cyan-400">&lt;</span><span class="text-pink-400">Inspector</span> <span class="text-cyan-400">/&gt;</span>
        </a>
        <h2 class="mt-8 font-mono text-4xl font-bold tracking-tight text-zinc-900 dark:text-white">
          Tree-sitter parsing every token.
        </h2>

        <div class="mt-12 grid gap-8 lg:grid-cols-[1fr_320px]">
          <div>
            <div class="mb-3 flex items-center justify-between">
              <span class="font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">HTML + JavaScript</span>
              <div class="flex gap-3">
                <span class="font-mono text-[11px] text-zinc-400" id="inspector-token-count"></span>
                <span class="font-mono text-[11px] text-zinc-400" id="inspector-scope-count"></span>
                <span class="font-mono text-[11px] text-zinc-400" id="inspector-lang-count"></span>
              </div>
            </div>
            <div class="overflow-hidden border border-zinc-200 bg-white p-4 dark:border-zinc-800 dark:bg-[#0d1117] sm:p-5">
              <div id="inspector-output" class="inspector-output overflow-x-auto [&_code]:font-mono"></div>
            </div>
          </div>
          <div class="space-y-3">
            <div class="border border-zinc-200 p-4 dark:border-zinc-800">
              <span class="block font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Token</span>
              <code class="mt-1 block font-mono text-sm text-zinc-900 dark:text-white" id="inspector-text">-</code>
            </div>
            <div class="border border-zinc-200 p-4 dark:border-zinc-800">
              <span class="block font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Scope</span>
              <span class="mt-1 block font-mono text-sm text-zinc-900 dark:text-white" id="inspector-scope">-</span>
            </div>
            <div class="border border-zinc-200 p-4 dark:border-zinc-800">
              <span class="block font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Language</span>
              <span class="mt-1 block font-mono text-sm text-zinc-900 dark:text-white" id="inspector-language">-</span>
            </div>
            <div class="border border-zinc-200 p-4 dark:border-zinc-800">
              <span class="block font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Byte range</span>
              <span class="mt-1 block font-mono text-sm text-zinc-900 dark:text-white" id="inspector-range">-</span>
            </div>
            <div class="border border-zinc-200 p-4 dark:border-zinc-800">
              <span class="block font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Foreground</span>
              <span class="mt-1 block font-mono text-sm text-zinc-900 dark:text-white" id="inspector-fg">-</span>
            </div>
          </div>
        </div>
      </div>
    </section>`;
}

const INSPECTOR_SOURCE = `<article class="profile-card">
  <h2>User profile</h2>
  <script>
    async function loadUserProfile(userId) {
      const response = await fetch(\`/api/users/\${userId}\`)
      if (!response.ok) throw new Error('Failed to load')

      return response.json()
    }
  </script>
</article>`;

function themedColor(light: string | undefined, dark: string | undefined): string | undefined {
  return light && dark ? `light-dark(${light}, ${dark})` : (light ?? dark);
}

export async function setupTokenInspector(root: HTMLElement) {
  const output = root.querySelector<HTMLDivElement>("#inspector-output");
  if (!output) return;

  const observer = new IntersectionObserver(
    async (entries) => {
      const entry = entries[0];
      if (!entry.isIntersecting) return;
      observer.disconnect();
      await initInspector(root, output);
    },
    { rootMargin: "200px" },
  );
  observer.observe(output);
}

function tokenForeground(token: HTMLElement, dark: boolean): string {
  if (dark) return token.dataset.fgDark ?? token.dataset.fg ?? "none";
  return token.dataset.fg ?? "none";
}

function countTokens(tokens: HTMLElement[]): {
  scopeTotals: Map<string, number>;
  languageTotals: Map<string, number>;
} {
  const scopeTotals = new Map<string, number>();
  const languageTotals = new Map<string, number>();

  for (const token of tokens) {
    const scope = token.dataset.scope;
    if (!scope) continue;

    scopeTotals.set(scope, (scopeTotals.get(scope) ?? 0) + 1);
    const lang = token.dataset.language;
    if (lang) languageTotals.set(lang, (languageTotals.get(lang) ?? 0) + 1);
  }

  return { scopeTotals, languageTotals };
}

async function initInspector(root: HTMLElement, output: HTMLDivElement) {
  const [
    { createHighlighter, highlightIter, withWasm },
    { escape, openSpanTag, openPreTag, openCodeTag, closingTags, wrapLine, styleToCss },
    html,
    javascript,
    wasmHtml,
    wasmJavascript,
    lightTheme,
    darkTheme,
  ] = await Promise.all([
    import("@lumis-sh/lumis"),
    import("@lumis-sh/lumis/formatters/html"),
    import("@lumis-sh/lumis/langs/html").then((m) => m.default),
    import("@lumis-sh/lumis/langs/javascript").then((m) => m.default),
    import("@lumis-sh/wasm-html").then((m) => m.default),
    import("@lumis-sh/wasm-javascript").then((m) => m.default),
    loadTheme("catppuccin_latte"),
    loadTheme("catppuccin_frappe"),
  ]);

  const hl = await createHighlighter({
    languages: [withWasm(html, wasmHtml), withWasm(javascript, wasmJavascript)],
  });

  const docsFormatter = {
    language: html,
    format(source: string) {
      let tokenId = 0;
      const lines = [""];
      const darkStyles: Array<{ fg?: string; bg?: string } | undefined> = [];

      highlightIter(
        source,
        html,
        darkTheme,
        (
          text: string,
          _language: string,
          _range: { start: number; end: number },
          scope: string,
          style: { fg?: string; bg?: string } | undefined,
        ) => {
          const parts = text.split("\n");
          for (let i = 0; i < parts.length; i++) {
            if (parts[i].length > 0 && scope) darkStyles.push(style);
          }
        },
      );

      let darkStyleIndex = 0;
      function appendHighlightedText(
        text: string,
        language: string,
        range: { start: number; end: number },
        scope: string,
        style: { fg?: string; bg?: string } | undefined,
      ): void {
        const parts = text.split("\n");
        for (let i = 0; i < parts.length; i++) {
          const part = parts[i];
          if (part.length > 0) {
            lines[lines.length - 1] += scope
              ? tokenSpan(part, language, range, scope, style, darkStyles[darkStyleIndex++])
              : escape(part);
          }
          if (i < parts.length - 1) lines.push("");
        }
      }

      // Both palettes travel on the span so the page can switch colour scheme
      // without re-highlighting.
      function tokenSpan(
        part: string,
        language: string,
        range: { start: number; end: number },
        scope: string,
        style: { fg?: string; bg?: string } | undefined,
        darkStyle: { fg?: string; bg?: string } | undefined,
      ): string {
        const attrs = openSpanTag({
          class: "tok",
          tabindex: 0,
          "data-token-id": String(++tokenId),
          "data-scope": scope,
          "data-language": language,
          "data-start": String(range.start),
          "data-end": String(range.end),
          "data-fg": style?.fg,
          "data-bg": style?.bg,
          "data-fg-dark": darkStyle?.fg,
          "data-bg-dark": darkStyle?.bg,
          style: styleToCss(
            {
              fg: themedColor(style?.fg, darkStyle?.fg),
              bg: themedColor(style?.bg, darkStyle?.bg),
            },
            { italic: true },
          ),
        });

        return `${attrs}${escape(part)}</span>`;
      }

      highlightIter(source, html, lightTheme, appendHighlightedText);

      if (source.endsWith("\n")) lines.pop();
      const body = lines.map((line: string, i: number) => wrapLine(i + 1, line)).join("\n");
      return `${openPreTag({ preClass: "inspector-demo" })}${openCodeTag(html)}${body}${closingTags()}`;
    },
  };

  output.innerHTML = hl.highlight(INSPECTOR_SOURCE, docsFormatter);

  const tokens = [...output.querySelectorAll<HTMLElement>(".tok")];
  const { scopeTotals, languageTotals } = countTokens(tokens);

  const tokenCountEl = root.querySelector("#inspector-token-count");
  const scopeCountEl = root.querySelector("#inspector-scope-count");
  const langCountEl = root.querySelector("#inspector-lang-count");
  if (tokenCountEl) tokenCountEl.textContent = `${tokens.length} tokens`;
  if (scopeCountEl) scopeCountEl.textContent = `${scopeTotals.size} scopes`;
  if (langCountEl) langCountEl.textContent = `${languageTotals.size} languages`;

  const textEl = root.querySelector("#inspector-text")!;
  const scopeEl = root.querySelector("#inspector-scope")!;
  const languageEl = root.querySelector("#inspector-language")!;
  const rangeEl = root.querySelector("#inspector-range")!;
  const fgEl = root.querySelector("#inspector-fg")!;
  const colorScheme = window.matchMedia("(prefers-color-scheme: dark)");
  let activeToken: HTMLElement | null = null;

  function selectToken(token: HTMLElement) {
    activeToken = token;
    for (const t of tokens) t.classList.remove("is-active");
    token.classList.add("is-active");

    textEl.textContent = token.textContent ?? "";
    scopeEl.textContent = token.dataset.scope ?? "-";
    languageEl.textContent = token.dataset.language ?? "-";
    rangeEl.textContent = `${token.dataset.start ?? "?"}..${token.dataset.end ?? "?"}`;
    fgEl.textContent = tokenForeground(token, colorScheme.matches);
  }

  for (const token of tokens) {
    token.addEventListener("mouseenter", () => selectToken(token));
    token.addEventListener("focus", () => selectToken(token));
    token.addEventListener("click", () => selectToken(token));
  }

  colorScheme.addEventListener("change", () => {
    if (activeToken) selectToken(activeToken);
  });

  if (tokens[0]) selectToken(tokens[0]);
}
