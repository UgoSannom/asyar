import { describe, it, expect, vi } from 'vitest';

const host = vi.hoisted(() => ({ platform: 'macos' as 'macos' | 'windows' | 'other' }));
vi.mock('./hostPlatform', () => ({ getHostPlatform: () => host.platform }));

import { LAUNCHER_SHORTCUTS } from './shortcutCatalog';

const keysOf = () => LAUNCHER_SHORTCUTS.map((s) => s.keys);

describe('LAUNCHER_SHORTCUTS keycaps per platform', () => {
  it('draws ⌘ on macOS', () => {
    host.platform = 'macos';
    expect(keysOf()).toEqual(
      expect.arrayContaining([
        ['⌘', ','],
        ['⌘', 'K'],
        ['⌘', 'P'],
      ]),
    );
  });

  it('names Ctrl on Windows and Linux, with no macOS glyph left', () => {
    for (const platform of ['windows', 'other'] as const) {
      host.platform = platform;
      expect(keysOf()).toEqual(
        expect.arrayContaining([
          ['Ctrl', ','],
          ['Ctrl', 'K'],
          ['Ctrl', 'P'],
        ]),
      );
      expect(keysOf().flat().join('')).not.toMatch(/[⌘⌥⇧⌃]/);
    }
  });
});
