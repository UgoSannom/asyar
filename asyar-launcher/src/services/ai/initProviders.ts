import { providerRegistry } from './providerRegistry';
import { t } from '../i18n';
import { aiListModels } from '../../lib/ipc/commands';
import type { IProviderPlugin, ProviderId } from './IProviderPlugin';

const ALL_REASONING_EFFORTS = ['none', 'minimal', 'low', 'medium', 'high', 'xhigh', 'max'] as const;

function descriptor(
  metadata: Omit<IProviderPlugin, 'getModels'> & { id: ProviderId },
  /** Catalog key for a translatable name, read lazily (the locale is set after module load). */
  nameKey?: string,
): IProviderPlugin {
  const plugin: IProviderPlugin = {
    ...metadata,
    getModels: (config, instanceId) => aiListModels(instanceId ?? metadata.id, config),
  };
  if (nameKey) {
    Object.defineProperty(plugin, 'name', { get: () => t(nameKey), enumerable: true });
  }
  return plugin;
}

const PROVIDERS: IProviderPlugin[] = [
  descriptor({
    id: 'openai',
    name: 'OpenAI',
    requiresApiKey: true,
    requiresBaseUrl: false,
    supportsOpenAIApiMode: true,
    supportsHostedWebSearch: true,
    supportsCliMode: true,
    cliName: 'OpenAI Codex CLI (codex)',
  }),
  descriptor({
    id: 'anthropic',
    name: 'Anthropic',
    requiresApiKey: true,
    requiresBaseUrl: false,
    supportsCliMode: true,
    cliName: 'Claude Code CLI (claude)',
  }),
  descriptor({
    id: 'google',
    name: 'Google Gemini',
    supportsHostedWebSearch: true,
    requiresApiKey: true,
    requiresBaseUrl: false,
    supportsCliMode: true,
    cliName: 'Google Antigravity CLI (agy)',
    reasoningEfforts: ['minimal', 'low', 'medium', 'high'],
  }),
  descriptor(
    {
      id: 'ollama',
      name: 'Ollama (local)',
      requiresApiKey: false,
      requiresBaseUrl: true,
      reasoningEfforts: ['low', 'medium', 'high'],
    },
    'services_extra.ai_providers.ollama',
  ),
  descriptor({
    id: 'openrouter',
    name: 'OpenRouter',
    requiresApiKey: true,
    requiresBaseUrl: false,
    reasoningEfforts: ALL_REASONING_EFFORTS,
  }),
  descriptor(
    {
      id: 'custom',
      name: 'Custom (OpenAI-compatible)',
      requiresApiKey: false,
      optionalApiKey: true,
      requiresBaseUrl: true,
      supportsOpenAIApiMode: true,
      supportsHostedWebSearch: true,
      reasoningEfforts: ALL_REASONING_EFFORTS,
    },
    'services_extra.ai_providers.custom',
  ),
];

export function initProviders(): void {
  for (const provider of PROVIDERS) providerRegistry.register(provider);
}
