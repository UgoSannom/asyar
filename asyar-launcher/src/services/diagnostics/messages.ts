import { t } from '../i18n';
import { defineDiagnosticMessages } from './defineDiagnosticMessages';

/**
 * User-facing copy for every DiagnosticKind. The strings live in the locale
 * catalogs under `diagnostics_messages.*` and are resolved through `t()` each
 * time a template runs (never at module load, since the locale is set
 * asynchronously). When an optional context value is missing, a separate
 * `*_generic` key holds the full fallback sentence so translators never have
 * to stitch fragments together.
 */
const k = (key: string, params?: Record<string, string>) =>
  t(`diagnostics_messages.${key}`, params);

/** Picks `<key>` with the value when it is present (not null/undefined), else `<key>_generic`. */
const withOr = (key: string, param: string, value: string | undefined) =>
  value != null ? k(key, { [param]: value }) : k(`${key}_generic`);

/** Same as withOr but treats empty strings as missing (truthiness check). */
const withIf = (key: string, param: string, value: string | undefined) =>
  value ? k(key, { [param]: value }) : k(`${key}_generic`);

export const DIAGNOSTIC_MESSAGES = defineDiagnosticMessages({
  // Rust-derived (AppError)
  permission_denied: ({ permission }) => withOr('permission_denied', 'permission', permission),
  policy_denied: ({ policyError }) => withIf('policy_denied', 'policyError', policyError),
  network_failure: () => k('network_failure'),
  lock_poisoned: () => k('lock_poisoned'),
  database_failure: () => k('database_failure'),
  not_found: ({ target }) => withOr('not_found', 'target', target),
  extension_failure: ({ extension }) => withOr('extension_failure', 'extension', extension),
  shortcut_failure: ({ shortcut }) => withOr('shortcut_failure', 'shortcut', shortcut),
  platform_failure: ({ platform }) => withOr('platform_failure', 'platform', platform),
  validation_failure: ({ field }) => withOr('validation_failure', 'field', field),
  encryption_failure: () => k('encryption_failure'),
  auth_failure: ({ provider }) => withIf('auth_failure', 'provider', provider),
  oauth_failure: ({ provider }) => withIf('oauth_failure', 'provider', provider),
  power_failure: () => k('power_failure'),
  run_failed: ({ id, message }) =>
    message ? k('run_failed', { message }) : withOr('run_failed_id', 'id', id),
  sync_entitlement_required: () => k('sync_entitlement_required'),
  sync_quota_exceeded: ({ usedBytes, limitBytes }) =>
    Number(limitBytes) > 0
      ? k('sync_quota_exceeded', { usedBytes: String(usedBytes), limitBytes })
      : k('sync_quota_exceeded_generic'),
  sync_rate_limited: ({ retryAfter }) => k('sync_rate_limited', { retryAfter: retryAfter ?? '60' }),
  sync_server_unavailable: ({ detail }) => withIf('sync_server_unavailable', 'detail', detail),
  sync_e2ee_locked: () => k('sync_e2ee_locked'),
  sync_e2ee_not_enrolled: () => k('sync_e2ee_not_enrolled'),
  sync_e2ee_already_enrolled: () => k('sync_e2ee_already_enrolled'),
  sync_e2ee_wrong_mode: () => k('sync_e2ee_wrong_mode'),
  sync_finalize_behind: () => k('sync_finalize_behind'),
  sync_pairing_not_found: () => k('sync_pairing_not_found'),
  sync_pairing_not_pending: () => k('sync_pairing_not_pending'),
  sync_pairing_fingerprint_mismatch: () => k('sync_pairing_fingerprint_mismatch'),
  sync_pairing_too_many: () => k('sync_pairing_too_many'),
  io_failure: () => k('io_failure'),
  json_failure: () => k('json_failure'),
  unknown: ({ message }) => message ?? k('unexpected_error'),

  // Rust-derived (SearchError)
  search_lock_poisoned: () => k('search_lock_poisoned'),
  search_json_failure: () => k('search_json_failure'),
  search_io_failure: () => k('search_io_failure'),
  search_not_found: ({ target }) => withOr('search_not_found', 'target', target),
  search_other: ({ detail }) => withIf('search_other', 'detail', detail),

  // Frontend / extension
  uncaught_exception: ({ message }) => message ?? k('unexpected_error'),
  unhandled_rejection: ({ message }) => message ?? k('unexpected_error'),
  render_error: () => k('render_error'),
  invoke_unknown: ({ command }) => withIf('invoke_unknown', 'command', command),
  extension_proxy_error: ({ method }) => withIf('extension_proxy_error', 'method', method),
  extension_crash: ({ extensionId, role }) =>
    extensionId != null
      ? k('extension_crash', { extensionId, role: role ?? '?' })
      : k('extension_crash_generic', { role: role ?? '?' }),
  iframe_uncaught: ({ extensionId }) => withOr('iframe_uncaught', 'extensionId', extensionId),
  iframe_unhandled_rejection: ({ extensionId }) =>
    withOr('iframe_unhandled_rejection', 'extensionId', extensionId),
  rpc_timeout: ({ method }) => withOr('rpc_timeout', 'method', method),
  panic: () => k('panic'),
  manual: ({ message }) => message ?? k('error'),
  action_failed: ({ message }) => message ?? k('action_failed'),
  mcp_permission_required: ({ tool }) => withOr('mcp_permission_required', 'tool', tool),

  // E2EE cloud sync
  e2ee_enrollment_failed: () => k('e2ee_enrollment_failed'),
  e2ee_passphrase_required: () => k('e2ee_passphrase_required'),
  e2ee_key_derivation_failed: () => k('e2ee_key_derivation_failed'),
  e2ee_decrypt_failed: () => k('e2ee_decrypt_failed'),
  e2ee_key_version_mismatch: () => k('e2ee_key_version_mismatch'),
});
