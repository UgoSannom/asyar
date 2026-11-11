import type { TaskProgress } from '../../lib/ipc/walkthroughCommands';
import { t } from '../../services/i18n';

/**
 * Null when a label would say nothing the tick mark doesn't: a manual task,
 * or a single-step one where "0 of 1" just means "not done".
 */
export function taskProgressLabel(progress: TaskProgress | null | undefined): string | null {
  if (!progress || progress.target <= 1) return null;

  const { current, target, unit } = progress;
  switch (unit) {
    case 'days':
      return t('features_extra.walkthrough.progress_days', { current, target });
    case 'launches':
      return t('features_extra.walkthrough.progress_times', { current, target });
    default:
      return t('features_extra.walkthrough.progress_of', { current, target });
  }
}

/** Fill fraction for MeterBar, 0..1. */
export function taskProgressFraction(progress: TaskProgress | null | undefined): number {
  if (!progress || progress.target <= 0) return 0;
  return Math.max(0, Math.min(1, progress.current / progress.target));
}
