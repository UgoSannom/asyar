import { afterEach, describe, expect, it, vi } from 'vitest';

// Keep the system locale probe out of the test: the locale is set explicitly.
vi.mock('../../lib/ipc/commands', () => ({ getSystemLocale: vi.fn(async () => null) }));

import enCatalog from '../../locales/en.json';
import frCatalog from '../../locales/fr.json';
import { i18nService } from './i18nService.svelte';
import {
  collectBuiltInManifestTexts,
  getSourceCommandName,
  localizeBuiltInManifest,
  localizeBuiltInRecords,
} from './builtInManifestI18n';

const manifests = import.meta.glob<Record<string, any>>('../../built-in-features/*/manifest.json', {
  eager: true,
  import: 'default',
});

function lookup(catalog: Record<string, any>, key: string): unknown {
  return key.split('.').reduce<any>((node, part) => node?.[part], catalog);
}

function leafKeys(node: Record<string, any>, prefix: string): string[] {
  return Object.entries(node).flatMap(([k, v]) =>
    typeof v === 'object' && v !== null ? leafKeys(v, `${prefix}.${k}`) : [`${prefix}.${k}`],
  );
}

describe('built-in manifest localization', () => {
  const originalLocale = i18nService.locale;
  afterEach(() => {
    i18nService.setLocale(originalLocale);
  });

  const all = Object.entries(manifests);

  it('finds the built-in manifests', () => {
    expect(all.length).toBeGreaterThan(20);
  });

  // Keeps en.json in lockstep with the manifests: editing a displayed manifest
  // text without regenerating the `builtin` keys fails here.
  for (const [path, manifest] of all) {
    it(`en.json mirrors every displayed text of ${manifest.id}`, () => {
      const texts = collectBuiltInManifestTexts(manifest);
      expect(texts.length, `${path} has no displayed text`).toBeGreaterThan(0);
      for (const { key, value } of texts) {
        expect(lookup(enCatalog, key), `en.json ${key}`).toBe(value);
      }
    });
  }

  it('en.json has no stale builtin keys', () => {
    const expected = new Set(
      all.flatMap(([, m]) => collectBuiltInManifestTexts(m)).map((x) => x.key),
    );
    const builtin = (enCatalog as Record<string, any>).builtin ?? {};
    for (const key of leafKeys(builtin, 'builtin')) {
      expect(expected.has(key), `stale key ${key}`).toBe(true);
    }
  });

  it('is the identity in English', () => {
    i18nService.setLocale('en');
    for (const [, manifest] of all) {
      expect(localizeBuiltInManifest(manifest)).toEqual(manifest);
    }
  });

  it('translates every displayed field and leaves ids alone', () => {
    i18nService.setLocale('fr');
    const manifest = all.find(([, m]) => m.id === 'file-search')![1];
    const localized = localizeBuiltInManifest(manifest);

    expect(localized.name).toBe(lookup(frCatalog, 'builtin.file-search.name'));
    const cmd = localized.commands[0];
    expect(cmd.id).toBe(manifest.commands[0].id);
    expect(cmd.trigger).toBe(manifest.commands[0].trigger);
    expect(cmd.name).toBe(lookup(frCatalog, `builtin.file-search.commands.${cmd.id}.name`));
    const option = cmd.searchBarAccessory.options[0];
    expect(option.value).toBe(manifest.commands[0].searchBarAccessory.options[0].value);
    expect(option.title).toBe(
      lookup(frCatalog, `builtin.file-search.commands.${cmd.id}.accessory.${option.value}`),
    );
    expect(localized.walkthrough[0].body).toBe(
      lookup(frCatalog, `builtin.file-search.walkthrough.${manifest.walkthrough[0].id}.body`),
    );
    // The source manifest is never mutated.
    expect(manifest.name).toBe('File Search');
    // The English name stays reachable for search keywords.
    expect(getSourceCommandName(cmd)).toBe(manifest.commands[0].name);
  });

  it('falls back to the manifest text when a key is missing', () => {
    i18nService.setLocale('fr');
    const manifest = {
      id: 'not-a-real-feature',
      name: 'Untranslated',
      commands: [{ id: 'x', name: 'Do it', description: 'Does it' }],
    };
    expect(localizeBuiltInManifest(manifest)).toEqual(manifest);
  });

  it('only localizes built-in records', async () => {
    i18nService.setLocale('fr');
    const manifest = all.find(([, m]) => m.id === 'notes')![1];
    const [builtIn, installed] = await localizeBuiltInRecords([
      { isBuiltIn: true, manifest },
      { isBuiltIn: false, manifest },
    ]);
    expect((builtIn.manifest as any).commands[1].name).toBe(
      lookup(frCatalog, 'builtin.notes.commands.quick-note.name'),
    );
    expect(installed.manifest).toBe(manifest);
  });
});
