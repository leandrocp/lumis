// RFC 9728 Protected Resource Metadata. docs.lumis.sh is public: this says so rather than gating it.
const protectedResource = {
  resource: "https://docs.lumis.sh/",
  resource_name: "Lumis documentation and MCP server",
  authorization_servers: ["https://docs.lumis.sh"],
  scopes_supported: ["public"],
  bearer_methods_supported: ["header"],
};

const headers = {
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "public, max-age=3600",
  "Content-Type": "application/json; charset=utf-8",
};

export const revalidate = false;

export function GET() {
  return Response.json(protectedResource, { headers });
}

export function HEAD() {
  return new Response(null, { headers });
}
