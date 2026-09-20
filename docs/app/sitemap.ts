import type { MetadataRoute } from "next";
import { source } from "@/lib/source";

export default function sitemap(): MetadataRoute.Sitemap {
  return source.getPages().map((page) => ({
    url: new URL(page.url, "https://docs.lumis.sh").toString(),
    changeFrequency: "weekly",
  }));
}
