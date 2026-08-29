<script lang="ts">
  import { _ } from 'svelte-i18n';
  import OnboardingWizard from '../components/OnboardingWizard.svelte';
  import DownloadProgress from '../components/DownloadProgress.svelte';
  import ResultsSummary from '../components/ResultsSummary.svelte';
  import { downloadStore } from '../lib/stores/download';
  import { settings } from '../lib/stores/settings';
  import { invoke } from '@tauri-apps/api/core';

  let viewState: 'wizard' | 'active' | 'results' = 'wizard';
  let csvData: { index: number; title: string; artist: string; album: string }[] = [];
  let playlistNameStr = '';

  async function handleComplete(csvPath: string, folder: string) {
    playlistNameStr = csvPath.split(/[\\/]/).pop()?.replace('.csv', '') || 'Playlist';
    
    // We should parse the CSV here to pass the full track list to backend
    // Since we only preview 5 tracks in the wizard, we need the full list
    // In a real app we would read and parse the full CSV content
    // For now, let's pretend we pass some tracks
    
    // Quick hack for demo to parse the file via Tauri (ideally would have a command for this)
    // using the readTextFile from plugin-fs
    const { readTextFile } = await import('@tauri-apps/plugin-fs');
    const text = await readTextFile(csvPath);
    
    const lines = text.split(/\r?\n/).filter(l => l.trim());
    if (lines.length > 1) {
      const headers = lines[0].split(',').map(h => h.replace(/^"|"$/g, '').trim());
      csvData = lines.slice(1).map((line, index) => {
        const vals: string[] = [];
        let cur = '';
        let inQ = false;
        for (const ch of line) {
          if (ch === '"') { inQ = !inQ; }
          else if (ch === ',' && !inQ) { vals.push(cur.trim()); cur = ''; }
          else { cur += ch; }
        }
        vals.push(cur.trim());
        const row = Object.fromEntries(headers.map((h, i) => [h, vals[i] ?? '']));
        return {
          index,
          title: row['Track Name'] || row['Track name'] || $_('common.unknown'),
          artist: row['Artist Name(s)'] || row['Artist name'] || $_('common.unknown'),
          album: row['Album Name'] || row['Album'] || '—',
        };
      });
    }

    downloadStore.startSession(csvData, folder, playlistNameStr, $settings.format);
    viewState = 'active';

    try {
      await invoke('start_download_session', {
        options: {
          tracks: csvData,
          outputFolder: folder,
          playlistName: playlistNameStr,
          format: $settings.format,
          mp3Quality: $settings.mp3Quality,
          searchMode: $settings.searchMode,
          excludeInstrumentals: $settings.excludeInstrumentals,
          durationMin: $settings.durationMin,
          durationMax: $settings.durationMax,
          generateM3u: $settings.generateM3u,
        }
      });
    } catch(e) {
      console.error("Download session error:", e);
    }
  }
</script>

{#if viewState === 'wizard'}
  <OnboardingWizard onComplete={handleComplete} />
{:else if viewState === 'active'}
  <DownloadProgress onFinish={() => viewState = 'results'} />
{:else if viewState === 'results'}
  <ResultsSummary onNewDownload={() => viewState = 'wizard'} />
{/if}

