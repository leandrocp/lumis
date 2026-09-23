export function GET() {
  return Response.json(
    { status: "ok", service: "lumis-website" },
    {
      headers: {
        "Access-Control-Allow-Origin": "*",
        "Cache-Control": "no-store",
      },
    },
  );
}
