import type { ReactNode } from "react";
import type { InlineCodeProps } from "./clientShared.js";
import { useLumis } from "./useLumis.js";

export type { InlineCodeProps } from "./clientShared.js";

export function InlineCode(props: InlineCodeProps): ReactNode {
  const { content, error } = useLumis({ ...props, inline: true });

  if (error) {
    throw error;
  }

  return content;
}
