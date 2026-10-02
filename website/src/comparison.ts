import "./styles.css";
import { inject } from "@vercel/analytics";
import { setupNav } from "./sections/nav";
import { setupComparison } from "./sections/comparison";
import { renderComparisonPage } from "./comparison-page";

inject();

const root = document.querySelector<HTMLDivElement>("#app")!;

// The build prerenders this markup into the page; the dev server does not.
if (import.meta.env.DEV) root.innerHTML = renderComparisonPage();

setupNav(root);
void setupComparison(root);
