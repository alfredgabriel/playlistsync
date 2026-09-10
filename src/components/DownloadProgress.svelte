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

  $: elapsedMs = session ? Date.now() - session.startedAt : 0;
  $: processed = done + failed;
  $: etaStr = processed > 0 && total > processed 
      ? formatEta(((elapsedMs / processed) * (total - processed)) / 1000)
      : '---';

  function formatEta(secs: number) {
    if (!isFinite(secs)) return '---';
    const m = Math.floor(secs / 60);
    const s = Math.floor(secs % 60);
    return `${m}M ${s}S`;
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

  $: if (isFinished && session && !session.completedAt) {
    downloadStore.completeSession();
    setTimeout(onFinish, 1500);
  }
</script>

<div class="progress-view animate-fade-in">
  <div class="header-card">
    <div class="info-top">
      <div>
        <div class="meta-tag">// BATCH PROCESSING PIPELINE</div>
        <h2 class="title">{session?.playlistName.toUpperCase() || 'PLAYLIST'}</h2>
      </div>
      <button class="btn btn-danger btn-sm" on:click={handleCancel}>
        [ABORT] {$_('download.cancel_btn')}
      </button>
    </div>

    <div class="progress-bar-wrap">
      <div class="progress-track">
        <div class="progress-fill" style="width: {progress}%"></div>
      </div>
    </div>

    <div class="stats-row">
      <span class="stat-item">
        <strong>PROGRESS:</strong> {Math.round(progress)}% ({processed}/{total})
      </span>
      <span class="stat-item">
        <strong>SUCCESS:</strong> {done}
      </span>
      {#if failed > 0}
        <span class="stat-item error">
          <strong>FAILED:</strong> {failed}
        </span>
      {/if}
      <span class="stat-item">
        <strong>EST. REMAINING:</strong> {etaStr}
      </span>
    </div>
  </div>

  <div class="tracks-list">
    {#each tracks as track (track.index)}
      <TrackRow {track} />
    {/each}
  </div>
</div>

<style>
  .progress-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: var(--space-6);
    background: #000000;
    gap: var(--space-4);
  }

  .header-card {
    background: #050505;
    border: 1px solid var(--border-muted);
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .info-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .meta-tag {
    font-size: 10px;
    letter-spacing: 0.12em;
    color: #666666;
    margin-bottom: var(--space-1);
    font-family: var(--font-mono);
  }

  .title {
    font-size: 18px;
    font-weight: 700;
    color: #ffffff;
    font-family: var(--font-mono);
    letter-spacing: 0.05em;
  }

  .progress-bar-wrap {
    width: 100%;
  }

  .stats-row {
    display: flex;
    gap: var(--space-6);
    font-size: 11px;
    font-family: var(--font-mono);
    color: #777777;
    flex-wrap: wrap;
  }

  .stat-item strong {
    color: #ffffff;
  }

  .stat-item.error strong {
    color: var(--status-error);
  }

  .tracks-list {
    flex: 1;
    overflow-y: auto;
    border: 1px solid var(--border-muted);
    background: #000000;
  }
</style>
