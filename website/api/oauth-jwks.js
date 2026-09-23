// Advertised as jwks_uri so the metadata resolves. lumis.sh signs nothing, so the key set is empty.
const headers = {
  "Access-Control-Allow-Origin": "*",
  "Cache-Control": "public, max-age=86400",
  "Content-Type": "application/jwk-set+json; charset=utf-8",
};

export function GET() {
  return Response.json({ keys: [] }, { headers });
}

export function HEAD() {
  return new Response(null, { headers });
}
