/**
 * Human-readable display metadata for manifest permission strings, used by the
 * consent dialog and the read-only permission lists in settings and the store.
 *
 * Descriptions are derived from `docs/reference/permissions.md`. Permission
 * strings not in the catalog (a newer launcher's permission, or a typo in a
 * manifest) render with the raw string and `known: false` so the UI can show a
 * caution note instead of silently pretending to understand them.
 *
 * The copy itself lives in the locale catalogs (`src/locales/*.json`) under
 * `permissions_catalog.entries.<key>.{title,description}`. Because permission
 * ids contain `:` and `.` (and `t()` splits keys on `.`), the catalog key is
 * derived with {@link permissionCatalogKey}: every `:`, `.` and `-` becomes `_`
 * (`browser:tabs.read` -> `browser_tabs_read`, `clipboard-history:read` ->
 * `clipboard_history_read`). Titles and
 * descriptions are resolved through `t()` on every property read rather than
 * at module load, because the locale is set asynchronously at startup.
 *
 * To add a permission: add its id to PERMISSION_IDS and its title/description
 * to every locale catalog under the derived key.
 */
import { t } from '../i18n';

export interface PermissionInfo {
  title: string;
  description: string;
}

const PERMISSION_IDS = [
  'network',
  'network:lan',
  'network:secrets',
  'notifications:send',
  'clipboard:read',
  'clipboard:write',
  'clipboard-history:read',
  'clipboard-history:capture',
  'clipboard-history:manage',
  'storage:read',
  'storage:write',
  'store:read',
  'store:write',
  'cache:read',
  'cache:write',
  'fs:read',
  'fs:write',
  'fs:watch',
  'files:search',
  'files:read',
  'files:search-provider',
  'notes:read',
  'notes:write',
  'shell:spawn',
  'shell:open-url',
  'shell:open-path',
  'entitlements:read',
  'selection:read',
  'oauth:use',
  'extension:invoke',
  'application:read',
  'window:manage',
  'screen:pick-color',
  'screen:capture',
  'calculator:evaluate',
  'power:inhibit',
  'media:control',
  'calendar:read',
  'calendar:write',
  'reminders:read',
  'reminders:write',
  'shortcuts:read',
  'shortcuts:run',
  'process:read',
  'process:kill',
  'systemEvents:read',
  'app:frontmost-watch',
  'timers:schedule',
  'timers:cancel',
  'timers:list',
  'preferences:read',
  'preferences:write',
  'feedback:announce',
  'tools:register',
  'snippets:contribute',
  'runs:track',
  'browser:tabs.read',
  'browser:tabs.write',
  'browser:bookmarks.read',
  'browser:history.read',
  'browser:page.read',
  'browser:page.write',
  'mcp',
  'ai',
] as const;

/** Locale-catalog key segment for a permission id (`:` `.` `-` -> `_`). */
export function permissionCatalogKey(permission: string): string {
  return permission.replace(/[:.-]/g, '_');
}

function localizedEntry(permission: string): PermissionInfo {
  const base = `permissions_catalog.entries.${permissionCatalogKey(permission)}`;
  return {
    get title() {
      return t(`${base}.title`);
    },
    get description() {
      return t(`${base}.description`);
    },
  };
}

export const PERMISSION_CATALOG: Record<string, PermissionInfo> = Object.fromEntries(
  PERMISSION_IDS.map((permission) => [permission, localizedEntry(permission)]),
);

export interface PermissionDisplay extends PermissionInfo {
  /** False when the permission string is not in the catalog. */
  known: boolean;
}

export function describePermission(permission: string): PermissionDisplay {
  const info = Object.hasOwn(PERMISSION_CATALOG, permission)
    ? PERMISSION_CATALOG[permission]
    : undefined;
  if (info) {
    return { title: info.title, description: info.description, known: true };
  }
  return {
    title: permission,
    description: t('permissions_catalog.not_recognized'),
    known: false,
  };
}
