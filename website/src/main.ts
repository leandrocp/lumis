import "./styles.css";
import { inject } from "@vercel/analytics";
import { mountApp } from "./app";
import { registerWebMcpTools } from "./webmcp";

inject();
try {
  await registerWebMcpTools();
} catch (error: unknown) {
  console.warn("Could not register Lumis WebMCP tools", error);
}
void mountApp(document.querySelector<HTMLDivElement>("#app")!);
