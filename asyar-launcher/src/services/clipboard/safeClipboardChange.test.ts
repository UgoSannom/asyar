import { describe, it, expect, vi, beforeEach } from 'vitest';

let handler: (() => Promise<void>) | null = null;

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(async (_event: string, cb: () => Promise<void>) => {
    handler = cb;
    return vi.fn();
  }),
}));

vi.mock('tauri-plugin-clipboard-x-api', () => ({
  COMMAND: { CLIPBOARD_CHANGED: 'plugin:clipboard-x://clipboard_changed' },
  readClipboard: vi.fn(),
}));

import { readClipboard } from 'tauri-plugin-clipboard-x-api';
import { onClipboardChangeSafe } from './safeClipboardChange';

const locked = new Error('Get text error, code = OSError(5): Access is denied.');

describe('onClipboardChangeSafe', () => {
  beforeEach(() => {
    handler = null;
    vi.mocked(readClipboard).mockReset();
  });

  it('passes the read result to the callback', async () => {
    const cb = vi.fn();
    vi.mocked(readClipboard).mockResolvedValue({ text: { value: 'hi' } } as any);
    await onClipboardChangeSafe(cb, { delayMs: 0 });
    await handler!();
    expect(cb).toHaveBeenCalledWith({ text: { value: 'hi' } });
  });

  it('retries while the clipboard is locked by another app', async () => {
    const cb = vi.fn();
    vi.mocked(readClipboard)
      .mockRejectedValueOnce(locked)
      .mockResolvedValueOnce({ text: { value: 'ok' } } as any);
    await onClipboardChangeSafe(cb, { delayMs: 0 });
    await handler!();
    expect(readClipboard).toHaveBeenCalledTimes(2);
    expect(cb).toHaveBeenCalledWith({ text: { value: 'ok' } });
  });

  it('never rejects when every attempt fails, and reports once', async () => {
    const cb = vi.fn();
    const onError = vi.fn();
    vi.mocked(readClipboard).mockRejectedValue(locked);
    await onClipboardChangeSafe(cb, { retries: 2, delayMs: 0, onError });
    await expect(handler!()).resolves.toBeUndefined();
    expect(readClipboard).toHaveBeenCalledTimes(3);
    expect(cb).not.toHaveBeenCalled();
    expect(onError).toHaveBeenCalledOnce();
    expect(onError).toHaveBeenCalledWith(locked);
  });
});
