import type {
  BBCodeScopedFormatter,
  Formatter,
  HtmlInlineFormatter,
  HtmlLinkedFormatter,
  HtmlMultiThemesFormatter,
  TerminalFormatter,
} from "../types.js";

type BuiltinFormatter = Readonly<
  | { kind: "html-inline"; formatter: HtmlInlineFormatter }
  | { kind: "html-linked"; formatter: HtmlLinkedFormatter }
  | { kind: "html-multi-themes"; formatter: HtmlMultiThemesFormatter }
  | { kind: "bbcode-scoped"; formatter: BBCodeScopedFormatter }
  | { kind: "terminal"; formatter: TerminalFormatter }
>;

export type BuiltinFormatterKind = BuiltinFormatter["kind"];

// Only the factories register built-ins. Retaining the typed entry avoids
// asserting a formatter's options from a marker attached to an arbitrary object.
const builtins = new WeakMap<Formatter, BuiltinFormatter>();

export function markBuiltinFormatter<T extends BuiltinFormatter>(entry: T): T["formatter"] {
  builtins.set(entry.formatter, entry);
  return entry.formatter;
}

export function getBuiltinFormatter(formatter: Formatter): BuiltinFormatter | undefined {
  return builtins.get(formatter);
}

export function builtinFormatterKind(formatter: Formatter): BuiltinFormatterKind | undefined {
  return getBuiltinFormatter(formatter)?.kind;
}
