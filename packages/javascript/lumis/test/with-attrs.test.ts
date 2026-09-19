import { describe, expect, it } from "vitest";
import type { Formatter } from "../src/types.js";
import {
  bbcodeScoped,
  htmlInline,
  htmlLinked,
  htmlMultiThemes,
  withAttrs,
} from "../src/formatter.js";
import { terminal } from "../src/terminal.js";
import theme from "../../themes/dist/json/dracula.json";

const source = "x";
const events = [] as never[];

function render(formatter: Formatter): string {
  return formatter.render(source, events);
}

describe("withAttrs", () => {
  it("derives a formatter without touching the one it was given", () => {
    const base = htmlLinked({ language: "javascript" });
    const derived = withAttrs(base, { preAttrs: { id: "example" } });

    expect(render(derived)).toContain('id="example"');
    expect(render(base)).not.toContain("id=");
    expect(base).not.toBe(derived);
  });

  it("keeps the options the base formatter was built with", () => {
    const base = htmlLinked({ language: "javascript", preClass: "card" });
    const derived = withAttrs(base, { preAttrs: { id: "example" } });

    expect(render(derived)).toContain('class="lumis card"');
    expect(render(derived)).toContain('class="language-javascript"');
  });

  it("unions class, appends style, and replaces everything else", () => {
    const base = htmlInline({ language: "javascript", theme, preClass: "card" });
    const derived = withAttrs(base, {
      preAttrs: { class: "wide", style: "padding: 1rem;", id: "example" },
      codeAttrs: { translate: "yes" },
    });
    const html = render(derived);

    expect(html).toContain('class="lumis card wide"');
    expect(html).toMatch(/style="color: #f8f8f2; background-color: #282a36; padding: 1rem;"/);
    expect(html).toContain('id="example"');
    expect(html).toContain('translate="yes"');
    expect(html).not.toContain('translate="no"');
  });

  it("layers over attributes the base formatter already carried", () => {
    const base = htmlLinked({
      language: "javascript",
      preAttrs: { class: "first", id: "base", style: "margin: 0;" },
    });
    const derived = withAttrs(base, {
      preAttrs: { class: "second", id: "override", style: "padding: 1rem;" },
    });
    const html = render(derived);

    expect(html).toContain('class="lumis first second"');
    expect(html).toContain('id="override"');
    expect(html).toContain("margin: 0; padding: 1rem;");
  });

  it("removes a generated attribute asked for with false", () => {
    const base = htmlLinked({ language: "javascript" });

    expect(render(base)).toContain('tabindex="0"');
    expect(render(withAttrs(base, { codeAttrs: { tabindex: false } }))).not.toContain("tabindex");
  });

  it("carries the themes a multi-theme formatter needs through the rebuild", () => {
    const base = htmlMultiThemes({ language: "javascript", themes: { dark: theme } });
    const html = render(withAttrs(base, { preAttrs: { id: "example" } }));

    expect(html).toContain('id="example"');
    expect(html).toContain("--lumis-dark");
  });

  it("returns formatters with no attribute contract untouched", () => {
    const ansi = terminal({ language: "javascript", theme });
    const bbcode = bbcodeScoped({ language: "javascript" });
    const custom: Formatter = { render: () => "<pre><code>custom</code></pre>" };

    expect(withAttrs(ansi, { preAttrs: { id: "x" } })).toBe(ansi);
    expect(withAttrs(bbcode, { preAttrs: { id: "x" } })).toBe(bbcode);
    expect(withAttrs(custom, { preAttrs: { id: "x" } })).toBe(custom);
  });

  it("is a no-op when given nothing to add", () => {
    const base = htmlLinked({ language: "javascript" });

    expect(render(withAttrs(base, {}))).toBe(render(base));
  });
});
