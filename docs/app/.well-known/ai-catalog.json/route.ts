const catalog = {
  specVersion: "1.0",
  host: {
    displayName: "Lumis Docs",
    identifier: "did:web:docs.lumis.sh",
  },
  entries: [
    {
      identifier: "urn:air:docs.lumis.sh:mcp:documentation",
      displayName: "Lumis Documentation MCP Server",
      type: "application/mcp-server-card+json",
      url: "https://docs.lumis.sh/.well-known/mcp/server-card.json",
      representativeQueries: [
        "How do I install Lumis for JavaScript?",
        "Which syntax highlighting formatters does Lumis support?",
        "How do I cache parsers before deployment?",
      ],
    },
    {
      identifier: "urn:air:docs.lumis.sh:skill:documentation",
      displayName: "Use Lumis Documentation",
      type: "text/markdown",
      url: "https://docs.lumis.sh/.well-known/agent-skills/lumis-docs/SKILL.md",
      representativeQueries: [
        "Find the Lumis documentation for multi-theme HTML",
        "Look up how to use Lumis from Elixir",
        "Search the Lumis docs for line highlighting",
      ],
    },
  ],
};

export const revalidate = false;

export function GET() {
  return Response.json(catalog, {
    headers: { "Access-Control-Allow-Origin": "*" },
  });
}
