export function renderWasm() {
  return `
    <section id="parsers" class="py-24 sm:py-36">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#parsers" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-accent">&lt;</span><span class="text-accent">Parsers</span> <span class="text-accent">/&gt;</span>
        </a>
        <div class="mt-8">
          <h2 class="font-display text-4xl font-bold leading-tight tracking-tight text-fg">
            Pre-built WASM parsers.
          </h2>
          <div class="mt-8 grid gap-6 lg:grid-cols-2">
            <div class="border border-line rounded-lg">
               <div class="border-b border-line px-5 py-3 ">
                 <span class="font-mono text-[11px] tracking-wider text-muted uppercase dark:text-subtle">Install one parser package</span>
               </div>
               <div class="px-5 py-4">
                 <code class="font-mono text-sm text-muted"><span class="mr-2 text-subtle select-none">&gt;</span>npm install @lumis-sh/wasm-html</code>
               </div>
             </div>
            <div class="border border-line rounded-lg">
              <div class="border-b border-line px-5 py-3 ">
                <span class="font-mono text-[11px] tracking-wider text-muted uppercase dark:text-subtle">Or choose a preset bundle</span>
              </div>
              <div class="px-5 py-4">
                <code class="font-mono text-sm text-muted"><span class="mr-2 text-subtle select-none">&gt;</span>npm install @lumis-sh/wasm-bundle-full</code>
              </div>
            </div>
          </div>
          <dl class="mt-10 grid gap-6 lg:grid-cols-2">
              <div class="border-l-2 border-line py-1 pl-5 ">
                <dt class="font-mono text-sm font-medium text-fg">110+ grammars</dt>
                <dd class="mt-1 font-mono text-xs leading-relaxed text-muted">Every language includes highlight queries and injection support, tested for conformance.</dd>
              </div>
              <div class="border-l-2 border-line py-1 pl-5 ">
                <dt class="font-mono text-sm font-medium text-fg">Loaded when a document needs them</dt>
                <dd class="mt-1 font-mono text-xs leading-relaxed text-muted">Node reads each parser from the package a project installed, and loads the languages injected inside a document in the same pass. A browser loads the packages it imports. Nothing comes from a CDN unless you configure a resolver.</dd>
              </div>
              <div class="border-l-2 border-line py-1 pl-5 ">
                <dt class="font-mono text-sm font-medium text-fg">Persistent and verified</dt>
                <dd class="mt-1 font-mono text-xs leading-relaxed text-muted">Every parser Lumis downloads is checked against the SHA-256 its package declares before it runs. Node.js, Bun, Deno, browsers, and Elixir keep parsers across restarts.</dd>
              </div>
              <div class="border-l-2 border-line py-1 pl-5 ">
                <dt class="font-mono text-sm font-medium text-fg">Verified on npm</dt>
                <dd class="mt-1 font-mono text-xs leading-relaxed text-muted">Published with provenance. Browse all packages at
                  <a href="https://www.npmjs.com/search?q=keywords:lumis-sh" target="_blank" rel="noreferrer" class="text-muted underline decoration-line underline-offset-2 hover:text-accent">npm</a>.
                </dd>
              </div>
          </dl>
        </div>
      </div>
    </section>`;
}
