import { describe, it, expect, vi } from 'vitest';
import type { Feedback } from 'asyar-sdk/contracts';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

import { IpcError } from './ipc/invokeSafe';
import { userErrorMessage } from './userErrorMessage';

function rust(kind: string, context: Record<string, string>, developerDetail: string): Feedback {
  return { source: 'rust', kind, severity: 'error', retryable: false, context, developerDetail };
}

describe('userErrorMessage', () => {
  it('returns plain errors and strings unchanged', () => {
    expect(userErrorMessage(new Error('boom'))).toBe('boom');
    expect(userErrorMessage('plain')).toBe('plain');
  });

  it('drops the command and the English AppError prefix from a free-text Rust error', () => {
    const diagnostic = rust(
      'validation_failure',
      { field: "API key for provider 'openai' is not configured" },
      "Validation error: API key for provider 'openai' is not configured",
    );
    const err = new IpcError('agents_run_thread', diagnostic, diagnostic);
    expect(err.message).toBe(
      "agents_run_thread: Validation error: API key for provider 'openai' is not configured",
    );
    expect(userErrorMessage(err)).toBe("API key for provider 'openai' is not configured");
  });

  it('shows the payload of AppError::Other and of plain String rejections', () => {
    const other = rust(
      'unknown',
      {},
      'Codex login not found. Please log in using the Codex CLI first.',
    );
    expect(userErrorMessage(new IpcError('ai_stream_chat', other, other))).toBe(
      'Codex login not found. Please log in using the Codex CLI first.',
    );
    const plain: Feedback = {
      source: 'frontend',
      kind: 'invoke_unknown',
      severity: 'error',
      retryable: false,
      context: { command: 'scripts_run' },
      developerDetail: 'script exited',
    };
    expect(userErrorMessage(new IpcError('scripts_run', plain, 'script exited'))).toBe(
      'script exited',
    );
  });

  it('uses the localised per-kind message for kinds without a user sentence', () => {
    const network = rust('network_failure', {}, 'Network error: connection refused');
    expect(userErrorMessage(new IpcError('store_install', network, network))).toBe('Network error');
  });

  it('accepts a bare diagnostic object', () => {
    expect(userErrorMessage(rust('not_found', { target: 'thread x' }, 'Not found: thread x'))).toBe(
      'thread x',
    );
  });
});
