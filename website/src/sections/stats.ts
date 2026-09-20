export function renderStats() {
  return `
    <section class="border-y border-zinc-200 dark:border-zinc-800">
      <div class="mx-auto max-w-6xl">
        <div class="grid gap-px bg-zinc-200 dark:bg-zinc-800 sm:grid-cols-2 lg:grid-cols-4">
          <div class="bg-white px-6 py-8 dark:bg-[#09090b]">
            <p class="font-mono text-2xl font-bold tabular-nums text-zinc-900 dark:text-white">6 <span class="text-pink-400">Runtimes</span></p>
            <p class="mt-1 font-mono text-xs text-zinc-500">CLI, Rust, Elixir, JavaScript / TypeScript, Browsers / CDN, Java. Multiple engines, same output.</p>
          </div>
          <div class="bg-white px-6 py-8 dark:bg-[#09090b]">
            <p class="font-mono text-2xl font-bold tabular-nums text-zinc-900 dark:text-white">110+ <span class="text-sky-400">Languages</span></p>
            <p class="mt-1 font-mono text-xs text-zinc-500">Compiled Tree-sitter grammars containing highlights and injections.</p>
          </div>
          <div class="bg-white px-6 py-8 dark:bg-[#09090b]">
            <p class="font-mono text-2xl font-bold tabular-nums text-zinc-900 dark:text-white">250+ <span class="text-amber-400">Themes</span></p>
            <p class="mt-1 font-mono text-xs text-zinc-500">Neovim colorschemes btw. Pick one of the built-in themes or bring your own.</p>
          </div>
          <div class="bg-white px-6 py-8 dark:bg-[#09090b]">
            <p class="font-mono text-2xl font-bold tabular-nums text-zinc-900 dark:text-white">5 <span class="text-emerald-400">Formatters</span></p>
            <p class="mt-1 font-mono text-xs text-zinc-500">HTML Inline, HTML Linked, Multi Theme, Terminal, BBCode + bring your own.</p>
          </div>
        </div>
      </div>
    </section>`;
}
