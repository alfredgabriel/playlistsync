<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { settings } from '../lib/stores/settings';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  type ToolStatus = {
    ytdlpAvailable: boolean,
    ytdlpPath: string | null,
    ffmpegAvailable: boolean,
    ffmpegPath: string | null,
  };

  let toolStatus: ToolStatus | null = null;
  let ytdlpVersion = '...';

  onMount(async () => {
    await settings.init();
    try {
      toolStatus = await invoke<ToolStatus>('check_tools');
      if (toolStatus.ytdlpAvailable) {
        ytdlpVersion = await invoke<string>('get_ytdlp_version');
      }
    } catch(e) {
      console.error(e);
    }
  });

  function handleUpdate<K extends keyof import('../lib/stores/settings').AppSettings>(key: K, value: any) {
    settings.updateSetting(key, value);
  }
</script>

<div class="settings-view animate-fade-in">
  <h1 class="page-title">{$_('nav.settings')}</h1>

  <div class="settings-grid">
    <!-- Format & Quality -->
    <section class="settings-group card-elevated">
      <h2 class="group-title">{$_('settings.audio_format_title')}</h2>
      
      <div class="setting-item">
        <label>{$_('settings.format_label')}</label>
        <div class="btn-group">
          <button class="btn btn-sm" class:btn-primary={$settings.format === 'm4a'} class:btn-secondary={$settings.format !== 'm4a'} on:click={() => handleUpdate('format', 'm4a')}>
            M4A (AAC)
          </button>
          <button class="btn btn-sm" class:btn-primary={$settings.format === 'mp3'} class:btn-secondary={$settings.format !== 'mp3'} on:click={() => handleUpdate('format', 'mp3')}>
            MP3
          </button>
        </div>
      </div>

      {#if $settings.format === 'mp3'}
        <div class="setting-item animate-fade-in">
          <label>{$_('settings.mp3_quality_label')}</label>
          <select class="input-field" value={$settings.mp3Quality} on:change={(e) => handleUpdate('mp3Quality', e.currentTarget.value)}>
            <option value="vbr0">VBR0 (Best, Variable)</option>
            <option value="192">192 kbps (Good)</option>
            <option value="128">128 kbps (Smaller)</option>
          </select>
        </div>
      {/if}

      <div class="setting-item">
        <label>{$_('settings.generate_m3u_label')}</label>
        <label class="toggle-switch">
          <input type="checkbox" checked={$settings.generateM3u} on:change={(e) => handleUpdate('generateM3u', e.currentTarget.checked)} />
          <span class="slider"></span>
        </label>
      </div>
    </section>

    <!-- Search & Filters -->
    <section class="settings-group card-elevated">
      <h2 class="group-title">{$_('settings.search_filters_title')}</h2>

      <div class="setting-item">
        <div class="item-info">
          <label>{$_('settings.search_mode_label')}</label>
          <span class="hint">{$_('settings.search_mode_hint')}</span>
        </div>
        <div class="btn-group">
          <button class="btn btn-sm" class:btn-primary={$settings.searchMode === 'fast'} class:btn-secondary={$settings.searchMode !== 'fast'} on:click={() => handleUpdate('searchMode', 'fast')}>
            {$_('settings.search_fast_short')}
          </button>
          <button class="btn btn-sm" class:btn-primary={$settings.searchMode === 'deep'} class:btn-secondary={$settings.searchMode !== 'deep'} on:click={() => handleUpdate('searchMode', 'deep')}>
            {$_('settings.search_deep_short')}
          </button>
        </div>
      </div>

      <div class="setting-item">
        <div class="item-info">
          <label>{$_('settings.exclude_instrumentals_label')}</label>
        </div>
        <label class="toggle-switch">
          <input type="checkbox" checked={$settings.excludeInstrumentals} on:change={(e) => handleUpdate('excludeInstrumentals', e.currentTarget.checked)} />
          <span class="slider"></span>
        </label>
      </div>

      <div class="setting-item inline-inputs">
        <div class="item-info">
          <label>{$_('settings.duration_range_label')}</label>
        </div>
        <div class="inputs">
          <input type="number" class="input-field short" value={$settings.durationMin} on:change={(e) => handleUpdate('durationMin', parseInt(e.currentTarget.value))} min="0" />
          <span>-</span>
          <input type="number" class="input-field short" value={$settings.durationMax} on:change={(e) => handleUpdate('durationMax', parseInt(e.currentTarget.value))} min="0" />
          <span>{$_('settings.seconds_unit')}</span>
        </div>
      </div>
    </section>

    <!-- System Tools -->
    <section class="settings-group card-elevated">
      <h2 class="group-title">{$_('settings.tools_title')}</h2>
      
      {#if toolStatus}
        <div class="tool-item">
          <span class="tool-icon">{toolStatus.ytdlpAvailable ? '✅' : '❌'}</span>
          <div class="tool-info">
            <strong>yt-dlp</strong>
            <span class="tool-path" title={toolStatus.ytdlpPath || ''}>{toolStatus.ytdlpPath || $_('settings.not_found_path')}</span>
            {#if toolStatus.ytdlpAvailable}
              <span class="tool-version">v{ytdlpVersion}</span>
            {/if}
          </div>
        </div>

        <div class="tool-item">
          <span class="tool-icon">{toolStatus.ffmpegAvailable ? '✅' : '❌'}</span>
          <div class="tool-info">
            <strong>ffmpeg</strong>
            <span class="tool-path" title={toolStatus.ffmpegPath || ''}>{toolStatus.ffmpegPath || $_('settings.not_found_path')}</span>
          </div>
        </div>
      {:else}
        <div class="loading-tools">{$_('settings.checking_tools')}</div>
      {/if}
    </section>
  </div>
</div>

<style>
  .settings-view {
    padding: var(--space-6);
    height: 100%;
    overflow-y: auto;
  }

  .page-title {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    margin-bottom: var(--space-6);
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: var(--space-6);
  }

  .settings-group {
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .group-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--text-primary);
    margin-bottom: var(--space-2);
    border-bottom: 1px solid var(--border-subtle);
    padding-bottom: var(--space-2);
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-4);
  }

  .item-info {
    display: flex;
    flex-direction: column;
  }

  .setting-item label {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--text-secondary);
  }

  .hint {
    font-size: var(--text-xs);
    color: var(--text-muted);
  }

  .btn-group {
    display: flex;
    gap: var(--space-1);
  }

  .inline-inputs .inputs {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--text-sm);
    color: var(--text-muted);
  }

  .input-field.short {
    width: 70px;
    padding: var(--space-2);
    text-align: right;
  }

  /* Toggle Switch */
  .toggle-switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
  }

  .toggle-switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0; left: 0; right: 0; bottom: 0;
    background-color: var(--bg-base);
    transition: .3s;
    border-radius: 24px;
    border: 1px solid var(--border-subtle);
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 2px;
    bottom: 2px;
    background-color: var(--text-secondary);
    transition: .3s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: var(--primary-base);
    border-color: var(--primary-base);
  }

  input:checked + .slider:before {
    transform: translateX(20px);
    background-color: white;
  }

  /* Tools */
  .tool-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3);
    background: var(--bg-base);
    border-radius: var(--radius-md);
  }

  .tool-icon {
    font-size: var(--text-lg);
  }

  .tool-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .tool-info strong {
    font-size: var(--text-sm);
    color: var(--text-primary);
  }

  .tool-path {
    font-size: var(--text-xs);
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tool-version {
    font-size: var(--text-xs);
    color: var(--primary-base);
    margin-top: 2px;
  }
</style>
