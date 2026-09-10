<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { readTextFile } from '@tauri-apps/plugin-fs';
  import { _ } from 'svelte-i18n';

  export let csvPath: string = '';
  export let preview: { title: string; artist: string; album: string }[] = [];
  export let error: string = '';

  const dispatch = createEventDispatcher<{
    loaded: { path: string; preview: typeof preview; trackCount: number };
    error: { message: string };
  }>();

  let isDragOver = false;
  let isLoading = false;

  const REQUIRED_COLS = [
    ['Track Name', 'Track name'],
    ['Artist Name(s)', 'Artist name', 'Artist Name'],
  ];

  function parseCSV(text: string): { headers: string[]; rows: Record<string, string>[] } {
    const lines = text.split(/\r?\n/).filter(l => l.trim());
    if (lines.length < 2) return { headers: [], rows: [] };
    const headers = lines[0].split(',').map(h => h.replace(/^"|"$/g, '').trim());
    const rows = lines.slice(1).map(line => {
      const vals: string[] = [];
      let cur = '';
      let inQ = false;
      for (const ch of line) {
        if (ch === '"') { inQ = !inQ; }
        else if (ch === ',' && !inQ) { vals.push(cur.trim()); cur = ''; }
        else { cur += ch; }
      }
      vals.push(cur.trim());
      return Object.fromEntries(headers.map((h, i) => [h, vals[i] ?? '']));
    });
    return { headers, rows };
  }

  function validateHeaders(headers: string[]): boolean {
    return REQUIRED_COLS.every(group => group.some(col => headers.includes(col)));
  }

  function buildPreview(rows: Record<string, string>[]): typeof preview {
    return rows.slice(0, 5).map(row => ({
      title:  row['Track Name']    || row['Track name']    || '---',
      artist: row['Artist Name(s)']|| row['Artist name']   || row['Artist Name'] || '---',
      album:  row['Album Name']    || row['Album']         || '---',
    }));
  }

  async function processFile(path: string) {
    isLoading = true;
    error = '';
    try {
      const text = await readTextFile(path);
      const { headers, rows } = parseCSV(text);
      if (!validateHeaders(headers)) {
        error = $_('onboarding.step2.error_missing_cols');
        dispatch('error', { message: error });
        return;
      }
      const prev = buildPreview(rows);
      preview = prev;
      csvPath = path;
      dispatch('loaded', { path, preview: prev, trackCount: rows.length });
    } catch (e) {
      error = $_('errors.csv_read_error');
      dispatch('error', { message: error });
    } finally {
      isLoading = false;
    }
  }

  async function browseFile() {
    const selected = await open({
      filters: [{ name: 'CSV files', extensions: ['csv'] }],
      multiple: false,
    });
    if (selected && typeof selected === 'string') {
      await processFile(selected);
    }
  }

  function onDragOver(e: DragEvent) {
    e.preventDefault();
    isDragOver = true;
  }
  function onDragLeave() { isDragOver = false; }
  async function onDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
    const file = e.dataTransfer?.files[0];
    if (file && file.name.endsWith('.csv')) {
      await processFile((file as unknown as { path: string }).path ?? file.name);
    } else {
      error = $_('onboarding.step2.error_invalid');
    }
  }
</script>

<div
  class="drop-zone"
  class:drag-over={isDragOver}
  class:has-file={!!csvPath}
  class:has-error={!!error}
  on:dragover={onDragOver}
  on:dragleave={onDragLeave}
  on:drop={onDrop}
  role="button"
  tabindex="0"
  aria-label={$_('onboarding.step2.aria_drop_area')}
  id="csv-drop-zone"
  on:click={browseFile}
  on:keydown={(e) => e.key === 'Enter' && browseFile()}
>
  {#if isLoading}
    <div class="dz-inner">
      <div class="dz-status animate-spin">[...]</div>
      <p class="dz-label">{$_('common.loading').toUpperCase()}</p>
    </div>
  {:else if csvPath}
    <div class="dz-inner">
      <span class="dz-status">[FILE LOADED]</span>
      <p class="dz-label success-text">{csvPath.split(/[\\/]/).pop()}</p>
      <p class="dz-sub">{$_('onboarding.step2.browse_btn')} {$_('onboarding.step2.to_replace')}</p>
    </div>
  {:else}
    <div class="dz-inner">
      <span class="dz-status">[DRAG CSV OR CLICK TO BROWSE]</span>
      <p class="dz-label">{$_('onboarding.step2.drop_label').toUpperCase()}</p>
      <p class="dz-sub">{$_('common.or')} <span class="dz-link">{$_('onboarding.step2.browse_btn')}</span></p>
    </div>
  {/if}
</div>

{#if error}
  <div class="error-banner animate-slide-up">
    <strong>[ERROR]</strong> {error}
  </div>
{/if}

<style>
  .drop-zone {
    position: relative;
    border: 1px dashed var(--border-muted);
    background: #050505;
    padding: var(--space-8) var(--space-6);
    cursor: pointer;
    transition: all 0.1s ease;
    text-align: center;
    outline: none;
  }
  .drop-zone:hover,
  .drop-zone:focus-visible { border-color: #ffffff; background: #0a0a0a; }
  .drop-zone.drag-over {
    border-color: #ffffff;
    border-style: solid;
    background: #111111;
  }
  .drop-zone.has-file { border-color: #ffffff; border-style: solid; background: #000000; }
  .drop-zone.has-error { border-color: var(--status-error); }

  .dz-inner { display: flex; flex-direction: column; align-items: center; gap: var(--space-2); }

  .dz-status {
    font-size: 11px;
    letter-spacing: 0.1em;
    font-family: var(--font-mono);
    color: #888888;
    font-weight: 700;
  }

  .dz-label { font-size: 13px; font-weight: 700; color: #ffffff; letter-spacing: 0.05em; font-family: var(--font-mono); }
  .dz-label.success-text { color: #ffffff; }
  .dz-sub { font-size: 11px; color: #555555; }
  .dz-link { color: #ffffff; text-decoration: underline; }

  .error-banner {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    background: #000000;
    border: 1px solid var(--status-error);
    color: var(--status-error);
    font-size: 11px;
    letter-spacing: 0.05em;
    padding: var(--space-3) var(--space-4);
    margin-top: var(--space-3);
    font-family: var(--font-mono);
  }
</style>
