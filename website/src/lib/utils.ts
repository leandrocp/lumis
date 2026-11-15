const ACTIVE_TAB_CLASSES = ["border-accent", "text-fg"];
const INACTIVE_TAB_CLASSES = ["border-transparent", "text-muted"];

export function markTab(tab: HTMLElement, isSelected: boolean) {
  tab.setAttribute("aria-selected", String(isSelected));
  // With `nomemory`, focus entering the tablist lands on `focusgroupstart`.
  tab.toggleAttribute("focusgroupstart", isSelected);
  ACTIVE_TAB_CLASSES.forEach((c) => tab.classList.toggle(c, isSelected));
  INACTIVE_TAB_CLASSES.forEach((c) => tab.classList.toggle(c, !isSelected));
}

export async function polyfillFocusgroup(root: HTMLElement) {
  if ("focusGroup" in HTMLElement.prototype) return;
  const { polyfill } = await import("@microsoft/focusgroup-polyfill/shadowless");
  polyfill(root);
}

export function setupTabs(
  root: HTMLElement,
  tabSelector: string,
  dataAttr: string,
  panelSelector: string,
  panelDataAttr: string,
) {
  const tabs = root.querySelectorAll<HTMLButtonElement>(tabSelector);
  const select = (tab: HTMLButtonElement) => {
    const id = tab.dataset[dataAttr]!;
    tabs.forEach((t) => markTab(t, t === tab));
    root.querySelectorAll<HTMLElement>(panelSelector).forEach((panel) => {
      panel.classList.toggle("hidden", panel.dataset[panelDataAttr] !== id);
    });
  };
  tabs.forEach((tab) => {
    // Selection follows focus, so arrow keys switch panels. Safari does not
    // focus a clicked button, so clicks select too.
    tab.addEventListener("focus", () => select(tab));
    tab.addEventListener("click", () => select(tab));
  });
  void polyfillFocusgroup(root);
}

export function escapeHtml(str: string) {
  return str
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

const FEEDBACK_MS = 1500;
const iconResets = new WeakMap<HTMLButtonElement, number>();
let lastAnnouncement = "";
let announcementReset = 0;

// Every copy button shares one live region, so it gets one reset timer: an
// earlier button's timer must not clear a later button's message.
function announce(message: string) {
  const liveRegion = document.querySelector("#live-region");
  if (!liveRegion) return;
  // VoiceOver skips a message identical to the previous one, even after the
  // region was emptied, and a no-break space is not read aloud.
  lastAnnouncement = message === lastAnnouncement ? `${message}\u00A0` : message;
  liveRegion.textContent = lastAnnouncement;
  clearTimeout(announcementReset);
  announcementReset = window.setTimeout(() => (liveRegion.textContent = ""), FEEDBACK_MS);
}

export async function copyText(button: HTMLButtonElement, text: string) {
  let copied = true;
  try {
    await navigator.clipboard.writeText(text);
  } catch {
    copied = false;
  }
  // Cleared after the write settles, so overlapping clicks leave one timer.
  clearTimeout(iconResets.get(button));
  button.querySelector("svg")!.outerHTML = copied ? CHECK_SVG : COPY_SVG;
  button.classList.toggle("text-accent", copied);
  announce(copied ? "Copied to clipboard" : "Copy failed");
  iconResets.set(
    button,
    window.setTimeout(() => {
      button.querySelector("svg")!.outerHTML = COPY_SVG;
      button.classList.remove("text-accent");
    }, FEEDBACK_MS),
  );
}

export function setupCopyButtons(root: HTMLElement) {
  root.querySelectorAll<HTMLButtonElement>(".copy-install").forEach((btn) => {
    btn.addEventListener("click", () => void copyText(btn, decodeURIComponent(btn.dataset.copy!)));
  });
}

export const GITHUB_SVG = `<svg class="h-4 w-4" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>`;

export const BURGER_SVG = `<svg class="h-5 w-5" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="3" y1="6" x2="21" y2="6"/><line x1="3" y1="12" x2="21" y2="12"/><line x1="3" y1="18" x2="21" y2="18"/></svg>`;
export const CLOSE_SVG = `<svg class="h-5 w-5" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>`;

export const COPY_SVG = `<svg class="h-4 w-4" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg>`;
export const CHECK_SVG = `<svg class="h-4 w-4" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="20 6 9 17 4 12"/></svg>`;
