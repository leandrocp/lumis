const serverCard = {
  serverInfo: {
    name: "docs",
    version: "1.0.0",
  },
  protocolVersion: "2025-06-18",
  transport: {
    type: "streamable-http",
    endpoint: "https://docs.lumis.sh/api/mcp",
  },
  capabilities: {
    tools: {
      listChanged: true,
    },
  },
};

export const revalidate = false;

export function GET() {
  return Response.json(serverCard, {
    headers: { "Access-Control-Allow-Origin": "*" },
  });
}
