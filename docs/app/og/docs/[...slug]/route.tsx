import { source } from "@/lib/source";
import { notFound } from "next/navigation";
import { ImageResponse } from "next/og";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { appName, getPageImageUrl } from "@/lib/shared";

export const revalidate = false;

const assets = Promise.all([
  readFile(join(process.cwd(), "public/brand/icon-dark.png")),
  fetch("https://fonts.gstatic.com/s/nunito/v32/XRXI3I6Li01BKofiOc5wtlZ2di8HDGUmdTQ3ig.ttf").then(
    (response) => response.arrayBuffer(),
  ),
  fetch("https://fonts.gstatic.com/s/nunito/v32/XRXI3I6Li01BKofiOc5wtlZ2di8HDDsmdTQ3ig.ttf").then(
    (response) => response.arrayBuffer(),
  ),
]);

export async function GET(_req: Request, { params }: RouteContext<"/og/docs/[...slug]">) {
  const { slug } = await params;
  const page = source.getPage(slug.slice(0, -1));
  if (!page) notFound();
  const [icon, nunito600, nunito800] = await assets;

  return new ImageResponse(
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: "100%",
        height: "100%",
        padding: 72,
        backgroundColor: "#0E1530",
        color: "#F7F5F0",
        fontFamily: "Nunito",
        fontWeight: 800,
      }}
    >
      <div style={{ display: "flex", alignItems: "center", gap: 24 }}>
        <img
          src={`data:image/png;base64,${icon.toString("base64")}`}
          alt=""
          width={64}
          height={64}
        />
        <span style={{ color: "#A58BFF", fontSize: 28 }}>{appName}</span>
      </div>
      <div style={{ marginTop: 48, fontSize: 64, lineHeight: 1.1 }}>{page.data.title}</div>
      <div style={{ marginTop: 24, color: "#AEB3CF", fontSize: 30, fontWeight: 600 }}>
        {page.data.description}
      </div>
      <div
        style={{
          display: "flex",
          position: "absolute",
          bottom: 0,
          left: 0,
          width: "100%",
          height: 12,
        }}
      >
        {["#8C6BFF", "#FF5FAE", "#FF6F6F", "#FF7A45", "#FFC23D"].map((color) => (
          <div key={color} style={{ width: "20%", height: "100%", backgroundColor: color }} />
        ))}
      </div>
    </div>,
    {
      width: 1200,
      height: 630,
      fonts: [
        { name: "Nunito", data: nunito600, weight: 600, style: "normal" },
        { name: "Nunito", data: nunito800, weight: 800, style: "normal" },
      ],
    },
  );
}

export function generateStaticParams() {
  return source.getPages().map((page) => ({
    lang: page.locale,
    slug: getPageImageUrl(page).segments,
  }));
}
