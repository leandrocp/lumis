import { beforeAll, describe, expect, it } from "vitest";

import { createHighlighter } from "../src/index.js";
import { htmlLinked } from "../src/formatters.js";
import type { Highlighter } from "../src/index.js";
import json from "../langs/json.ts";
import { configureLocalWasmResolver } from "./wasm.js";

let hl: Highlighter;

beforeAll(async () => {
  configureLocalWasmResolver(["json"]);
  hl = await createHighlighter({ languages: [json] });
}, 120_000);

// The port of `a_run_of_one_scope_renders_as_one_span` in
// `crates/lumis/src/highlight.rs`. Rust is the reference, so a difference here
// is a bug in the `Coalescing` port in `src/events.ts`.
describe("coalesced highlight events", () => {
  it("renders a run of one scope as one span", () => {
    const html = hl.highlight("[".repeat(64), htmlLinked({ language: json }));

    expect(html).toContain(`<span class="l-punctuation-bracket">${"[".repeat(64)}</span>`);
    expect(html.match(/<span(?! class="l-line")/g)).toHaveLength(1);
  });

  it("keeps neighbouring tokens of different scopes apart", () => {
    const html = hl.highlight("[0,0]", htmlLinked({ language: json }));

    expect(html).toContain(
      '<span class="l-punctuation-bracket">[</span>' +
        '<span class="l-number">0</span>' +
        '<span class="l-punctuation-delimiter">,</span>' +
        '<span class="l-number">0</span>' +
        '<span class="l-punctuation-bracket">]</span>',
    );
  });
});
