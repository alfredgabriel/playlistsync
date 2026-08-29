<script lang="ts">
  import { _ } from 'svelte-i18n';
  import logoUrl from '../assets/logo.png';

  export let activeView: string;

  const navItems = [
    { id: 'home',     icon: '🏠', labelKey: 'nav.home' },
    { id: 'download', icon: '⬇️', labelKey: 'nav.download' },
    { id: 'history',  icon: '🕐', labelKey: 'nav.history' },
    { id: 'settings', icon: '⚙️', labelKey: 'nav.settings' },
  ];
</script>

<aside class="sidebar">
  <div class="sidebar-logo">
    <div class="logo-icon">
      <img src={logoUrl} alt="PlaylistSync Logo" style="width: 100%; height: 100%; border-radius: inherit; object-fit: cover;" />
    </div>
    <span class="logo-text">PlaylistSync</span>
  </div>

  <nav class="sidebar-nav">
    {#each navItems as item}
      <button
        class="nav-item"
        class:active={activeView === item.id}
        on:click={() => activeView = item.id}
        aria-label={$_(item.labelKey)}
      >
        <span class="nav-icon">{item.icon}</span>
        <span class="nav-label">{$_(item.labelKey)}</span>
        {#if activeView === item.id}
          <span class="nav-indicator" />
        {/if}
      </button>
    {/each}
  </nav>

  <div class="sidebar-footer">
    <span class="version-badge">v1.0.0</span>
  </div>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    height: 100vh;
    background: var(--bg-surface);
    border-right: 1px solid var(--border-subtle);
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
    border-radius: var(--radius-md);
    background: var(--bg-elevated);
    box-shadow: var(--shadow-glow-green);
    flex-shrink: 0;
  }

  .logo-text {
    font-size: var(--text-base);
    font-weight: var(--font-bold);
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    flex: 1;
  }

  .nav-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--text-muted);
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all var(--transition-fast);
    text-align: left;
  }

  .nav-item:hover {
    color: var(--text-primary);
    background: var(--bg-glass);
  }

  .nav-item.active {
    color: var(--text-primary);
    background: var(--bg-glass-hover);
    font-weight: var(--font-semibold);
  }

  .nav-icon { font-size: var(--text-lg); line-height: 1; }
  .nav-label { flex: 1; }

  .nav-indicator {
    position: absolute;
    right: 0;
    top: 50%;
    transform: translateY(-50%);
    width: 3px;
    height: 60%;
    background: var(--accent-gradient);
    border-radius: var(--radius-full) 0 0 var(--radius-full);
  }

  .sidebar-footer {
    display: flex;
    justify-content: center;
    padding-top: var(--space-4);
    border-top: 1px solid var(--border-subtle);
  }

  .version-badge {
    font-size: var(--text-xs);
    color: var(--text-muted);
    background: var(--bg-elevated);
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-full);
  }
</style>
