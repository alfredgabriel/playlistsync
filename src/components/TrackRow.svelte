<script lang="ts">
  import type { TrackState } from '../lib/stores/download';
  import { _ } from 'svelte-i18n';

  export let track: TrackState;

  function getStatusText(status: string, translate: any) {
    try {
      return (translate(`download.status_${status}`) || status).toUpperCase();
    } catch {
      return status.toUpperCase();
    }
  }

  function getStatusBadge(status: string) {
    switch (status) {
      case 'pending': return '[PENDING]';
      case 'searching': return '[SEARCH]';
      case 'downloading': return '[DOWNLOADING]';
      case 'done': return '[OK]';
      case 'error': return '[FAIL]';
      default: return '[..]';
    }
  }
</script>

<div class="track-row" class:is-active={track.status === 'searching' || track.status === 'downloading'} class:is-done={track.status === 'done'} class:is-error={track.status === 'error'}>
  <div class="track-index">#{String(track.index + 1).padStart(2, '0')}</div>
  <div class="track-badge">
    <span class="badge badge-{track.status === 'searching' || track.status === 'downloading' ? 'active' : track.status}">
      {getStatusBadge(track.status)}
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
      {getStatusText(track.status, $_)}
    {/if}
  </div>
</div>

<style>
  .track-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 10px 12px;
    background: #000000;
    border-bottom: 1px solid #1c1c1c;
    transition: all 0.1s ease;
    font-family: var(--font-mono);
  }

  .track-row.is-active {
    background: #0f0f0f;
    border-left: 2px solid #ffffff;
  }

  .track-row.is-done {
    opacity: 0.6;
  }

  .track-row.is-error {
    background: #110505;
    border-left: 2px solid var(--status-error);
  }

  .track-index {
    width: 32px;
    font-size: 11px;
    color: #555555;
  }

  .track-badge {
    flex-shrink: 0;
  }

  .track-info {
    flex: 1;
    min-width: 0;
  }

  .track-title {
    font-size: 12px;
    font-weight: 700;
    color: #ffffff;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .track-artist {
    font-size: 10px;
    color: #777777;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .track-status-text {
    font-size: 10px;
    color: #666666;
    letter-spacing: 0.08em;
    flex-shrink: 0;
    text-align: right;
  }

  .error-msg {
    color: var(--status-error);
    max-width: 140px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: inline-block;
  }
</style>
