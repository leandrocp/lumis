/** Internal signal: async callers may wait, but must not retry a parser trap. */
export class ParserRecoveringError extends Error {
  constructor() {
    super("Parser engine is recovering; await highlighter.ready() before highlighting again");
  }
}

/** A trap can surface during parsing, queries, tree reads, or cleanup. */
export function isParserTrap(error: unknown): boolean {
  if (error instanceof WebAssembly.RuntimeError) return true;
  return error instanceof Error && error.cause !== error && error.cause != null
    ? isParserTrap(error.cause)
    : false;
}
