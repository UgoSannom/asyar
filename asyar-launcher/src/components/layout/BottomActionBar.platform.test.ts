// @vitest-environment jsdom
import { fireEvent, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const host = vi.hoisted(() => ({ platform: 'macos' as 'macos' | 'windows' | 'other' }));
vi.mock('../../lib/keyboard/hostPlatform', () => ({ getHostPlatform: () => host.platform }));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(async () => () => {}),
}));

const mockAppRelaunch = vi.fn(async () => {});

vi.mock('../../lib/ipc/commands', () => ({
  appRelaunch: () => mockAppRelaunch(),
}));

vi.mock('../../services/i18n', () => ({
  t: (key: string, params?: Record<string, string | number>) => {
    if (key === 'settings.about.restart_now') return 'Restart Now';
    if (key === 'ui_extra.about.update_ready') return `Update ${params?.version} ready`;
    if (key === 'ui_extra.bottom_bar.update_ready_restart')
      return `Update ${params?.version} ready. Click to restart.`;
    if (key === 'actions.title') return 'Actions';
    if (key === 'common.show_more') return 'Show More';
    return key;
  },
}));

vi.mock('../../services/action/actionService.svelte', () => ({
  actionService: {
    filteredActions: [],
  },
}));

vi.mock('../../services/extension/viewManager.svelte', () => ({
  viewManager: {
    activeView: null,
    activeViewPrimaryActionLabel: null,
  },
}));

vi.mock('../../services/extension/extensionManager.svelte', () => ({
  default: {
    getManifestById: () => null,
  },
}));

vi.mock('../../services/feedback/feedbackService.svelte', () => ({
  feedbackService: {
    current: null,
  },
}));

import { appUpdateState } from '../../services/update/appUpdateStore.svelte';
import { viewManager } from '../../services/extension/viewManager.svelte';
import BottomActionBar from './BottomActionBar.svelte';

describe('BottomActionBar - Actions key hint', () => {
  const renderBar = () =>
    render(BottomActionBar, {
      props: {
        isActionListOpen: false,
        onactionListToggled: () => {},
        onactionListClosed: () => {},
      },
    });
  const actionsKeycaps = () =>
    Array.from(screen.getByText('Actions').closest('button')!.querySelectorAll('kbd'), (k) =>
      k.textContent?.trim(),
    );

  it('shows ⌘ K on macOS', () => {
    host.platform = 'macos';
    renderBar();
    expect(actionsKeycaps()).toEqual(['⌘', 'K']);
  });

  it('shows Ctrl K on Windows and Linux, where the panel is bound to Ctrl+K', () => {
    host.platform = 'windows';
    renderBar();
    expect(actionsKeycaps()).toEqual(['Ctrl', 'K']);
  });
});
