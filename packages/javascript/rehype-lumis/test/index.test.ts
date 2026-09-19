import type { Element, Properties, Root } from "hast";
import { describe, expect, it } from "vitest";
import { visit } from "unist-util-visit";
import dracula from "../../themes/dist/json/dracula.json";
import githubLight from "../../themes/dist/json/github_light.json";
import javascript from "../../lumis/langs/javascript.ts";
import json from "../../lumis/langs/json.ts";
import { configureLocalWasmResolver } from "../../lumis/test/wasm.ts";
import { configureLanguagePackageResolver, configureWasmResolver } from "@lumis-sh/lumis";
import { htmlInline, htmlLinked, htmlMultiThemes, terminal } from "@lumis-sh/lumis/formatters";
import rehypeLumis from "../src/index.js";

configureLocalWasmResolver(["javascript", "json"], {
  configureLanguagePackageResolver,
  configureWasmResolver,
});

function codeBlockTree({
  code = "const answer = 42",
  codeClassName,
  codeProperties = {},
  preProperties = {},
}: {
  code?: string;
  codeClassName?: string[];
  codeProperties?: Properties;
  preProperties?: Properties;
} = {}): Root {
  return {
    type: "root",
    children: [
      {
        type: "element",
        tagName: "pre",
        properties: preProperties,
        children: [
          {
            type: "element",
            tagName: "code",
            properties: {
              ...codeProperties,
              ...(codeClassName ? { className: codeClassName } : {}),
            },
            children: [{ type: "text", value: code }],
          },
        ],
      },
    ],
  };
}

function findElements(root: Root, tagName: string): Element[] {
  const results: Element[] = [];
  visit(root, "element", (node) => {
    if (node.tagName === tagName) results.push(node);
  });
  return results;
}

function classNames(element: Element): string[] {
  const value = element.properties.className;
  expect(Array.isArray(value)).toBe(true);
  return Array.isArray(value)
    ? value.filter((entry): entry is string => typeof entry === "string")
    : [];
}

function style(element: Element): string {
  const value = element.properties.style;
  expect(typeof value).toBe("string");
  return typeof value === "string" ? value : "";
}

function assertLumisPreElement(tree: Root) {
  const pres = findElements(tree, "pre");
  expect(pres.length).toBeGreaterThan(0);
  const pre = pres[0];
  expect(classNames(pre)).toContain("lumis");
  return pre;
}

function assertSpansExist(tree: Root) {
  const spans = findElements(tree, "span");
  expect(spans.length).toBeGreaterThan(0);
  return spans;
}

