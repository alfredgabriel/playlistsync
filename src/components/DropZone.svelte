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
      // Simple CSV parse (handles quoted fields)
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
      title:  row['Track Name']    || row['Track name']    || '—',
      artist: row['Artist Name(s)']|| row['Artist name']   || row['Artist Name'] || '—',
      album:  row['Album Name']    || row['Album']         || '—',
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
      // In Tauri, dropped files give us a path via the file object
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
  aria-label="Drop CSV file area"
  id="csv-drop-zone"
  on:click={browseFile}
  on:keydown={(e) => e.key === 'Enter' && browseFile()}
>
  {#if isLoading}
    <div class="dz-inner">
      <div class="spinner animate-spin">⟳</div>
      <p class="dz-label">{$_('common.loading')}</p>
    </div>
  {:else if csvPath}
    <div class="dz-inner">
      <span class="dz-icon success">✅</span>
      <p class="dz-label success-text">{csvPath.split(/[\\/]/).pop()}</p>
      <p class="dz-sub">{$_('onboarding.step2.browse_btn')} to replace</p>
    </div>
  {:else}
    <div class="dz-inner">
      <div class="dz-icon-wrap" class:bounce={isDragOver}>
        <span class="dz-icon">📂</span>
      </div>
      <p class="dz-label">{$_('onboarding.step2.drop_label')}</p>
      <p class="dz-sub">or <span class="dz-link">{$_('onboarding.step2.browse_btn')}</span></p>
    </div>
  {/if}

  <!-- Ripple rings on drag over -->
  {#if isDragOver}
    <div class="ripple r1" /><div class="ripple r2" /><div class="ripple r3" />
  {/if}
</div>

{#if error}
  <div class="error-banner animate-slide-up">
    <span>⚠️</span> {error}
  </div>
{/if}

<style>
  .drop-zone {
    position: relative;
    border: 2px dashed var(--border-muted);
    border-radius: var(--radius-xl);
    background: var(--bg-glass);
    padding: var(--space-10) var(--space-8);
    cursor: pointer;
    transition: all var(--transition-normal);
    overflow: hidden;
    text-align: center;
    outline: none;
  }
  .drop-zone:hover,
  .drop-zone:focus-visible { border-color: var(--accent-primary); background: var(--bg-glass-hover); }
  .drop-zone.drag-over {
    border-color: var(--accent-primary);
    background: rgba(30, 215, 96, 0.06);
    box-shadow: var(--shadow-glow-green);
    transform: scale(1.01);
  }
  .drop-zone.has-file { border-color: var(--accent-primary); border-style: solid; }
  .drop-zone.has-error { border-color: var(--status-error); }

  .dz-inner { display: flex; flex-direction: column; align-items: center; gap: var(--space-3); position: relative; z-index: 1; }

  .dz-icon-wrap { font-size: 3rem; transition: transform var(--transition-fast); }
  .dz-icon-wrap.bounce { transform: scale(1.15) translateY(-4px); }
  .dz-icon { font-size: 3rem; }
  .dz-icon.success { font-size: 2.5rem; }

  .dz-label { font-size: var(--text-base); font-weight: var(--font-semibold); color: var(--text-primary); }
  .dz-label.success-text { color: var(--accent-primary); }
  .dz-sub { font-size: var(--text-sm); color: var(--text-muted); }
  .dz-link { color: var(--accent-primary); text-decoration: underline; }

  .spinner { font-size: var(--text-2xl); color: var(--accent-primary); }

  /* Ripple animation */
  .ripple {
    position: absolute;
    top: 50%; left: 50%;
    transform: translate(-50%, -50%);
    border-radius: 50%;
    border: 2px solid rgba(30, 215, 96, 0.4);
    animation: rippleOut 1.2s ease-out infinite;
    pointer-events: none;
  }
  .r1 { width: 80px;  height: 80px;  animation-delay: 0s; }
  .r2 { width: 140px; height: 140px; animation-delay: 0.3s; }
  .r3 { width: 200px; height: 200px; animation-delay: 0.6s; }

  @keyframes rippleOut {
    from { opacity: 0.8; transform: translate(-50%,-50%) scale(0.8); }
    to   { opacity: 0;   transform: translate(-50%,-50%) scale(1.4); }
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    background: rgba(248,113,113,0.08);
    border: 1px solid rgba(248,113,113,0.3);
    border-radius: var(--radius-md);
    color: var(--status-error);
    font-size: var(--text-sm);
    padding: var(--space-3) var(--space-4);
    margin-top: var(--space-3);
  }
</style>
