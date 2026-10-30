import { COPY_SVG, GITHUB_SVG } from "../lib/utils";

const installs = [
  ["cli", "curl -LsSf https://lumis.sh/install.sh | sh"],
  ["rust", "cargo add lumis"],
  ["javascript", "npm install @lumis-sh/lumis"],
  ["browser", "https://cdn.jsdelivr.net/npm/@lumis-sh/lumis/+esm"],
  ["elixir", '{:lumis, "~> 0.10"}'],
  ["java", "io.roastedroot:lumis4j:0.0.7"],
  ["python", "pip install fastpylight"],
] as const;

export function renderHero() {
  return `<section class="relative isolate overflow-hidden pt-36 pb-20 sm:pt-48 sm:pb-28"><div id="hero-wow" class="pointer-events-none absolute inset-0 -top-16 z-0 overflow-hidden" aria-hidden="true"></div><div class="pointer-events-none relative z-10 mx-auto max-w-4xl px-6 text-center [&_a]:pointer-events-auto [&_button]:pointer-events-auto"><img src="/brand/badge-light.svg" alt="Lumis" fetchpriority="high" class="mx-auto w-44 dark:hidden sm:w-52"><img src="/brand/badge-dark.svg" alt="Lumis" fetchpriority="high" class="mx-auto hidden w-44 dark:block sm:w-52"><h1 class="mt-8 font-display text-4xl font-extrabold leading-[1.05] tracking-tight text-fg sm:text-5xl lg:text-6xl">One highlighter, 7 runtimes.</h1><p class="mt-5 font-display text-2xl font-bold text-muted sm:text-3xl">Powered by Tree-sitter and Neovim themes.</p><div class="mt-12 flex items-center justify-center gap-3"><a href="#quickstart" class="inline-flex h-10 items-center gap-2 rounded-md bg-accent px-5 font-display text-sm font-bold text-accent-fg transition-colors hover:bg-accent-strong">get started<svg class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 8h10M9 4l4 4-4 4"/></svg></a><a href="https://github.com/leandrocp/lumis" target="_blank" rel="noreferrer" class="inline-flex h-10 items-center gap-2 rounded-md border border-control px-5 font-display text-sm font-bold text-fg transition-colors hover:border-fg hover:bg-accent-soft">${GITHUB_SVG}star on github</a></div><div class="pointer-events-auto mx-auto mt-6 max-w-2xl"><div class="flex overflow-x-auto border-b border-line sm:justify-center" role="tablist" aria-label="Install commands">${installs.map(([id], index) => `<button data-install="${id}" role="tab" aria-selected="${index === 0}" aria-controls="install-panel-${id}" class="install-tab shrink-0 cursor-pointer border-b-2 px-4 py-2 font-mono text-xs tracking-wider uppercase ${index === 0 ? "border-accent text-fg" : "border-transparent text-muted hover:text-fg"}">${id === "javascript" ? "npm" : id === "browser" ? "browsers" : id}</button>`).join("")}</div><div class="overflow-hidden rounded-b-lg border-x border-b border-line bg-surface">${installs.map(([id, command], index) => `<div data-install-panel="${id}" id="install-panel-${id}" role="tabpanel" class="${index === 0 ? "" : "hidden"} px-4 py-3"><div class="flex items-center justify-between gap-2"><code class="min-w-0 truncate font-mono text-sm text-fg"><span class="mr-2 text-subtle select-none">&gt;</span>${command.replaceAll(">", "&gt;")}</code><button class="copy-install shrink-0 cursor-pointer rounded-sm text-muted transition-colors hover:bg-accent-soft hover:text-fg" aria-label="Copy to clipboard" data-copy="${encodeURIComponent(command)}">${COPY_SVG}</button></div>${id === "java" ? '<p class="pt-3 text-left font-mono text-xs text-muted">Community-maintained by roastedroot.</p>' : id === "python" ? '<p class="pt-3 text-left font-mono text-xs text-muted">Community-maintained by Answer.AI.</p>' : ""}</div>`).join("")}</div></div></div></section>`;
}

export function setupHero(root: HTMLElement) {
  const host = root.querySelector<HTMLDivElement>("#hero-wow");
  if (!host) return;
  const idle =
    window.requestIdleCallback ??
    ((callback: IdleRequestCallback) => window.setTimeout(callback, 200));
  const start = () =>
    idle(() => import("../lib/hero-wow").then(({ mountHeroWow }) => mountHeroWow(host)), {
      timeout: 1500,
    });
  if (document.readyState === "complete") start();
  else window.addEventListener("load", start, { once: true });
}
