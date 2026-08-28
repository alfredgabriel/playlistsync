<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { open } from '@tauri-apps/plugin-shell';
  import DropZone from './DropZone.svelte';

  export let onComplete: (csvPath: string, outputFolder: string) => void = () => {};

  let currentStep = 1;
  const TOTAL_STEPS = 3;

  // Step 2 state
  let csvPath: string = '';
  let csvPreview: { title: string; artist: string; album: string }[] = [];
  let trackCount: number = 0;
  let csvError: string = '';

  // Step 3 state
  let outputFolder: string = '';
  let format: 'm4a' | 'mp3' = 'm4a';
  let generateM3u = true;

  function openExportify() {
    open('https://exportify.net');
  }
  function openTuneMyMusic() {
    open('https://www.tunemymusic.com/transfer');
  }

  function nextStep() {
    if (currentStep < TOTAL_STEPS) currentStep++;
  }
  function prevStep() {
    if (currentStep > 1) currentStep--;
  }

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
</script>

<div class="onboarding-shell">
  <!-- Step indicator -->
  <div class="step-indicator">
    {#each Array(TOTAL_STEPS) as _, i}
      <div class="step-dot" class:active={currentStep === i + 1} class:done={currentStep > i + 1}>
        {#if currentStep > i + 1}
          <span>✓</span>
        {:else}
          <span>{i + 1}</span>
        {/if}
      </div>
      {#if i < TOTAL_STEPS - 1}
        <div class="step-line" class:filled={currentStep > i + 1} />
      {/if}
    {/each}
  </div>

  <!-- Steps content -->
  <div class="step-content">

    <!-- ===================== STEP 1 ===================== -->
    {#if currentStep === 1}
      <div class="step-panel animate-slide-up">
        <div class="step-icon-wrap">
          <span class="step-big-icon">📋</span>
        </div>
        <h2 class="step-title">{$_('onboarding.step1.title')}</h2>
        <p class="step-desc">{$_('onboarding.step1.description')}</p>

        <div class="export-options">
          <button class="export-card" id="btn-exportify" on:click={openExportify}>
            <div class="export-card-icon">
              <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
                <circle cx="16" cy="16" r="16" fill="#1DB954"/>
                <path d="M8 20.5c4.5-1.5 11.5-1 16 2" stroke="white" stroke-width="2" stroke-linecap="round"/>
                <path d="M9 16c5-2 13-1.5 18 1.5" stroke="white" stroke-width="2.2" stroke-linecap="round"/>
                <path d="M10.5 11.5c4.5-2 13.5-1.5 17 2" stroke="white" stroke-width="2.5" stroke-linecap="round"/>
              </svg>
            </div>
            <div class="export-card-text">
              <span class="export-card-title">Spotify</span>
              <span class="export-card-sub">via Exportify</span>
            </div>
            <span class="export-card-arrow">↗</span>
          </button>

          <button class="export-card" id="btn-tunemymusic" on:click={openTuneMyMusic}>
            <div class="export-card-icon multi">
              <span style="font-size:1.4rem">🎵</span>
            </div>
            <div class="export-card-text">
              <span class="export-card-title">Apple Music / YouTube Music</span>
              <span class="export-card-sub">via TuneMyMusic</span>
            </div>
            <span class="export-card-arrow">↗</span>
          </button>
        </div>

        <div class="tip-box">
          <span class="tip-icon">💡</span>
          <p class="tip-text">{$_('onboarding.step1.tip')}</p>
        </div>

        <div class="step-footer">
          <span />
          <button class="btn btn-primary" id="step1-next-btn" on:click={nextStep}>
            I have my CSV → Next
          </button>
        </div>
      </div>
    {/if}

    <!-- ===================== STEP 2 ===================== -->
    {#if currentStep === 2}
      <div class="step-panel animate-slide-up">
        <h2 class="step-title">{$_('onboarding.step2.title')}</h2>
        <p class="step-desc">{$_('onboarding.step2.description')}</p>

        <DropZone 
          bind:csvPath 
          bind:preview={csvPreview} 
          bind:error={csvError}
          on:loaded={handleCsvLoaded}
          on:error={handleCsvError}
        />

        {#if csvPreview.length > 0}
          <div class="preview-panel animate-fade-in">
            <div class="preview-header">
              <span class="preview-title">{$_('onboarding.step2.preview_title')}</span>
              <span class="badge badge-done">{$_('onboarding.step2.tracks_found', { values: { count: trackCount } })}</span>
            </div>
            <div class="preview-list">
              {#each csvPreview as track}
                <div class="preview-item">
                  <span class="preview-track-icon">🎵</span>
                  <div class="preview-track-info">
                    <span class="preview-track-title">{track.title}</span>
                    <span class="preview-track-sub">{track.artist} • {track.album}</span>
                  </div>
                </div>
              {/each}
              {#if trackCount > csvPreview.length}
                <div class="preview-more">...and {trackCount - csvPreview.length} more</div>
              {/if}
            </div>
          </div>
        {/if}

        <div class="step-footer">
          <button class="btn btn-ghost" on:click={prevStep}>
            ← Back
          </button>
          <button class="btn btn-primary" disabled={!csvPath || !!csvError} on:click={nextStep}>
            Continue →
          </button>
        </div>
      </div>
    {/if}

    <!-- ===================== STEP 3 ===================== -->
    {#if currentStep === 3}
      <div class="step-panel animate-slide-up">
        <slot name="step3" />
      </div>
    {/if}

  </div>
</div>

<style>
  .onboarding-shell {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  /* ---- Step indicator ---- */
  .step-indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0;
    padding: var(--space-6) var(--space-8);
    flex-shrink: 0;
  }

  .step-dot {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: var(--radius-full);
    border: 2px solid var(--border-muted);
    background: var(--bg-elevated);
    font-size: var(--text-sm);
    font-weight: var(--font-bold);
    color: var(--text-muted);
    transition: all var(--transition-normal);
    flex-shrink: 0;
  }
  .step-dot.active {
    border-color: var(--accent-primary);
    background: rgba(30, 215, 96, 0.15);
    color: var(--accent-primary);
    box-shadow: var(--shadow-glow-green);
  }
  .step-dot.done {
    border-color: var(--accent-primary);
    background: var(--accent-primary);
    color: #000;
  }

  .step-line {
    flex: 1;
    max-width: 80px;
    height: 2px;
    background: var(--border-muted);
    transition: background var(--transition-normal);
  }
  .step-line.filled { background: var(--accent-primary); }

  /* ---- Step content ---- */
  .step-content {
    flex: 1;
    overflow-y: auto;
    padding: 0 var(--space-8) var(--space-8);
  }

  .step-panel {
    max-width: 600px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .step-icon-wrap { text-align: center; }
  .step-big-icon  { font-size: 3.5rem; }

  .step-title {
    text-align: center;
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--text-primary);
    letter-spacing: -0.03em;
  }

  .step-desc {
    text-align: center;
    font-size: var(--text-base);
    color: var(--text-secondary);
    line-height: 1.6;
  }

  /* ---- Export cards ---- */
  .export-options {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .export-card {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-4) var(--space-5);
    background: var(--bg-glass);
    border: 1px solid var(--border-muted);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: all var(--transition-fast);
    text-align: left;
    width: 100%;
  }
  .export-card:hover {
    border-color: var(--border-accent);
    background: var(--bg-glass-hover);
    transform: translateY(-1px);
    box-shadow: var(--shadow-glow-green);
  }

  .export-card-icon {
    width: 48px;
    height: 48px;
    border-radius: var(--radius-md);
    background: var(--bg-elevated);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }
  .export-card-icon.multi { font-size: 1.5rem; }

  .export-card-text {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .export-card-title { font-size: var(--text-base); font-weight: var(--font-semibold); color: var(--text-primary); }
  .export-card-sub   { font-size: var(--text-sm);   color: var(--text-muted); }
  .export-card-arrow { font-size: var(--text-xl); color: var(--text-muted); }

  /* ---- Tip box ---- */
  .tip-box {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    background: rgba(124, 58, 237, 0.08);
    border: 1px solid rgba(124, 58, 237, 0.2);
    border-radius: var(--radius-md);
    padding: var(--space-4);
  }
  .tip-icon { font-size: var(--text-xl); flex-shrink: 0; }
  .tip-text  { font-size: var(--text-sm); color: var(--text-secondary); line-height: 1.5; }

  /* ---- Footer ---- */
  .step-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: var(--space-2);
  }

  /* ---- Preview panel ---- */
  .preview-panel {
    background: var(--bg-elevated);
    border: 1px solid var(--border-muted);
    border-radius: var(--radius-lg);
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border-subtle);
    padding-bottom: var(--space-2);
  }
  .preview-title { font-size: var(--text-sm); font-weight: var(--font-semibold); color: var(--text-secondary); }
  .preview-list { display: flex; flex-direction: column; gap: var(--space-2); }
  .preview-item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2);
    border-radius: var(--radius-md);
    background: var(--bg-glass);
  }
  .preview-track-icon { font-size: var(--text-base); }
  .preview-track-info { display: flex; flex-direction: column; overflow: hidden; }
  .preview-track-title { font-size: var(--text-sm); font-weight: var(--font-medium); color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .preview-track-sub { font-size: var(--text-xs); color: var(--text-muted); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .preview-more { text-align: center; font-size: var(--text-xs); color: var(--text-muted); padding-top: var(--space-2); font-style: italic; }
</style>
