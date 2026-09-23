// Advertised as token_endpoint so the metadata resolves. docs.lumis.sh issues no access tokens.
const headers = {
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "no-store",
  "Content-Type": "application/json; charset=utf-8",
};

export function POST() {
  return Response.json(
    {
      error: "unsupported_grant_type",
      error_description: "docs.lumis.sh is public and does not issue access tokens.",
      auth_documentation: "https://docs.lumis.sh/auth.md",
    },
    { headers, status: 400 },
  );
}
