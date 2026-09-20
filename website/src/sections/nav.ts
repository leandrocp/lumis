import { BURGER_SVG, CLOSE_SVG, GITHUB_SVG } from "../lib/utils";

export function renderNav(home = "") {
  return `
    <nav class="fixed top-0 left-0 right-0 z-50 bg-white/80 backdrop-blur-lg dark:bg-[#09090b]/80">
      <div class="mx-auto flex h-16 max-w-6xl items-center justify-between px-6">
        <a href="/" class="flex items-center gap-2.5 font-mono text-sm font-bold tracking-wider text-zinc-900 uppercase dark:text-white">
          <span class="inline-flex h-7 w-7 items-center justify-center bg-zinc-900 text-[11px] font-black text-white dark:bg-white dark:text-zinc-900">L</span>
          Lumis
        </a>
        <div class="flex items-center gap-4 font-mono text-xs tracking-wider uppercase sm:gap-6">
          <a href="${home}#quickstart" class="hidden text-zinc-500 transition-colors hover:text-zinc-900 dark:hover:text-white md:inline-block">Quickstart</a>
          <a href="${home}#runtimes" class="hidden text-zinc-500 transition-colors hover:text-zinc-900 dark:hover:text-white md:inline-block">Runtimes</a>
          <a href="${home}#playground" class="hidden text-zinc-500 transition-colors hover:text-zinc-900 dark:hover:text-white md:inline-block">Playground</a>
          <a href="/comparison/" class="hidden text-zinc-500 transition-colors hover:text-zinc-900 dark:hover:text-white md:inline-block">Comparison</a>
          <a href="/showcase/" class="hidden text-zinc-500 transition-colors hover:text-zinc-900 dark:hover:text-white md:inline-block">Showcase</a>
          <a href="${home}#integrations" class="hidden text-zinc-500 transition-colors hover:text-zinc-900 dark:hover:text-white lg:inline-block">Integrations</a>
          <a href="${home}#formatters" class="hidden text-zinc-500 transition-colors hover:text-zinc-900 dark:hover:text-white lg:inline-block">Formatters</a>
          <a href="https://docs.lumis.sh"
             class="hidden items-center border border-zinc-900 bg-zinc-900 px-3 py-1.5 text-white transition-colors hover:bg-white hover:text-zinc-900 dark:border-white dark:bg-white dark:text-zinc-900 dark:hover:bg-transparent dark:hover:text-white md:inline-flex">
            Docs
          </a>
          <a href="https://github.com/leandrocp/lumis" target="_blank" rel="noreferrer"
              class="inline-flex items-center gap-1.5 border border-zinc-900 px-3 py-1.5 text-zinc-900 transition-colors hover:bg-zinc-900 hover:text-white dark:border-white dark:text-white dark:hover:bg-white dark:hover:text-zinc-900">
            ${GITHUB_SVG}
            <span class="hidden md:inline">GitHub</span>
          </a>
          <button class="mobile-menu-toggle cursor-pointer text-zinc-900 dark:text-white lg:hidden" aria-label="Toggle menu">
            ${BURGER_SVG}
          </button>
        </div>
      </div>
      <div class="mobile-menu hidden border-t border-zinc-200 bg-white/95 backdrop-blur-lg dark:border-zinc-800 dark:bg-[#09090b]/95 lg:hidden">
        <div class="mx-auto flex max-w-6xl flex-col gap-1 px-6 py-4 font-mono text-sm tracking-wider uppercase">
          <a href="https://docs.lumis.sh" class="mobile-menu-link mb-2 inline-flex w-fit items-center border border-zinc-900 bg-zinc-900 px-3 py-2 text-white transition-colors hover:bg-white hover:text-zinc-900 dark:border-white dark:bg-white dark:text-zinc-900 dark:hover:bg-transparent dark:hover:text-white">Docs</a>
          <a href="${home}#quickstart" class="mobile-menu-link block py-2 text-zinc-600 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white">Quickstart</a>
          <a href="${home}#runtimes" class="mobile-menu-link block py-2 text-zinc-600 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white">Runtimes</a>
          <a href="${home}#playground" class="mobile-menu-link block py-2 text-zinc-600 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white">Playground</a>
          <a href="/comparison/" class="mobile-menu-link block py-2 text-zinc-600 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white">Comparison</a>
          <a href="/showcase/" class="mobile-menu-link block py-2 text-zinc-600 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white">Showcase</a>
          <a href="${home}#integrations" class="mobile-menu-link block py-2 text-zinc-600 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white">Integrations</a>
          <a href="${home}#formatters" class="mobile-menu-link block py-2 text-zinc-600 transition-colors hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-white">Formatters</a>
        </div>
      </div>
    </nav>`;
}

export function setupNav(root: HTMLElement) {
  const menuToggle = root.querySelector<HTMLButtonElement>(".mobile-menu-toggle")!;
  const mobileMenu = root.querySelector<HTMLDivElement>(".mobile-menu")!;
  menuToggle.addEventListener("click", () => {
    const isOpen = !mobileMenu.classList.contains("hidden");
    mobileMenu.classList.toggle("hidden");
    menuToggle.innerHTML = isOpen ? BURGER_SVG : CLOSE_SVG;
  });
  root.querySelectorAll<HTMLAnchorElement>(".mobile-menu-link").forEach((link) => {
    link.addEventListener("click", () => {
      mobileMenu.classList.add("hidden");
      menuToggle.innerHTML = BURGER_SVG;
    });
  });
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && !mobileMenu.classList.contains("hidden")) {
      mobileMenu.classList.add("hidden");
      menuToggle.innerHTML = BURGER_SVG;
      menuToggle.focus();
    }
  });
}
