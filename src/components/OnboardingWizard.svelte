<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { open } from '@tauri-apps/plugin-shell';
  import { open as openDialog } from '@tauri-apps/plugin-dialog';
  import DropZone from './DropZone.svelte';

  export let onComplete: (csvPath: string, outputFolder: string) => void = () => {};

  let currentStep = 1;
  const TOTAL_STEPS = 3;

  let csvPath: string = '';
  let csvPreview: { title: string; artist: string; album: string }[] = [];
  let trackCount: number = 0;
  let csvError: string = '';

  let outputFolder: string = '';
  let format: 'm4a' | 'mp3' = 'm4a';
  
  $: stepLabels = [
    $_('onboarding.steps.step1'),
    $_('onboarding.steps.step2'),
    $_('onboarding.steps.step3'),
  ];
  let generateM3u = true;

  function openExportify() { open('https://exportify.net'); }
  function openTuneMyMusic() { open('https://www.tunemymusic.com/transfer'); }

  function nextStep() { if (currentStep < TOTAL_STEPS) currentStep++; }
  function prevStep() { if (currentStep > 1) currentStep--; }

  function handleCsvLoaded(e: CustomEvent<{ path: string; preview: typeof csvPreview; trackCount: number }>) {
    csvPath = e.detail.path;
    csvPreview = e.detail.preview;
    trackCount = e.detail.trackCount;
    csvError = '';
  }
  
  function handleCsvError(e: CustomEvent<{ message: string }>) {
    csvError = e.detail.message;
    csvPath = '';
  }

  async function browseFolder() {
    const selected = await openDialog({
      directory: true,
      multiple: false,
    });
    if (selected && typeof selected === 'string') {
      outputFolder = selected;
    }
  }

  function handleStart() {
    if (csvPath && outputFolder) {
      onComplete(csvPath, outputFolder);
    }
  }
</script>

