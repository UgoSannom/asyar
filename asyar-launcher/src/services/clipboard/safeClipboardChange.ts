import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { COMMAND, readClipboard, type ReadClipboard } from 'tauri-plugin-clipboard-x-api';

export interface SafeClipboardChangeOptions {
  /** Extra read attempts after the first one fails. */
  retries?: number;
  /** Base delay between attempts; attempt n waits n × delayMs. */
  delayMs?: number;
  /** Called once when every attempt failed. */
  onError?: (error: unknown) => void;
}

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/**
 * Like the plugin's `onClipboardChange`, but the read can fail without
 * surfacing an unhandled promise rejection.
 *
 * On Windows another process often holds the clipboard open for a few
 * milliseconds right after it writes to it (password managers, Office,
 * remote desktop, clipboard managers). Reading at that moment fails with
 * `OSError(5)` ("Access denied"). The plugin's helper awaits the read inside
 * its event listener, so that failure became an unhandled rejection and the
 * launcher showed it as a red error. Here the read is retried a few times
 * and a final failure is handed to `onError` instead.
 */
export function onClipboardChangeSafe(
  cb: (result: ReadClipboard) => void | Promise<void>,
  { retries = 3, delayMs = 50, onError }: SafeClipboardChangeOptions = {},
): Promise<UnlistenFn> {
  return listen(COMMAND.CLIPBOARD_CHANGED, async () => {
    let result: ReadClipboard | undefined;
    for (let attempt = 0; ; attempt++) {
      try {
        result = await readClipboard();
        break;
      } catch (error) {
        if (attempt >= retries) {
          onError?.(error);
          return;
        }
        await sleep(delayMs * (attempt + 1));
      }
    }
    await cb(result);
  });
}
