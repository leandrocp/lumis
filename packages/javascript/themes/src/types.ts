/** Theme JSON shape matching themes/*.json */
export interface ThemeData {
  name: string;
  appearance: "light" | "dark";
  revision?: string;
  /** SPDX license expression of the plugin the theme was extracted from. */
  license?: string;
  highlights: Record<string, StyleEntry>;
}

/** A style entry in a theme JSON file. */
export interface StyleEntry {
  fg?: string;
  bg?: string;
  bold?: boolean;
  italic?: boolean;
  underline?: "solid" | "wavy" | "double" | "dotted" | "dashed" | "undercurl";
  strikethrough?: boolean;
}
