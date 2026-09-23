// Advertised as authorization_endpoint so the metadata resolves. lumis.sh is public: it never
// issues an authorization response.
const headers = {
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "public, max-age=3600",
  "Content-Type": "application/json; charset=utf-8",
};

const error = {
  error: "unsupported_response_type",
  error_description: "lumis.sh is public and does not require an OAuth authorization response.",
  auth_documentation: "https://lumis.sh/auth.md",
};

export function GET() {
  return Response.json(error, { headers, status: 400 });
}

export function HEAD() {
  return new Response(null, { headers, status: 400 });
}
