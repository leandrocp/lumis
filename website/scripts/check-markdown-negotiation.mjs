import { readFile } from "node:fs/promises";

const pages = [
  {
    source: "^/$",
    html: "index.html",
    htmlSources: ["/"],
    markdown: "public/index.md",
  },
  {
    source: "^/comparison/?$",
    html: "comparison/index.html",
    htmlSources: ["/comparison", "/comparison/"],
    markdown: "public/comparison/index.md",
  },
  {
    source: "^/showcase/?$",
    html: "showcase/index.html",
    htmlSources: ["/showcase", "/showcase/"],
    markdown: "public/showcase/index.md",
  },
];

const config = JSON.parse(await readFile(new URL("../vercel.json", import.meta.url), "utf8"));

for (const page of pages) {
  const route = config.routes?.find((candidate) => candidate.src === page.source);
  if (!route) throw new Error(`Missing Markdown route for ${page.source}`);

  const accept = route.has?.find(
    (condition) => condition.type === "header" && condition.key.toLowerCase() === "accept",
  );
  if (!accept?.value.includes("text/markdown")) {
    throw new Error(`Route ${page.source} does not negotiate Accept: text/markdown`);
  }
  if (route.headers?.["Content-Type"] !== "text/markdown; charset=utf-8") {
    throw new Error(`Route ${page.source} does not return Content-Type: text/markdown`);
  }
  if (!route.headers?.Vary?.split(",").some((value) => value.trim() === "Accept")) {
    throw new Error(`Route ${page.source} does not vary on Accept`);
  }
  for (const source of page.htmlSources) {
    const headers = config.headers?.find((candidate) => candidate.source === source)?.headers;
    const vary = headers?.find((header) => header.key === "Vary")?.value;
    if (!vary?.split(",").some((value) => value.trim() === "Accept")) {
      throw new Error(`HTML route ${source} does not vary on Accept`);
    }
  }

  const markdownUrl = new URL(`../${page.markdown}`, import.meta.url);
  const markdown = await readFile(markdownUrl, "utf8");
  const expectedTokens = Math.ceil(Buffer.byteLength(markdown, "utf8") / 4);
  const configuredTokens = Number(route.headers?.["x-markdown-tokens"]);
  if (configuredTokens !== expectedTokens) {
    throw new Error(
      `${page.markdown} is approximately ${expectedTokens} tokens; update x-markdown-tokens`,
    );
  }

  const html = await readFile(new URL(`../${page.html}`, import.meta.url), "utf8");
  const publicPath = page.markdown.replace(/^public/, "");
  if (!html.includes(`rel="alternate"`) || !html.includes(`href="${publicPath}"`)) {
    throw new Error(`${page.html} does not advertise ${publicPath}`);
  }
}

console.log("Markdown content negotiation configuration is valid.");
