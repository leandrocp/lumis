import MarkdownIt from "markdown-it";
import { describe, expect, it } from "vitest";
import dracula from "../../themes/dist/json/dracula.json";
import githubLight from "../../themes/dist/json/github_light.json";
import javascript from "../../lumis/langs/javascript.ts";
import json from "../../lumis/langs/json.ts";
import { configureLocalWasmResolver } from "../../lumis/test/wasm.ts";
import markdownItLumis, { fromHighlighter } from "../src/index.js";
import {
  configureLanguagePackageResolver,
  configureWasmResolver,
  createHighlighter,
} from "@lumis-sh/lumis";
import type { LanguageRef } from "@lumis-sh/lumis";
import { htmlInline, htmlLinked, htmlMultiThemes, terminal } from "@lumis-sh/lumis/formatters";
import { bundledLanguages } from "@lumis-sh/lumis/bundles/web";

const JS_SOURCE = "```javascript\nconst x = 1\n```";
const JSON_SOURCE = '```json\n{"a": 1}\n```';

configureLocalWasmResolver(["javascript", "json"], {
  configureLanguagePackageResolver,
  configureWasmResolver,
});

async function renderWithFenceAttrs(
  attrs: Record<string, string>,
  options: { fenceAttrsOnPre?: boolean } = {},
): Promise<string> {
  const plugin = await markdownItLumis({
    formatter: (language) => htmlInline({ language, theme: dracula }),
    languages: [javascript],
    ...options,
  });
  const md = new MarkdownIt();
  md.core.ruler.after("block", "test-fence-attributes", (state) => {
    const fence = state.tokens.find((token) => token.type === "fence");
    if (!fence) throw new Error("expected a fence token");
    for (const [name, value] of Object.entries(attrs)) fence.attrSet(name, value);
    return true;
  });
  md.use(plugin);

  return md.render(JS_SOURCE);
}

