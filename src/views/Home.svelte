<script lang="ts">
  import { _ } from 'svelte-i18n';
  export let onNavigate: (view: string) => void = () => {};
</script>

<div class="home-view animate-fade-in">
  <div class="home-hero">
    <div class="hero-glow" />
    <h2 class="hero-title">{$_('app.tagline')}</h2>
    <p class="hero-sub">{$_('home.hero_sub')}</p>
    <button class="btn btn-primary btn-lg" id="home-start-btn" on:click={() => onNavigate('download')}>
      {$_('home.get_started')}
    </button>
  </div>

  <div class="feature-grid">
    {#each [
      { icon: '📂', titleKey: 'home.feature1_title', descKey: 'home.feature1_desc' },
      { icon: '🎵', titleKey: 'home.feature2_title', descKey: 'home.feature2_desc' },
      { icon: '🏷️', titleKey: 'home.feature3_title', descKey: 'home.feature3_desc' },
      { icon: '🌍', titleKey: 'home.feature4_title', descKey: 'home.feature4_desc' },
    ] as feature}
      <div class="feature-card card">
        <span class="feature-icon">{feature.icon}</span>
        <h3 class="feature-title">{$_(feature.titleKey)}</h3>
        <p class="feature-desc">{$_(feature.descKey)}</p>
      </div>
    {/each}
  </div>
</div>

<style>
  .home-view { padding: var(--space-8); overflow-y: auto; height: 100%; }

  .home-hero {
    position: relative;
    text-align: center;
    padding: var(--space-12) var(--space-8);
    margin-bottom: var(--space-8);
  }

  .hero-glow {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 400px;
    height: 200px;
    background: radial-gradient(ellipse, rgba(30,215,96,0.12) 0%, transparent 70%);
    pointer-events: none;
  }

  .hero-title {
    font-size: var(--text-4xl);
    font-weight: var(--font-extrabold);
    letter-spacing: -0.04em;
    background: var(--accent-gradient);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: var(--space-4);
  }

  .hero-sub {
    font-size: var(--text-lg);
    color: var(--text-secondary);
    margin-bottom: var(--space-8);
    max-width: 480px;
    margin-left: auto;
    margin-right: auto;
  }

  .feature-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-4);
    max-width: 640px;
    margin: 0 auto;
  }

  .feature-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-5);
    transition: border-color var(--transition-fast), transform var(--transition-fast);
  }
  .feature-card:hover { border-color: var(--border-accent); transform: translateY(-2px); }

  .feature-icon { font-size: var(--text-2xl); }
  .feature-title { font-size: var(--text-base); font-weight: var(--font-semibold); color: var(--text-primary); }
  .feature-desc  { font-size: var(--text-sm);  color: var(--text-secondary); line-height: 1.5; }
</style>
