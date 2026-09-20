import { RootProvider } from "fumadocs-ui/provider/next";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { Analytics } from "@vercel/analytics/next";
import Script from "next/script";
import { source } from "@/lib/source";
import { baseOptions } from "@/lib/layout.shared";
import type { Metadata } from "next";
import "./global.css";

export const metadata: Metadata = {
  metadataBase: new URL("https://docs.lumis.sh"),
  title: { default: "Lumis Docs", template: "%s | Lumis Docs" },
  description:
    "Syntax highlighting with Lumis for JavaScript / TypeScript, Rust, Elixir, Java and the CLI.",
  applicationName: "Lumis Docs",
  icons: { icon: "/img/favicon.ico" },
};

export default function Layout({ children }: LayoutProps<"/">) {
  return (
    <html lang="en" suppressHydrationWarning>
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
