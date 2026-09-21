# Website deployment

`lumis.sh` is built from this Vite app by the `lumis-website` Vercel project.
The project uses `website/` as its root directory and `main` as its production
branch. `vercel.json` defines the build, skips deploys without website changes,
redirects the former `/docs` entry to `docs.lumis.sh`, and negotiates Markdown
responses for agents. Each HTML entry point has a matching file under `public/`;
`pnpm run check:markdown` verifies the routes, response headers, alternate links,
and token estimates before every build.

Agent discovery metadata lives under `public/.well-known/`: an ARD catalog,
the Lumis Docs MCP Server Card, and a digest-pinned Lumis skill index. The home
page also registers a read-only `highlight-code` WebMCP tool when the browser
supports the current API. `pnpm run check:discovery` verifies these files and
their production headers before every build.

Cloudflare DNS points `lumis.sh` and `www.lumis.sh` to this Vercel project with
proxying disabled. Vercel redirects `www.lumis.sh` to `lumis.sh`.
