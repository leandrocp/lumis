// @vitest-environment jsdom

import { afterEach, describe, expect, it } from "vitest";
import { act } from "react";
import { createRoot } from "react-dom/client";
import { renderToStaticMarkup } from "react-dom/server";
import {
  configureLanguagePackageResolver,
  configureWasmResolver,
  createHighlighter,
} from "@lumis-sh/lumis";
import {
  configureLanguagePackageResolver as configureClientLanguagePackageResolver,
  configureWasmResolver as configureClientWasmResolver,
} from "@lumis-sh/lumis/client";
import { bundledLanguages } from "@lumis-sh/lumis/bundles/web";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import { configureLocalWasmResolver, ensureLocalParserWasmDataUrl } from "../../lumis/test/wasm.ts";
import dracula from "../../themes/themes/dracula.ts";
import githubLight from "../../themes/themes/github_light.ts";
import { CodeBlock, useLumis } from "../src/index.js";
import { renderCodeBlock } from "../src/server.js";

const SOURCE = "const x = 1";

globalThis.IS_REACT_ACT_ENVIRONMENT = true;

configureLocalWasmResolver(["javascript"], {
  configureLanguagePackageResolver,
  configureWasmResolver,
});
configureLocalWasmResolver(
  ["javascript"],
  {
    configureLanguagePackageResolver: configureClientLanguagePackageResolver,
    configureWasmResolver: configureClientWasmResolver,
  },
  (language, wasm) => ensureLocalParserWasmDataUrl(language, wasm.name),
);

function createDeferred<T>() {
  let resolve!: (value: T | PromiseLike<T>) => void;
  const promise = new Promise<T>((nextResolve) => {
    resolve = nextResolve;
  });

  return { promise, resolve };
}

async function createDeferredJavaScript() {
  const language = await bundledLanguages.javascript();
  const deferred = createDeferred<typeof language>();
  const lazy = Object.assign(() => deferred.promise, {
    id: language.id,
    aliases: language.aliases,
  });

  return { deferred, language, lazy };
}

async function waitForHtml(container: HTMLElement, expected: string) {
  const deadline = Date.now() + 2_000;

  while (!container.innerHTML.includes(expected)) {
    if (Date.now() >= deadline) {
      throw new Error(`Timed out waiting for ${expected}`);
    }

    await act(async () => {
      await new Promise((resolve) => {
        setTimeout(resolve, 10);
      });
    });
  }
}

function HookHarness({
  children,
  formatter,
  highlighter,
}: {
  children: string;
  formatter: ReturnType<typeof htmlInline>;
  highlighter?:
    | Awaited<ReturnType<typeof createHighlighter>>
    | Promise<Awaited<ReturnType<typeof createHighlighter>>>;
}) {
  const { content, error, isLoading } = useLumis({
    children,
    formatter,
    highlighter,
  });

  if (isLoading) {
    return <div data-state="loading" />;
  }
  if (error) {
    return <div data-error={error.message} />;
  }

  return content;
}

afterEach(() => {
  document.body.innerHTML = "";
});

