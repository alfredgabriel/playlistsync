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
  <div class="header-section">
    <div class="meta-tag">// SYSTEM CONFIGURATION</div>
    <h1 class="page-title">{$_('nav.settings').toUpperCase()}</h1>
  </div>

  <div class="settings-grid">
    <!-- Format & Quality -->
    <section class="settings-group card-elevated">
      <h2 class="group-title">// {$_('settings.audio_format_title').toUpperCase()}</h2>
      
      <div class="setting-item">
        <span class="item-label">{$_('settings.format_label').toUpperCase()}</span>
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
          <span class="item-label">{$_('settings.mp3_quality_label').toUpperCase()}</span>
          <select class="form-input form-select" value={$settings.mp3Quality} on:change={(e) => handleUpdate('mp3Quality', e.currentTarget.value)}>
            <option value="vbr0">VBR0 (Best, Variable)</option>
            <option value="192">192 kbps (Good)</option>
            <option value="128">128 kbps (Smaller)</option>
          </select>
        </div>
      {/if}

      <div class="setting-item">
        <span class="item-label">{$_('settings.generate_m3u_label').toUpperCase()}</span>
        <label class="toggle-checkbox">
          <input type="checkbox" checked={$settings.generateM3u} on:change={(e) => handleUpdate('generateM3u', e.currentTarget.checked)} />
          <span class="box-indicator"></span>
        </label>
      </div>
    </section>

    <!-- Search & Filters -->
    <section class="settings-group card-elevated">
      <h2 class="group-title">// {$_('settings.search_filters_title').toUpperCase()}</h2>

      <div class="setting-item">
        <div class="item-info">
          <span class="item-label">{$_('settings.search_mode_label').toUpperCase()}</span>
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
          <span class="item-label">{$_('settings.exclude_instrumentals_label').toUpperCase()}</span>
        </div>
        <label class="toggle-checkbox">
          <input type="checkbox" checked={$settings.excludeInstrumentals} on:change={(e) => handleUpdate('excludeInstrumentals', e.currentTarget.checked)} />
          <span class="box-indicator"></span>
        </label>
      </div>

      <div class="setting-item inline-inputs">
        <div class="item-info">
          <span class="item-label">{$_('settings.duration_range_label').toUpperCase()}</span>
        </div>
        <div class="inputs">
          <input type="number" class="form-input short" value={$settings.durationMin} on:change={(e) => handleUpdate('durationMin', parseInt(e.currentTarget.value))} min="0" />
          <span>-</span>
          <input type="number" class="form-input short" value={$settings.durationMax} on:change={(e) => handleUpdate('durationMax', parseInt(e.currentTarget.value))} min="0" />
          <span class="unit">{$_('settings.seconds_unit').toUpperCase()}</span>
        </div>
      </div>
    </section>

    <!-- System Diagnostics -->
    <section class="settings-group card-elevated">
      <h2 class="group-title">// {$_('settings.tools_title').toUpperCase()}</h2>

      {#if toolStatus}
        <div class="tool-item">
          <span class="badge {toolStatus.ytdlpAvailable ? 'badge-done' : 'badge-error'}">
            {toolStatus.ytdlpAvailable ? '[OK]' : '[FAIL]'}
          </span>
          <div class="tool-info">
            <strong>YT-DLP</strong>
            <span class="tool-path" title={toolStatus.ytdlpPath || ''}>{toolStatus.ytdlpPath || $_('settings.not_found_path')}</span>
            {#if toolStatus.ytdlpAvailable}
              <span class="tool-version">V{ytdlpVersion}</span>
            {/if}
          </div>
        </div>

        <div class="tool-item">
          <span class="badge {toolStatus.ffmpegAvailable ? 'badge-done' : 'badge-error'}">
            {toolStatus.ffmpegAvailable ? '[OK]' : '[FAIL]'}
          </span>
          <div class="tool-info">
            <strong>FFMPEG</strong>
            <span class="tool-path" title={toolStatus.ffmpegPath || ''}>{toolStatus.ffmpegPath || $_('settings.not_found_path')}</span>
          </div>
        </div>
      {:else}
        <div class="loading-tools">{$_('settings.checking_tools').toUpperCase()}</div>
      {/if}
    </section>
  </div>
</div>

<style>
  .settings-view {
    padding: var(--space-6);
    height: 100%;
    overflow-y: auto;
    background: #000000;
  }

  .header-section {
    margin-bottom: var(--space-6);
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

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(340px, 1fr));
    gap: var(--space-4);
  }

  .settings-group {
    background: #050505;
    border: 1px solid var(--border-muted);
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .group-title {
    font-size: 11px;
    font-weight: 700;
    color: #888888;
    letter-spacing: 0.1em;
    font-family: var(--font-mono);
    border-bottom: 1px solid var(--border-muted);
    padding-bottom: var(--space-2);
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-4);
  }

  .item-label {
    font-size: 11px;
    font-weight: 700;
    color: #ffffff;
    letter-spacing: 0.05em;
    font-family: var(--font-mono);
  }

  .item-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .hint {
    font-size: 10px;
    color: #555555;
    line-height: 1.4;
  }

  .btn-group {
    display: flex;
    gap: 4px;
  }

  .inline-inputs .inputs {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: 11px;
    color: #888888;
    font-family: var(--font-mono);
  }

  .form-input.short {
    width: 65px;
    padding: 6px;
    text-align: right;
    font-family: var(--font-mono);
  }

  .unit {
    font-size: 10px;
    color: #555555;
  }

  .toggle-checkbox {
    cursor: pointer;
    display: inline-flex;
    align-items: center;
  }

  .toggle-checkbox input {
    display: none;
  }

  .box-indicator {
    width: 18px;
    height: 18px;
    border: 1px solid #444444;
    background: #000000;
    display: inline-block;
    position: relative;
    transition: all 0.1s;
  }

  .toggle-checkbox input:checked + .box-indicator {
    background: #ffffff;
    border-color: #ffffff;
  }

  .toggle-checkbox input:checked + .box-indicator::after {
    content: "";
    position: absolute;
    top: 3px;
    left: 3px;
    right: 3px;
    bottom: 3px;
    background: #000000;
  }

  .tool-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3);
    background: #000000;
    border: 1px solid var(--border-muted);
  }

  .tool-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
    font-family: var(--font-mono);
  }

  .tool-info strong {
    font-size: 11px;
    color: #ffffff;
    letter-spacing: 0.05em;
  }

  .tool-path {
    font-size: 10px;
    color: #666666;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tool-version {
    font-size: 10px;
    color: #888888;
    margin-top: 2px;
  }

  .loading-tools {
    font-size: 11px;
    color: #666666;
    font-family: var(--font-mono);
  }
</style>
