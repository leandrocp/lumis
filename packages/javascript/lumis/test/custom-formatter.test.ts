import { beforeAll, describe, expect, it } from "vitest";

import dracula from "../../themes/dist/json/dracula.json";
import diff from "../langs/diff.ts";
import json from "../langs/json.ts";
import { createHighlighter, highlightEvents, highlightIter } from "../src/index.js";
import { type Formatter, htmlInline } from "../src/formatters.js";
import {
  closingTags,
  openCodeTag,
  openPreTag,
  openSpanTag,
  renderLinesFromEvents,
  wrapLine,
} from "../src/formatter/html.js";
import type { Theme } from "../src/types.js";
import { configureLocalWasmResolver } from "./wasm.js";

const theme: Theme = dracula;

describe("custom formatter", () => {
  beforeAll(() => {
    configureLocalWasmResolver(["diff", "json"]);
  }, 120_000);

  it("flows through the public formatter contract using rendered lines", async () => {
    const hl = await createHighlighter({ languages: [json] });

    const formatter: Formatter = {
      language: json,
      render(source: string, events) {
        const lines = renderLinesFromEvents(
          source,
          events,
          (scope) => `class="tok ${scope.replaceAll(".", "-")}"`,
        );

        const body = lines
          .map((line, index) =>
            wrapLine(index + 1, `${openSpanTag({ class: "line-no" })}${index + 1}</span>${line}`, {
              className: index === 0 ? "first-line" : undefined,
            }),
          )
          .join("");

        return `${openPreTag({ preClass: "custom-frame" })}${openCodeTag(json)}${body}${closingTags()}`;
      },
    };

    const output = hl.highlight('{"name":"lumis"}', formatter);

    expect(output).toContain('class="lumis custom-frame"');
    expect(output).toContain('class="language-json"');
    expect(output).toContain('class="line-no"');
    expect(output).toContain('class="tok string"');
    expect(output).toContain('class="l-line first-line"');
    expect(output).not.toContain("\n");
  }, 30_000);

  it("keeps built-in formatters as plain convenience objects", async () => {
    const hl = await createHighlighter({ languages: [json] });
    const output = hl.highlight('{"name":"lumis"}', htmlInline({ language: json, theme }));

    expect(output).toContain('<pre class="lumis"');
    expect(output).toContain('class="language-json"');
  }, 30_000);

  it("reports rainbow bracket scopes when highlightIter asks for them", async () => {
    const hl = await createHighlighter({ languages: [json] });
    const source = '{"outer":{"inner":[1]}}';

    const collect = (options?: { rainbowBrackets?: boolean }): string[] => {
      const scopes: string[] = [];
      const formatter: Formatter = {
        language: json,
        render(src: string) {
          highlightIter(
            src,
            this.language,
            undefined,
            (_text, _language, _range, scope) => {
              scopes.push(scope);
            },
            options,
          );
          return "";
        },
      };
      hl.highlight(source, formatter);
      return scopes;
    };

    const plain = collect();
    expect(plain).toContain("punctuation.bracket");
    expect(plain.some((scope) => scope.startsWith("punctuation.bracket.rainbow"))).toBe(false);

    const rainbow = collect({ rainbowBrackets: true });
    expect(rainbow).toContain("punctuation.bracket.rainbow.1");
    expect(rainbow).toContain("punctuation.bracket.rainbow.2");
  }, 30_000);

  it("reports rainbow bracket scopes from hl.highlightIter", async () => {
    const hl = await createHighlighter({ languages: [json] });
    const source = '{"outer":{"inner":[1]}}';

    const scopes: string[] = [];
    let text = "";
    hl.highlightIter(
      source,
      json,
      undefined,
      (token, _language, range, scope) => {
        expect(source.slice(range.start, range.end)).toBe(token);
        scopes.push(scope);
        text += token;
      },
      { rainbowBrackets: true },
    );

    expect(text).toBe(source);
    expect(scopes).toContain("punctuation.bracket.rainbow.1");
  }, 30_000);

  it("keeps real rainbow depth in the nested event API", async () => {
    const hl = await createHighlighter({ languages: [json] });
    const source = "[[[[[[[0]]]]]]]";
    let depths: number[] = [];
    let syntaxSmuggledRainbow = false;
    const formatter: Formatter = {
      language: json,
      render(rendered) {
        const events = highlightEvents(rendered, this.language, { rainbowBrackets: true });
        depths = events.flatMap((event) =>
          event.type === "decorationStart" && event.decoration.type === "rainbowBracket"
            ? [event.decoration.depth]
            : [],
        );
        syntaxSmuggledRainbow = events.some(
          (event) =>
            event.type === "start" && event.scope.startsWith("punctuation.bracket.rainbow"),
        );
        return "";
      },
    };

    hl.highlight(source, formatter);

    expect(depths).toContain(6);
    expect(Math.max(...depths)).toBe(6);
    expect(syntaxSmuggledRainbow).toBe(false);
  }, 30_000);

  it("restores the outer runtime after nested formatter calls", async () => {
    const outerHighlighter = await createHighlighter({ languages: [json] });
    const innerHighlighter = await createHighlighter({ languages: [diff] });

    const collectScopes = (source: string, language: Formatter["language"]): string[] => {
      const scopes: string[] = [];

      highlightIter(source, language, undefined, (text, tokenLanguage, _range, scope) => {
        scopes.push(`${tokenLanguage}:${scope}:${text}`);
      });

      return scopes;
    };

    const innerFormatter: Formatter = {
      language: diff,
      render(source: string) {
        return collectScopes(source, this.language).join("|");
      },
    };

    const outerFormatter: Formatter = {
      language: json,
      render(source: string) {
        const beforeNested = collectScopes(source, this.language);
        const nested = innerHighlighter.highlight("- old\n+ new", innerFormatter);
        const afterNested = collectScopes(source, this.language);

        return JSON.stringify({ beforeNested, nested, afterNested });
      },
    };

    const output = JSON.parse(outerHighlighter.highlight('{"name":"lumis"}', outerFormatter)) as {
      beforeNested: string[];
      nested: string;
      afterNested: string[];
    };

    expect(output.beforeNested).toEqual(output.afterNested);
    expect(output.nested).toContain("diff:");
    expect(output.afterNested.some((entry) => entry.startsWith("json:"))).toBe(true);
  }, 30_000);
});
