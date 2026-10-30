export function renderFooter() {
  const links = [
    ["llms.txt", "https://docs.lumis.sh/llms.txt"],
    ["github", "https://github.com/leandrocp/lumis"],
    ["crates", "https://crates.io/crates/lumis"],
    ["npm", "https://www.npmjs.com/package/@lumis-sh/lumis"],
    ["hex", "https://hex.pm/packages/lumis"],
    ["maven", "https://central.sonatype.com/artifact/io.roastedroot/lumis4j"],
    ["pypi", "https://pypi.org/project/fastpylight/"],
  ] as const;
  return `<footer class="border-t border-line"><div class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 px-6 py-8 sm:flex-row"><div class="flex items-center gap-2.5 font-mono text-xs text-muted"><img src="/brand/icon-light.svg" alt="" class="h-6 w-6 dark:hidden"><img src="/brand/icon-dark.svg" alt="" class="hidden h-6 w-6 dark:block"><span>Lumis &middot; One highlighter, 7 runtimes.</span></div><div class="flex flex-wrap items-center justify-center gap-4 font-mono text-xs text-muted">${links.map(([label, href], index) => `${index ? '<span aria-hidden="true" class="size-[3px] self-center rounded-full bg-subtle"></span>' : ""}<a href="${href}"${href.startsWith("https://") ? ' target="_blank" rel="noreferrer"' : ""} class="text-accent no-underline hover:underline underline-offset-3">${label}</a>`).join("")}</div></div></footer>`;
}
