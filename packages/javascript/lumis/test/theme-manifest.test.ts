import { describe, expect, it } from "vitest";
import { parseThemePack } from "../../themes/src/manifest.js";

const sample = {
  name: "sample",
  appearance: "dark",
  highlights: { normal: { fg: "#ffffff", bg: "#000000" }, comment: { underline: true } },
};

describe("theme pack manifests", () => {
  it("normalizes source styles without mutating the manifest", () => {
    const data = { schema_version: 1, pack: "default", themes: [sample] };
    expect(parseThemePack(data, "default")[0]?.highlights.comment?.underline).toBe("solid");
    expect(sample.highlights.comment.underline).toBe(true);
    const curled = { ...sample, highlights: { comment: { underline: true, undercurl: true } } };
    expect(
      parseThemePack({ ...data, themes: [curled] }, "default")[0]?.highlights.comment?.underline,
    ).toBe("undercurl");
  });

  it.each([
    null,
    { schema_version: 2, pack: "default", themes: [] },
    { schema_version: 1, pack: "copyleft", themes: [] },
    { schema_version: 1, pack: "default", themes: {} },
    { schema_version: 1, pack: "default", themes: [sample, sample] },
    { schema_version: 1, pack: "default", themes: [{ ...sample, name: "../escape" }] },
    { schema_version: 1, pack: "default", themes: [{ ...sample, appearance: "blue" }] },
    { schema_version: 1, pack: "default", themes: [{ ...sample, highlights: [] }] },
    { schema_version: 1, pack: "default", themes: [{ ...sample, revision: 42 }] },
    { schema_version: 1, pack: "default", themes: [{ ...sample, highlights: { normal: null } }] },
    {
      schema_version: 1,
      pack: "default",
      themes: [{ ...sample, highlights: { normal: { fg: 1 } } }],
    },
    {
      schema_version: 1,
      pack: "default",
      themes: [{ ...sample, highlights: { normal: { bold: "yes" } } }],
    },
    {
      schema_version: 1,
      pack: "default",
      themes: [{ ...sample, highlights: { normal: { underline: "invalid" } } }],
    },
  ])("rejects invalid boundary data %#", (data) => {
    expect(() => parseThemePack(data, "default")).toThrow(TypeError);
  });
});
