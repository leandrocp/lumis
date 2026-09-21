export const revalidate = false;

export function GET() {
  return Response.json(
    { status: "ok", service: "lumis-docs" },
    { headers: { "Access-Control-Allow-Origin": "*" } },
  );
}
