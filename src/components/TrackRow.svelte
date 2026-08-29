<script lang="ts">
  import type { TrackState } from '../lib/stores/download';
  import { _ } from 'svelte-i18n';

  export let track: TrackState;

  function getStatusIcon(status: string) {
    switch (status) {
      case 'pending': return '⏳';
      case 'searching': return '🔍';
      case 'downloading': return '⬇️';
      case 'done': return '✅';
      case 'error': return '❌';
      default: return '⏳';
    }
  }
</script>

<div class="track-row" class:is-active={track.status === 'searching' || track.status === 'downloading'} class:is-done={track.status === 'done'} class:is-error={track.status === 'error'}>
  <div class="track-index">{track.index + 1}</div>
  <div class="track-status-icon">
    <span class:animate-spin={track.status === 'searching' || track.status === 'downloading'}>
      {getStatusIcon(track.status)}
    </span>
  </div>
  <div class="track-info">
    <div class="track-title" title={track.title}>{track.title}</div>
    <div class="track-artist" title={track.artist}>{track.artist}</div>
  </div>
  <div class="track-status-text">
    {#if track.status === 'error' && track.error}
      <span class="error-msg" title={track.error}>{track.error}</span>
    {:else}
      {$_(`download.status_${track.status}`)}
    {/if}
  </div>
</div>

<style>
  .track-row {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-md);
    background: var(--bg-glass);
    transition: all var(--transition-fast);
  }

  .track-row.is-active {
    background: rgba(124, 58, 237, 0.1);
    border-left: 3px solid var(--status-active);
  }

  .track-row.is-done {
    opacity: 0.7;
  }

  .track-row.is-error {
    background: rgba(248, 113, 113, 0.05);
    border-left: 3px solid var(--status-error);
  }

  .track-index {
    width: 24px;
    font-size: var(--text-sm);
    color: var(--text-muted);
    text-align: right;
  }

  .track-status-icon {
    width: 24px;
    display: flex;
    justify-content: center;
  }

  .track-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .track-title {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .track-artist {
    font-size: var(--text-xs);
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .track-status-text {
    width: 120px;
    font-size: var(--text-xs);
    color: var(--text-muted);
    text-align: right;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .track-row.is-active .track-status-text {
    color: var(--status-active);
    font-weight: var(--font-medium);
  }

  .track-row.is-done .track-status-text {
    color: var(--status-done);
  }

  .error-msg {
    color: var(--status-error);
  }
</style>
