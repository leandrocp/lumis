import { htmlLinked } from "../src/formatters.js";
import { expect, it, vi } from "vitest";
import { createHighlighter, highlight, highlightEvents, runtimeKind } from "../src/index.js";
import { createHighlighterModule } from "../src/core/highlighter.js";
import { createLanguagesModule } from "../src/core/languages.js";
import { browserRuntime } from "../src/runtime/browser.js";
import { exerciseParserRecovery } from "./parser-recovery.js";
import { recoveryLanguages } from "./parser-recovery-fixture.js";

it("recovers automatically from repeated real parser traps, including injections", async () => {
  const result = await exerciseParserRecovery(
    createHighlighter,
    recoveryLanguages(),
    runtimeKind() === "wasm",
  );
  expect(result.largeParserBytes).toBeGreaterThan(8 * 1024 * 1024);
  expect(result.failures).toHaveLength(4);
  expect(result.recovered).toBe(20);
  expect(result.baseline).toContain('class="l-string');
  expect(result.markdown).toContain('class="l-number');
});

it("the async highlight API waits for recovery before its next document", async () => {
  const [php] = recoveryLanguages();
  if (!php) throw new Error("PHP fixture is missing");
  const formatter = htmlLinked({ language: php });
  const simple = '<?php $a = "x $b";';
  const baseline = await highlight(simple, formatter);
  for (let attempt = 0; attempt < 3; attempt++) {
    await expect(highlight("<?php\n$a = <<<END\nEND;\n", formatter)).rejects.toThrow(
      "parser returned no tree",
    );
    expect(await highlight(simple, formatter)).toBe(baseline);
  }
});

it("an async render waits if another render traps while it is being prepared", async () => {
  const [php] = recoveryLanguages();
  if (!php) throw new Error("PHP fixture is missing");
  const formatter = htmlLinked({ language: php });
  const simple = '<?php $a = "x $b";';
  const baseline = await highlight(simple, formatter);
  const [failed, valid] = await Promise.allSettled([
    highlight("<?php\n$a = <<<END\nEND;\n", formatter),
    highlight(simple, formatter),
  ]);
  expect(failed.status).toBe("rejected");
  expect(valid).toEqual({ status: "fulfilled", value: baseline });
});

it.skipIf(runtimeKind() !== "wasm")(
  "never replays a custom formatter during recovery",
  async () => {
    const [php] = recoveryLanguages();
    if (!php) throw new Error("PHP fixture is missing");
    const simple = '<?php $a = "x $b";';
    let calls = 0;
    await expect(
      highlight(simple, {
        language: php,
        render() {
          calls++;
          expect(() => highlightEvents("<?php\n$a = <<<END\nEND;\n", php)).toThrow(
            "parser returned no tree",
          );
          return JSON.stringify(highlightEvents(simple, php));
        },
      }),
    ).rejects.toThrow("await highlighter.ready()");
    expect(calls).toBe(1);
    expect(await highlight(simple, htmlLinked({ language: php }))).toContain('class="l-string');
  },
);

it("reports a failed background restoration through ready without an unhandled rejection", async () => {
  const isolated = createHighlighterModule(createLanguagesModule(browserRuntime));
  const [php] = recoveryLanguages();
  if (!php) throw new Error("PHP fixture is missing");
  const highlighter = await isolated.createHighlighter({ languages: [php] });
  const failure = new Error("Cannot instantiate replacement engine");
  const instantiate = vi.spyOn(WebAssembly, "instantiate").mockRejectedValueOnce(failure);
  try {
    expect(() =>
      highlighter.highlight("<?php\n$a = <<<END\nEND;\n", htmlLinked({ language: php })),
    ).toThrow("parser returned no tree");
    // Let the background promise reject before anyone awaits it.
    await new Promise((resolve) => {
      setTimeout(resolve, 20);
    });
    await expect(highlighter.ready()).rejects.toBe(failure);
    await expect(highlighter.ready()).rejects.toBe(failure);
  } finally {
    instantiate.mockRestore();
  }
});
