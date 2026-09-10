<script lang="ts">
  import { downloadStore } from '../lib/stores/download';
  import { _ } from 'svelte-i18n';
  import { open } from '@tauri-apps/plugin-shell';

  export let onNewDownload: () => void;

  $: session = $downloadStore;
  $: failedTracks = session?.tracks.filter(t => t.status === 'error') || [];

  function formatDuration(ms: number) {
    const s = Math.floor(ms / 1000);
    const m = Math.floor(s / 60);
    return `${m}M ${s % 60}S`;
  }

  async function handleOpenFolder() {
    if (session?.outputFolder) {
      await open(session.outputFolder);
    }
  }

  function handleNew() {
    downloadStore.clear();
    onNewDownload();
  }
</script>

{#if session}
  <div class="results-container animate-fade-in">
    <div class="results-content">
      <div class="status-indicator">
        [PROCESS {session.failedTracks === 0 ? 'COMPLETED' : 'FINISHED WITH WARNINGS'}]
      </div>
      <h2 class="title">{$_('results.title').toUpperCase()}</h2>
      
      <div class="stats-grid">
        <div class="stat-card">
          <span class="stat-lbl">// {$_('results.downloaded_label').toUpperCase()}</span>
          <span class="stat-val">{session.doneTracks} / {session.totalTracks}</span>
        </div>
        {#if session.failedTracks > 0}
          <div class="stat-card error">
            <span class="stat-lbl">// {$_('results.failed_label').toUpperCase()}</span>
            <span class="stat-val">{session.failedTracks}</span>
          </div>
        {/if}
        <div class="stat-card">
          <span class="stat-lbl">// {$_('results.time_taken').toUpperCase()}</span>
          <span class="stat-val">{formatDuration((session.completedAt || Date.now()) - session.startedAt)}</span>
        </div>
      </div>

      {#if failedTracks.length > 0}
        <div class="failed-section">
          <h3 class="failed-title">[FAILED TRACKS ({failedTracks.length})]</h3>
          <div class="failed-list">
            {#each failedTracks as track}
              <div class="failed-item">
                <span class="track-name">{track.artist} - {track.title}</span>
                <span class="track-err">{track.error || 'Unknown error'}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <div class="actions">
        <button class="btn btn-secondary btn-lg" on:click={handleOpenFolder}>
          {$_('results.open_folder_btn')} →
        </button>
        <button class="btn btn-primary btn-lg" on:click={handleNew}>
          {$_('results.new_download_btn')}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .results-container {
    padding: var(--space-8);
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    background: #000000;
    overflow-y: auto;
  }

  .results-content {
    max-width: 600px;
    width: 100%;
    border: 1px solid var(--border-muted);
    background: #050505;
    padding: var(--space-8);
    text-align: left;
  }

  .status-indicator {
    font-size: 11px;
    letter-spacing: 0.12em;
    color: #888888;
    font-family: var(--font-mono);
    font-weight: 700;
    margin-bottom: var(--space-2);
  }

  .title {
    font-size: 1.6rem;
    font-weight: 700;
    color: #ffffff;
    font-family: var(--font-mono);
    margin-bottom: var(--space-6);
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: var(--space-3);
    margin-bottom: var(--space-6);
  }

  .stat-card {
    background: #000000;
    border: 1px solid var(--border-muted);
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    font-family: var(--font-mono);
  }

  .stat-card.error {
    border-color: var(--status-error);
  }

  .stat-card.error .stat-val {
    color: var(--status-error);
  }

  .stat-lbl {
    font-size: 10px;
    color: #666666;
  }

  .stat-val {
    font-size: 1.4rem;
    font-weight: 700;
    color: #ffffff;
  }

  .failed-section {
    margin-bottom: var(--space-6);
    border: 1px solid var(--status-error);
    background: #110505;
    padding: var(--space-4);
  }

  .failed-title {
    font-size: 11px;
    color: var(--status-error);
    font-family: var(--font-mono);
    margin-bottom: var(--space-2);
  }

  .failed-list {
    max-height: 120px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 10px;
    font-family: var(--font-mono);
  }

  .failed-item {
    display: flex;
    justify-content: space-between;
    gap: var(--space-2);
  }

  .track-name { color: #ffffff; }
  .track-err { color: #888888; }

  .actions {
    display: flex;
    gap: var(--space-3);
    margin-top: var(--space-6);
  }
</style>
