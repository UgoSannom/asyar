import { t } from '../../services/i18n';
import type { McpServerInstallInput } from './types';

export interface EnvRow {
  key: string;
  value: string;
}

export interface InstallFormState {
  id: string;
  displayName: string;
  description: string;
  transportKind: 'stdio' | 'http';
  command: string;
  args: string[];
  env: EnvRow[];
  cwd: string;
  url: string;
  headers: EnvRow[];
}

export type ValidationResult = { ok: true } | { ok: false; error: string };

export function validateInstallForm(state: InstallFormState): ValidationResult {
  if (!state.id.trim()) {
    return { ok: false, error: t('features_extra.mcp.err_id_required') };
  }
  if (!state.displayName.trim()) {
    return { ok: false, error: t('features_extra.mcp.err_display_name_required') };
  }
  if (state.transportKind === 'stdio') {
    if (!state.command.trim()) {
      return { ok: false, error: t('features_extra.mcp.err_command_required') };
    }
  } else {
    if (!/^https?:\/\/.+/.test(state.url)) {
      return { ok: false, error: t('features_extra.mcp.err_url_required') };
    }
  }
  return { ok: true };
}

export function buildInstallInput(state: InstallFormState): McpServerInstallInput {
  const description = state.description.trim() || null;

  if (state.transportKind === 'stdio') {
    const env: Record<string, string> = {};
    for (const row of state.env) {
      if (row.key.trim()) {
        env[row.key] = row.value;
      }
    }
    const args = state.args.filter((a) => a.trim() !== '');
    const cwd = state.cwd.trim() || null;
    return {
      id: state.id.trim(),
      displayName: state.displayName.trim(),
      description,
      transport: { kind: 'stdio', command: state.command.trim(), args, env, cwd },
    };
  } else {
    const headers: Record<string, string> = {};
    for (const row of state.headers) {
      if (row.key.trim()) {
        headers[row.key] = row.value;
      }
    }
    return {
      id: state.id.trim(),
      displayName: state.displayName.trim(),
      description,
      transport: { kind: 'http', url: state.url.trim(), headers },
    };
  }
}
