import { renderNav, setupNav } from "./sections/nav";
import { renderHero, setupHero } from "./sections/hero";
import { renderStats } from "./sections/stats";
import { renderPlayground, setupPlayground } from "./sections/playground";
import { renderTokenInspector, setupTokenInspector } from "./sections/token-inspector";
import { renderStreaming, setupStreaming } from "./sections/streaming";
import { renderQuickstart, setupQuickstart } from "./sections/quickstart";
import { renderFormatters } from "./sections/formatters";
import { renderInjections, setupInjections } from "./sections/injections";
import { renderWasm } from "./sections/wasm";
import { renderCommunity } from "./sections/community";
import { renderRuntimes } from "./sections/runtimes";
import { renderIntegrations } from "./sections/integrations";
import { renderFooter } from "./sections/footer";
import { setupTabs, setupCopyButtons, SECTION_DIVIDER } from "./lib/utils";
import { FIRST_PAINT_LANGUAGES } from "./data/languages";
import { loadLanguages } from "./lib/highlighter";

export function renderApp() {
  return [
    '<a href="#main-content" class="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-[100] focus:bg-zinc-900 focus:px-4 focus:py-2 focus:font-mono focus:text-sm focus:text-white dark:focus:bg-white dark:focus:text-zinc-900">Skip to content</a>',
    renderNav(),
    '<main id="main-content">',
    renderHero(),
    renderStats(),
    renderPlayground(),
    SECTION_DIVIDER,
    renderQuickstart(),
    SECTION_DIVIDER,
    renderRuntimes(),
    SECTION_DIVIDER,
    renderIntegrations(),
    SECTION_DIVIDER,
    renderFormatters(),
    SECTION_DIVIDER,
    renderTokenInspector(),
    SECTION_DIVIDER,
    renderStreaming(),
    SECTION_DIVIDER,
    renderInjections(),
    SECTION_DIVIDER,
    renderWasm(),
    SECTION_DIVIDER,
    renderCommunity(),
    "</main>",
    renderFooter(),
    '<div id="live-region" class="sr-only" aria-live="polite" aria-atomic="true"></div>',
  ].join("\n");
}

export async function mountApp(root: HTMLDivElement) {
  // Deliberately not awaited: every section below still loads what it needs on
  // demand, so this only decides whether those parsers are already in flight by
  // the time they ask. Awaiting it would make the whole page wait for the
  // slowest one.
  void loadLanguages(FIRST_PAINT_LANGUAGES);

  // The build prerenders this markup into the page; the dev server does not.
  if (import.meta.env.DEV) root.innerHTML = renderApp();

  setupNav(root);
  setupHero(root);
  setupTabs(root, ".install-tab", "install", "[data-install-panel]", "installPanel");
  setupCopyButtons(root);

  // Playground first, so its parser and theme are the first things asked for,
  // but not awaited: nothing below needs its result, and `main.ts` does not wait
  // on `mountApp`, so awaiting only held the other four sections back.
  void setupPlayground(root);
  void setupTokenInspector(root);
  void setupStreaming(root);
  void setupQuickstart(root);
  void setupInjections(root);
}
