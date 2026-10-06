import type { Theme } from "@lumis-sh/lumis";
import type { WorkerCommand, WorkerResponse } from "./highlight-worker";
import type { LanguageOption } from "../data/languages";

let worker: Worker | undefined;
let nextId = 0;
const pending = new Map<number, { resolve: (v: WorkerResponse) => void }>();

function createWorker() {
  const instance = new Worker(new URL("./highlight-worker.ts", import.meta.url), {
    type: "module",
  });
  instance.onmessage = (e: MessageEvent<WorkerResponse>) => {
    const handler = pending.get(e.data.id);
    if (handler) {
      pending.delete(e.data.id);
      handler.resolve(e.data);
    }
  };
  return instance;
}

function send(req: WorkerCommand): Promise<WorkerResponse> {
  const activeWorker = (worker ??= createWorker());
  const id = nextId++;
  return new Promise((resolve) => {
    pending.set(id, { resolve });
    // oxlint-disable-next-line unicorn/require-post-message-target-origin -- Worker.postMessage has a transfer-list/options argument, not a Window target origin.
    activeWorker.postMessage({ ...req, id });
  });
}

/**
 * Load languages into the runtime the worker highlights with, concurrently.
 *
 * Returns every language the runtime now holds, so a caller can tell a warm
 * language from one that still has to be fetched. Highlighting loads on demand
 * regardless, so a caller that does not need the answer should not await this.
 */
export async function loadLanguages(languageIds: string[]): Promise<string[]> {
  const res = await send({ type: "loadLanguages", languageIds });
  if (res.type === "error") throw new Error(res.message);
  return (res as Extract<WorkerResponse, { type: "done" }>).loaded;
}

export async function renderHighlight(
  language: LanguageOption,
  theme: Theme,
  source: string,
  preClass?: string,
): Promise<string> {
  const res = await send({ type: "highlight", languageId: language.id, theme, source, preClass });
  if (res.type === "error") throw new Error(res.message);
  return (res as Extract<WorkerResponse, { type: "result" }>).html;
}

export async function renderHighlightMultiTheme(
  language: LanguageOption,
  lightTheme: Theme,
  darkTheme: Theme,
  source: string,
  preClass?: string,
): Promise<string> {
  const res = await send({
    type: "highlightMultiTheme",
    languageId: language.id,
    lightTheme,
    darkTheme,
    source,
    preClass,
  });
  if (res.type === "error") throw new Error(res.message);
  return (res as Extract<WorkerResponse, { type: "result" }>).html;
}
