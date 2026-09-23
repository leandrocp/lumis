# Lumis auth.md

Nothing on lumis.sh requires authentication. Agents do not register an account, request a token, or send an `Authorization` header to read the site, install the CLI, or call the public APIs.

## Audience

These instructions are for agents helping users evaluate, install, or use Lumis.

## Discover

Fetch the public discovery documents:

- Protected Resource Metadata: https://lumis.sh/.well-known/oauth-protected-resource
- Authorization Server Metadata: https://lumis.sh/.well-known/oauth-authorization-server
- Registration descriptor: https://lumis.sh/agent/auth
- API Catalog (RFC 9727): https://lumis.sh/.well-known/api-catalog

The supported scope is `public`.

## Register

Registration is not required. `register_uri` and `claim_uri` both resolve to a read-only descriptor confirming that no account is created and no credential is issued. Fetch it with `GET`; do not submit user identity or authentication data.

## Identity and credentials

- Supported identity type: `anonymous`
- Supported credential type: `none`
- Authorization header: not required

The authorization and token endpoints exist so the metadata resolves, and they refuse every request: `/oauth/authorize` returns `unsupported_response_type`, `/oauth/token` returns `unsupported_grant_type`, and `/oauth/jwks` serves an empty key set. Lumis issues no tokens.

Request public resources directly:

```http
GET /index.md HTTP/1.1
Host: lumis.sh
Accept: text/markdown
```

## Public resources

| Resource                      | URL                                                      |
| ----------------------------- | -------------------------------------------------------- |
| Agent-friendly Lumis brief    | https://lumis.sh/index.md                                |
| Lumis skill                   | https://lumis.sh/.well-known/agent-skills/lumis/SKILL.md |
| Documentation                 | https://docs.lumis.sh                                    |
| Documentation MCP server      | https://docs.lumis.sh/api/mcp                            |
| Versioned installer API       | https://lumis.sh/api/versioned-installer                 |
| Installer OpenAPI description | https://lumis.sh/openapi/versioned-installer.json        |
| Source code                   | https://github.com/leandrocp/lumis                       |

## Claim and revocation

Claiming and revocation do not apply. Lumis does not create an account, session, token, API key, or other credential through this website, so there is nothing to claim or revoke.
