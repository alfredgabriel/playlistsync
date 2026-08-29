<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { setLocale, SUPPORTED_LOCALES } from '$lib/i18n/index';
  import { locale } from 'svelte-i18n';

  export let title: string = '';

  function handleLocaleChange(e: Event) {
    const code = (e.target as HTMLSelectElement).value;
    locale.set(code);
    setLocale(code);
  }
</script>

<header class="app-header">
  <div class="header-left">
    {#if title}
      <h1 class="header-title">{title}</h1>
    {/if}
  </div>
  <div class="header-right">
    <div class="locale-picker">
      <span class="locale-icon">🌐</span>
      <select
        class="form-input form-select locale-select"
        value={$locale?.split('-')[0] ?? 'en'}
        on:change={handleLocaleChange}
        aria-label={$_('settings.language')}
        id="language-selector"
      >
        {#each SUPPORTED_LOCALES as loc}
          <option value={loc.code}>{loc.label}</option>
        {/each}
      </select>
    </div>
  </div>
</header>

<style>
  .app-header {
    height: var(--header-height);
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-6);
    flex-shrink: 0;
  }

  .header-title {
    font-size: var(--text-xl);
    font-weight: var(--font-bold);
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .header-right { display: flex; align-items: center; gap: var(--space-4); }

  .locale-picker {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .locale-icon { font-size: var(--text-base); }

  .locale-select {
    width: auto;
    min-width: 110px;
    background: var(--bg-elevated);
    border: 1px solid var(--border-subtle);
    color: var(--text-secondary);
    font-size: var(--text-sm);
    padding: var(--space-2) var(--space-6) var(--space-2) var(--space-3);
    cursor: pointer;
  }
</style>
