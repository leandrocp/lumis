"use client";

import { useMemo, useRef } from "react";
import { Check, Clipboard } from "lucide-react";
import { buttonVariants } from "fumadocs-ui/components/ui/button";
import { useCopyButton } from "fumadocs-ui/utils/use-copy-button";
import { cn } from "@/lib/cn";

/** A code block highlighted by `plugins/remark-lumis.mjs`, with a copy button. */
export function LumisCodeBlock({ html, title }: { html: string; title?: string }) {
  const ref = useRef<HTMLDivElement>(null);
  const [checked, onClick] = useCopyButton(() => {
    const pre = ref.current?.querySelector("pre");
    if (pre) void navigator.clipboard.writeText(pre.textContent ?? "");
  });

  const copyButton = (
    <button
      type="button"
      data-checked={checked || undefined}
      aria-label={checked ? "Copied Text" : "Copy Text"}
      onClick={onClick}
      className={cn(
        buttonVariants({ size: "icon-xs" }),
        "text-fd-muted-foreground hover:text-fd-accent-foreground data-checked:text-fd-accent-foreground",
        !title && "absolute top-2 right-2 backdrop-blur-lg",
      )}
    >
      {checked ? <Check /> : <Clipboard />}
    </button>
  );
  const innerHtml = useMemo(() => ({ __html: html }), [html]);
  const code = <div ref={ref} dangerouslySetInnerHTML={innerHtml} />;

  if (!title) {
    return (
      <div className="not-prose relative">
        {code}
        {copyButton}
      </div>
    );
  }

  return (
    <div className="not-prose codeBlockContainer_lumis">
      <div className="codeBlockTitle_lumis">
        {title}
        {copyButton}
      </div>
      {code}
    </div>
  );
}
