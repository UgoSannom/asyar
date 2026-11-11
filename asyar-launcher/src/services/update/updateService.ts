import { appUpdaterCheckNow } from '../../lib/ipc/updateCommands';
import { logService } from '../log/logService';
import { userErrorMessage } from '../../lib/userErrorMessage';

export type UpdateChannel = 'stable' | 'beta';

export type UpdateResult =
  | { kind: 'up-to-date' }
  | { kind: 'installed'; version: string }
  | { kind: 'error'; message: string }
  | { kind: 'busy' };

let inFlight = false;

export function resetUpdateCheckState(): void {
  inFlight = false;
}

export async function runUpdateCheck(): Promise<UpdateResult> {
  if (inFlight) {
    logService.debug('updateService: ignoring concurrent check request');
    return { kind: 'busy' };
  }

  inFlight = true;
  logService.info('updateService: checking for updates');

  try {
    const version = await appUpdaterCheckNow();
    if (!version) {
      logService.info('updateService: no update available');
      return { kind: 'up-to-date' };
    }
    logService.info(`updateService: update ${version} downloaded`);
    return { kind: 'installed', version };
  } catch (error) {
    const detail = error instanceof Error ? error.message : String(error);
    logService.error(`updateService: check failed — ${detail}`);
    return { kind: 'error', message: userErrorMessage(error) };
  } finally {
    inFlight = false;
  }
}
