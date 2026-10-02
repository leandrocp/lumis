import { renderApp } from "./app";
import { renderComparisonPage } from "./comparison-page";
import { renderShowcasePage } from "./showcase-page";

export function renderPages() {
  return new Map([
    ["/index.html", renderApp()],
    ["/comparison/index.html", renderComparisonPage()],
    ["/showcase/index.html", renderShowcasePage()],
  ]);
}