describe("markdown-it-lumis", () => {
  describe("htmlInline formatter", () => {
    it("produces pre with lumis class, inline styles, and colored spans", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JS_SOURCE);

      expect(html).toMatch(
        /<pre class="lumis" style="color: #[0-9a-f]+; background-color: #[0-9a-f]+;">/,
      );
      expect(html).toMatch(/<code class="language-javascript"/);
      expect(html).toMatch(/<span style="color: #[0-9a-f]+;">const<\/span>/);
      expect(html).toMatch(/<span style="color: #[0-9a-f]+;">1<\/span>/);
      expect(html).toContain('translate="no"');
      expect(html).toContain('tabindex="0"');
      expect(html).toContain('<div class="l-line" data-line="1">');
      expect(html).toContain("</code></pre>");
    });

    it("applies preClass", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula, preClass: "my-pre" }),
        languages: [javascript],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JS_SOURCE);

      expect(html).toMatch(/<pre class="lumis my-pre"/);
    });

    it("loads an identifier-only reference from a registered bundle", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [bundledLanguages, { id: "javascript", aliases: ["js"] }],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JS_SOURCE);

      expect(html).toContain('class="language-javascript"');
      expect(html).toMatch(/<span style="color: #[0-9a-f]+;">const<\/span>/);
    });

    it("puts a fence's attributes on pre, where markdown-it-attrs puts them", async () => {
      const html = await renderWithFenceAttrs({
        id: "example",
        class: "authored-pre lumis",
        style: "padding: 1rem;",
        "data-panel": "javascript",
        role: "tabpanel",
        "aria-label": "JavaScript example",
        title: "Example",
      });

      expect(html).toMatch(/<pre class="lumis authored-pre"/);
      expect(html.match(/lumis/g)).toHaveLength(1);
      expect(html).toContain('id="example"');
      expect(html).toMatch(
        /style="color: #[0-9a-f]+; background-color: #[0-9a-f]+; padding: 1rem;"/,
      );
      expect(html).toContain('data-panel="javascript"');
      expect(html).toContain('role="tabpanel"');
      expect(html).toContain('aria-label="JavaScript example"');
      expect(html).toContain('title="Example"');
      // Untouched, because the attributes went to <pre>.
      expect(html).toContain('<code class="language-javascript" translate="no" tabindex="0">');
    });

    it("puts them on code when fenceAttrsOnPre is false, as markdown-it core does", async () => {
      const html = await renderWithFenceAttrs(
        {
          id: "example-code",
          class: "authored-code language-javascript",
          style: "font-variant-ligatures: none;",
          translate: "yes",
          tabindex: "-1",
        },
        { fenceAttrsOnPre: false },
      );

      expect(html).toContain(
        '<code class="language-javascript authored-code" translate="yes" tabindex="-1"',
      );
      expect(html.match(/language-javascript/g)).toHaveLength(1);
      expect(html).toContain('id="example-code"');
      expect(html).toContain('style="font-variant-ligatures: none;"');
      expect(html).toMatch(/<pre class="lumis" style="color: #[0-9a-f]+; background-color/);
    });

    it("removes an attribute Lumis generates when the fence asks for it", async () => {
      const html = await renderWithFenceAttrs({ tabindex: "" }, { fenceAttrsOnPre: false });

      expect(html).toContain('tabindex=""');
      expect(html).not.toContain('tabindex="0"');
    });

    it("leaves a fence without attributes exactly as the formatter wrote it", async () => {
      const plain = await renderWithFenceAttrs({});

      expect(plain).toMatch(/^<pre class="lumis" style="color: #[0-9a-f]+/);
      expect(plain).not.toContain("id=");
    });
  });

  describe("htmlLinked formatter", () => {
    it("produces class-based spans without inline styles", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlLinked({ language }),
        languages: [json],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JSON_SOURCE);

      expect(html).toMatch(/<pre class="lumis"><code class="language-json"/);
      expect(html).toMatch(/<span class="[a-z-]+">/);
      expect(html).not.toMatch(/style="/);
      expect(html).toContain("</code></pre>");
    });
  });

  describe("htmlMultiThemes formatter", () => {
    it("produces inline styles with CSS custom properties", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) =>
          htmlMultiThemes({
            language,
            themes: { light: githubLight, dark: dracula },
            defaultTheme: "light",
          }),
        languages: [javascript],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JS_SOURCE);

      expect(html).toMatch(/<pre class="lumis lumis-themes/);
      expect(html).toMatch(/--lumis-dark/);
      expect(html).toMatch(/<code class="language-javascript"/);
      expect(html).toContain("</code></pre>");
    });
  });

  describe("terminal formatter", () => {
    it("produces ANSI escape codes", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => terminal({ language, theme: dracula }),
        languages: [javascript],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JS_SOURCE);

      // terminal output has ANSI codes, no HTML tags
      // oxlint-disable-next-line no-control-regex -- matching ANSI escapes is the point
      expect(html).toMatch(/\u001B\[38;2;\d+;\d+;\d+m/);
      expect(html).toContain("\u001B[0m");
      expect(html).not.toContain("<pre");
      expect(html).not.toContain("<span");
    });
  });

  describe("language handling", () => {
    it.each([
      ["empty definition ID", { id: "", aliases: [] }],
      ["non-string definition alias", { id: "javascript", aliases: [1] }],
      ["empty lazy ID", Object.assign(async () => javascript, { id: "", aliases: [] })],
      [
        "non-string lazy alias",
        Object.assign(async () => javascript, { id: "javascript", aliases: [1] }),
      ],
    ])("rejects %s at the adapter boundary", async (_name, language) => {
      await expect(
        markdownItLumis({
          formatter: (name) => htmlInline({ language: name, theme: dracula }),
          languages: [language as unknown as LanguageRef],
        }),
      ).rejects.toThrow("Invalid markdown-it-lumis language metadata");
    });

    it("auto-detects unannotated fences", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render("```\nsome text\n```");

      expect(html).toContain("some text");
    });

    it("strips info string metadata, keeping only the language", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render('```javascript title="example"\nconst x = 1\n```');

      expect(html).toMatch(/<code class="language-javascript"/);
    });

    it("falls back to markdown-it default when language is not available", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JS_SOURCE);

      expect(html).toMatch(/<pre><code class="language-javascript">/);
    });

    it("renders different languages in the same document", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript, json],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(`${JS_SOURCE}\n\n${JSON_SOURCE}`);

      expect(html).toMatch(/<code class="language-javascript"/);
      expect(html).toMatch(/<code class="language-json"/);
    });

    it("accepts a bundle and loads languages by name", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [bundledLanguages, "javascript"],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JS_SOURCE);

      expect(html).toMatch(/<pre class="lumis"/);
      expect(html).toMatch(/<span style="color: #[0-9a-f]+;">/);
    });
  });

  describe("document structure", () => {
    it("renders multiple fenced blocks", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(
        "```javascript\nconst a = 1\n```\n\nSome text\n\n```javascript\nconst b = 2\n```",
      );

      expect((html.match(/<pre class="lumis"/g) ?? []).length).toBe(2);
      expect(html).toContain("<p>Some text</p>");
    });

    it("preserves non-fenced content", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render("# Title\n\nParagraph\n\n```javascript\ncode\n```\n\n- list item");

      expect(html).toContain("<h1>Title</h1>");
      expect(html).toContain("<p>Paragraph</p>");
      expect(html).toContain("<li>list item</li>");
      expect(html).toMatch(/<pre class="lumis"/);
    });

    it("handles empty code blocks", async () => {
      const plugin = await markdownItLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render("```javascript\n\n```");

      expect(html).toMatch(/<pre class="lumis"/);
      expect(html).toContain("</code></pre>");
    });
  });

  describe("fromHighlighter", () => {
    it("works with a pre-configured highlighter", async () => {
      const highlighter = await createHighlighter({ languages: [javascript] });
      const plugin = fromHighlighter(highlighter, {
        formatter: (language) => htmlInline({ language, theme: dracula }),
      });
      const md = new MarkdownIt();
      md.use(plugin);

      const html = md.render(JS_SOURCE);

      expect(html).toMatch(/<pre class="lumis"/);
      expect(html).toMatch(/<span style="color: #[0-9a-f]+;">const<\/span>/);
    });
  });
});
