import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";
import { gitConfig } from "./shared";

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: (
        <>
          <img
            src="/brand/lockup-horizontal-light.svg"
            alt="Lumis"
            className="h-10 w-auto dark:hidden"
          />
          <img
            src="/brand/lockup-horizontal-dark.svg"
            alt="Lumis"
            className="hidden h-10 w-auto dark:block"
          />
          <span className="ms-1 rounded-full bg-fd-accent px-2 py-0.5 text-xs font-semibold text-fd-muted-foreground">
            Docs
          </span>
        </>
      ),
      url: "/",
    },
    links: [
      { text: "Website", url: "https://lumis.sh", external: true },
      { text: "Playground", url: "https://lumis.sh/#playground", external: true },
    ],
    githubUrl: `https://github.com/${gitConfig.user}/${gitConfig.repo}`,
  };
}
