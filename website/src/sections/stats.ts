export function renderStats() {
  return `
    <section class="bg-surface">
      <div class="mx-auto max-w-6xl px-6">
        <div class="grid gap-x-10 sm:grid-cols-2 lg:grid-cols-4">
          <div class="py-10">
            <p class="font-mono text-2xl font-bold tabular-nums text-fg">7 <span class="text-accent">Runtimes</span></p>
            <p class="mt-1 font-mono text-xs text-muted">CLI, Rust, Elixir, JavaScript / TypeScript, Browsers / CDN. Java and Python maintained by the community.</p>
          </div>
          <div class="py-10">
            <p class="font-mono text-2xl font-bold tabular-nums text-fg">110+ <span class="text-accent">Languages</span></p>
            <p class="mt-1 font-mono text-xs text-muted">Compiled Tree-sitter grammars containing highlights and injections.</p>
          </div>
          <div class="py-10">
            <p class="font-mono text-2xl font-bold tabular-nums text-fg">200+ <span class="text-accent">Themes</span></p>
            <p class="mt-1 font-mono text-xs text-muted">Neovim colorschemes btw. Pick one of the built-in themes or bring your own.</p>
          </div>
          <div class="py-10">
            <p class="font-mono text-2xl font-bold tabular-nums text-fg">5 <span class="text-accent">Formatters</span></p>
            <p class="mt-1 font-mono text-xs text-muted">HTML Inline, HTML Linked, Multi Theme, Terminal, BBCode + bring your own.</p>
          </div>
        </div>
      </div>
    </section>`;
}
