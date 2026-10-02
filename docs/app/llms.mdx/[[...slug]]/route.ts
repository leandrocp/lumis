import { docsLlms, source } from "@/lib/source";
import { getPageMarkdownUrl } from "@/lib/shared";

export const revalidate = false;

export async function GET(_req: Request, { params }: RouteContext<"/llms.mdx/[[...slug]]">) {
  const { slug } = await params;
  const page = source.getPage(slug?.slice(0, -1));
  return new Response(page ? await docsLlms.page(page) : "# Page not found\n", {
    status: page ? 200 : 404,
    headers: {
      "Content-Type": "text/markdown; charset=utf-8",
      Vary: "Accept",
    },
  });
}

export function generateStaticParams() {
  return source.getPages().map((page) => ({
    lang: page.locale,
    slug: getPageMarkdownUrl(page).segments,
  }));
}
