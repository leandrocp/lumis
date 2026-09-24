/**
 * Loading languages into the runtime `highlight()` already uses, by name.
 *
 * Built once here and wired to each entry's default runtime, so Node and the
 * browser cannot disagree about what loading a name does.
 */
import { loadLanguageDefinition } from "./highlighter.js";
import { expandBundles, resolveLanguage } from "./language-names.js";
import type { RuntimeLike } from "./languages.js";

export type LoadLanguages = (names: Iterable<string>) => Promise<string[]>;

export function createLoadLanguages(loadLanguage: RuntimeLike["loadLanguage"]): LoadLanguages {
  return async function loadLanguages(names: Iterable<string>): Promise<string[]> {
    const failures: Error[] = [];
    const wanted: string[] = [];

    // Expanded one name at a time. Expanding the whole list at once makes an
    // unknown bundle abandon every name beside it, and `Lumis.Languages.load/1`
    // instead reports that bundle and loads the rest.
    for (const name of names) {
      try {
        wanted.push(...expandBundles([name]));
      } catch (error) {
        failures.push(new Error(`could not load "${name}"`, { cause: error }));
      }
    }

    // Every name is attempted and every failure reported, rather than stopping
    // at the first: one unpublished parser in a bundle should not cost the rest,
    // the same way one bad block does not cost a document.
    const unique = [...new Set(wanted)];
    const settled = await Promise.allSettled(
      unique.map(async (name) => {
        const language = await resolveLanguage(name);
        await loadLanguageDefinition({ loadLanguage }, language);
        return language.id;
      }),
    );

    const loaded: string[] = [];
    for (const [index, result] of settled.entries()) {
      if (result.status === "fulfilled") {
        loaded.push(result.value);
      } else {
        failures.push(new Error(`could not load "${unique[index]}"`, { cause: result.reason }));
      }
    }

    if (failures.length > 0) {
      throw new AggregateError(
        failures,
        `could not load ${failures.length} of ${failures.length + loaded.length} languages`,
      );
    }

    return [...new Set(loaded)];
  };
}
