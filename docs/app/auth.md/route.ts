const fence = "```";

const authMd = `# Lumis docs auth.md

Nothing on docs.lumis.sh requires authentication. Agents do not register an account, request a token, or send an \`Authorization\` header to read the documentation, call the search API, or connect to the MCP server.

## Audience

These instructions are for agents reading the Lumis documentation or connecting to its MCP server on a user's behalf.

## Discover

Fetch the public discovery documents:

- Protected Resource Metadata: https://docs.lumis.sh/.well-known/oauth-protected-resource
- Authorization Server Metadata: https://docs.lumis.sh/.well-known/oauth-authorization-server
- Registration descriptor: https://docs.lumis.sh/agent/auth
- API Catalog (RFC 9727): https://docs.lumis.sh/.well-known/api-catalog
- MCP server card: https://docs.lumis.sh/.well-known/mcp/server-card.json

The supported scope is \`public\`.

## Register

Registration is not required. \`register_uri\` and \`claim_uri\` both resolve to a read-only descriptor confirming that no account is created and no credential is issued. Fetch it with \`GET\`; do not submit user identity or authentication data.

## Identity and credentials

- Supported identity type: \`anonymous\`
- Supported credential type: \`none\`
- Authorization header: not required

The authorization and token endpoints exist so the metadata resolves, and they refuse every request: \`/oauth/authorize\` returns \`unsupported_response_type\`, \`/oauth/token\` returns \`unsupported_grant_type\`, and \`/oauth/jwks\` serves an empty key set. Lumis issues no tokens.

## Connect to the MCP server

Connect over Streamable HTTP with no credential:

${fence}http
POST /api/mcp HTTP/1.1
Host: docs.lumis.sh
Content-Type: application/json
${fence}

## Read the documentation as Markdown

Every documentation page has a Markdown representation, selected by \`Accept\` or by appending \`.md\`:

${fence}http
GET /installation.md HTTP/1.1
Host: docs.lumis.sh
${fence}

## Public resources

| Resource | URL |
| --- | --- |
| Documentation index for LLMs | https://docs.lumis.sh/llms.txt |
| Full documentation for LLMs | https://docs.lumis.sh/llms-full.txt |
| MCP server | https://docs.lumis.sh/api/mcp |
| Lumis docs skill | https://docs.lumis.sh/.well-known/agent-skills/lumis-docs/SKILL.md |
| Project website | https://lumis.sh |
| Source code | https://github.com/leandrocp/lumis |

## Claim and revocation

Claiming and revocation do not apply. Lumis does not create an account, session, token, API key, or other credential through this site, so there is nothing to claim or revoke.
`;

const headers = {
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "public, max-age=3600",
  "Content-Type": "text/markdown; charset=utf-8",
};

export const revalidate = false;

export function GET() {
  return new Response(authMd, { headers });
}

export function HEAD() {
  return new Response(null, { headers });
}
