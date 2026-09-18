import type { Plugin } from "vite";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { build } from "vite";
import { describe, expect, it } from "vitest";
import dracula from "../../themes/dist/json/dracula.json";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import lumis from "../src/index.js";

const plaintext = { id: "plaintext", aliases: ["text", "txt", "plain"] } as const;

function runTransform(plugin: Plugin, html: string): Promise<unknown> {
  const hook = plugin.transformIndexHtml;
  if (typeof hook !== "function") throw new TypeError("transformIndexHtml hook is missing");

  return Promise.resolve(
    hook.call({} as never, html, {
      path: "/index.html",
      filename: "/project/index.html",
    } as never),
  );
}

async function transformHtml(plugin: Plugin, html: string): Promise<string> {
  const result = await runTransform(plugin, html);
  if (typeof result !== "string") throw new TypeError("transformIndexHtml did not return HTML");

  return result;
}

function createPlugin(): Plugin {
  return lumis({
    languages: [plaintext],
    formatter: (language) => htmlInline({ language, theme: dracula }),
  });
}

describe("vite-plugin-lumis", () => {
  it("runs through Vite's HTML build pipeline", async () => {
    const root = await mkdtemp(join(process.cwd(), ".tmp-vite-plugin-lumis-"));
    try {
      await writeFile(
        join(root, "index.html"),
        '<!doctype html><pre id="example" data-language="plaintext"><code>built by Vite</code></pre>',
      );

      await build({
        root,
        logLevel: "silent",
        plugins: [createPlugin()],
      });

      const result = await readFile(join(root, "dist/index.html"), "utf8");
      expect(result).toContain('<pre class="lumis"');
      expect(result).toContain('id="example"');
      expect(result).toContain("built by Vite");
    } finally {
      await rm(root, { recursive: true, force: true });
    }
  });

  it("highlights static HTML and preserves authored attributes", async () => {
    const html = `<!doctype html>
<html>
  <head><title>Lumis</title></head>
  <body>
    <pre id="panel" class="overflow-auto" data-language="plaintext" data-panel="install" role="tabpanel" aria-label="Install" style="padding: 1rem"><code id="source" class="copy-target" tabindex="-1">&lt;span&gt;&amp;</code></pre>
  </body>
</html>`;

    const result = await transformHtml(createPlugin(), html);

    expect(result).toContain('<pre class="lumis overflow-auto"');
    expect(result).toContain('id="panel"');
    expect(result).toContain('data-panel="install"');
    expect(result).toContain('role="tabpanel"');
    expect(result).toContain('aria-label="Install"');
    expect(result).toContain("padding: 1rem");
    expect(result).toContain('<code class="language-plaintext copy-target"');
    expect(result).toContain('id="source"');
    expect(result).toContain('tabindex="-1"');
    expect(result).toContain("&#x3C;span>&#x26;");
  });

  it("uses one warmed processor for repeated HTML transforms", async () => {
    const plugin = createPlugin();
    const first = await transformHtml(
      plugin,
      '<!doctype html><pre data-language="plaintext"><code>first</code></pre>',
    );
    const second = await transformHtml(
      plugin,
      '<!doctype html><pre data-language="plaintext"><code>second</code></pre>',
    );

    expect(first).toContain("first");
    expect(second).toContain("second");
    expect(first).toContain('class="lumis"');
    expect(second).toContain('class="lumis"');
  });

  it("leaves unrelated markup alone", async () => {
    const html =
      '<!doctype html><pre>no code child</pre><main><code data-language="plaintext">inline</code></main>';

    const result = await transformHtml(createPlugin(), html);

    expect(result).toContain('<main><code data-language="plaintext">inline</code></main>');
    expect(result).not.toContain('class="lumis"');
  });

  it("returns a document with no pre element untouched", async () => {
    const html = '<div id="app"></div>\n<script type="module" src="/main.js"></script>\n';

    const result = await runTransform(createPlugin(), html);

    expect(result).toBeUndefined();
  });
});
