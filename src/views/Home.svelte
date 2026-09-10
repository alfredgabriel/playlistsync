<script lang="ts">
  import { _ } from 'svelte-i18n';
  export let onNavigate: (view: string) => void = () => {};
</script>

<div class="home-view animate-fade-in">
  <div class="home-hero">
    <div class="hero-tag">// AUDIO PIPELINE · LOCAL SIDECAR ENGINE</div>
    <h2 class="hero-title">{$_('app.tagline').toUpperCase()}</h2>
    <p class="hero-sub">{$_('home.hero_sub')}</p>
    <button class="btn btn-primary btn-lg" id="home-start-btn" on:click={() => onNavigate('download')}>
      {$_('home.get_started')} →
    </button>
  </div>

  <div class="feature-grid">
    {#each [
      { tag: '[01]', titleKey: 'home.feature1_title', descKey: 'home.feature1_desc' },
      { tag: '[02]', titleKey: 'home.feature2_title', descKey: 'home.feature2_desc' },
      { tag: '[03]', titleKey: 'home.feature3_title', descKey: 'home.feature3_desc' },
      { tag: '[04]', titleKey: 'home.feature4_title', descKey: 'home.feature4_desc' },
    ] as feature}
      <div class="feature-card card">
        <div class="feature-header">
          <span class="feature-tag">{feature.tag}</span>
          <h3 class="feature-title">{$_(feature.titleKey).toUpperCase()}</h3>
        </div>
        <p class="feature-desc">{$_(feature.descKey)}</p>
      </div>
    {/each}
  </div>
</div>

<style>
  .home-view { padding: var(--space-8); overflow-y: auto; height: 100%; background: #000000; }

  .home-hero {
    text-align: left;
    padding: var(--space-10) var(--space-6);
    margin-bottom: var(--space-8);
    border: 1px solid var(--border-muted);
    background: #050505;
  }

  .hero-tag {
    font-size: 10px;
    letter-spacing: 0.15em;
    color: #666666;
    margin-bottom: var(--space-3);
    font-family: var(--font-mono);
  }

  .hero-title {
    font-size: 1.8rem;
    font-weight: 700;
    letter-spacing: 0.04em;
    color: #ffffff;
    margin-bottom: var(--space-3);
    line-height: 1.2;
    font-family: var(--font-mono);
  }

  .hero-sub {
    font-size: var(--text-sm);
    color: var(--text-secondary);
    margin-bottom: var(--space-6);
    max-width: 600px;
    line-height: 1.6;
  }

  .feature-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-4);
  }

  .feature-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-5);
    background: #000000;
    border: 1px solid var(--border-muted);
    transition: border-color 0.1s ease;
  }
  .feature-card:hover { border-color: #ffffff; }

  .feature-header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-bottom: var(--space-1);
  }

  .feature-tag {
    font-size: 10px;
    color: #666666;
    font-family: var(--font-mono);
    font-weight: 700;
  }

  .feature-title {
    font-size: 12px;
    font-weight: 700;
    color: #ffffff;
    letter-spacing: 0.06em;
    font-family: var(--font-mono);
  }
  
  .feature-desc {
    font-size: 11px;
    color: #777777;
    line-height: 1.6;
  }

  @media (max-width: 768px) {
    .feature-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
