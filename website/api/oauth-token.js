// Advertised as token_endpoint so the metadata resolves. lumis.sh issues no access tokens.
export function POST() {
  return Response.json(
    {
      error: "unsupported_grant_type",
      error_description: "lumis.sh is public and does not issue access tokens.",
      auth_documentation: "https://lumis.sh/auth.md",
    },
    {
      status: 400,
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Cache-Control": "no-store",
      },
    },
  );
}
