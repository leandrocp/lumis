import "./styles.css";
import { inject } from "@vercel/analytics";
import { setupNav } from "./sections/nav";
import { setupComparison } from "./sections/comparison";

import { renderComparisonPage } from "./comparison-page";

inject();

const root = document.querySelector<HTMLDivElement>("#app")!;

if (!root.querySelector("main")) root.innerHTML = renderComparisonPage();

setupNav(root);
void setupComparison(root);