<div class="wizard-container animate-fade-in">
  <!-- Stepper Header -->
  <div class="stepper">
    {#each Array(TOTAL_STEPS) as _, i}
      <div class="step-node" class:active={currentStep === i + 1} class:done={currentStep > i + 1}>
        <span class="step-num">[0{i + 1}]</span>
        <span class="step-label">{stepLabels[i]}</span>
      </div>
      {#if i < TOTAL_STEPS - 1}
        <div class="step-line" class:filled={currentStep > i + 1}></div>
      {/if}
    {/each}
  </div>

  <!-- Step Content -->
  <div class="step-content">
    {#if currentStep === 1}
      <div class="step-pane animate-fade-in">
        <div class="step-header">
          <div class="step-tag">// STEP 01</div>
          <h2 class="step-title">{$_('onboarding.step1.title').toUpperCase()}</h2>
          <p class="step-desc">{$_('onboarding.step1.subtitle')}</p>
        </div>

        <div class="export-options">
          <div class="export-card">
            <div class="export-header">
              <span class="export-tag">[OPTION A]</span>
              <h4>SPOTIFY (EXPORTIFY)</h4>
            </div>
            <p>{$_('onboarding.step1.exportify_desc')}</p>
            <button class="btn btn-secondary btn-sm" on:click={openExportify}>
              EXPORTIFY.NET →
            </button>
          </div>

          <div class="export-card">
            <div class="export-header">
              <span class="export-tag">[OPTION B]</span>
              <h4>TUNEMYMUSIC</h4>
            </div>
            <p>{$_('onboarding.step1.tunemymusic_desc')}</p>
            <button class="btn btn-secondary btn-sm" on:click={openTuneMyMusic}>
              TUNEMYMUSIC.COM →
            </button>
          </div>
        </div>

        <div class="step-footer">
          <span></span>
          <button class="btn btn-primary" id="step1-next-btn" on:click={nextStep}>
            {$_('onboarding.step1.next_btn')} →
          </button>
        </div>
      </div>

    {:else if currentStep === 2}
      <div class="step-pane animate-fade-in">
        <div class="step-header">
          <div class="step-tag">// STEP 02</div>
          <h2 class="step-title">{$_('onboarding.step2.title').toUpperCase()}</h2>
          <p class="step-desc">{$_('onboarding.step2.subtitle')}</p>
        </div>

        <DropZone 
          bind:csvPath 
          bind:preview={csvPreview}
          bind:error={csvError}
          on:loaded={handleCsvLoaded}
          on:error={handleCsvError}
        />

        {#if csvPreview.length > 0}
          <div class="preview-box">
            <div class="preview-header">
              <strong>[PARSED TRACKS]: {trackCount} TOTAL</strong>
            </div>
            <div class="preview-list">
              {#each csvPreview as tr, idx}
                <div class="preview-row">
                  <span class="p-num">#{idx + 1}</span>
                  <span class="p-title">{tr.title}</span>
                  <span class="p-artist">{tr.artist}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}

        <div class="step-footer">
          <button class="btn btn-secondary" on:click={prevStep}>← {$_('common.back')}</button>
          <button class="btn btn-primary" id="step2-next-btn" disabled={!csvPath} on:click={nextStep}>
            {$_('common.next')} →
          </button>
        </div>
      </div>

    {:else if currentStep === 3}
      <div class="step-pane animate-fade-in">
        <div class="step-header">
          <div class="step-tag">// STEP 03</div>
          <h2 class="step-title">{$_('onboarding.step3.title').toUpperCase()}</h2>
          <p class="step-desc">{$_('onboarding.step3.subtitle')}</p>
        </div>

        <div class="config-form">
          <div class="form-group">
            <label class="form-label" for="out-folder">{$_('onboarding.step3.folder_label')}</label>
            <div class="folder-input-row">
              <input 
                id="out-folder"
                type="text" 
                class="form-input" 
                placeholder="C:\Music" 
                bind:value={outputFolder} 
                readonly
              />
              <button class="btn btn-secondary" on:click={browseFolder}>
                {$_('onboarding.step3.browse_btn')}
              </button>
            </div>
          </div>

          <div class="form-row">
            <div class="form-group">
              <label class="form-label" for="fmt-select">{$_('onboarding.step3.format_label')}</label>
              <select id="fmt-select" class="form-input form-select" bind:value={format}>
                <option value="m4a">M4A (AAC 192kbps - Best)</option>
                <option value="mp3">MP3 (320kbps)</option>
              </select>
            </div>

            <div class="form-group checkbox-wrap">
              <label class="form-checkbox">
                <input type="checkbox" bind:checked={generateM3u} />
                <span>{$_('onboarding.step3.m3u_label')}</span>
              </label>
            </div>
          </div>
        </div>

        <div class="step-footer">
          <button class="btn btn-secondary" on:click={prevStep}>← {$_('common.back')}</button>
          <button class="btn btn-primary" id="step3-start-btn" disabled={!outputFolder} on:click={handleStart}>
            {$_('onboarding.step3.start_btn')} →
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .wizard-container {
    max-width: 680px;
    margin: 0 auto;
    padding: var(--space-8) var(--space-4);
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .stepper {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-8);
    border-bottom: 1px solid var(--border-muted);
    padding-bottom: var(--space-4);
  }

  .step-node {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-family: var(--font-mono);
    font-size: 11px;
    color: #555555;
    letter-spacing: 0.08em;
  }

  .step-node.active {
    color: #ffffff;
    font-weight: 700;
  }

  .step-node.done {
    color: #888888;
  }

  .step-line {
    flex: 1;
    height: 1px;
    background: var(--border-muted);
    margin: 0 var(--space-3);
  }

  .step-line.filled {
    background: #555555;
  }

  .step-pane {
    background: #050505;
    border: 1px solid var(--border-muted);
    padding: var(--space-6);
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .step-header {
    border-bottom: 1px solid var(--border-muted);
    padding-bottom: var(--space-4);
  }

  .step-tag {
    font-size: 10px;
    color: #666666;
    letter-spacing: 0.12em;
    font-family: var(--font-mono);
    margin-bottom: var(--space-1);
  }

  .step-title {
    font-size: 1.4rem;
    font-weight: 700;
    color: #ffffff;
    font-family: var(--font-mono);
    letter-spacing: 0.04em;
    margin-bottom: var(--space-2);
  }

  .step-desc {
    font-size: 12px;
    color: #888888;
    line-height: 1.5;
  }

  .export-options {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: var(--space-4);
  }

  .export-card {
    border: 1px solid var(--border-muted);
    background: #000000;
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .export-header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .export-tag {
    font-size: 10px;
    color: #666666;
    font-family: var(--font-mono);
  }

  .export-card h4 {
    font-size: 11px;
    color: #ffffff;
    font-family: var(--font-mono);
  }

  .export-card p {
    font-size: 11px;
    color: #777777;
    flex: 1;
    line-height: 1.5;
  }

  .step-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-top: 1px solid var(--border-muted);
    padding-top: var(--space-4);
  }

  .preview-box {
    border: 1px solid var(--border-muted);
    background: #000000;
    padding: var(--space-3);
    font-family: var(--font-mono);
    font-size: 11px;
  }

  .preview-header {
    color: #888888;
    margin-bottom: var(--space-2);
    border-bottom: 1px solid #1a1a1a;
    padding-bottom: 4px;
  }

  .preview-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .preview-row {
    display: flex;
    gap: var(--space-2);
    color: #ffffff;
  }

  .p-num { color: #555555; }
  .p-title { font-weight: 700; flex: 1; }
  .p-artist { color: #888888; }

  .config-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .folder-input-row {
    display: flex;
    gap: var(--space-2);
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-4);
    align-items: flex-end;
  }

  .checkbox-wrap {
    padding-bottom: 10px;
  }
</style>
