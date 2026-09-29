export function renderWasm() {
  return `
    <section id="parsers" class="py-24 sm:py-36">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#parsers" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-orange-400">&lt;</span><span class="text-cyan-400">Parsers</span> <span class="text-orange-400">/&gt;</span>
        </a>
        <div class="mt-8">
          <h2 class="font-mono text-4xl font-bold leading-tight tracking-tight text-zinc-900 dark:text-white">
            Pre-built WASM parsers.
          </h2>
          <div class="mt-8 grid gap-6 lg:grid-cols-2">
            <div class="border border-zinc-200 dark:border-zinc-800">
               <div class="border-b border-zinc-200 px-5 py-3 dark:border-zinc-800">
                 <span class="font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Install one parser package</span>
               </div>
               <div class="px-5 py-4">
                 <code class="font-mono text-sm text-zinc-700 dark:text-zinc-300"><span class="mr-2 text-zinc-400 select-none">&gt;</span>npm install @lumis-sh/wasm-html</code>
               </div>
             </div>
            <div class="border border-zinc-200 dark:border-zinc-800">
              <div class="border-b border-zinc-200 px-5 py-3 dark:border-zinc-800">
                <span class="font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Or choose a preset bundle</span>
              </div>
              <div class="px-5 py-4">
                <code class="font-mono text-sm text-zinc-700 dark:text-zinc-300"><span class="mr-2 text-zinc-400 select-none">&gt;</span>npm install @lumis-sh/wasm-bundle-full</code>
              </div>
            </div>
          </div>
          <dl class="mt-10 grid gap-6 lg:grid-cols-2">
              <div class="border-l-2 border-zinc-200 py-1 pl-5 dark:border-zinc-800">
                <dt class="font-mono text-sm font-medium text-zinc-900 dark:text-white">110+ grammars</dt>
                <dd class="mt-1 font-mono text-xs leading-relaxed text-zinc-500">Every language includes highlight queries and injection support, tested for conformance.</dd>
              </div>
              <div class="border-l-2 border-zinc-200 py-1 pl-5 dark:border-zinc-800">
                <dt class="font-mono text-sm font-medium text-zinc-900 dark:text-white">Loaded when a document needs them</dt>
                <dd class="mt-1 font-mono text-xs leading-relaxed text-zinc-500">Highlighting fetches, verifies and loads what a document turns out to name, including languages injected inside it, in a single pass. One it cannot fetch costs that block, not the document. Load ahead of time to keep the download off a first request.</dd>
              </div>
              <div class="border-l-2 border-zinc-200 py-1 pl-5 dark:border-zinc-800">
                <dt class="font-mono text-sm font-medium text-zinc-900 dark:text-white">Persistent and verified</dt>
                <dd class="mt-1 font-mono text-xs leading-relaxed text-zinc-500">Every parser Lumis downloads is checked against the SHA-256 its package declares before it runs. Node.js, Bun, Deno, browsers, and Elixir keep parsers across restarts.</dd>
              </div>
              <div class="border-l-2 border-zinc-200 py-1 pl-5 dark:border-zinc-800">
                <dt class="font-mono text-sm font-medium text-zinc-900 dark:text-white">Verified on npm</dt>
                <dd class="mt-1 font-mono text-xs leading-relaxed text-zinc-500">Published with provenance. Browse all packages at
                  <a href="https://www.npmjs.com/search?q=keywords:lumis-sh" target="_blank" rel="noreferrer" class="text-zinc-700 underline decoration-zinc-300 underline-offset-2 hover:text-zinc-900 dark:text-zinc-300 dark:decoration-zinc-700 dark:hover:text-white">npm</a>.
                </dd>
              </div>
          </dl>
        </div>
      </div>
    </section>`;
}
