import { i18nService } from './i18nService.svelte';

/**
 * Localization of the display text that built-in features declare in their
 * `src/built-in-features/<id>/manifest.json`.
 *
 * The manifests stay English and remain the source of truth; translations
 * live in the `builtin` namespace of the locale catalogs:
 *
 *   builtin.<ext>.name | .description
 *   builtin.<ext>.commands.<cmd>.name | .description
 *   builtin.<ext>.commands.<cmd>.arguments.<arg>.placeholder | .options.<value>
 *   builtin.<ext>.commands.<cmd>.accessory.<value>          (searchBarAccessory option titles)
 *   builtin.<ext>.commands.<cmd>.preferences.<pref>.…        (same shape as below)
 *   builtin.<ext>.commands.<cmd>.actions.<action>.title | .description
 *   builtin.<ext>.preferences.<pref>.title | .description | .placeholder | .options.<value>
 *   builtin.<ext>.actions.<action>.title | .description
 *   builtin.<ext>.walkthrough.<step>.title | .summary | .body
 *
 * Every lookup falls back to the manifest value, so a missing key never
 * blanks a label. `builtInManifestI18n.test.ts` keeps en.json identical to
 * the manifests.
 */

type Obj = Record<string, any>;

/** One displayed text field of a manifest: its catalog key and English value. */
export interface BuiltInManifestText {
  key: string;
  value: string;
}

/** Original (English) name of each localized command, for search keywords. */
const sourceCommandNames = new WeakMap<object, string>();

function isObj(value: unknown): value is Obj {
  return typeof value === 'object' && value !== null;
}

/**
 * Visit every displayed text field of `manifest`. `apply` receives the
 * catalog key and current value and returns the value to store (or nothing to
 * leave it). Shared by the localizer and the en-parity test so both always
 * agree on which fields are localized.
 */
function visitManifestTexts(
  manifest: Obj,
  apply: (key: string, value: string) => string | void,
  write: boolean,
): Obj {
  const out: Obj = write ? { ...manifest } : manifest;
  const base = `builtin.${manifest.id}`;

  const field = (target: Obj, prop: string, key: string) => {
    const value = target[prop];
    if (typeof value !== 'string' || value.length === 0) return;
    const next = apply(key, value);
    if (write && typeof next === 'string') target[prop] = next;
  };

  const list = (source: unknown, each: (item: Obj) => Obj | void): Obj[] | undefined => {
    if (!Array.isArray(source)) return undefined;
    return source.map((item) => {
      if (!isObj(item)) return item;
      const copy = write ? { ...item } : item;
      each(copy);
      return copy;
    });
  };

  const options = (target: Obj, prop: string, prefix: string) => {
    const mapped = list(target[prop], (opt) => {
      if (typeof opt.value === 'string' || typeof opt.value === 'number') {
        field(opt, 'title', `${prefix}.${opt.value}`);
      }
    });
    if (write && mapped) target[prop] = mapped;
  };

  const preferences = (target: Obj, prefix: string) => {
    const mapped = list(target.preferences, (pref) => {
      if (typeof pref.name !== 'string') return;
      const p = `${prefix}.preferences.${pref.name}`;
      field(pref, 'title', `${p}.title`);
      field(pref, 'description', `${p}.description`);
      field(pref, 'placeholder', `${p}.placeholder`);
      field(pref, 'label', `${p}.label`);
      options(pref, 'data', `${p}.options`);
    });
    if (write && mapped) target.preferences = mapped;
  };

  const actions = (target: Obj, prefix: string) => {
    const mapped = list(target.actions, (action) => {
      if (typeof action.id !== 'string') return;
      const p = `${prefix}.actions.${action.id}`;
      field(action, 'title', `${p}.title`);
      field(action, 'description', `${p}.description`);
    });
    if (write && mapped) target.actions = mapped;
  };

  field(out, 'name', `${base}.name`);
  field(out, 'description', `${base}.description`);
  preferences(out, base);
  actions(out, base);

  const commands = list(manifest.commands, (cmd) => {
    if (typeof cmd.id !== 'string') return;
    const p = `${base}.commands.${cmd.id}`;
    const englishName = cmd.name;
    field(cmd, 'name', `${p}.name`);
    field(cmd, 'description', `${p}.description`);
    if (write && typeof englishName === 'string') sourceCommandNames.set(cmd, englishName);

    const args = list(cmd.arguments, (arg) => {
      if (typeof arg.name !== 'string') return;
      const a = `${p}.arguments.${arg.name}`;
      field(arg, 'placeholder', `${a}.placeholder`);
      options(arg, 'data', `${a}.options`);
    });
    if (write && args) cmd.arguments = args;

    if (isObj(cmd.searchBarAccessory)) {
      if (write) cmd.searchBarAccessory = { ...cmd.searchBarAccessory };
      options(cmd.searchBarAccessory, 'options', `${p}.accessory`);
    }

    preferences(cmd, p);
    actions(cmd, p);
  });
  if (write && commands) out.commands = commands;

  const walkthrough = list(manifest.walkthrough, (step) => {
    if (typeof step.id !== 'string') return;
    const p = `${base}.walkthrough.${step.id}`;
    field(step, 'title', `${p}.title`);
    field(step, 'summary', `${p}.summary`);
    field(step, 'body', `${p}.body`);
  });
  if (write && walkthrough) out.walkthrough = walkthrough;

  return out;
}

