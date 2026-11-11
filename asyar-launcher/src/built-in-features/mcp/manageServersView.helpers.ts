import { t } from '../../services/i18n';

export function formatRelativeTime(timestampMillis: number, now: number = Date.now()): string {
  const diffMs = now - timestampMillis;
  const diffSec = Math.floor(diffMs / 1000);
  if (diffSec < 60) return t('features_extra.time.just_now');
  const diffMin = Math.floor(diffMs / 60_000);
  if (diffMin < 60) return t('features_extra.time.m_ago_short', { count: diffMin });
  const diffHr = Math.floor(diffMs / 3_600_000);
  if (diffHr < 24) return t('features_extra.time.h_ago_short', { count: diffHr });
  const diffDay = Math.floor(diffMs / 86_400_000);
  return t('features_extra.time.d_ago_short', { count: diffDay });
}

export function statusBadgeColor(status: 'starting' | 'connected' | 'failed' | 'disabled'): string {
  switch (status) {
    case 'connected':
      return 'var(--accent-success)';
    case 'starting':
      return 'var(--accent-warning)';
    case 'failed':
      return 'var(--accent-danger)';
    case 'disabled':
      return 'var(--text-tertiary)';
  }
}

export function transportLabel(kind: string): string {
  if (kind === 'stdio') return 'Stdio';
  if (kind === 'http') return 'HTTP';
  return kind;
}

export function truncateArgs(s: string, max: number = 80): string {
  if (s.length <= max) return s;
  return s.slice(0, max) + '…';
}
