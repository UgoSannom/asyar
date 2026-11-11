import type { Feedback } from 'asyar-sdk/contracts';
import { IpcError, isFeedbackShape } from './ipc/invokeSafe';
import { extractErrorMessage } from './errors';
import { DIAGNOSTIC_MESSAGES } from '../services/diagnostics/messages';
import type { DiagnosticKind } from '../services/diagnostics/kinds';

/**
 * Rust `AppError` kinds whose payload is a complete sentence written for the
 * user (and localised in Rust, see `locale/native.rs`), keyed to the context
 * field that carries it.
 */
const FREE_TEXT_CONTEXT: Partial<Record<string, string>> = {
  validation_failure: 'field',
  not_found: 'target',
  extension_failure: 'extension',
  platform_failure: 'platform',
  shortcut_failure: 'shortcut',
};

/**
 * The text to show the user for a caught error, in the UI language.
 *
 * `IpcError.message` is `"<command>: <English AppError Display>"`, which is
 * meant for logs. For a Rust diagnostic this returns the message Rust wrote
 * for the user (already localised) or the localised per-kind message; the
 * raw text stays available as `developerDetail` ("Technical detail") and in
 * the logs. Plain `Error`s and strings are returned as they are.
 */
export function userErrorMessage(err: unknown): string {
  const diagnostic: Feedback | null =
    err instanceof IpcError ? err.diagnostic : isFeedbackShape(err) ? err : null;
  if (!diagnostic) return extractErrorMessage(err);

  const context = (diagnostic.context ?? {}) as Record<string, string>;
  // `AppError::Other` displays its payload as-is, and a non-AppError
  // rejection (a plain `String` error) is carried verbatim in developerDetail.
  if (diagnostic.kind === 'unknown' || diagnostic.kind === 'invoke_unknown') {
    if (diagnostic.developerDetail) return diagnostic.developerDetail;
  }
  if (diagnostic.source === 'rust') {
    const field = FREE_TEXT_CONTEXT[diagnostic.kind];
    const text: string | undefined = field ? context[field] : undefined;
    if (text) return text;
  }
  const template = DIAGNOSTIC_MESSAGES[diagnostic.kind as DiagnosticKind];
  if (template) return template(context);
  return diagnostic.developerDetail ?? extractErrorMessage(err);
}
