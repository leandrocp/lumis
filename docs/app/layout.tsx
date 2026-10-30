import { RootProvider } from "fumadocs-ui/provider/next";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { Analytics } from "@vercel/analytics/next";
import Script from "next/script";
import { source } from "@/lib/source";
import { baseOptions } from "@/lib/layout.shared";
import { WebMcp } from "@/components/webmcp";
import { Inter, Nunito } from "next/font/google";
import type { Metadata, Viewport } from "next";
import "./global.css";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
  display: "swap",
});

const nunito = Nunito({
  subsets: ["latin"],
  weight: ["700", "800"],
  variable: "--font-nunito",
  display: "swap",
});

export const metadata: Metadata = {
  metadataBase: new URL("https://docs.lumis.sh"),
  title: { default: "Lumis Docs", template: "%s | Lumis Docs" },
  description: "One highlighter, 7 runtimes. Powered by Tree-sitter and Neovim themes.",
  applicationName: "Lumis Docs",
  icons: {
    icon: [
      { url: "/favicon.svg", type: "image/svg+xml" },
      { url: "/favicon-32.png", sizes: "32x32", type: "image/png" },
    ],
    apple: "/apple-touch-icon.png",
  },
  manifest: "/site.webmanifest",
};

export const viewport: Viewport = {
  themeColor: [
    { media: "(prefers-color-scheme: light)", color: "#F7F5F0" },
    { media: "(prefers-color-scheme: dark)", color: "#0E1530" },
  ],
};

export default function Layout({ children }: LayoutProps<"/">) {
  return (
    <html lang="en" suppressHydrationWarning className={`${inter.variable} ${nunito.variable}`}>
      <head>
        <link rel="describedby" type="text/plain" href="/llms.txt" />
        <link rel="api-catalog" href="/.well-known/api-catalog" />
        <link rel="ai-catalog" href="/.well-known/ai-catalog.json" />
      </head>
      <body className="flex flex-col min-h-screen">
        <Script id="migrate-runtime-tab" strategy="beforeInteractive">
          {`try {
            // Match the value stored by the docs tabs component.
            const newValue = "JavaScript / TypeScript".toLowerCase().replace(" ", "-");
            for (const storage of [sessionStorage, localStorage]) {
              if (storage.getItem("runtime") === "javascript") {
                storage.setItem("runtime", newValue);
              }
            }
          } catch {}`}
        </Script>
        <WebMcp />
        <RootProvider>
          <DocsLayout tree={source.getPageTree()} {...baseOptions()}>
            {children}
          </DocsLayout>
        </RootProvider>
        <Analytics />
      </body>
    </html>
  );
}
