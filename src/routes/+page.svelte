<script lang="ts">
  import { onMount } from 'svelte';
  import { _, isLoading } from 'svelte-i18n';
  import '../styles/global.css';
  import '../lib/i18n/index';

  import Sidebar from '../components/Sidebar.svelte';
  import Header  from '../components/Header.svelte';
  import ToolCheckBanner from '../components/ToolCheckBanner.svelte';
  import Home     from '../views/Home.svelte';
  import Download from '../views/Download.svelte';
  import History  from '../views/History.svelte';
  import Settings from '../views/Settings.svelte';

  let activeView = 'home';
  let ready = false;

  const VIEW_TITLES: Record<string, string> = {
    home:     '',
    download: 'Download',
    history:  'History',
    settings: 'Settings',
  };

  onMount(() => { ready = true; });
</script>

{#if ready && !$isLoading}
  <div class="app-root animate-fade-in">
    <Sidebar bind:activeView />

    <div class="app-main">
      <ToolCheckBanner />
      <Header title={$_(VIEW_TITLES[activeView] ? `nav.${activeView}` : '')} />

      <main class="app-content">
        {#if activeView === 'home'}
          <Home onNavigate={(v) => (activeView = v)} />
        {:else if activeView === 'download'}
          <Download />
        {:else if activeView === 'history'}
          <History />
        {:else if activeView === 'settings'}
          <Settings />
        {/if}
      </main>
    </div>
  </div>
{/if}

<style>
  :global(body) { overflow: hidden; }

  .app-root {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background: var(--bg-base);
  }

  .app-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
    overflow: hidden;
  }

  .app-content {
    flex: 1;
    overflow: hidden;
    position: relative;
  }
</style>
