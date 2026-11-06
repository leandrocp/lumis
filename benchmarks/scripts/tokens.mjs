// A token is a span the highlighter gave a colour to, whether that colour arrives
// inline, as Lumis, Shiki, syntect and Highlights write it, or through a token class: highlight.js's `hljs-`, TanStack Highlight's
// `th-token`, Prism's `token`, speed-highlight's `shj-syn-` or starry-night's
// `pl-`. Spans that carry none are structure rather than a token: Shiki wraps
// every line in `<span class="line">`, and TanStack Highlight in
// `<span class="th-line">`, so counting all spans would credit one per line.
// Sugar High colours every span inline, including the ones it wraps each run of
// whitespace in, which hold nothing a colour could show.
export function countTokens(fragment, implementation) {
  let tokens = 0;
  for (const tag of asciiLowercase(fragment).match(/<span\b[^>]*>/gu) ?? []) {
    if (/class=(?:"[^"]*|'[^']*)\bsh__token--(?:space|break)\b/u.test(tag)) continue;
    if (
      /style=(?:"[^"]*|'[^']*)\bcolor\s*:/u.test(tag) ||
      /class=(?:"[^"]*|'[^']*)\b(?:hljs-|th-token\b|token\b|shj-syn-|pl-)/u.test(tag)
    ) {
      tokens += 1;
    }
  }
  if (tokens === 0) throw new Error(`${implementation} produced no coloured tokens`);
  return tokens;
}

export function asciiLowercase(value) {
  // Keep the old ASCII-only /i matching behavior; /iu also folds letters such as long s.
  return value.replaceAll(/[A-Z]/gu, (letter) => letter.toLowerCase());
}
