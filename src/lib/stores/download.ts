import { writable } from 'svelte/store';

export type TrackStatus = 'pending' | 'searching' | 'downloading' | 'done' | 'error';

export interface TrackState {
  index: number;
  title: string;
  artist: string;
  album: string;
  status: TrackStatus;
  filename?: string;
  error?: string;
}

export interface DownloadSession {
  tracks: TrackState[];
  totalTracks: number;
  doneTracks: number;
  failedTracks: number;
  startedAt: number;
  completedAt?: number;
  outputFolder: string;
  playlistName: string;
  format: 'm4a' | 'mp3';
}

function createDownloadStore() {
  const { subscribe, set, update } = writable<DownloadSession | null>(null);

  return {
    subscribe,
    startSession: (tracks: Omit<TrackState, 'status'>[], outputFolder: string, playlistName: string, format: 'm4a' | 'mp3') => {
      set({
        tracks: tracks.map(t => ({ ...t, status: 'pending' })),
        totalTracks: tracks.length,
        doneTracks: 0,
        failedTracks: 0,
        startedAt: Date.now(),
        outputFolder,
        playlistName,
        format,
      });
    },
    updateTrack: (index: number, updates: Partial<TrackState>) => {
      update(session => {
        if (!session) return session;
        const tracks = [...session.tracks];
        tracks[index] = { ...tracks[index], ...updates };
        const doneTracks = tracks.filter(t => t.status === 'done').length;
        const failedTracks = tracks.filter(t => t.status === 'error').length;
        return { ...session, tracks, doneTracks, failedTracks };
      });
    },
    completeSession: () => {
      update(s => s ? { ...s, completedAt: Date.now() } : s);
    },
    clear: () => set(null),
  };
}

export const downloadStore = createDownloadStore();
