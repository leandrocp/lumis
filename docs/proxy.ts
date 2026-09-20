import { NextRequest, NextResponse } from "next/server";
import { isMarkdownPreferred, rewritePath } from "fumadocs-core/negotiation";
import { docsContentRoute } from "@/lib/shared";

const { rewrite: rewriteDocs } = rewritePath("{/*path}", `${docsContentRoute}{/*path}/content.md`);
const { rewrite: rewriteSuffix } = rewritePath(
  "{/*path}.md",
  `${docsContentRoute}{/*path}/content.md`,
);

export default function proxy(request: NextRequest) {
  const path = request.nextUrl.pathname;
  if (
    ["/api/", "/llms", "/og/", "/img/", "/_next/"].some((prefix) => path.startsWith(prefix)) ||
    ["/robots.txt", "/sitemap.xml", "/favicon.ico"].includes(path)
  )
    return NextResponse.next();

  const result = rewriteSuffix(path);
  if (result) {
    return NextResponse.rewrite(new URL(result, request.nextUrl));
  }

  if (isMarkdownPreferred(request)) {
    const markdownPath = rewriteDocs(path);

    if (markdownPath) {
      return NextResponse.rewrite(new URL(markdownPath, request.nextUrl), {
        // this URL has two representations, selected by `Accept`
        headers: { Vary: "Accept" },
      });
    }
  }

  return NextResponse.next();
}