describe("@lumis-sh/react", () => {
  it("renders highlighted React nodes on the server", async () => {
    const highlighter = await createHighlighter();

    const node = await renderCodeBlock({
      children: SOURCE,
      formatter: htmlInline({ language: bundledLanguages.javascript, theme: dracula }),
      highlighter,
    });

    const html = renderToStaticMarkup(node);

    expect(html).toMatch(
      /<pre class="lumis" style="color:#[0-9a-f]+;background-color:#[0-9a-f]+">/u,
    );
    expect(html).toMatch(/<code class="language-javascript"/u);
    expect(html).toMatch(/<span style="color:#[0-9a-f]+">const<\/span>/u);
  });

  it("supports string formatter languages through renderCodeBlock", async () => {
    const highlighter = await createHighlighter({ languages: [bundledLanguages] });

    const node = await renderCodeBlock({
      children: SOURCE,
      formatter: htmlInline({ language: "javascript", theme: dracula }),
      highlighter,
    });

    const html = renderToStaticMarkup(node);

    expect(html).toContain('class="language-javascript"');
    expect(html).toContain('<pre class="lumis"');
  });

  it("renders synchronously when given a resolved highlighter", async () => {
    const highlighter = await createHighlighter({ languages: [bundledLanguages] });
    await highlighter.loadLanguage("javascript");
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(
        <CodeBlock
          highlighter={highlighter}
          formatter={htmlInline({ language: "javascript", theme: dracula })}
        >
          {SOURCE}
        </CodeBlock>,
      );
    });

    expect(container.innerHTML).toContain('class="lumis"');
    expect(container.innerHTML).toContain('class="language-javascript"');

    await act(async () => {
      root.unmount();
    });
  });

  it("loads a lazy language then highlights on a resolved highlighter", async () => {
    const { deferred, language, lazy } = await createDeferredJavaScript();
    const highlighter = await createHighlighter({ languages: [{ javascript: lazy }] });
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(
        <CodeBlock
          highlighter={highlighter}
          formatter={htmlInline({ language: "javascript", theme: dracula })}
        >
          {SOURCE}
        </CodeBlock>,
      );
    });

    expect(container.innerHTML).toBe("");

    await act(async () => {
      deferred.resolve(language);
    });

    await waitForHtml(container, 'class="lumis"');
    expect(container.innerHTML).toContain('class="lumis"');
    expect(container.innerHTML).toContain('class="language-javascript"');

    await act(async () => {
      root.unmount();
    });
  });

  it("loads a lazy language handle then highlights on a resolved highlighter", async () => {
    const { deferred, language, lazy } = await createDeferredJavaScript();
    const highlighter = await createHighlighter({ languages: [{ javascript: lazy }] });
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(
        <CodeBlock
          highlighter={highlighter}
          formatter={htmlInline({ language: lazy, theme: dracula })}
        >
          {SOURCE}
        </CodeBlock>,
      );
    });

    expect(container.innerHTML).toBe("");

    await act(async () => {
      deferred.resolve(language);
    });

    await waitForHtml(container, 'class="lumis"');
    expect(container.innerHTML).toContain('class="lumis"');
    expect(container.innerHTML).toContain('class="language-javascript"');

    await act(async () => {
      root.unmount();
    });
  });

  it("renders null then highlights when given a promise", async () => {
    const deferred = createDeferred<Awaited<ReturnType<typeof createHighlighter>>>();
    const container = document.createElement("div");
    document.body.append(container);

    const root = createRoot(container);
    const highlighter = await createHighlighter({ languages: [bundledLanguages] });
    await highlighter.loadLanguage("javascript");

    await act(async () => {
      root.render(
        <CodeBlock
          highlighter={deferred.promise}
          formatter={htmlInline({ language: "javascript", theme: dracula })}
        >
          {SOURCE}
        </CodeBlock>,
      );
    });

    expect(container.innerHTML).toBe("");

    await act(async () => {
      deferred.resolve(highlighter);
    });

    await waitForHtml(container, 'class="lumis"');
    expect(container.innerHTML).toContain('class="lumis"');
    expect(container.innerHTML).toContain('class="language-javascript"');

    await act(async () => {
      root.unmount();
    });
  });

  it("supports custom rendering through useLumis", async () => {
    const deferred = createDeferred<Awaited<ReturnType<typeof createHighlighter>>>();
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    const highlighter = await createHighlighter({ languages: [bundledLanguages] });
    await highlighter.loadLanguage("javascript");

    await act(async () => {
      root.render(
        <HookHarness
          highlighter={deferred.promise}
          formatter={htmlInline({ language: "javascript", theme: dracula })}
        >
          {SOURCE}
        </HookHarness>,
      );
    });

    expect(container.innerHTML).toContain('data-state="loading"');

    await act(async () => {
      deferred.resolve(highlighter);
    });

    await waitForHtml(container, 'class="lumis"');
    expect(container.innerHTML).toContain('class="lumis"');
    expect(container.innerHTML).toContain('class="language-javascript"');

    await act(async () => {
      root.unmount();
    });
  });

  it("normalizes non-Error failures returned by useLumis", async () => {
    const highlighter = Promise.reject<Awaited<ReturnType<typeof createHighlighter>>>("boom");
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(
        <HookHarness
          highlighter={highlighter}
          formatter={htmlInline({ language: "javascript", theme: dracula })}
        >
          {SOURCE}
        </HookHarness>,
      );
    });

    expect(container.querySelector("[data-error]").dataset.error).toBe(
      "Failed to render code block",
    );

    await act(async () => {
      root.unmount();
    });
  });

  it("re-highlights when the formatter changes", async () => {
    const highlighter = await createHighlighter({ languages: [bundledLanguages] });
    await highlighter.loadLanguage("javascript");
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(
        <CodeBlock
          highlighter={highlighter}
          formatter={htmlInline({ language: "javascript", theme: dracula })}
        >
          {SOURCE}
        </CodeBlock>,
      );
    });

    const firstHtml = container.innerHTML;
    expect(firstHtml).toContain('class="lumis"');

    await act(async () => {
      root.render(
        <CodeBlock
          highlighter={highlighter}
          formatter={htmlInline({ language: "javascript", theme: githubLight })}
        >
          {SOURCE}
        </CodeBlock>,
      );
    });

    const secondHtml = container.innerHTML;
    expect(secondHtml).toContain('class="lumis"');
    expect(secondHtml).not.toEqual(firstHtml);

    await act(async () => {
      root.unmount();
    });
  });

  it("throws immediately for unknown string languages", async () => {
    const highlighter = await createHighlighter({ languages: [bundledLanguages] });
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await expect(
      act(async () => {
        root.render(
          <CodeBlock
            highlighter={highlighter}
            formatter={htmlInline({ language: "not-a-language", theme: dracula })}
          >
            {SOURCE}
          </CodeBlock>,
        );
      }),
    ).rejects.toThrow('Language "not-a-language" is not loaded.');

    await act(async () => {
      root.unmount();
    });
  });
});
