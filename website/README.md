# Website deployment

`lumis.sh` is built from this Vite app by the `lumis-website` Vercel project.
The project uses `website/` as its root directory and `main` as its production
branch. `vercel.json` defines the build, skips deploys without website changes,
redirects the former `/docs` entry to `docs.lumis.sh`, and negotiates Markdown
responses for agents. Each HTML entry point has a matching file under `public/`;
`pnpm run check:markdown` verifies the routes, response headers, alternate links,
and token estimates before every build.

Agent discovery metadata lives under `public/.well-known/`: an ARD catalog,
the Lumis Docs MCP Server Card, a digest-pinned Lumis skill index, and an
[RFC 9727](https://www.rfc-editor.org/rfc/rfc9727) API catalog. The API catalog
lists the two APIs Lumis operates, the Docs MCP server and the installer, each
with a machine-readable description, human documentation, and a health endpoint;
the home page and the `/` response advertise it with an `api-catalog` link. The
home page also registers a read-only `highlight-code` WebMCP tool when the
browser supports the current API. `pnpm run check:discovery` verifies these
files, the OpenAPI description of the installer, and their production headers
before every build.

Lumis serves no protected API, so the site publishes no OAuth or OpenID Connect
discovery document. Adding one would advertise an authorization server that does
not exist.

Cloudflare DNS points `lumis.sh` and `www.lumis.sh` to this Vercel project with
proxying disabled. Vercel redirects `www.lumis.sh` to `lumis.sh`.
