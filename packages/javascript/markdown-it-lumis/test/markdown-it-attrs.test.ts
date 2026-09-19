/**
 * Against the real `markdown-it-attrs`, not a hand-set `token.attrs`.
 *
 * The two plugins have to agree about an element neither of them names to the
 * other. `markdown-it-attrs` sets the attributes from a core rule and then
 * declines to place them as soon as it sees a custom `fence` renderer, which
 * this plugin installs, so where they land is entirely this plugin's decision
 * and only an end-to-end render can show it made the same one.
 *
 * This is the pairing that does not work anywhere else: `@shikijs/markdown-it`
 * loses these attributes outright (shikijs/shiki#852), because markdown-it's
 * default fence renderer returns early on a `<pre`-prefixed highlight result
 * and never reaches the branch that would have rendered them.
 */
import MarkdownIt from "markdown-it";
import markdownItAttrs from "markdown-it-attrs";
import { describe, expect, it } from "vitest";
import dracula from "../../themes/dist/json/dracula.json";
import javascript from "../../lumis/langs/javascript.ts";
import { configureLocalWasmResolver } from "../../lumis/test/wasm.ts";
import markdownItLumis from "../src/index.js";
import { configureLanguagePackageResolver, configureWasmResolver } from "@lumis-sh/lumis";
import { htmlInline } from "@lumis-sh/lumis/formatters";

configureLocalWasmResolver(["javascript"], {
  configureLanguagePackageResolver,
  configureWasmResolver,
});

const SOURCE = '```javascript {#example .code-card data-panel="install"}\nconst x = 1\n```';

async function render(options: { fenceAttrsOnPre?: boolean } = {}): Promise<string> {
  const plugin = await markdownItLumis({
    formatter: (language) => htmlInline({ language, theme: dracula }),
    languages: [javascript],
    ...options,
  });
  const md = new MarkdownIt().use(markdownItAttrs).use(plugin);

  return md.render(SOURCE);
}

describe("with markdown-it-attrs", () => {
  it("renders a fence's attributes on pre, as markdown-it-attrs would have", async () => {
    const html = await render();

    expect(html).toMatch(/^<pre class="lumis code-card"/);
    expect(html).toContain('id="example"');
    expect(html).toContain('data-panel="install"');
    expect(html).toContain('<code class="language-javascript"');
  });

  it("renders them on code when asked, as markdown-it core would have", async () => {
    const html = await render({ fenceAttrsOnPre: false });

    expect(html).toMatch(/^<pre class="lumis" style=/);
    expect(html).toMatch(/<code class="language-javascript code-card"[^>]*id="example"/);
    expect(html).toContain('data-panel="install"');
  });

  /**
   * `fenceAttrsOnPre` defaults to `true` in `markdown-it-attrs` 5, but the
   * renderer that honours it only installs when the plugin recognises
   * markdown-it's default `fence` rule by reference. It compares against the
   * rule from its own `require('markdown-it')`, so under ESM — a different
   * module instance, a different function — the check says "custom renderer"
   * and the wrapper stands down. An ESM app therefore sees `<code>` from
   * `markdown-it-attrs` alone, whatever the option says.
   *
   * Lumis installs a custom `fence` renderer, so the wrapper stands down for
   * it either way and the placement is Lumis' to pick. It picks the one the
   * option documents rather than the one the reference check happens to leave.
   */
  it("places them where markdown-it-attrs documents, not where ESM leaves them", async () => {
    const attrsOnly = new MarkdownIt().use(markdownItAttrs).render(SOURCE);
    expect(attrsOnly).toMatch(/<code[^>]*id="example"/);
    expect(attrsOnly).not.toMatch(/<pre[^>]*id="example"/);

    const highlighted = await render();
    expect(highlighted).toMatch(/<pre[^>]*id="example"/);
    expect(highlighted).not.toMatch(/<code[^>]*id="example"/);

    const onCode = await render({ fenceAttrsOnPre: false });
    expect(onCode).toMatch(/<code[^>]*id="example"/);
    expect(onCode).not.toMatch(/<pre[^>]*id="example"/);
  });

  it("leaves a fence carrying no attributes alone", async () => {
    const plugin = await markdownItLumis({
      formatter: (language) => htmlInline({ language, theme: dracula }),
      languages: [javascript],
    });
    const md = new MarkdownIt().use(markdownItAttrs).use(plugin);

    const html = md.render("```javascript\nconst x = 1\n```");

    expect(html).toMatch(/^<pre class="lumis" style="color: #[0-9a-f]+/);
    expect(html).not.toContain("id=");
  });
});
