import { createMDX } from "fumadocs-mdx/next";

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const config = {
  reactStrictMode: true,
  async redirects() {
    return [{ source: "/operations/warm-up", destination: "/languages", permanent: true }];
  },
};

export default withMDX(config);
