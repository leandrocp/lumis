import { BURGER_SVG, CLOSE_SVG, GITHUB_SVG } from "../lib/utils";

const links = [
  "Quickstart",
  "Runtimes",
  "Playground",
  "Comparison",
  "Showcase",
  "Integrations",
  "Community",
  "Formatters",
];

function hrefFor(label: string, home: string): string {
  if (label === "Comparison") return "/comparison/";
  if (label === "Showcase") return "/showcase/";
  return `${home}#${label.toLowerCase()}`;
}

export function renderNav(home = "", current?: string) {
  return `
    <nav class="fixed top-0 left-0 right-0 z-50 border-b border-line bg-canvas/80 backdrop-blur-lg">
      <div class="mx-auto flex h-16 max-w-6xl items-center justify-between px-6">
        <a href="/" aria-label="Lumis home"><picture><source srcset="/brand/lockup-horizontal-dark.svg" media="(prefers-color-scheme: dark)"><img src="/brand/lockup-horizontal-light.svg" alt="" width="108" height="40" class="h-10 w-auto"></picture></a>
        <div class="flex items-center gap-4 font-display text-sm font-bold text-muted sm:gap-6">
          ${links.map((label, index) => `<a href="${hrefFor(label, home)}"${label === current ? ' aria-current="page"' : ""} class="hidden transition-[color,background-color,border-color,text-decoration-color] hover:text-fg ${index > 4 ? "xl:inline-block" : "lg:inline-block"}">${label}</a>`).join("")}
          <a href="https://docs.lumis.sh" class="hidden h-10 items-center rounded-md bg-accent px-5 text-accent-fg transition-[color,background-color,border-color,text-decoration-color] hover:bg-accent-strong md:inline-flex">Docs</a>
          <a href="https://github.com/leandrocp/lumis" target="_blank" rel="noreferrer" class="inline-flex h-10 items-center gap-1.5 rounded-md border border-control px-3 text-fg transition-[color,background-color,border-color,text-decoration-color] hover:border-fg hover:bg-accent-soft">${GITHUB_SVG}<span class="hidden md:inline">GitHub</span></a>
          <button type="button" class="mobile-menu-toggle cursor-pointer text-fg xl:hidden" aria-label="Toggle menu" aria-expanded="false" aria-controls="mobile-menu">${BURGER_SVG}</button>
        </div>
      </div>
      <div id="mobile-menu" class="mobile-menu hidden border-t border-line bg-surface p-3 xl:hidden"><div class="mx-auto flex max-w-6xl flex-col gap-1 px-3 py-2 font-display font-bold text-muted"><a href="https://docs.lumis.sh" class="mobile-menu-link mb-2 inline-flex h-10 w-fit items-center rounded-md bg-accent px-5 text-accent-fg">Docs</a>${links.map((label) => `<a href="${hrefFor(label, home)}"${label === current ? ' aria-current="page"' : ""} class="mobile-menu-link rounded-sm px-3 py-2 transition-[color,background-color,border-color,text-decoration-color] hover:bg-accent-soft hover:text-fg">${label}</a>`).join("")}</div></div>
    </nav>`;
}

export function setupNav(root: HTMLElement) {
  const menuToggle = root.querySelector<HTMLButtonElement>(".mobile-menu-toggle")!;
  const mobileMenu = root.querySelector<HTMLDivElement>(".mobile-menu")!;
  const setOpen = (open: boolean) => {
    mobileMenu.classList.toggle("hidden", !open);
    menuToggle.setAttribute("aria-expanded", String(open));
    menuToggle.innerHTML = open ? CLOSE_SVG : BURGER_SVG;
  };
  menuToggle.addEventListener("click", () => setOpen(mobileMenu.classList.contains("hidden")));
  root
    .querySelectorAll<HTMLAnchorElement>(".mobile-menu-link")
    .forEach((link) => link.addEventListener("click", () => setOpen(false)));
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && !mobileMenu.classList.contains("hidden")) {
      setOpen(false);
      menuToggle.focus();
    }
  });
}
