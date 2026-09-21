import { COPY_SVG, GITHUB_SVG } from "../lib/utils";

export function renderHero() {
  return `
    <section class="relative pt-36 pb-20 sm:pt-48 sm:pb-28">
      <div id="hero-fluid" class="absolute inset-0 -top-16 z-0 overflow-hidden" aria-hidden="true"></div>
      <div class="pointer-events-none relative z-10 mx-auto max-w-4xl px-6 text-center [&_a]:pointer-events-auto [&_button]:pointer-events-auto [&_select]:pointer-events-auto">
        <h1 class="font-mono text-4xl font-bold leading-[1.05] tracking-tighter text-zinc-900 sm:text-5xl lg:text-6xl dark:text-white">
          Syntax Highlighter powered by Tree-sitter
        </h1>
        <p class="mt-5 font-mono text-2xl font-medium text-zinc-500 sm:text-3xl dark:text-zinc-400">Unified API for 6 Runtimes</p>

        <div class="mt-12 flex items-center justify-center gap-3">
          <a href="#quickstart"
             class="inline-flex h-10 items-center gap-2 bg-zinc-900 px-5 font-mono text-sm font-medium text-white transition-colors hover:bg-zinc-700 dark:bg-white dark:text-zinc-900 dark:hover:bg-zinc-200">
            get started
            <svg class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 8h10M9 4l4 4-4 4"/></svg>
          </a>
          <a href="https://github.com/leandrocp/lumis" target="_blank" rel="noreferrer"
             class="inline-flex h-10 items-center gap-2 border border-zinc-200 px-5 font-mono text-sm text-zinc-600 transition-colors hover:border-zinc-400 hover:text-zinc-900 dark:border-zinc-800 dark:text-zinc-400 dark:hover:border-zinc-600 dark:hover:text-white">
            ${GITHUB_SVG}
            star on github
          </a>
        </div>

        <div class="mx-auto mt-6 max-w-lg">
          <div class="flex justify-center overflow-x-auto border-b border-zinc-200 dark:border-zinc-800" role="tablist" aria-label="Install commands">
            <button data-install="cli" role="tab" aria-selected="true" aria-controls="install-panel-cli" class="install-tab shrink-0 cursor-pointer border-b-2 border-zinc-900 px-4 py-2 font-mono text-xs tracking-wider text-zinc-900 uppercase dark:border-white dark:text-white">cli</button>
            <button data-install="rust" role="tab" aria-selected="false" aria-controls="install-panel-rust" class="install-tab shrink-0 cursor-pointer border-b-2 border-transparent px-4 py-2 font-mono text-xs tracking-wider text-zinc-500 uppercase hover:text-zinc-600 dark:text-zinc-400 dark:hover:text-zinc-300">cargo</button>
            <button data-install="javascript" role="tab" aria-selected="false" aria-controls="install-panel-javascript" class="install-tab shrink-0 cursor-pointer border-b-2 border-transparent px-4 py-2 font-mono text-xs tracking-wider text-zinc-500 uppercase hover:text-zinc-600 dark:text-zinc-400 dark:hover:text-zinc-300">npm</button>
            <button data-install="browser" role="tab" aria-selected="false" aria-controls="install-panel-browser" class="install-tab shrink-0 cursor-pointer border-b-2 border-transparent px-4 py-2 font-mono text-xs tracking-wider text-zinc-500 uppercase hover:text-zinc-600 dark:text-zinc-400 dark:hover:text-zinc-300">browsers</button>
            <button data-install="elixir" role="tab" aria-selected="false" aria-controls="install-panel-elixir" class="install-tab shrink-0 cursor-pointer border-b-2 border-transparent px-4 py-2 font-mono text-xs tracking-wider text-zinc-500 uppercase hover:text-zinc-600 dark:text-zinc-400 dark:hover:text-zinc-300">hex</button>
            <button data-install="java" role="tab" aria-selected="false" aria-controls="install-panel-java" class="install-tab shrink-0 cursor-pointer border-b-2 border-transparent px-4 py-2 font-mono text-xs tracking-wider text-zinc-500 uppercase hover:text-zinc-600 dark:text-zinc-400 dark:hover:text-zinc-300">maven</button>
          </div>
          <div class="border-x border-b border-zinc-200 dark:border-zinc-800">
            <div data-install-panel="cli" id="install-panel-cli" role="tabpanel" class="flex items-center justify-between gap-2 px-4 py-3">
              <code class="min-w-0 truncate font-mono text-sm text-zinc-700 dark:text-zinc-300"><span class="mr-2 text-zinc-400 select-none">&gt;</span>curl -LsSf https://lumis.sh/install.sh | sh</code>
              <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="curl -LsSf https://lumis.sh/install.sh | sh">${COPY_SVG}</button>
            </div>
            <div data-install-panel="rust" id="install-panel-rust" role="tabpanel" class="hidden">
              <div class="flex items-center justify-between gap-2 px-4 py-3">
                <code class="font-mono text-sm text-zinc-700 dark:text-zinc-300"><span class="mr-2 text-zinc-400 select-none">&gt;</span>cargo add lumis</code>
                <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="cargo add lumis">${COPY_SVG}</button>
              </div>
            </div>
            <div data-install-panel="javascript" id="install-panel-javascript" role="tabpanel" class="hidden">
              <div class="flex items-center justify-between gap-2 px-4 py-3">
                <code class="min-w-0 truncate font-mono text-sm text-zinc-700 dark:text-zinc-300"><span class="mr-2 text-zinc-400 select-none">&gt;</span>npm install @lumis-sh/lumis</code>
                <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="npm install @lumis-sh/lumis">${COPY_SVG}</button>
              </div>
            </div>
            <div data-install-panel="browser" id="install-panel-browser" role="tabpanel" class="hidden">
              <div class="flex items-center justify-between gap-2 px-4 py-3">
                <code class="min-w-0 truncate font-mono text-sm text-zinc-700 dark:text-zinc-300"><span class="mr-2 text-zinc-400 select-none">&gt;</span>https://esm.sh/@lumis-sh/lumis</code>
                <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="https://esm.sh/@lumis-sh/lumis">${COPY_SVG}</button>
              </div>
            </div>
            <div data-install-panel="elixir" id="install-panel-elixir" role="tabpanel" class="hidden">
              <div class="flex items-center justify-between gap-2 px-4 py-3">
                <code class="font-mono text-sm text-zinc-700 dark:text-zinc-300"><span class="mr-2 text-zinc-400 select-none">&gt;</span>{:lumis, "~&gt; 0.7"}</code>
                <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy='{:lumis, "~> 0.7"}'>${COPY_SVG}</button>
              </div>
            </div>
            <div data-install-panel="java" id="install-panel-java" role="tabpanel" class="hidden">
              <div class="flex items-center justify-between gap-2 px-4 py-3">
                <code class="font-mono text-sm text-zinc-700 dark:text-zinc-300"><span class="mr-2 text-zinc-400 select-none">&gt;</span>io.roastedroot:lumis4j:0.0.7</code>
                <button class="copy-install shrink-0 cursor-pointer text-zinc-400 transition-colors hover:text-zinc-900 dark:hover:text-white" aria-label="Copy to clipboard" data-copy="io.roastedroot:lumis4j:0.0.7">${COPY_SVG}</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>`;
}

export function setupHero(root: HTMLElement) {
  import("../lib/hero-fluid").then(({ mountHeroFluid }) => {
    const heroFluidContainer = root.querySelector<HTMLDivElement>("#hero-fluid");
    if (
      heroFluidContainer &&
      window.matchMedia("(min-width: 640px)").matches &&
      !window.matchMedia("(prefers-reduced-motion: reduce)").matches
    ) {
      mountHeroFluid(heroFluidContainer);
    }
  });
}
