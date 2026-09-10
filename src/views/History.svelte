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
    if (confirm($_('history.clear_confirm'))) {
      historyStore.clear();
    }
  }

  function formatDate(ts: number) {
    return new Date(ts * 1000).toLocaleString();
  }
</script>

<div class="history-view animate-fade-in">
  <div class="header">
    <div>
      <div class="meta-tag">// DOWNLOAD LOGS</div>
      <h1 class="page-title">{$_('nav.history').toUpperCase()}</h1>
    </div>
    {#if $historyStore.length > 0}
      <button class="btn btn-secondary btn-sm" on:click={handleClear}>
        [CLEAR] {$_('history.clear_btn')}
      </button>
    {/if}
  </div>

  {#if $historyStore.length === 0}
    <div class="empty-state">
      <div class="empty-tag">[NO LOGS FOUND]</div>
      <p class="empty-text">{$_('history.empty')}</p>
    </div>
  {:else}
    <div class="history-list">
      {#each $historyStore as session, i}
        <div class="history-card" style="animation-delay: {i * 0.05}s">
          <div class="card-header">
            <h3 class="playlist-name">{session.playlistName}</h3>
            <span class="date">{formatDate(session.startedAt)}</span>
          </div>
          
          <div class="card-stats">
            <div class="stat">
              <span class="val success">{session.doneTracks}</span>
              <span class="lbl">{$_('results.downloaded_label').toUpperCase()}</span>
            </div>
            <div class="stat">
              <span class="val error">{session.failedTracks}</span>
              <span class="lbl">{$_('results.failed_label').toUpperCase()}</span>
            </div>
            <div class="stat">
              <span class="val format">{session.format.toUpperCase()}</span>
              <span class="lbl">{$_('history.format_lbl').toUpperCase()}</span>
            </div>
            <div class="stat">
              <span class="val time">{session.elapsedSecs}S</span>
              <span class="lbl">{$_('history.time_lbl').toUpperCase()}</span>
            </div>
          </div>

          <div class="card-actions">
            <button class="btn btn-secondary btn-sm" on:click={() => handleOpen(session.outputFolder)}>
              {$_('results.open_folder')} →
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
    background: #000000;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin-bottom: var(--space-6);
    flex-shrink: 0;
  }

  .meta-tag {
    font-size: 10px;
    letter-spacing: 0.15em;
    color: #666666;
    font-family: var(--font-mono);
    margin-bottom: var(--space-1);
  }

  .page-title {
    font-size: 1.4rem;
    font-weight: 700;
    color: #ffffff;
    font-family: var(--font-mono);
    letter-spacing: 0.05em;
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: #555555;
    border: 1px dashed var(--border-muted);
    background: #050505;
    padding: var(--space-8);
  }

  .empty-tag {
    font-size: 11px;
    font-weight: 700;
    color: #888888;
    margin-bottom: var(--space-2);
    font-family: var(--font-mono);
    letter-spacing: 0.1em;
  }

  .empty-text {
    font-size: 11px;
    color: #666666;
  }

  .history-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: var(--space-4);
  }

  .history-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-5);
    background: #050505;
    border: 1px solid var(--border-muted);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .playlist-name {
    font-size: 13px;
    font-weight: 700;
    color: #ffffff;
    font-family: var(--font-mono);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 65%;
  }

  .date {
    font-size: 10px;
    color: #555555;
    font-family: var(--font-mono);
  }

  .card-stats {
    display: flex;
    justify-content: space-between;
    background: #000000;
    padding: var(--space-3);
    border: 1px solid var(--border-muted);
    font-family: var(--font-mono);
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
  }

  .val {
    font-size: 13px;
    font-weight: 700;
    color: #ffffff;
  }

  .val.success { color: #ffffff; }
  .val.error { color: var(--status-error); }
  .val.format { color: #888888; }
  .val.time { color: #ffffff; }

  .lbl {
    font-size: 9px;
    color: #555555;
    letter-spacing: 0.05em;
  }

  .card-actions {
    display: flex;
    justify-content: flex-end;
  }
</style>
