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
    return `${m}m ${s % 60}s`;
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
    <div class="confetti-bg"></div>
    <div class="results-content">
      <div class="icon-big">{session.failedTracks === 0 ? '🎉' : '✅'}</div>
      <h2 class="title">{$_('results.title')}</h2>
      
      <div class="stats-grid">
        <div class="stat-card success">
          <span class="stat-val">{session.doneTracks}</span>
          <span class="stat-lbl">{$_('results.downloaded_label')}</span>
        </div>
        {#if session.failedTracks > 0}
          <div class="stat-card error">
            <span class="stat-val">{session.failedTracks}</span>
            <span class="stat-lbl">{$_('results.failed_label')}</span>
          </div>
        {/if}
        <div class="stat-card neutral">
          <span class="stat-val">{formatDuration((session.completedAt || Date.now()) - session.startedAt)}</span>
          <span class="stat-lbl">{$_('results.time_taken')}</span>
        </div>
      </div>

      {#if failedTracks.length > 0}
        <div class="failed-list card-elevated">
          <h3 class="failed-title">{$_('results.failed_list_title')}</h3>
          <div class="failed-items">
            {#each failedTracks as track}
              <div class="failed-item">
                <span class="failed-track">{track.title} - {track.artist}</span>
                <span class="failed-reason">{track.error || $_('errors.unknown')}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <div class="actions">
        <button class="btn btn-secondary btn-lg" on:click={handleOpenFolder}>
          📁 {$_('results.open_folder')}
        </button>
        <button class="btn btn-primary btn-lg" on:click={handleNew}>
          🔄 {$_('results.new_download')}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .results-container {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-8);
    position: relative;
    overflow-y: auto;
  }

  .results-content {
    max-width: 600px;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-6);
    z-index: 1;
  }

  .icon-big {
    font-size: 5rem;
    animation: slideUp var(--transition-normal) ease both;
  }

  .title {
    font-size: var(--text-3xl);
    font-weight: var(--font-bold);
    color: var(--text-primary);
  }

  .stats-grid {
    display: flex;
    gap: var(--space-4);
    width: 100%;
  }

  .stat-card {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: var(--space-4);
    border-radius: var(--radius-lg);
    background: var(--bg-glass);
    border: 1px solid var(--border-subtle);
  }

  .stat-card.success { border-color: rgba(30, 215, 96, 0.3); background: rgba(30, 215, 96, 0.05); }
  .stat-card.error { border-color: rgba(248, 113, 113, 0.3); background: rgba(248, 113, 113, 0.05); }

  .stat-val { font-size: var(--text-2xl); font-weight: var(--font-bold); }
  .success .stat-val { color: var(--status-done); }
  .error .stat-val { color: var(--status-error); }
  .neutral .stat-val { color: var(--text-primary); }

  .stat-lbl { font-size: var(--text-xs); color: var(--text-muted); text-align: center; }

  .failed-list {
    width: 100%;
    padding: var(--space-4);
  }

  .failed-title {
    font-size: var(--text-sm);
    font-weight: var(--font-bold);
    color: var(--status-error);
    margin-bottom: var(--space-2);
  }

  .failed-items {
    max-height: 200px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .failed-item {
    display: flex;
    flex-direction: column;
    font-size: var(--text-xs);
    background: var(--bg-base);
    padding: var(--space-2);
    border-radius: var(--radius-sm);
  }

  .failed-track { color: var(--text-secondary); font-weight: var(--font-medium); }
  .failed-reason { color: var(--status-error); }

  .actions {
    display: flex;
    gap: var(--space-4);
    margin-top: var(--space-4);
  }
</style>
