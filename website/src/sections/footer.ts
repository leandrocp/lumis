export function renderFooter() {
  return `
    <footer class="border-t border-zinc-200 dark:border-zinc-800">
      <div class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-4 px-6 py-8 sm:flex-row">
        <div class="flex items-center gap-2.5 font-mono text-xs text-zinc-400">
          <span class="inline-flex h-5 w-5 items-center justify-center bg-zinc-900 text-[9px] font-black text-white dark:bg-white dark:text-zinc-900">L</span>
          <span>lumis &middot; syntax highlighting for every runtime</span>
        </div>
        <div class="flex items-center gap-4 font-mono text-xs text-zinc-400">
          <a href="https://docs.lumis.sh/llms.txt" class="transition-colors hover:text-zinc-900 dark:hover:text-white">llms.txt</a>
          <span class="text-zinc-300 dark:text-zinc-700">/</span>
          <a href="https://github.com/leandrocp/lumis" target="_blank" rel="noreferrer" class="transition-colors hover:text-zinc-900 dark:hover:text-white">github</a>
          <span class="text-zinc-300 dark:text-zinc-700">/</span>
          <a href="https://crates.io/crates/lumis" target="_blank" rel="noreferrer" class="transition-colors hover:text-zinc-900 dark:hover:text-white">crates</a>
          <span class="text-zinc-300 dark:text-zinc-700">/</span>
          <a href="https://www.npmjs.com/package/@lumis-sh/lumis" target="_blank" rel="noreferrer" class="transition-colors hover:text-zinc-900 dark:hover:text-white">npm</a>
          <span class="text-zinc-300 dark:text-zinc-700">/</span>
          <a href="https://hex.pm/packages/lumis" target="_blank" rel="noreferrer" class="transition-colors hover:text-zinc-900 dark:hover:text-white">hex</a>
          <span class="text-zinc-300 dark:text-zinc-700">/</span>
          <a href="https://central.sonatype.com/artifact/io.roastedroot/lumis4j" target="_blank" rel="noreferrer" class="transition-colors hover:text-zinc-900 dark:hover:text-white">maven</a>
        </div>
      </div>
    </footer>`;
}
