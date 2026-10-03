function leb128(value: number): number[] {
  const bytes: number[] = [];
  let rest = value;
  do {
    const byte = rest & 0x7f;
    rest >>>= 7;
    bytes.push(rest === 0 ? byte : byte | 0x80);
  } while (rest !== 0);
  return bytes;
}

/** The same parser with an empty custom section appended, `padding` bytes long. */
export function padded(parser: Uint8Array, padding: number): Uint8Array {
  const name = [4, ...new TextEncoder().encode("lumi")];
  const header = [0, ...leb128(name.length + padding)];
  const bytes = new Uint8Array(parser.byteLength + header.length + name.length + padding);
  bytes.set(parser);
  bytes.set(header, parser.byteLength);
  bytes.set(name, parser.byteLength + header.length);
  return bytes;
}
