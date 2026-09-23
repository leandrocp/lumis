// RFC 8414 Authorization Server Metadata plus the Auth.md agent_auth block. The endpoints resolve
// and refuse every request: docs.lumis.sh authenticates nobody and issues no tokens.
const authorizationServer = {
  issuer: "https://docs.lumis.sh",
  authorization_endpoint: "https://docs.lumis.sh/oauth/authorize",
  token_endpoint: "https://docs.lumis.sh/oauth/token",
  jwks_uri: "https://docs.lumis.sh/oauth/jwks",
  grant_types_supported: [],
  response_types_supported: [],
  scopes_supported: ["public"],
  token_endpoint_auth_methods_supported: [],
  agent_auth: {
    skill: "https://docs.lumis.sh/auth.md",
    register_uri: "https://docs.lumis.sh/agent/auth",
    claim_uri: "https://docs.lumis.sh/agent/auth",
    registration_required: false,
    identity_types_supported: ["anonymous"],
    credential_types_supported: ["none"],
    anonymous: {
      credential_types_supported: ["none"],
      claim_uri: "https://docs.lumis.sh/agent/auth",
    },
  },
};

const headers = {
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "public, max-age=3600",
  "Content-Type": "application/json; charset=utf-8",
};

export const revalidate = false;

export function GET() {
  return Response.json(authorizationServer, { headers });
}

export function HEAD() {
  return new Response(null, { headers });
}
