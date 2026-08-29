<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let ytdlpAvailable = true;
  let ffmpegAvailable = true;

  onMount(async () => {
    try {
      const status = await invoke<{ytdlpAvailable: boolean, ffmpegAvailable: boolean}>('check_tools');
      ytdlpAvailable = status.ytdlpAvailable;
      ffmpegAvailable = status.ffmpegAvailable;
    } catch(e) {}
  });
</script>

{#if !ytdlpAvailable || !ffmpegAvailable}
  <div class="tool-banner">
    <div class="banner-content">
      <strong>⚠️ {$_('settings.tools_title')}:</strong> 
      {#if !ytdlpAvailable} <span>{$_('banner.ytdlp_missing')}</span> {/if}
      {#if !ffmpegAvailable} <span>{$_('banner.ffmpeg_missing')}</span> {/if}
      <span class="muted">{$_('banner.install_hint')}</span>
    </div>
  </div>
{/if}

<style>
  .tool-banner {
    background: rgba(248, 113, 113, 0.1);
    border-bottom: 1px solid rgba(248, 113, 113, 0.2);
    color: var(--status-error);
    padding: var(--space-2) var(--space-4);
    font-size: var(--text-sm);
    display: flex;
    justify-content: center;
  }
  .banner-content {
    display: flex;
    gap: var(--space-2);
    align-items: center;
  }
  .muted {
    color: var(--text-muted);
    font-size: var(--text-xs);
    margin-left: var(--space-4);
  }
</style>
