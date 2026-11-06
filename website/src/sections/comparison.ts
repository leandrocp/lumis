import { ACTIVE_TAB_CLASSES, INACTIVE_TAB_CLASSES } from "../lib/utils";

const DATA = "/comparison-data";
// Every library timed on a small and a large file, published by
// `mise run -C benchmarks comparison-timings-publish`.
const TIMINGS = "/benchmark-data/comparison.json";

interface ComparisonDocument {
  id: string;
  label: string;
  language: string;
  languageLabel: string;
  source: string;
  lines: number;
  injections: string[];
  unsupported?: string[];
}

interface ComparisonTheme {
  id: string;
  name: string;
  appearance: "light" | "dark";
  source: string;
}

interface Manifest {
  themes: ComparisonTheme[];
  lumisRuntimes: string[];
  implementations: Array<{ id: string; label: string; version: string; theme: string }>;
  documents: ComparisonDocument[];
}

interface SavedState {
  document?: string;
  implementation?: string;
  positions: Record<string, number>;
}

const STATE_KEY = "lumis-comparison";

function readState(): SavedState {
  try {
    const value: unknown = JSON.parse(sessionStorage.getItem(STATE_KEY) ?? "null");
    if (typeof value !== "object" || value === null) return { positions: {} };
    const field = (key: string): unknown => (key in value ? Reflect.get(value, key) : undefined);
    const documentId = field("document");
    const implementationId = field("implementation");
    const positions = field("positions");
    return {
      document: typeof documentId === "string" ? documentId : undefined,
      implementation: typeof implementationId === "string" ? implementationId : undefined,
      positions: Object.fromEntries(
        Object.entries(typeof positions === "object" && positions !== null ? positions : {}).filter(
          (entry): entry is [string, number] => typeof entry[1] === "number",
        ),
      ),
    };
  } catch {
    return { positions: {} };
  }
}

function writeState(state: SavedState) {
  try {
    sessionStorage.setItem(STATE_KEY, JSON.stringify(state));
  } catch {
    // Storage can be disabled or full; the page still works without it.
  }
}

function navigate(frame: HTMLIFrameElement, page: string, at?: number) {
  // Setting `src` would add an entry to the session history, so Back would step
  // the frame through earlier tabs, with stale offsets, while the tabs above it
  // stayed on the current selection.
  const location = frame.contentWindow?.location;
  if (at === undefined) return location?.replace(page);
  // Back restores the frame to the URL it had when the page was left, so `page`
  // can be the document it already shows. Changing only the fragment does not
  // reload it, and its script never sees the new `#at=`.
  const panel = location?.pathname === page ? frame.contentDocument?.querySelector("pre") : null;
  location?.replace(`${page}#at=${at}`);
  if (panel) panel.scrollTop = at;
}

function paint(container: HTMLElement, selected: string) {
  for (const button of Array.from(container.children) as HTMLButtonElement[]) {
    const isSelected = button.dataset.id === selected;
    button.setAttribute("aria-selected", String(isSelected));
    button.classList.remove(...ACTIVE_TAB_CLASSES, ...INACTIVE_TAB_CLASSES);
    button.classList.add(...(isSelected ? ACTIVE_TAB_CLASSES : INACTIVE_TAB_CLASSES));
  }
}

