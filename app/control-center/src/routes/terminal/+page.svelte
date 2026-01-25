<script lang="ts">
  import { page } from '$app/stores';
  import Terminal from '$lib/components/Terminal.svelte';

  // Helper to darken a hex color for background use
  function darkenColor(hex: string, factor: number = 0.85): string {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    const darkenedR = Math.round(r * (1 - factor));
    const darkenedG = Math.round(g * (1 - factor));
    const darkenedB = Math.round(b * (1 - factor));
    return `#${darkenedR.toString(16).padStart(2, '0')}${darkenedG.toString(16).padStart(2, '0')}${darkenedB.toString(16).padStart(2, '0')}`;
  }

  $: terminalId = $page.url.searchParams.get('id') || 'default';
  $: command = $page.url.searchParams.get('cmd') || undefined;
  $: themeColor = $page.url.searchParams.get('color') || '#FFAA00';
  $: hostname = $page.url.searchParams.get('host') || 'TERMINAL';
  $: backgroundColor = darkenColor(themeColor);
</script>

<main style="--theme-color: {themeColor}; --bg-color: {backgroundColor}">
  <div class="scanlines"></div>
  <div class="vignette"></div>

  <div class="terminal-chrome">
    <header>
      <span class="terminal-title">{hostname.toUpperCase()}</span>
      <div class="status">
        <span class="status-dot"></span>
        <span class="status-label">ACTIVE</span>
      </div>
    </header>
    <div class="terminal-body">
      <Terminal {terminalId} {command} {themeColor} />
    </div>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: #000000;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
  }

  main {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    box-sizing: border-box;
    position: relative;
    background: #000000;
  }

  .scanlines {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 1000;
    background: repeating-linear-gradient(
      0deg,
      rgba(0, 0, 0, 0.15) 0px,
      rgba(0, 0, 0, 0.15) 1px,
      transparent 1px,
      transparent 2px
    );
  }

  .vignette {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 999;
    background: radial-gradient(
      ellipse at center,
      transparent 50%,
      rgba(0, 0, 0, 0.4) 100%
    );
  }

  .terminal-chrome {
    flex: 1;
    max-width: 100%;
    max-height: 100%;
    display: flex;
    flex-direction: column;
    border: 1px solid var(--theme-color);
    background: var(--bg-color);
    position: relative;
    z-index: 1;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    border-bottom: 1px solid color-mix(in srgb, var(--theme-color) 30%, transparent);
    background: color-mix(in srgb, var(--theme-color) 10%, transparent);
  }

  .terminal-title {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 2px;
    color: var(--theme-color);
    text-shadow: 0 0 8px color-mix(in srgb, var(--theme-color) 50%, transparent);
  }

  .status {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #00FF41;
    box-shadow: 0 0 6px rgba(0, 255, 65, 0.6);
  }

  .status-label {
    font-size: 9px;
    letter-spacing: 1px;
    color: #00FF41;
    text-shadow: 0 0 6px rgba(0, 255, 65, 0.4);
  }

  .terminal-body {
    flex: 1;
    display: flex;
    min-height: 0;
  }
</style>
