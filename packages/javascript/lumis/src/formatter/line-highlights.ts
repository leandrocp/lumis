import type { LineSpec } from "../types.js";

/** Resolve line specs once before a formatter walks its rendered output. */
export function selectedLineFlags(
  lines: readonly LineSpec[] | undefined,
  lineCount: number,
): Uint8Array | undefined {
  if (!lines) return undefined;

  const intervals = lines
    .map((line) => lineInterval(line, lineCount))
    .filter((interval): interval is [number, number] => interval !== undefined)
    .sort((left, right) => left[0] - right[0] || left[1] - right[1]);
  const selected = new Uint8Array(lineCount);
  const first = intervals.shift();
  if (!first) return selected;

  let [start, end] = first;
  for (const [nextStart, nextEnd] of intervals) {
    if (nextStart <= end + 1) {
      end = Math.max(end, nextEnd);
    } else {
      selected.fill(1, start - 1, end);
      [start, end] = [nextStart, nextEnd];
    }
  }
  selected.fill(1, start - 1, end);

  return selected;
}

function lineInterval(line: LineSpec, lineCount: number): [number, number] | undefined {
  if (typeof line === "number") {
    return Number.isInteger(line) && line >= 1 && line <= lineCount ? [line, line] : undefined;
  }

  const start = Math.max(1, Math.ceil(line[0]));
  const end = Math.min(lineCount, Math.floor(line[1]));
  return start <= end ? [start, end] : undefined;
}
