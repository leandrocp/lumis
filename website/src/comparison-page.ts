import { renderNav } from "./sections/nav";
import { renderFooter } from "./sections/footer";
import { renderComparison } from "./sections/comparison";

export function renderComparisonPage() {
  return [
    '<a href="#main-content" class="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-[100] focus:rounded-md focus:bg-accent focus:px-4 focus:py-2 focus:font-mono focus:text-sm focus:text-accent-fg">Skip to content</a>',
    renderNav("/"),
    '<main id="main-content">',
    renderComparison(),
    "</main>",
    renderFooter(),
  ].join("");
}
