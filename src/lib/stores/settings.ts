import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface AppSettings {
  format: 'm4a' | 'mp3';
  mp3Quality: '128' | '192' | 'vbr0';
  searchMode: 'fast' | 'deep';
  excludeInstrumentals: boolean;
  durationMin: number;
  durationMax: number;
  generateM3u: boolean;
  variants: string[];
}

const defaultSettings: AppSettings = {
  format: 'm4a',
  mp3Quality: 'vbr0',
  searchMode: 'deep',
  excludeInstrumentals: false,
  durationMin: 30,
  durationMax: 600,
  generateM3u: true,
  variants: [],
};

function createSettingsStore() {
  const { subscribe, set, update } = writable<AppSettings>(defaultSettings);
  let initialized = false;

  return {
    subscribe,
    init: async () => {
      if (initialized) return;
      try {
        const backendSettings = await invoke<AppSettings>('load_app_config');
        set(backendSettings);
      } catch (e) {
        console.error("Failed to load settings from backend", e);
      }
      initialized = true;
    },
    updateSetting: async <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => {
      let newState: AppSettings = get({ subscribe });
      update(s => {
        newState = { ...s, [key]: value };
        return newState;
      });
      try {
        await invoke('save_app_config', { config: newState });
      } catch (e) {
        console.error("Failed to save settings", e);
      }
    }
  };
}

export const settings = createSettingsStore();
