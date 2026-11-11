import { t } from '../i18n';

/**
 * `null` means safe to remove silently; otherwise a human-readable warning
 * naming every consumer id (e.g. `mcp:<id>`, `ext:<id>`, `builtin:ext-builder`)
 * for `feedbackService.confirmAlert`'s message.
 */
export function describeRuntimeRemovalWarning(consumers: string[]): string | null {
  if (consumers.length === 0) return null;

  const list = consumers.join(', ');
  if (consumers.length === 1) {
    return t('services_extra.runtime.removal_warning_one', { list });
  }
  return t('services_extra.runtime.removal_warning_other', { count: consumers.length, list });
}
