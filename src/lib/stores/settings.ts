import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export type AudioFormat = 'm4a' | 'mp3';
export type SearchMode = 'fast' | 'deep';
export type Mp3Quality = 'vbr0' | '192' | '128';

export interface AppSettings {
  format: AudioFormat;
  searchMode: SearchMode;
  mp3Quality: Mp3Quality;
  excludeInstrumentals: boolean;
  durationMin: number;
  durationMax: number;
  generateM3u: boolean;
}

const DEFAULT_SETTINGS: AppSettings = {
  format: 'm4a',
  searchMode: 'deep',
  mp3Quality: 'vbr0',
  excludeInstrumentals: false,
  durationMin: 30,
  durationMax: 600,
  generateM3u: true,
};

function loadSettings(): AppSettings {
  if (!browser) return DEFAULT_SETTINGS;
  try {
    const raw = localStorage.getItem('playlistsync_settings');
    if (raw) return { ...DEFAULT_SETTINGS, ...JSON.parse(raw) };
  } catch {}
  return DEFAULT_SETTINGS;
}

function createSettingsStore() {
  const { subscribe, set, update } = writable<AppSettings>(loadSettings());

  return {
    subscribe,
    set: (value: AppSettings) => {
      if (browser) localStorage.setItem('playlistsync_settings', JSON.stringify(value));
      set(value);
    },
    update,
    reset: () => {
      if (browser) localStorage.removeItem('playlistsync_settings');
      set(DEFAULT_SETTINGS);
    },
  };
}

export const settings = createSettingsStore();
