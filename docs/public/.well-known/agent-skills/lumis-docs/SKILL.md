---
name: lumis-docs
description: Find and read authoritative Lumis documentation through Markdown, search, or MCP.
---

# Use Lumis documentation

Use `https://docs.lumis.sh` as the authoritative source for Lumis usage and
integration questions.

## Choose the smallest interface

- Fetch a documentation URL with `Accept: text/markdown` when you already know
  the page. Appending `.md` to a page URL is also supported.
- Fetch `/llms.txt` for the page index or `/llms-full.txt` for all documentation
  in one response.
- Query `/api/search?query=TERMS` when you need to find a page.
- Connect to `/api/mcp` with MCP Streamable HTTP when your client supports MCP.
  Initialize the server and call `tools/list`; do not assume tool names.

Prefer the narrowest relevant page over the full documentation. Preserve code
examples exactly, and cite the canonical HTML page URL in user-facing answers.
