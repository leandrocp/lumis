// Advertised as jwks_uri so the metadata resolves. docs.lumis.sh signs nothing, so the set is empty.
const headers = {
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "public, max-age=86400",
  "Content-Type": "application/jwk-set+json; charset=utf-8",
};

export const revalidate = false;

export function GET() {
  return Response.json({ keys: [] }, { headers });
}

export function HEAD() {
  return new Response(null, { headers });
}