/** Every displayed text of a built-in manifest with its catalog key (used by tests and the key generator). */
export function collectBuiltInManifestTexts(manifest: Obj): BuiltInManifestText[] {
  const texts: BuiltInManifestText[] = [];
  visitManifestTexts(manifest, (key, value) => void texts.push({ key, value }), false);
  return texts;
}

/** Translation for `key` in the active locale, or `fallback` when no catalog has it. */
function translateOr(key: string, fallback: string): string {
  const value = i18nService.t(key);
  return value === key ? fallback : value;
}

/**
 * Copy of a built-in manifest with its display text translated into the
 * active locale. The input is never mutated; ids, triggers and other
 * non-display fields are kept as is.
 */
export function localizeBuiltInManifest<T>(manifest: T): T {
  if (!isObj(manifest) || typeof manifest.id !== 'string') return manifest;
  return visitManifestTexts(manifest, translateOr, true) as T;
}

/**
 * The English (manifest) name of a command, even after localization. Search
 * keeps it as the fallback keyword so commands without an explicit `trigger`
 * stay findable by the name they had before.
 */
export function getSourceCommandName(cmd: { name: string }): string {
  return sourceCommandNames.get(cmd) ?? cmd.name;
}

let localeReady: Promise<void> | null = null;

/**
 * Resolve the system locale once before built-in manifests are localized.
 * `i18nService.init()` is fired without `await` by the windows' onMount, so
 * extension discovery could otherwise run (and index names) while the locale
 * is still the `en` default.
 */
export function ensureLocaleInitialized(): Promise<void> {
  localeReady ??= i18nService.init().catch(() => {});
  return localeReady;
}

/** Localize the manifest of every built-in record (installed extensions are left untouched). */
export async function localizeBuiltInRecords<R extends { isBuiltIn?: boolean; manifest?: unknown }>(
  records: R[],
): Promise<R[]>;
export async function localizeBuiltInRecords<R extends { isBuiltIn?: boolean; manifest?: unknown }>(
  records: R,
): Promise<R>;
export async function localizeBuiltInRecords(records: any): Promise<any> {
  if (records == null) return records;
  const list = Array.isArray(records) ? records : [records];
  if (!list.some((r) => isObj(r) && r.isBuiltIn)) return records;
  await ensureLocaleInitialized();
  const localized = list.map((r) =>
    isObj(r) && r.isBuiltIn && isObj(r.manifest)
      ? { ...r, manifest: localizeBuiltInManifest(r.manifest) }
      : r,
  );
  return Array.isArray(records) ? localized : localized[0];
}
