const apiCatalog = {
  linkset: [
    {
      anchor: "https://docs.lumis.sh/api/mcp",
      "service-desc": [
        {
          href: "https://docs.lumis.sh/.well-known/mcp/server-card.json",
          type: "application/json",
        },
      ],
      "service-doc": [
        {
          href: "https://docs.lumis.sh/",
          type: "text/html",
        },
      ],
      status: [
        {
          href: "https://docs.lumis.sh/api/health",
          type: "application/json",
        },
      ],
    },
  ],
};

const headers = {
  "Access-Control-Allow-Origin": "*",
  "Content-Type": 'application/linkset+json; profile="https://www.rfc-editor.org/info/rfc9727"',
  Link: '</.well-known/api-catalog>; rel="api-catalog"',
};

export const revalidate = false;

export function GET() {
  return Response.json(apiCatalog, { headers });
}

export function HEAD() {
  return new Response(null, { headers });
}
