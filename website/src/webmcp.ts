import { LANGUAGES_BY_ID } from "./data/languages";
import { loadTheme, THEMES_BY_ID } from "./data/themes";
import { renderHighlight } from "./lib/highlighter";

const DEFAULT_THEME = "catppuccin_frappe";
const MAX_SOURCE_LENGTH = 100_000;

interface ToolExecutionOptions {
  signal: AbortSignal;
}

interface ModelContextTool {
  name: string;
  title?: string;
  description: string;
  inputSchema: object;
  annotations?: {
    readOnlyHint?: boolean;
    untrustedContentHint?: boolean;
    consequentialHint?: boolean;
  };
  execute(input: object, options: ToolExecutionOptions): Promise<unknown>;
}

interface ModelContext {
  registerTool(tool: ModelContextTool, options?: { signal?: AbortSignal }): Promise<void>;
}

declare global {
  interface Document {
    readonly modelContext?: ModelContext;
  }

  interface Navigator {
    readonly modelContext?: ModelContext;
  }
}

function stringProperty(input: object, name: string): string {
  const value: unknown = Reflect.get(input, name);
  if (typeof value !== "string") {
    throw new TypeError(`${name} must be a string`);
  }
  return value;
}

function optionalStringProperty(input: object, name: string): string | undefined {
  const value: unknown = Reflect.get(input, name);
  if (value === undefined) return undefined;
  if (typeof value !== "string") {
    throw new TypeError(`${name} must be a string`);
  }
  return value;
}

const highlightTool: ModelContextTool = {
  name: "highlight-code",
  title: "Highlight code with Lumis",
  description: "Highlight source code with Lumis and return safe, themed HTML.",
  annotations: {
    readOnlyHint: true,
    untrustedContentHint: true,
    consequentialHint: false,
  },
  inputSchema: {
    type: "object",
    properties: {
      source: {
        type: "string",
        maxLength: MAX_SOURCE_LENGTH,
        description: "Source code to highlight.",
      },
      language: {
        type: "string",
        description: "Lumis language identifier, such as rust, elixir, javascript, or html.",
      },
      theme: {
        type: "string",
        default: DEFAULT_THEME,
        description: "Lumis theme identifier. Defaults to catppuccin_frappe.",
      },
    },
    required: ["source", "language"],
    additionalProperties: false,
  },
  async execute(input, { signal }) {
    signal.throwIfAborted();
    const source = stringProperty(input, "source");
    if (source.length > MAX_SOURCE_LENGTH) {
      throw new RangeError(`source must contain at most ${MAX_SOURCE_LENGTH} characters`);
    }

    const languageId = stringProperty(input, "language");
    const language = LANGUAGES_BY_ID.get(languageId);
    if (!language) {
      throw new RangeError(`unknown Lumis language: ${languageId}`);
    }

    const themeId = optionalStringProperty(input, "theme") ?? DEFAULT_THEME;
    if (!THEMES_BY_ID.has(themeId)) {
      throw new RangeError(`unknown Lumis theme: ${themeId}`);
    }

    const theme = await loadTheme(themeId);
    signal.throwIfAborted();
    const html = await renderHighlight(language, theme, source);
    signal.throwIfAborted();

    return { html, language: languageId, theme: themeId };
  },
};

export async function registerWebMcpTools(): Promise<void> {
  // `navigator.modelContext` is the early-preview location. Keep the fallback
  // while browsers migrate to the current `document.modelContext` contract.
  const modelContext = document.modelContext ?? navigator.modelContext;
  if (!modelContext) return;

  const registrations = new AbortController();
  window.addEventListener("pagehide", () => registrations.abort(), { once: true });
  await modelContext.registerTool(highlightTool, { signal: registrations.signal });
}