export function renderComparison() {
  return `
    <section id="comparison" class="pt-32 pb-24 sm:pt-40 sm:pb-36">
      <div class="mx-auto max-w-6xl px-6">
        <h1 class="inline-flex items-center gap-1 font-mono text-sm font-semibold tracking-wider">
          <span class="text-accent">&lt;</span><span class="text-accent">Comparison</span> <span class="text-accent">/&gt;</span>
        </h1>

        <div class="comparison-shell mt-8 border border-line rounded-lg">
          <div class="border-b border-line px-5 py-4 ">
            <p id="comparison-documents-label" class="mb-3 font-mono text-[11px] tracking-wider text-muted uppercase dark:text-subtle">
              Source files
            </p>
            <div class="comparison-documents flex flex-wrap gap-2" role="tablist" aria-labelledby="comparison-documents-label"></div>
          </div>
          <div class="border-b border-line px-5 ">
            <div class="comparison-implementations flex flex-wrap gap-x-4 sm:gap-x-6" role="tablist" aria-label="Implementation"></div>
          </div>
          <div class="comparison-viewport h-[70vh] min-h-[420px] bg-surface-2">
            <iframe class="comparison-frame block h-full w-full border-0" title="Highlighted output" loading="eager"></iframe>
            <p class="comparison-unsupported hidden h-full items-center justify-center px-6 text-center font-mono text-sm text-muted"></p>
          </div>
        </div>

        <p class="comparison-missing mt-6 hidden font-mono text-sm text-muted"></p>

        <div class="comparison-timings mt-12 hidden">
          <h2 class="font-mono text-sm font-semibold tracking-wider text-accent">Speed and output</h2>
          <div class="comparison-metrics mt-4 flex flex-wrap gap-2" role="tablist" aria-label="Metric"></div>
          <div class="mt-4 overflow-x-auto rounded-lg border border-line">
            <table class="comparison-table w-full border-collapse font-mono text-xs"></table>
          </div>
          <p class="comparison-metric-note mt-3 text-sm text-muted"></p>
          <dl class="comparison-methodology mt-6 grid gap-x-6 gap-y-2 text-sm sm:grid-cols-[auto_1fr]"></dl>
        </div>
      </div>
    </section>`;
}

