<script lang="ts">
  import { _ } from 'svelte-i18n';
</script>
<div class="view-placeholder animate-fade-in">
  <span class="placeholder-icon">🕐</span>
  <h2>{$_('nav.history')}</h2>
  <p class="text-secondary">Coming soon — session history and folder access.</p>
</div>
<style>
  .view-placeholder { display:flex; flex-direction:column; align-items:center; justify-content:center; height:100%; gap:var(--space-4); text-align:center; padding:var(--space-8); }
  .placeholder-icon { font-size:4rem; }
  h2 { font-size:var(--text-2xl); font-weight:var(--font-bold); }
</style>
