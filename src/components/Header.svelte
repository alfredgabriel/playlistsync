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
      <h1 class="header-title">// {title.toUpperCase()}</h1>
    {/if}
  </div>
  <div class="header-right">
    <div class="locale-picker">
      <span class="locale-tag">[LANG]</span>
      <select
        class="form-input form-select locale-select"
        value={$locale?.split('-')[0] ?? 'en'}
        on:change={handleLocaleChange}
        aria-label={$_('settings.language')}
        id="language-selector"
      >
        {#each SUPPORTED_LOCALES as loc}
          <option value={loc.code}>{loc.label.toUpperCase()}</option>
        {/each}
      </select>
    </div>
  </div>
</header>

<style>
  .app-header {
    height: var(--header-height);
    background: #000000;
    border-bottom: 1px solid var(--border-muted);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--space-6);
    flex-shrink: 0;
  }

  .header-title {
    font-size: 13px;
    font-weight: 700;
    color: #ffffff;
    letter-spacing: 0.1em;
    font-family: var(--font-mono);
  }

  .header-right { display: flex; align-items: center; gap: var(--space-4); }

  .locale-picker {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .locale-tag {
    font-size: 10px;
    color: #666666;
    letter-spacing: 0.08em;
    font-family: var(--font-mono);
  }

  .locale-select {
    width: auto;
    min-width: 90px;
    background: #000000;
    border: 1px solid var(--border-muted);
    color: #ffffff;
    font-size: 11px;
    letter-spacing: 0.08em;
    padding: 4px 24px 4px 8px;
    cursor: pointer;
    font-family: var(--font-sans);
  }
  .locale-select:focus {
    border-color: #ffffff;
  }
</style>