describe("rehype-lumis", () => {
  describe("fence languages the caller did not declare", () => {
    it("loads what the document names, like every other runtime", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      // The document asks for json; the caller only declared javascript.
      const tree = codeBlockTree({ code: '{"a": 1}', codeClassName: ["language-json"] });

      await transform(tree);

      const pre = assertLumisPreElement(tree);
      expect(pre).toBeDefined();
      // Highlighted as json, not dropped to plain text.
      expect(JSON.stringify(pre)).toContain("language-json");
    });

    it("costs one block, not the document, when the language cannot be loaded", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({ codeClassName: ["language-no-such-language"] });

      await transform(tree);

      const pre = assertLumisPreElement(tree);
      expect(pre).toBeDefined();
    });
  });

  describe("htmlInline formatter", () => {
    it("produces pre.lumis with inline styles and colored spans", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({ codeClassName: ["language-javascript"] });

      await transform(tree);

      const pre = assertLumisPreElement(tree);
      expect(style(pre)).toMatch(/color: #[0-9a-f]+/);
      expect(style(pre)).toMatch(/background-color: #[0-9a-f]+/);

      const spans = assertSpansExist(tree);
      const spanWithStyle = spans.find((span) => typeof span.properties.style === "string");
      expect(spanWithStyle).toBeDefined();

      const codes = findElements(tree, "code");
      expect(classNames(codes[0])).toContain("language-javascript");
    });

    it("applies preClass", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula, preClass: "my-pre" }),
        languages: [javascript],
      });
      const tree = codeBlockTree({ codeClassName: ["language-javascript"] });

      await transform(tree);

      const pre = assertLumisPreElement(tree);
      expect(classNames(pre)).toContain("my-pre");
    });

    it("preserves authored pre and code properties", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        codeClassName: ["language-javascript", "copy-target"],
        codeProperties: {
          id: "source",
          tabIndex: -1,
          translate: true,
          style: "font-weight: bold",
        },
        preProperties: {
          id: "panel",
          className: ["overflow-auto"],
          dataPanel: "install",
          role: "tabpanel",
          ariaLabel: "Install",
          style: "padding: 1rem",
        },
      });

      await transform(tree);

      const pre = assertLumisPreElement(tree);
      expect(pre.properties).toMatchObject({
        id: "panel",
        dataPanel: "install",
        role: "tabpanel",
        ariaLabel: "Install",
      });
      expect(classNames(pre)).toEqual(expect.arrayContaining(["lumis", "overflow-auto"]));
      expect(style(pre)).toContain("background-color");
      expect(style(pre)).toContain("padding: 1rem");

      const code = findElements(tree, "code")[0];
      // `translate` is enumerated rather than boolean, so `true` is the bare
      // attribute form and comes back from HTML as the empty value it means.
      expect(code.properties).toMatchObject({ id: "source", tabIndex: -1, translate: "" });
      expect(classNames(code)).toEqual(
        expect.arrayContaining(["language-javascript", "copy-target"]),
      );
      expect(style(code)).toBe("font-weight: bold");
    });

    it("preserves authored properties when the formatter wraps the block", async () => {
      const transform = rehypeLumis({
        formatter: (language) =>
          htmlInline({
            language,
            theme: dracula,
            header: {
              openTag: "<figure><figcaption>example.js</figcaption>",
              closeTag: "</figure>",
            },
          }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        codeClassName: ["language-javascript", "copy-target"],
        codeProperties: { id: "source" },
        preProperties: { id: "panel", className: ["overflow-auto"] },
      });

      await transform(tree);

      expect(findElements(tree, "figure")).toHaveLength(1);

      const pre = assertLumisPreElement(tree);
      expect(pre.properties.id).toBe("panel");
      expect(classNames(pre)).toEqual(expect.arrayContaining(["lumis", "overflow-auto"]));

      const code = findElements(tree, "code")[0];
      expect(code.properties.id).toBe("source");
      expect(classNames(code)).toEqual(
        expect.arrayContaining(["language-javascript", "copy-target"]),
      );
    });

    it("preserves authored children after code", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({ codeClassName: ["language-javascript"] });
      const pre = tree.children[0] as Element;
      pre.children.push({
        type: "element",
        tagName: "span",
        properties: { id: "cursor" },
        children: [{ type: "text", value: "_" }],
      });

      await transform(tree);

      const transformedPre = assertLumisPreElement(tree);
      expect(transformedPre.children[1]).toMatchObject({
        type: "element",
        tagName: "span",
        properties: { id: "cursor" },
      });
    });

    it("deduplicates classes while letting authored properties override defaults", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        codeClassName: ["language-javascript", "language-javascript"],
        codeProperties: { tabIndex: 7, translate: true },
        preProperties: { className: ["lumis", "custom"] },
      });

      await transform(tree);

      const pre = assertLumisPreElement(tree);
      expect(classNames(pre)).toEqual(["lumis", "custom"]);

      const code = findElements(tree, "code")[0];
      expect(classNames(code)).toEqual(["language-javascript"]);
      expect(code.properties.tabIndex).toBe(7);
      expect(code.properties.translate).toBe("");
    });

    it("keeps a boolean attribute bare and treats a false one as absent", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        preProperties: { inert: true, hidden: false },
        codeProperties: { tabIndex: false },
      });

      await transform(tree);

      const pre = assertLumisPreElement(tree);
      expect(pre.properties.inert).toBe(true);
      expect(pre.properties).not.toHaveProperty("hidden");

      // `false` is how hast spells an attribute the authored node did not
      // carry, not a request to strip one Lumis generates, so `tabindex`
      // stays. A formatter's own `codeAttrs: {tabindex: false}` still removes
      // it, because there `false` is the caller asking.
      const code = findElements(tree, "code")[0];
      expect(code.properties.tabIndex).toBe(0);
    });

    it("preserves and normalizes string-valued classes", async () => {
      let formattedLanguage: string | undefined;
      const transform = rehypeLumis({
        formatter: (language) => {
          formattedLanguage = language;
          return htmlInline({ language, theme: dracula });
        },
        languages: [javascript],
      });
      const tree = codeBlockTree({
        codeProperties: { className: "language-javascript copy-target" },
        preProperties: { className: "lumis overflow-auto" },
      });

      await transform(tree);

      expect(formattedLanguage).toBe("javascript");

      const pre = assertLumisPreElement(tree);
      expect(classNames(pre)).toEqual(["lumis", "overflow-auto"]);

      const code = findElements(tree, "code")[0];
      expect(classNames(code)).toEqual(["language-javascript", "copy-target"]);
    });
  });

  describe("htmlLinked formatter", () => {
    it("produces class-based spans without inline styles", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlLinked({ language }),
        languages: [json],
      });
      const tree = codeBlockTree({
        codeClassName: ["language-json"],
        code: '{"a": 1}',
      });

      await transform(tree);

      assertLumisPreElement(tree);
      const spans = assertSpansExist(tree);

      // All spans should have class, none should have style
      for (const span of spans) {
        expect(span.properties.className).toBeDefined();
        expect(span.properties.style).toBeUndefined();
      }
    });
  });

  describe("htmlMultiThemes formatter", () => {
    it("produces inline styles with CSS custom properties", async () => {
      const transform = rehypeLumis({
        formatter: (language) =>
          htmlMultiThemes({
            language,
            themes: { light: githubLight, dark: dracula },
            defaultTheme: "light",
          }),
        languages: [javascript],
      });
      const tree = codeBlockTree({ codeClassName: ["language-javascript"] });

      await transform(tree);

      const pre = assertLumisPreElement(tree);
      expect(classNames(pre)).toContain("lumis-themes");

      // Spans should contain CSS custom properties for the non-default theme
      const spans = assertSpansExist(tree);
      const spanStyles = spans
        .map((span) => span.properties.style)
        .filter((value): value is string => typeof value === "string");
      expect(spanStyles.some((value) => value.includes("--lumis-dark"))).toBe(true);
    });
  });

  describe("terminal formatter", () => {
    it("produces ANSI output, replacing the pre element", async () => {
      const transform = rehypeLumis({
        formatter: (language) => terminal({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({ codeClassName: ["language-javascript"] });

      await transform(tree);

      // terminal output is plain text with ANSI codes, parsed into text nodes
      const text = JSON.stringify(tree);
      expect(text).toMatch(/\\u001b\[38;2;\d+;\d+;\d+m/);
      expect(text).toContain("\\u001b[0m");
      // Should not have lumis class since it's not HTML
      expect(findElements(tree, "span").length).toBe(0);
    });
  });

  describe("language detection", () => {
    it("reads language from code element class", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({ codeClassName: ["language-javascript"] });

      await transform(tree);

      const codes = findElements(tree, "code");
      expect(classNames(codes[0])).toContain("language-javascript");
    });

    it("reads language from pre element class", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        preProperties: { className: ["language-javascript"] },
      });

      await transform(tree);

      assertLumisPreElement(tree);
    });

    it("reads language from pre data-language attribute", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        preProperties: { dataLanguage: "javascript" },
      });

      await transform(tree);

      assertLumisPreElement(tree);
    });

    it("reads language from pre language attribute", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        preProperties: { language: "javascript" },
      });

      await transform(tree);

      assertLumisPreElement(tree);
    });

    it("prefers code class over pre class", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript, json],
      });
      const tree = codeBlockTree({
        codeClassName: ["language-json"],
        preProperties: { className: ["language-javascript"] },
        code: '{"key": "value"}',
      });

      await transform(tree);

      const codes = findElements(tree, "code");
      expect(classNames(codes[0])).toContain("language-json");
    });

    it("ignores empty string language attributes", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        preProperties: { dataLanguage: "" },
      });

      await transform(tree);

      // No language detected, lumis auto-detects
      assertLumisPreElement(tree);
    });
  });

  describe("error handling", () => {
    it("preserves original node when language is not available", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
      });
      const tree = codeBlockTree({
        codeClassName: ["language-nonexistent_xyz"],
        code: "some code",
      });

      await transform(tree);

      const pre = tree.children[0] as Element;
      expect(pre.tagName).toBe("pre");
      const code = pre.children[0] as Element;
      expect(code.tagName).toBe("code");
    });
  });

  describe("tree structure", () => {
    it("skips pre elements without a code child", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree: Root = {
        type: "root",
        children: [
          {
            type: "element",
            tagName: "pre",
            properties: {},
            children: [{ type: "text", value: "not a code block" }],
          },
        ],
      };

      await transform(tree);

      const pre = tree.children[0] as Element;
      expect(pre.tagName).toBe("pre");
      expect(pre.children[0]).toMatchObject({ type: "text", value: "not a code block" });
    });

    it("skips non-pre elements", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree: Root = {
        type: "root",
        children: [
          {
            type: "element",
            tagName: "div",
            properties: {},
            children: [
              {
                type: "element",
                tagName: "code",
                properties: { className: ["language-javascript"] },
                children: [{ type: "text", value: "not inside pre" }],
              },
            ],
          },
        ],
      };

      await transform(tree);

      const div = tree.children[0] as Element;
      expect(div.tagName).toBe("div");
    });

    it("transforms multiple code blocks", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript, json],
      });
      const tree: Root = {
        type: "root",
        children: [
          {
            type: "element",
            tagName: "pre",
            properties: {},
            children: [
              {
                type: "element",
                tagName: "code",
                properties: { className: ["language-javascript"] },
                children: [{ type: "text", value: "const a = 1" }],
              },
            ],
          },
          {
            type: "element",
            tagName: "pre",
            properties: {},
            children: [
              {
                type: "element",
                tagName: "code",
                properties: { className: ["language-json"] },
                children: [{ type: "text", value: '{"b": 2}' }],
              },
            ],
          },
        ],
      };

      await transform(tree);

      const pres = findElements(tree, "pre");
      expect(pres.length).toBe(2);
      for (const pre of pres) {
        expect(classNames(pre)).toContain("lumis");
      }
    });

    it("preserves sibling non-pre elements", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree: Root = {
        type: "root",
        children: [
          {
            type: "element",
            tagName: "h1",
            properties: {},
            children: [{ type: "text", value: "Title" }],
          },
          {
            type: "element",
            tagName: "pre",
            properties: {},
            children: [
              {
                type: "element",
                tagName: "code",
                properties: { className: ["language-javascript"] },
                children: [{ type: "text", value: "const x = 1" }],
              },
            ],
          },
          {
            type: "element",
            tagName: "p",
            properties: {},
            children: [{ type: "text", value: "Paragraph" }],
          },
        ],
      };

      await transform(tree);

      expect((tree.children[0] as Element).tagName).toBe("h1");
      assertLumisPreElement(tree);
      const lastElement = tree.children.at(-1) as Element;
      expect(lastElement.tagName).toBe("p");
    });

    it("handles mixed success and failure across blocks", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree: Root = {
        type: "root",
        children: [
          {
            type: "element",
            tagName: "pre",
            properties: {},
            children: [
              {
                type: "element",
                tagName: "code",
                properties: { className: ["language-javascript"] },
                children: [{ type: "text", value: "const ok = true" }],
              },
            ],
          },
          {
            type: "element",
            tagName: "pre",
            properties: {},
            children: [
              {
                type: "element",
                tagName: "code",
                properties: { className: ["language-nonexistent_xyz"] },
                children: [{ type: "text", value: "will fail" }],
              },
            ],
          },
        ],
      };

      await transform(tree);

      const pres = findElements(tree, "pre");
      // First: highlighted
      expect(classNames(pres[0])).toContain("lumis");
      // Second: preserved
      const secondCode = pres[1].children[0] as Element;
      expect(secondCode.tagName).toBe("code");
    });

    it("handles empty code content", async () => {
      const transform = rehypeLumis({
        formatter: (language) => htmlInline({ language, theme: dracula }),
        languages: [javascript],
      });
      const tree = codeBlockTree({
        codeClassName: ["language-javascript"],
        code: "",
      });

      await transform(tree);

      assertLumisPreElement(tree);
    });
  });
});
