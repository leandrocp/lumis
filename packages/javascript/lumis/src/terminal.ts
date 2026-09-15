import type { TerminalFormatter, TerminalOptions } from "./types.js";
import { markBuiltinFormatter } from "./core/builtin-formatter.js";
import { formatTerminal } from "./formatter/terminal.js";

export function terminal(options: TerminalOptions = {}): TerminalFormatter {
  const formatter: TerminalFormatter = {
    ...options,
    render(source, events): string {
      return formatTerminal(source, events, formatter);
    },
  };
  return markBuiltinFormatter(formatter, "terminal");
}

export type { TerminalFormatter, TerminalOptions } from "./types.js";
