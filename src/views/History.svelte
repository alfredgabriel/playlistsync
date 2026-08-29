<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { historyStore } from '../lib/stores/history';
  import { onMount } from 'svelte';
  import { open } from '@tauri-apps/plugin-shell';

  onMount(() => {
    historyStore.load();
  });

  async function handleOpen(folder: string) {
    try {
      await open(folder);
    } catch(e) {
      console.error(e);
    }
  }

  function handleClear() {
    if (confirm($_('history.clear_confirm') || 'Clear all history?')) {
      historyStore.clear();
    }
  }

  function formatDate(ts: number) {
    return new Date(ts * 1000).toLocaleString();
  }
</script>

<div class="history-view animate-fade-in">
  <div class="header">
    <h1 class="page-title">{$_('nav.history')}</h1>
    {#if $historyStore.length > 0}
      <button class="btn btn-secondary btn-sm" on:click={handleClear}>
        🗑 {$_('history.clear_btn')}
      </button>
    {/if}
  </div>

  {#if $historyStore.length === 0}
    <div class="empty-state">
      <div class="empty-icon">📭</div>
      <p class="empty-text">{$_('history.empty')}</p>
    </div>
  {:else}
    <div class="history-list">
      {#each $historyStore as session, i}
        <div class="history-card card-elevated" style="animation-delay: {i * 0.05}s">
          <div class="card-header">
            <h3 class="playlist-name">{session.playlistName}</h3>
            <span class="date">{formatDate(session.startedAt)}</span>
          </div>
          
          <div class="card-stats">
            <div class="stat">
              <span class="val success">{session.doneTracks}</span>
              <span class="lbl">{$_('results.downloaded', { values: { count: '' } }).replace('{count}', '')}</span>
            </div>
            <div class="stat">
              <span class="val error">{session.failedTracks}</span>
              <span class="lbl">{$_('results.failed', { values: { count: '' } }).replace('{count}', '')}</span>
            </div>
            <div class="stat">
              <span class="val format">{session.format.toUpperCase()}</span>
              <span class="lbl">Format</span>
            </div>
            <div class="stat">
              <span class="val time">{session.elapsedSecs}s</span>
              <span class="lbl">Time</span>
            </div>
          </div>

          <div class="card-actions">
            <button class="btn btn-secondary btn-sm" on:click={() => handleOpen(session.outputFolder)}>
              📁 {$_('results.open_folder')}
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .history-view {
    padding: var(--space-6);
    height: 100%;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-6);
    flex-shrink: 0;
  }

  .page-title {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: var(--space-4);
    opacity: 0.5;
  }

  .history-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: var(--space-4);
  }

  .history-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-4);
    animation: slideUp var(--transition-normal) ease backwards;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .playlist-name {
    font-size: var(--text-base);
    font-weight: var(--font-bold);
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 70%;
  }

  .date {
    font-size: var(--text-xs);
    color: var(--text-muted);
  }

  .card-stats {
    display: flex;
    justify-content: space-between;
    background: var(--bg-base);
    padding: var(--space-3);
    border-radius: var(--radius-md);
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .val {
    font-size: var(--text-lg);
    font-weight: var(--font-bold);
  }

  .val.success { color: var(--status-done); }
  .val.error { color: var(--status-error); }
  .val.format { color: var(--primary-base); }
  .val.time { color: var(--text-primary); }

  .lbl {
    font-size: var(--text-xs);
    color: var(--text-muted);
  }

  .card-actions {
    display: flex;
    justify-content: flex-end;
  }
</style>
