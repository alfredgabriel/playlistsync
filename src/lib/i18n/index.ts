import { browser } from '$app/environment';
import { init, register, getLocaleFromNavigator } from 'svelte-i18n';

const STORAGE_KEY = 'playlistsync_locale';

register('en', () => import('./en.json'));
register('es', () => import('./es.json'));
register('fr', () => import('./fr.json'));
register('de', () => import('./de.json'));

export const SUPPORTED_LOCALES = [
  { code: 'en', label: 'English' },
  { code: 'es', label: 'Espanol' },
  { code: 'fr', label: 'Francais' },
  { code: 'de', label: 'Deutsch' },
];

function getInitialLocale(): string {
  if (browser) {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored && SUPPORTED_LOCALES.find(l => l.code === stored)) return stored;
  }
  const nav = getLocaleFromNavigator() ?? 'en';
  const lang = nav.split('-')[0];
  return SUPPORTED_LOCALES.find(l => l.code === lang) ? lang : 'en';
}

export function setLocale(locale: string) {
  if (browser) localStorage.setItem(STORAGE_KEY, locale);
}

init({
  fallbackLocale: 'en',
  initialLocale: getInitialLocale(),
});
