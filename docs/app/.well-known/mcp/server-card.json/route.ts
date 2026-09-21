const serverCard = {
  $schema: "https://static.modelcontextprotocol.io/schemas/v1/server-card.schema.json",
  name: "sh.lumis/docs",
  version: "1.0.0",
  description: "Search and read the official Lumis documentation.",
  title: "Lumis Documentation",
  websiteUrl: "https://docs.lumis.sh",
  remotes: [
    {
      type: "streamable-http",
      url: "https://docs.lumis.sh/api/mcp",
      supportedProtocolVersions: ["2025-06-18"],
    },
  ],
};

const headers = {
  "Access-Control-Allow-Headers": "Content-Type, If-None-Match",
  "Access-Control-Allow-Methods": "GET",
  "Access-Control-Allow-Origin": "*",
  "Content-Type": "application/mcp-server-card+json",
};

export const revalidate = false;

export function GET() {
  return Response.json(serverCard, { headers });
}
