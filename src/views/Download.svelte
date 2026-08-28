<script lang="ts">
  import { _ } from 'svelte-i18n';
  import OnboardingWizard from '../components/OnboardingWizard.svelte';

  let sessionStarted = false;
  let csvPath = '';
  let outputFolder = '';

  function handleComplete(csv: string, folder: string) {
    csvPath = csv;
    outputFolder = folder;
    sessionStarted = true;
  }
</script>

{#if !sessionStarted}
  <OnboardingWizard onComplete={handleComplete} />
{:else}
  <div class="download-active animate-fade-in">
    <p class="text-secondary">Download session started for: {csvPath}</p>
  </div>
{/if}

<style>
  .download-active {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: var(--space-8);
  }
</style>
