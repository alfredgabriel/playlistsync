<script lang="ts">
  import { downloadStore } from '../lib/stores/download';
  import { _ } from 'svelte-i18n';
  import TrackRow from './TrackRow.svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let onFinish: () => void = () => {};

  $: session = $downloadStore;
  $: tracks = session?.tracks || [];
  $: done = session?.doneTracks || 0;
  $: failed = session?.failedTracks || 0;
  $: total = session?.totalTracks || 0;
  $: progress = total > 0 ? ((done + failed) / total) * 100 : 0;
  
  $: isFinished = session && (done + failed) === total;

  // Very basic ETA estimation
  $: elapsedMs = session ? Date.now() - session.startedAt : 0;
  $: processed = done + failed;
  $: etaStr = processed > 0 && total > processed 
      ? formatEta(((elapsedMs / processed) * (total - processed)) / 1000)
      : '...';

  function formatEta(secs: number) {
    if (!isFinite(secs)) return '...';
    const m = Math.floor(secs / 60);
    const s = Math.floor(secs % 60);
    return `${m}m ${s}s`;
  }

  async function handleCancel() {
    if (confirm($_('download.cancel_confirm'))) {
      try {
        await invoke('cancel_download');
        downloadStore.completeSession();
        onFinish();
      } catch(e) {
        console.error(e);
      }
    }
  }

  // Auto-finish
  $: if (isFinished && session && !session.completedAt) {
    downloadStore.completeSession();
    setTimeout(onFinish, 1500); // Wait a bit before showing results
  }
</script>

{#if session}
  <div class="progress-container animate-fade-in">
    <div class="progress-header card-elevated">
      <div class="header-main">
        <div>
          <h2 class="title">{$_('download.title')}</h2>
          <p class="subtitle">{session.playlistName} • {$_('download.progress_label', { values: { done: (done + failed), total } })}</p>
        </div>
        {#if !isFinished}
          <button class="btn btn-danger btn-sm" on:click={handleCancel}>
            {$_('download.cancel_btn')}
          </button>
        {/if}
      </div>

      <div class="progress-bar-wrap">
        <div class="progress-track">
          <div class="progress-fill" style="width: {progress}%"></div>
        </div>
      </div>

      <div class="stats">
        <span class="stat-item">✅ {$_('download.stat_successful', { values: { count: done } })}</span>
        {#if failed > 0}
          <span class="stat-item error">❌ {$_('download.stat_failed', { values: { count: failed } })}</span>
        {/if}
        {#if !isFinished}
          <span class="stat-item eta">⏱ {$_('download.eta_label', { values: { eta: etaStr } })}</span>
        {/if}
      </div>
    </div>

    <div class="track-list">
      {#each tracks as track (track.index)}
        <TrackRow {track} />
      {/each}
    </div>
  </div>
{/if}

<style>
  .progress-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    max-width: 800px;
    margin: 0 auto;
    width: 100%;
  }

  .progress-header {
    margin: var(--space-6) var(--space-6) var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    flex-shrink: 0;
  }

  .header-main {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .title {
    font-size: var(--text-xl);
    font-weight: var(--font-bold);
    color: var(--text-primary);
  }

  .subtitle {
    font-size: var(--text-sm);
    color: var(--text-secondary);
    margin-top: 2px;
  }

  .progress-bar-wrap {
    padding: var(--space-2) 0;
  }

  .stats {
    display: flex;
    gap: var(--space-4);
    font-size: var(--text-xs);
    color: var(--text-muted);
  }

  .stat-item.error {
    color: var(--status-error);
  }
  
  .stat-item.eta {
    margin-left: auto;
  }

  .track-list {
    flex: 1;
    overflow-y: auto;
    padding: 0 var(--space-6) var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
</style>
