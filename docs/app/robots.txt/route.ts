const body = `User-agent: *
Allow: /
Content-Signal: ai-train=no, search=yes, ai-input=yes

Sitemap: https://docs.lumis.sh/sitemap.xml
Agentmap: https://docs.lumis.sh/.well-known/ai-catalog.json
`;

export const revalidate = false;

export function GET() {
  return new Response(body, {
    headers: { "Content-Type": "text/plain; charset=utf-8" },
  });
}
