import { t } from '../../../services/i18n';
import { agentService } from '../../../built-in-features/agents/agentService.svelte';
import { shortcutService } from '../../../built-in-features/shortcuts/shortcutService';
import { userErrorMessage } from '../../../lib/userErrorMessage';

export async function setUpHiddenCommand(
  modifier: string,
  key: string,
): Promise<{ ok: boolean; error?: string }> {
  const def = agentService.getDefaultAgent();
  if (!def) return { ok: false, error: t('ui_extra.onboarding.no_ai_provider') };
  try {
    const agent = await agentService.seedGrammarFixAgent(def.providerId, def.modelId);
    const res = await shortcutService.register(
      `cmd_agents_dyn_${agent.id}`,
      agent.name,
      'command',
      `${modifier}+${key}`,
      undefined,
      'icon:sparkles',
    );
    if (!res.ok) {
      const shortcut = `${modifier}+${key}`;
      return {
        ok: false,
        error: res.conflict
          ? t('ui_extra.onboarding.bind_conflict', { shortcut, name: res.conflict.itemName })
          : t('ui_extra.onboarding.bind_failed', { shortcut }),
      };
    }
    return { ok: true };
  } catch (e) {
    return { ok: false, error: userErrorMessage(e) };
  }
}
