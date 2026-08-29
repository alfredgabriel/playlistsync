import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface HistorySession {
  id: string;
  playlistName: string;
  outputFolder: string;
  format: string;
  totalTracks: number;
  doneTracks: number;
  failedTracks: number;
  startedAt: number;
  completedAt: number;
  elapsedSecs: number;
}

function createHistoryStore() {
  const { subscribe, set } = writable<HistorySession[]>([]);

  return {
    subscribe,
    load: async () => {
      try {
        const sessions = await invoke<HistorySession[]>('load_history');
        set(sessions);
      } catch (e) {
        console.error("Failed to load history", e);
      }
    },
    clear: async () => {
      try {
        await invoke('clear_history');
        set([]);
      } catch (e) {
        console.error("Failed to clear history", e);
      }
    }
  };
}

export const historyStore = createHistoryStore();
