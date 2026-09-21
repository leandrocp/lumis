import "./styles.css";
import { inject } from "@vercel/analytics";
import { renderNav, setupNav } from "./sections/nav";
import { renderFooter } from "./sections/footer";
import { renderComparison, setupComparison } from "./sections/comparison";

inject();

const root = document.querySelector<HTMLDivElement>("#app")!;

root.innerHTML = [
  '<a href="#main-content" class="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-[100] focus:bg-zinc-900 focus:px-4 focus:py-2 focus:font-mono focus:text-sm focus:text-white dark:focus:bg-white dark:focus:text-zinc-900">Skip to content</a>',
  renderNav("/"),
  '<main id="main-content">',
  renderComparison(),
  "</main>",
  renderFooter(),
].join("");

setupNav(root);
void setupComparison(root);
