import { t } from '../../../services/i18n';
import { snippetStore } from '../../../built-in-features/snippets/snippetStore.svelte';
import { snippetService } from '../../../built-in-features/snippets/snippetService';

const SAMPLE_KEYWORD = ';email';

export function seedSampleSnippet(): void {
  if (snippetStore.snippets.some((s) => s.keyword === SAMPLE_KEYWORD)) return;
  snippetStore.add({
    id: crypto.randomUUID(),
    keyword: SAMPLE_KEYWORD,
    expansion: 'you@example.com',
    name: t('ui_extra.onboarding.sample_snippet_name'),
    createdAt: Date.now(),
  });
}

export async function enableExpansion(): Promise<boolean> {
  await snippetService.syncToRust();
  const res = await snippetService.setEnabled(true);
  return res.ok === true;
}
