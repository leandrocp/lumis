import "./styles.css";
import { inject } from "@vercel/analytics";
import { setupNav } from "./sections/nav";
import { setupShowcase } from "./sections/showcase";
import { setupCopyButtons } from "./lib/utils";

import { renderShowcasePage } from "./showcase-page";

inject();

const root = document.querySelector<HTMLDivElement>("#app")!;

if (!root.querySelector("main")) root.innerHTML = renderShowcasePage();

setupNav(root);
setupCopyButtons(root);
setupShowcase(root);
