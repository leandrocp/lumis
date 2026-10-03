import type { createHighlighter as CreateHighlighter } from "../src/index.js";
import type { Language } from "../src/types.js";
import { padded } from "./padded-parser.js";
import { htmlLinked } from "../src/formatters.js";

const simple = '<?php $a = "x $b";';
const heredoc = "<?php\n$a = <<<END\nEND;\n";
const nowdoc = "<?php\n$a = <<<'END'\nx\nEND;\n";
const injected = '```json\n{"answer": 42}\n```\n';

function parseFailure(error: unknown, wasmTrap: boolean): string {
  if (!(error instanceof Error) || !error.message.includes("parser returned no tree")) throw error;
  if (wasmTrap) {
    let cause: unknown = error;
    while (cause instanceof Error && cause.cause) cause = cause.cause;
    if (!(cause instanceof WebAssembly.RuntimeError)) throw error;
  }
  return error.message;
}

function recovering(error: unknown): boolean {
  return error instanceof Error && error.message.includes("await highlighter.ready()");
}

async function waitForAutomaticRecovery(render: () => string): Promise<void> {
  const deadline = Date.now() + 10_000;
  for (;;) {
    try {
      render();
      return;
    } catch (error) {
      if (!recovering(error) || Date.now() >= deadline) throw error;
    }
    await new Promise((resolve) => {
      setTimeout(resolve, 10);
    });
  }
}

function requireRecovering(render: () => string): void {
  try {
    render();
  } catch (error) {
    if (recovering(error)) return;
    throw error;
  }
  throw new Error("The synchronous call did not report pending recovery");
}

function padJson(languages: Language[]): number {
  const jsonLanguage = languages.find((language) => language.id === "json");
  if (!(jsonLanguage?.wasm instanceof Uint8Array)) throw new Error("Expected JSON fixture bytes");
  jsonLanguage.wasm = padded(jsonLanguage.wasm, 9 * 1024 * 1024);
  return jsonLanguage.wasm.byteLength;
}

/** Shared by disposable browser contexts and the isolated Node test process. */
export async function exerciseParserRecovery(
  createHighlighter: typeof CreateHighlighter,
  languages: Language[],
  wasmTrap = true,
) {
  const largeParserBytes = padJson(languages);
  const highlighter = await createHighlighter({ languages });
  const peer = await createHighlighter({ languages });
  const render = (source: string, language: string, target = highlighter) =>
    target.highlight(source, htmlLinked({ language }), { rainbowBrackets: true });
  const baseline = render(simple, "php");
  const json = render('{"answer": 42}', "json");
  const markdown = render(injected, "markdown");
  const peerBaseline = render(simple, "php", peer);
  const failures: string[] = [];
  let recovered = 0;

  // Restore a large preloaded grammar as well as the crashing parser and
  // injections, without a reload or a replacement highlighter.
  for (const [source, language] of [
    [heredoc, "php"],
    [nowdoc, "php"],
    [`\`\`\`php\n${heredoc}\`\`\`\n`, "markdown"],
    [heredoc, "php"],
  ]) {
    try {
      render(source, language);
      throw new Error("Broken PHP parser unexpectedly highlighted a heredoc");
    } catch (error) {
      failures.push(parseFailure(error, wasmTrap));
    }
    if (wasmTrap) requireRecovering(() => render(simple, "php"));
    if (failures.length === 1) {
      // No ready() call starts this restoration: catching the error is enough.
      await waitForAutomaticRecovery(() => render(simple, "php"));
    } else {
      await Promise.all([highlighter.ready(), peer.ready(), highlighter.ready()]);
    }
    for (const [actual, expected] of [
      [render(simple, "php"), baseline],
      [render('{"answer": 42}', "json"), json],
      [render(injected, "markdown"), markdown],
      [render(simple, "php", peer), peerBaseline],
      [render(injected, "markdown", peer), markdown],
    ]) {
      if (actual !== expected) throw new Error("Highlight output changed after the parser trap");
      recovered++;
    }
  }
  return { failures, recovered, baseline, json, markdown, largeParserBytes };
}
