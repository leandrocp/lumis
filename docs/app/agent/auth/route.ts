// The register_uri and claim_uri from agent_auth. Read-only: it creates no account and issues no
// credential, so POST returns the same descriptor as GET.
const registration = {
  registration_required: false,
  resource: "https://docs.lumis.sh/",
  identity_types_supported: ["anonymous"],
  credential_types_supported: ["none"],
  scope: "public",
  instructions:
    "Read the Lumis documentation and connect to the MCP server at /api/mcp without registering or sending an Authorization header.",
  auth_documentation: "https://docs.lumis.sh/auth.md",
};

const headers = {
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "public, max-age=3600",
  "Content-Type": "application/json; charset=utf-8",
};

export const revalidate = false;

export function GET() {
  return Response.json(registration, { headers });
}

export function HEAD() {
  return new Response(null, { headers });
}

export function POST() {
  return Response.json(registration, { headers });
}
