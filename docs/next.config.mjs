import { createMDX } from "fumadocs-mdx/next";

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const config = {
  reactStrictMode: true,
  async redirects() {
    return [
      { source: "/operations/warm-up", destination: "/languages", permanent: true },
      {
        source: "/formatters/html-line-migration",
        destination: "/migrate/html-lines",
        permanent: true,
      },
      {
        source: "/llms.mdx/formatters/html-line-migration/content.md",
        destination: "/llms.mdx/migrate/html-lines/content.md",
        permanent: true,
      },
      // Published `@lumis-sh/lumis` warnings link here with a
      // `#highlighting-loads-what-a-document-needs` fragment, which the
      // destination keeps as an anchor.
      { source: "/advanced/wasm-and-cdn", destination: "/languages", permanent: true },
      {
        source: "/llms.mdx/advanced/wasm-and-cdn/content.md",
        destination: "/llms.mdx/languages/content.md",
        permanent: true,
      },
    ];
  },
};

export default withMDX(config);
