import type { RuntimeDownloadProgress, RuntimeDownload } from '../../lib/ipc/runtimeCommands';
import { t } from '../i18n';

// Local copy, not imported from actionService.svelte.ts: that module
// instantiates a heavy singleton on load, which drags in Tauri/window
// dependencies this pure formatter shouldn't need — RuntimeBatchConsentDialog
// keeps its own copy for the same reason.
function formatBytes(bytes: number): string {
  if (bytes <= 0) return t('services_extra.runtime.unknown_size');
  const mb = bytes / (1024 * 1024);
  return mb >= 1
    ? t('services_extra.runtime.size_mb', { value: mb.toFixed(1) })
    : t('services_extra.runtime.size_kb', { value: Math.max(1, Math.round(bytes / 1024)) });
}

/** Human-readable label for the live runtime-download status shown while a
 * consumer (the AI Extension Builder, an extension's retry action, …)
 * downloads one or more runtimes sequentially. `fallback` covers the
 * pre-progress-event window (request sent, no event received yet) so the
 * caller never has to show a blank string. */
export function formatRuntimeDownloadStatus(
  progress: RuntimeDownloadProgress | null,
  fallback: string,
): string {
  if (!progress) return fallback;
  switch (progress.status) {
    case 'resolving':
      return t('services_extra.runtime.status_checking');
    case 'downloading': {
      if (progress.totalBytes <= 0) return t('common.downloading');
      return t('services_extra.runtime.status_downloading_progress', {
        done: formatBytes(progress.bytesDownloaded),
        total: formatBytes(progress.totalBytes),
      });
    }
    case 'verifying':
      return t('services_extra.runtime.status_verifying');
    case 'extracting':
      return t('common.installing');
    case 'signing':
      return t('services_extra.runtime.status_finishing');
    case 'ready':
      return fallback;
    case 'failed':
      return t('services_extra.runtime.status_failed');
  }
}

/** Confirm-dialog copy for downloading a list of missing runtimes — shared
 * by any "download this before proceeding" confirm prompt so the wording
 * (and the "no silent downloads" behavior it enables) stays consistent. */
export function describeMissingRuntimesForConfirm(missing: RuntimeDownload[]): {
  title: string;
  message: string;
} {
  const totalBytes = missing.reduce((sum, r) => sum + r.sizeBytes, 0);
  const names = missing.map((r) => `${r.name} (${formatBytes(r.sizeBytes)})`).join(', ');
  return {
    title:
      missing.length > 1
        ? t('services_extra.runtime.confirm_title_other')
        : t('services_extra.runtime.confirm_title_one'),
    message: t('services_extra.runtime.confirm_message', {
      names,
      total: formatBytes(totalBytes),
    }),
  };
}
