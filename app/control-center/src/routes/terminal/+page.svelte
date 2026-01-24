<script lang="ts">
  import { page } from '$app/stores';
  import Terminal from '$lib/components/Terminal.svelte';

  $: terminalId = $page.url.searchParams.get('id') || 'default';
  $: command = $page.url.searchParams.get('cmd') || undefined;
</script>

<main>
  <Terminal {terminalId} {command} />
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: #1e1e2e;
  }

  main {
    width: 100vw;
    height: 100vh;
    display: flex;
    background: #1e1e2e;
  }
</style>