export async function setupComparison(root: HTMLElement) {
  const documentTabs = root.querySelector<HTMLDivElement>(".comparison-documents")!;
  const implementationTabs = root.querySelector<HTMLDivElement>(".comparison-implementations")!;
  const frame = root.querySelector<HTMLIFrameElement>(".comparison-frame")!;
  const unsupported = root.querySelector<HTMLParagraphElement>(".comparison-unsupported")!;
  const shell = root.querySelector<HTMLDivElement>(".comparison-shell")!;
  const missing = root.querySelector<HTMLParagraphElement>(".comparison-missing")!;

  let manifest: Manifest;
  try {
    const response = await fetch(`${DATA}/manifest.json`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    manifest = (await response.json()) as Manifest;
  } catch {
    shell.classList.add("hidden");
    missing.classList.remove("hidden");
    missing.textContent =
      "The comparison assets are not published in this build. Run `mise run -C benchmarks showcase-publish` and reload.";
    return;
  }

  manifest.documents.sort(
    (a, b) =>
      Number(b.id === "readme") - Number(a.id === "readme") || a.label.localeCompare(b.label, "en"),
  );

  // Switching either tab reloads the frame, so the frame reports where it is and
  // the next page is asked to resume there. Positions are kept per document,
  // because the same offset in a different file is not the same place, while the
  // same offset in a different implementation of one file is. The browser does
  // not restore a scroll offset inside a frame whose src was set by script, so
  // the state is kept for the tab's session to survive a reload or a Back.
  const saved = readState();
  const positions = new Map(Object.entries(saved.positions));
  let currentDocument =
    manifest.documents.find((entry) => entry.id === saved.document) ?? manifest.documents[0]!;
  let currentImplementation =
    manifest.implementations.find((entry) => entry.id === saved.implementation) ??
    manifest.implementations[0]!;
  const saveState = () =>
    writeState({
      document: currentDocument.id,
      implementation: currentImplementation.id,
      positions: Object.fromEntries(positions),
    });

  // The outputs are files rendered ahead of time, so the page cannot restyle one
  // the way the live showcase restyles its own. Both flavours are published and
  // the reader's setting picks between them, which is also why this listens:
  // switching appearance mid-visit would otherwise leave a light page around a
  // dark panel until the next reload.
  const prefersDark = matchMedia("(prefers-color-scheme: dark)");
  const themeFor = (dark: boolean) =>
    manifest.themes.find((theme) => theme.appearance === (dark ? "dark" : "light")) ??
    manifest.themes[0]!;
  let currentTheme = themeFor(prefersDark.matches);
  prefersDark.addEventListener("change", (event) => {
    currentTheme = themeFor(event.matches);
    show();
  });

  addEventListener("message", (event: MessageEvent) => {
    const scroll = (event.data as { lumisShowcaseScroll?: number } | null)?.lumisShowcaseScroll;
    if (typeof scroll !== "number") return;
    // At fractional device pixel ratios the offset is fractional, and the frame
    // reads only whole pixels from `#at=`.
    positions.set(currentDocument.id, Math.round(scroll));
    saveState();
  });

  function show() {
    // Not every library reads every language, and a pair with no output has no
    // file to load, so the panel says so in its place.
    const isUnsupported = currentDocument.unsupported?.includes(currentImplementation.id) ?? false;
    frame.classList.toggle("hidden", isUnsupported);
    unsupported.classList.toggle("hidden", !isUnsupported);
    unsupported.classList.toggle("flex", isUnsupported);
    unsupported.textContent = isUnsupported
      ? `${currentImplementation.label} does not support ${currentDocument.languageLabel}.`
      : "";

    if (isUnsupported) navigate(frame, "about:blank");
    else
      navigate(
        frame,
        `${DATA}/${currentDocument.id}/${currentTheme.id}/${currentImplementation.id}.html`,
        positions.get(currentDocument.id) ?? 0,
      );
    frame.title =
      `${currentImplementation.label} highlighting ${currentDocument.label} ` +
      `in ${currentTheme.name}`;
  }

  const selectDocument = (entry: ComparisonDocument) => () => {
    currentDocument = entry;
    saveState();
    paint(documentTabs, entry.id);
    show();
  };

  const selectImplementation = (entry: Manifest["implementations"][number]) => () => {
    currentImplementation = entry;
    saveState();
    paint(implementationTabs, entry.id);
    show();
  };

  for (const entry of manifest.documents) {
    const button = document.createElement("button");
    button.dataset.id = entry.id;
    button.setAttribute("role", "tab");
    button.className =
      "shrink-0 cursor-pointer border border-control px-2.5 py-1 font-mono text-[11px] tracking-wider whitespace-nowrap uppercase transition-colors sm:px-3 sm:py-1.5 sm:text-xs ";
    button.textContent = entry.label;
    button.addEventListener("click", selectDocument(entry));
    documentTabs.append(button);
  }

  for (const entry of manifest.implementations) {
    const button = document.createElement("button");
    button.dataset.id = entry.id;
    button.setAttribute("role", "tab");
    button.className =
      "cursor-pointer border-b-2 py-2 font-mono text-[11px] tracking-wider whitespace-nowrap uppercase transition-colors sm:py-3 sm:text-xs";
    button.textContent = entry.label;
    button.addEventListener("click", selectImplementation(entry));
    implementationTabs.append(button);
  }

  paint(documentTabs, currentDocument.id);
  paint(implementationTabs, currentImplementation.id);
  show();

  const timings = await loadTimings();
  if (timings) setupTimings(root, timings);
}

interface TimedResult {
  id: string;
  totalNs: number;
  setupNs: number;
  peakRssBytes: number;
  tokens: number;
}

interface TimedDocument {
  id: string;
  label: string;
  languageLabel: string;
  source: string;
  bytes: number;
  lines: number;
  size: string;
  results: Map<string, TimedResult>;
}

interface Timings {
  implementations: Array<{ id: string; label: string }>;
  documents: TimedDocument[];
  methodology: Array<[string, string]>;
}

type MetricId = "highlight" | "setup" | "memory" | "tokens";

interface Metric {
  id: MetricId;
  label: string;
  note: string;
  value: (result: TimedResult) => number;
  format: (value: number) => string;
}

const METRICS: Metric[] = [
  {
    id: "highlight",
    label: "Highlight",
    note: "Median time of one highlight of the whole file, with the highlighter already built and its languages loaded.",
    value: (result) => result.totalNs,
    format: formatDuration,
  },
  {
    id: "setup",
    label: "Setup + first highlight",
    note:
      "A fresh process building the highlighter, loading the languages the file needs and highlighting it once. " +
      "Lumis' data directory already holds its compiled parsers, as it does after a first run.",
    value: (result) => result.setupNs,
    format: formatDuration,
  },
  {
    id: "memory",
    label: "Peak RSS",
    note: "The most memory that fresh process held, its runtime included: Node.js and V8, or the Rust executable.",
    value: (result) => result.peakRssBytes,
    format: formatBytes,
  },
  {
    id: "tokens",
    label: "Tokens",
    note:
      "Spans the highlighter gave a colour to. Grammars split punctuation and whitespace differently, " +
      "so read it as how finely the file was resolved, not as a score.",
    value: (result) => result.tokens,
    format: (value) => value.toLocaleString("en"),
  },
];

function setupTimings(root: HTMLElement, timings: Timings) {
  const section = root.querySelector<HTMLDivElement>(".comparison-timings")!;
  const tabs = root.querySelector<HTMLDivElement>(".comparison-metrics")!;
  const table = root.querySelector<HTMLTableElement>(".comparison-table")!;
  const note = root.querySelector<HTMLParagraphElement>(".comparison-metric-note")!;
  const methodology = root.querySelector<HTMLDListElement>(".comparison-methodology")!;
  const { documents } = timings;

  for (const [label, value] of timings.methodology) {
    const term = document.createElement("dt");
    term.className = "font-mono text-xs tracking-wider uppercase";
    term.textContent = label;
    const detail = document.createElement("dd");
    detail.className = "text-muted";
    detail.textContent = value;
    methodology.append(term, detail);
  }

  function draw(metric: Metric) {
    paint(tabs, metric.id);
    note.textContent = metric.note;

    const head = document.createElement("tr");
    head.append(cell("th", "Library", "sticky left-0 bg-surface-2 text-left align-bottom"));
    for (const entry of documents) {
      const header = cell("th", entry.size, "w-1/3 min-w-40 text-right align-bottom");
      const name = document.createElement("a");
      name.textContent = entry.label;
      name.href = entry.source;
      name.target = "_blank";
      name.rel = "noreferrer";
      name.className = "block font-normal underline-offset-2 hover:underline";
      const size = document.createElement("span");
      size.className = "block font-normal text-[11px] text-muted";
      size.textContent =
        `${entry.languageLabel} · ${formatBytes(entry.bytes)} · ` +
        `${entry.lines.toLocaleString("en")} lines`;
      header.append(name, size);
      head.append(header);
    }

    const rows = timings.implementations.map(({ id, label }) => {
      const row = document.createElement("tr");
      row.className = "border-t border-line";
      row.append(
        cell("th", label, "sticky left-0 min-w-32 bg-surface-2 text-left sm:whitespace-nowrap"),
      );
      for (const entry of documents) {
        const result = entry.results.get(id);
        const value = result && metric.value(result);
        row.append(
          cell(
            "td",
            value === undefined ? "—" : metric.format(value),
            `text-right tabular-nums whitespace-nowrap ${value === undefined ? "text-muted" : ""}`,
          ),
        );
      }
      return row;
    });
    table.replaceChildren(head, ...rows);
  }

  for (const metric of METRICS) {
    const button = document.createElement("button");
    button.dataset.id = metric.id;
    button.setAttribute("role", "tab");
    button.className =
      "cursor-pointer border border-control px-2.5 py-1 font-mono text-[11px] tracking-wider uppercase transition-colors sm:px-3 sm:py-1.5 sm:text-xs";
    button.textContent = metric.label;
    button.addEventListener("click", () => draw(metric));
    tabs.append(button);
  }

  draw(METRICS[0]!);
  section.classList.remove("hidden");
}

function cell(tag: "th" | "td", text: string, className: string) {
  const element = document.createElement(tag);
  element.className = `px-3 py-2 ${className}`;
  element.textContent = text;
  return element;
}

function formatDuration(ns: number) {
  if (ns >= 1e9) return `${(ns / 1e9).toFixed(2)} s`;
  if (ns >= 1e6) return `${(ns / 1e6).toFixed(ns >= 1e8 ? 0 : 1)} ms`;
  return `${(ns / 1e3).toFixed(0)} µs`;
}

function formatBytes(bytes: number) {
  if (bytes >= 2 ** 20) return `${(bytes / 2 ** 20).toFixed(1)} MiB`;
  return `${(bytes / 2 ** 10).toFixed(0)} KiB`;
}

// A build without the report still gets the gallery, so a missing or malformed
// report hides the table rather than breaking the page.
async function loadTimings(): Promise<Timings | undefined> {
  try {
    const response = await fetch(TIMINGS);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return readTimings(await response.json());
  } catch {
    return undefined;
  }
}

// The report is fetched, so its shape is checked here and anything unexpected
// drops the whole report: half a table would put `NaN` in some cells.
function readTimings(report: unknown): Timings {
  if (!isObject(report)) throw new TypeError("timing report is not an object");
  const implementations = arrayOf(report.implementations).map((entry) => {
    if (!isObject(entry)) throw new TypeError("bad implementation");
    return { id: stringField(entry, "id"), label: stringField(entry, "label") };
  });
  const documents = arrayOf(report.documents).map((entry): TimedDocument => {
    if (!isObject(entry)) throw new TypeError("bad document");
    return {
      id: stringField(entry, "id"),
      label: stringField(entry, "label"),
      languageLabel: stringField(entry, "languageLabel"),
      source: stringField(entry, "source"),
      bytes: numberField(entry, "bytes"),
      lines: numberField(entry, "lines"),
      size: stringField(entry, "size"),
      results: new Map(
        arrayOf(entry.results).map((result) => {
          if (!isObject(result)) throw new TypeError("bad result");
          const id = stringField(result, "id");
          return [
            id,
            {
              id,
              totalNs: numberField(result, "totalNs"),
              setupNs: numberField(result, "setupNs"),
              peakRssBytes: numberField(result, "peakRssBytes"),
              tokens: numberField(result, "tokens"),
            },
          ];
        }),
      ),
    };
  });
  return { implementations, documents, methodology: readMethodology(report) };
}

function readMethodology(report: { [key: string]: unknown }): Array<[string, string]> {
  const { system, runtimes } = report;
  if (!isObject(system) || !isObject(runtimes)) throw new TypeError("no system or runtimes");
  return [
    ["Measured", stringField(report, "measuredAt").slice(0, 10)],
    [
      "Hardware",
      `${stringField(system, "cpu")}, ${numberField(system, "cores")} cores, ` +
        `${formatBytes(numberField(system, "memoryBytes"))} RAM`,
    ],
    ["Operating system", `${stringField(system, "os")} (${stringField(system, "architecture")})`],
    ["Runtimes", `Node.js ${stringField(runtimes, "node")}, ${stringField(runtimes, "rust")}`],
    ["Highlight", stringField(report, "timingBoundary")],
    ["Setup", stringField(report, "setupBoundary")],
  ];
}

function isObject(value: unknown): value is { [key: string]: unknown } {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function arrayOf(value: unknown): unknown[] {
  if (!Array.isArray(value)) throw new TypeError("expected an array");
  return value;
}

function stringOf(value: unknown) {
  return typeof value === "string" ? value : undefined;
}

function numberOf(value: unknown) {
  return typeof value === "number" && Number.isFinite(value) ? value : undefined;
}

function stringField(object: { [key: string]: unknown }, key: string) {
  const value = stringOf(object[key]);
  if (value === undefined) throw new TypeError(`${key} is not a string`);
  return value;
}

function numberField(object: { [key: string]: unknown }, key: string) {
  const value = numberOf(object[key]);
  if (value === undefined || value < 0) throw new TypeError(`${key} is not a number`);
  return value;
}
