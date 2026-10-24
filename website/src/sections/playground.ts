import {
  LANGUAGES,
  LANGUAGES_BY_ID,
  PLAYGROUND_DEFAULT_LANGUAGE,
  getSample,
} from "../data/languages";
import { AVAILABLE_THEMES, loadTheme, THEMES, THEMES_BY_ID } from "../data/themes";
import { renderHighlight } from "../lib/highlighter";

function pickRandom<T>(items: T[]): T {
  return items[Math.floor(Math.random() * items.length)]!;
}

export function renderPlayground() {
  return `
    <section id="playground" class="py-24 sm:py-36">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#playground" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-pink-400">&lt;</span><span class="text-purple-400">Playground</span> <span class="text-pink-400">/&gt;</span>
        </a>
        <div class="mt-8 flex flex-col gap-6 sm:flex-row sm:items-end sm:justify-between">
          <h2 class="font-mono text-4xl font-bold tracking-tight text-zinc-900 dark:text-white">
            Try it live.
          </h2>
          <div class="flex flex-wrap items-end gap-3">
            <label>
              <span class="mb-1 block font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Language</span>
              <select name="language" class="h-9 border border-zinc-200 bg-white px-3 pr-8 font-mono text-xs text-zinc-900 outline-none focus:border-zinc-900 dark:border-zinc-800 dark:bg-[#09090b] dark:text-white dark:focus:border-white">
                ${LANGUAGES.map((lang) => `<option value="${lang.id}">${lang.label}</option>`).join("")}
              </select>
            </label>
            <label>
              <span class="mb-1 block font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Theme</span>
              <select name="theme" class="h-9 border border-zinc-200 bg-white px-3 pr-8 font-mono text-xs text-zinc-900 outline-none focus:border-zinc-900 dark:border-zinc-800 dark:bg-[#09090b] dark:text-white dark:focus:border-white">
                ${THEMES.map((theme) => `<option value="${theme.id}" ${theme.available ? "" : "disabled"}>${theme.label} [${theme.pack}]</option>`).join("")}
              </select>
            </label>
            <button type="button" class="randomize flex h-9 cursor-pointer items-center gap-2 border border-zinc-200 bg-white px-3 font-mono text-xs text-zinc-500 transition-colors hover:border-zinc-900 hover:text-zinc-900 dark:border-zinc-800 dark:bg-[#09090b] dark:text-zinc-400 dark:hover:border-white dark:hover:text-white">
              <svg class="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 3 21 3 21 8"/><line x1="4" y1="20" x2="21" y2="3"/><polyline points="21 16 21 21 16 21"/><line x1="15" y1="15" x2="21" y2="21"/><line x1="4" y1="4" x2="9" y2="9"/></svg>
              randomize
            </button>
          </div>
        </div>

        <p class="mt-4 font-mono text-xs text-zinc-500 dark:text-zinc-400">
          <span class="border border-zinc-300 px-1 dark:border-zinc-700">default</span> permissive themes included automatically.
          <span class="border border-zinc-300 px-1 dark:border-zinc-700">copyleft</span> optional pack, installed for this demo.
          <span class="border border-zinc-300 px-1 dark:border-zinc-700">unavailable</span> metadata only; redistribution permission missing.
          <a class="underline" href="https://docs.lumis.sh/themes">Theme packs and licenses</a>
        </p>
        <details class="mt-3 font-mono text-xs text-zinc-500 dark:text-zinc-400">
          <summary class="cursor-pointer">Unavailable themes (metadata only)</summary>
          <ul class="mt-2 grid gap-2 sm:grid-cols-2">
            ${THEMES.filter((theme) => !theme.available)
              .map(
                (theme) =>
                  `<li><button type="button" disabled class="cursor-not-allowed opacity-60">${theme.label} <span class="border border-zinc-300 px-1 dark:border-zinc-700">unavailable</span></button> — ${theme.license} · <a class="underline" href="${theme.url}">source</a></li>`,
              )
              .join("")}
          </ul>
        </details>
        <div class="mt-8 overflow-hidden border border-zinc-200 dark:border-zinc-800">
          <div class="preview-content max-h-[50rem] overflow-y-auto [&_code]:font-mono"><div class="flex items-center justify-center py-24 font-mono text-xs text-zinc-400">Highlighting...</div></div>
        </div>
      </div>
    </section>`;
}

export async function setupPlayground(root: HTMLElement) {
  const languageSelect = root.querySelector<HTMLSelectElement>('select[name="language"]')!;
  const themeSelect = root.querySelector<HTMLSelectElement>('select[name="theme"]')!;
  const preview = root.querySelector<HTMLDivElement>(".preview-content")!;
  const randomizeButton = root.querySelector<HTMLButtonElement>(".randomize")!;

  // The first render is the one the warm-up in `mountApp` has been fetching, so
  // it has to name the same language every time. Randomizing is what the button
  // is for.
  languageSelect.value = PLAYGROUND_DEFAULT_LANGUAGE;
  themeSelect.value = pickRandom(AVAILABLE_THEMES).id;

  let renderToken = 0;

  const render = async () => {
    const token = ++renderToken;
    const language = LANGUAGES_BY_ID.get(languageSelect.value) ?? LANGUAGES[0]!;
    const theme = THEMES_BY_ID.get(themeSelect.value) ?? THEMES[0]!;

    preview.dataset.state = "loading";

    try {
      const themeData = await loadTheme(theme.id);
      const html = await renderHighlight(language, themeData, getSample(language.id));
      if (token !== renderToken) return;

      preview.innerHTML = html;
      preview.dataset.state = "ready";
    } catch (err) {
      if (token !== renderToken) return;

      console.error(`Failed to highlight ${language.id}`, err);
      preview.innerHTML = `<div class="p-6 font-mono text-xs text-red-500">Failed to highlight ${language.label}: ${String(err)}</div>`;
      preview.dataset.state = "error";
    }
  };

  languageSelect.addEventListener("change", () => void render());
  themeSelect.addEventListener("change", () => void render());

  randomizeButton.addEventListener("click", () => {
    const language = pickRandom(LANGUAGES);
    const theme = pickRandom(AVAILABLE_THEMES);
    languageSelect.value = language.id;
    themeSelect.value = theme.id;
    void render();
  });

  await render();
}
