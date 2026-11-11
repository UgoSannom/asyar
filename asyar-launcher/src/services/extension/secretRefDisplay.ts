import { t } from '../i18n';

const k = (key: string, params?: Record<string, string>) =>
  t(`services_extra.secret_refs.${key}`, params);

function record(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function strings(value: unknown): string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === 'string' && item.length > 0)
    : [];
}

/** Display manifest declarations only; never resolves or reads a secret. */
export function describeSecretRefs(args: unknown): string[] {
  try {
    if (!Array.isArray(args)) return [];
    return args.flatMap((ref): string[] => {
      if (!record(ref) || !Array.isArray(ref.sources) || typeof ref.header !== 'string') return [];
      const hosts = strings(ref.hosts);
      // Each source has a `lead` form (first in the sentence, "in the Keychain
      // item …") and an `alt` form (inside "(or …)").
      const sources = ref.sources.flatMap((source): { lead: string; alt: string }[] => {
        if (!record(source) || typeof source.pointer !== 'string') return [];
        if (source.type === 'jsonFile' && typeof source.path === 'string') {
          const text = k('json', { pointer: source.pointer, path: source.path });
          return [{ lead: text, alt: text }];
        }
        if (source.type === 'keychain' && typeof source.service === 'string') {
          const params: Record<string, string> = {
            service: source.service,
            pointer: source.pointer,
          };
          const hasAccount = typeof source.account === 'string';
          if (hasAccount) params.account = source.account as string;
          const key = hasAccount ? 'keychain_account' : 'keychain';
          return [{ lead: k(`${key}_lead`, params), alt: k(key, params) }];
        }
        return [];
      });
      if (!sources.length || !hosts.length) return [];
      const methods = ref.methods === undefined ? ['GET'] : strings(ref.methods);
      const paths = strings(ref.pathPrefixes);
      const sourceText = [
        sources[0].lead,
        ...sources.slice(1).map((source) => k('or_source', { source: source.alt })),
      ].join(' ');
      const keychain = ref.sources.some((source) => record(source) && source.type === 'keychain');
      const sentence = k('sentence', {
        source: sourceText,
        hosts: hosts.join(', '),
        methods: methods.join(', '),
        paths: paths.length ? paths.join(', ') : k('all_paths'),
        header: ref.header,
      });
      return [keychain ? `${sentence} ${k('keychain_note')}` : sentence];
    });
  } catch {
    return [];
  }
}
