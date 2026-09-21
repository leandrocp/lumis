import "./styles.css";
import { inject } from "@vercel/analytics";
import { mountApp } from "./app";

inject();
void mountApp(document.querySelector<HTMLDivElement>("#app")!);
