"use client";

import { useEffect } from "react";

type Tool = {
  name: string;
  title: string;
  description: string;
  inputSchema: object;
  annotations: {
    readOnlyHint: boolean;
    untrustedContentHint: boolean;
    consequentialHint: boolean;
  };
  execute: (input: object, options?: { signal?: AbortSignal }) => Promise<unknown>;
};

type ModelContext = {
  registerTool?: (tool: Tool, options?: { signal?: AbortSignal }) => Promise<void>;
  provideContext?: (context: { tools: Tool[] }) => void;
};

declare global {
  interface Document {
    modelContext?: ModelContext;
  }

  interface Navigator {
    modelContext?: ModelContext;
  }
}

function searchQuery(input: object): string {
  const query = "query" in input ? input.query : undefined;
  if (typeof query !== "string" || query.trim() === "") {
    throw new TypeError("query must be a non-empty string");
  }
  return query;
}

export function WebMcp() {
  useEffect(() => {
    const modelContext = document.modelContext ?? navigator.modelContext;
    if (!modelContext) return;

    const controller = new AbortController();
    const tool: Tool = {
      name: "search_lumis_docs",
      title: "Search Lumis documentation",
      description:
        "Search the official Lumis documentation for guides, API usage, runtimes, formatters, themes, and recipes.",
      inputSchema: {
        type: "object",
        properties: {
          query: {
            type: "string",
            minLength: 1,
            description: "Terms to search for in the Lumis documentation.",
          },
        },
        required: ["query"],
        additionalProperties: false,
      },
      annotations: {
        readOnlyHint: true,
        untrustedContentHint: false,
        consequentialHint: false,
      },
      async execute(input, options) {
        const query = searchQuery(input);
        const response = await fetch(`/api/search?query=${encodeURIComponent(query)}`, {
          signal: options?.signal,
        });
        if (!response.ok) throw new Error(`Documentation search failed (${response.status})`);
        return response.json();
      },
    };

    if (modelContext.registerTool) {
      void modelContext.registerTool(tool, { signal: controller.signal }).catch(() => undefined);
    } else {
      modelContext.provideContext?.({ tools: [tool] });
    }

    return () => controller.abort();
  }, []);

  return null;
}
