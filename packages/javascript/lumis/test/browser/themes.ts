import { available, closest, get } from "../../dist/themes.browser.js";
import optional from "@lumis-sh/themes/modus_vivendi";
import cases from "../../../../../fixtures/theme-choice-cases.json";

const output = document.querySelector("#output");
if (!output) throw new Error("missing output element");
output.textContent = JSON.stringify(cases.map((fixture) => closest(fixture)?.name ?? null));
output.dataset.themeNames = JSON.stringify(available().map((theme) => theme.name));
output.dataset.optionalLookup = String(get("modus_vivendi") === undefined);
output.dataset.importedOptional = optional.name;
