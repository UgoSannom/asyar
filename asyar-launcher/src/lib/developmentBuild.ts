import { t } from '../services/i18n';

export interface DevelopmentBuildIndicator {
  text: string;
  title: string;
}

export function getDevelopmentBuildIndicator(
  isDevelopment: boolean,
): DevelopmentBuildIndicator | null {
  // `title` is a getter: this runs at module load, before the locale is set.
  return isDevelopment
    ? {
        text: 'DEV',
        get title() {
          return t('services_extra.launcher.dev_build');
        },
      }
    : null;
}

export const developmentBuildIndicator = getDevelopmentBuildIndicator(import.meta.env.DEV);
