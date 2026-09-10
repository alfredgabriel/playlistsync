<script lang="ts">
  import { _ } from 'svelte-i18n';
  import logoUrl from '../assets/logo.png';

  export let activeView: string;

  const navItems = [
    { id: 'home',     num: '01', labelKey: 'nav.home' },
    { id: 'download', num: '02', labelKey: 'nav.download' },
    { id: 'history',  num: '03', labelKey: 'nav.history' },
    { id: 'settings', num: '04', labelKey: 'nav.settings' },
  ];
</script>

<aside class="sidebar">
  <div class="sidebar-logo">
    <div class="logo-icon">
      <img src={logoUrl} alt="PlaylistSync" style="width: 100%; height: 100%; object-fit: contain; filter: grayscale(100%) contrast(200%);" />
    </div>
    <div class="logo-text">
      PLAYLIST<span class="logo-strike">SYNC</span>
    </div>
  </div>

  <nav class="sidebar-nav">
    {#each navItems as item}
      <button
        class="nav-item"
        class:active={activeView === item.id}
        on:click={() => activeView = item.id}
        aria-label={$_(item.labelKey)}
      >
        <span class="nav-num">[{item.num}]</span>
        <span class="nav-label">{$_(item.labelKey)}</span>
      </button>
    {/each}
  </nav>

  <div class="sidebar-footer">
    <span class="version-badge">SYS // V1.0.0</span>
  </div>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    height: 100vh;
    background: #000000;
    border-right: 1px solid var(--border-muted);
    display: flex;
    flex-direction: column;
    padding: var(--space-6) var(--space-4);
    flex-shrink: 0;
  }

  .sidebar-logo {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 0 var(--space-2);
    margin-bottom: var(--space-8);
  }

  .logo-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border: 1px solid #333333;
    overflow: hidden;
    flex-shrink: 0;
    background: #0a0a0a;
  }

  .logo-text {
    font-size: 13px;
    font-weight: 700;
    color: #ffffff;
    letter-spacing: 0.12em;
    font-family: var(--font-mono);
  }

  .logo-strike {
    text-decoration: line-through;
    color: #888888;
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
  }

  .nav-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    padding: 10px 12px;
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: #888888;
    background: transparent;
    border: 1px solid transparent;
    cursor: pointer;
    transition: all 0.1s ease;
    text-align: left;
    font-family: var(--font-sans);
  }

  .nav-item:hover {
    color: #ffffff;
    border-color: #333333;
    background: #0a0a0a;
  }

  .nav-item.active {
    color: #000000;
    background: #ffffff;
    border-color: #ffffff;
    font-weight: 700;
  }

  .nav-num {
    font-size: 10px;
    opacity: 0.7;
    font-family: var(--font-mono);
  }

  .nav-label {
    flex: 1;
  }

  .sidebar-footer {
    display: flex;
    justify-content: center;
    padding-top: var(--space-4);
    border-top: 1px solid var(--border-muted);
  }

  .version-badge {
    font-size: 10px;
    letter-spacing: 0.1em;
    color: #555555;
    font-family: var(--font-mono);
  }
</style>
