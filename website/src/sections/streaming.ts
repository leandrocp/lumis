import { loadTheme } from "../data/themes";
import { LANGUAGES_BY_ID } from "../data/languages";
import { renderHighlightMultiTheme } from "../lib/highlighter";

const INCOMPLETE_CODE = `## Solution

${"```"}javascript
async function fetchUserData(userId) {
  const response = await fetch(\`/api/users/\${userId}\`)
  if (!response.ok) throw new Error("request failed")

  const data = await response.json()
  return data.profile.displayNam
`;

export function renderStreaming() {
  return `
    <section id="streaming" class="py-24 sm:py-36">
      <div class="mx-auto max-w-6xl px-6">
        <a href="#streaming" class="group inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider no-underline transition-opacity hover:opacity-80">
          <span class="text-fuchsia-400">&lt;</span><span class="text-blue-400">Streaming</span> <span class="text-fuchsia-400">/&gt;</span>
        </a>
        <h2 class="mt-8 font-mono text-4xl font-bold tracking-tight text-zinc-900 dark:text-white">
          Fast and accurate even for incomplete code.
        </h2>
        <div class="mt-12">
          <div class="mb-3 flex items-center gap-2">
            <span class="inline-block h-2 w-2 rounded-full bg-emerald-400"></span>
            <span class="font-mono text-[11px] tracking-wider text-zinc-500 uppercase dark:text-zinc-400">Markdown + incomplete fenced code</span>
          </div>
          <div class="overflow-hidden border border-zinc-200 dark:border-zinc-800">
            <div class="streaming-lumis [&_code]:font-mono"></div>
          </div>
        </div>
      </div>
    </section>`;
}

export async function setupStreaming(root: HTMLElement) {
  const container = root.querySelector<HTMLDivElement>(".streaming-lumis");
  if (!container) return;

  const observer = new IntersectionObserver(
    (entries) => {
      if (!entries[0].isIntersecting) return;
      observer.disconnect();
      void initStreaming(container);
    },
    { rootMargin: "200px" },
  );
  observer.observe(container);
}

async function initStreaming(container: HTMLDivElement) {
  const lang = LANGUAGES_BY_ID.get("markdown");
  if (!lang) return;

  const [lightTheme, darkTheme] = await Promise.all([
    loadTheme("catppuccin_latte"),
    loadTheme("catppuccin_frappe"),
  ]);

  const highlighted = await renderHighlightMultiTheme(
    lang,
    lightTheme,
    darkTheme,
    INCOMPLETE_CODE,
    "m-0 overflow-x-auto p-5 font-mono text-[13px] leading-relaxed sm:p-6 sm:text-sm",
  );
  container.innerHTML = highlighted;

  if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;

  const pre = container.querySelector("pre");
  if (!pre) return;

  const textEntries: { node: Text; full: string }[] = [];
  const walker = document.createTreeWalker(pre, NodeFilter.SHOW_TEXT);
  let current: Node | null;
  while ((current = walker.nextNode())) {
    const textNode = current as Text;
    if (textNode.textContent) {
      textEntries.push({ node: textNode, full: textNode.textContent });
    }
  }

  const fullHeight = pre.scrollHeight;
  pre.style.minHeight = `${fullHeight}px`;

  for (const entry of textEntries) entry.node.textContent = "";

  let entryIndex = 0;
  let charIndex = 0;

  await new Promise<void>((resolve) => {
    const tick = () => {
      if (entryIndex >= textEntries.length) {
        pre.style.minHeight = "";
        resolve();
        return;
      }
      const entry = textEntries[entryIndex];
      const step = Math.min(1 + Math.floor(Math.random() * 2), entry.full.length - charIndex);
      charIndex += step;
      entry.node.textContent = entry.full.slice(0, charIndex);
      if (charIndex >= entry.full.length) {
        entryIndex++;
        charIndex = 0;
      }
      setTimeout(tick, 20 + Math.floor(Math.random() * 25));
    };
    tick();
  });
}
