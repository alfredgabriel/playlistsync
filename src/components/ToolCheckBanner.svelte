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
      <strong>[ALERT] {$_('settings.tools_title').toUpperCase()}:</strong> 
      {#if !ytdlpAvailable} <span>{$_('banner.ytdlp_missing')}</span> {/if}
      {#if !ffmpegAvailable} <span>{$_('banner.ffmpeg_missing')}</span> {/if}
      <span class="muted">{$_('banner.install_hint')}</span>
    </div>
  </div>
{/if}

<style>
  .tool-banner {
    background: #000000;
    border-bottom: 1px solid var(--status-error);
    color: var(--status-error);
    padding: var(--space-2) var(--space-4);
    font-size: 11px;
    font-family: var(--font-mono);
    display: flex;
    justify-content: center;
  }
  .banner-content {
    display: flex;
    gap: var(--space-2);
    align-items: center;
  }
  .muted {
    color: #888888;
    font-size: 10px;
    margin-left: var(--space-4);
  }
</style>
