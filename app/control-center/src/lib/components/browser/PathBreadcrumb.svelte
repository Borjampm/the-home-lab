<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let path: string = '/';
  export let canGoBack: boolean = false;
  export let canGoForward: boolean = false;
  export let themeColor: string = '#FFAA00';

  const dispatch = createEventDispatcher();

  $: pathParts = getPathParts(path);

  function getPathParts(p: string): { name: string; path: string }[] {
    const parts = p.split('/').filter(Boolean);
    const result: { name: string; path: string }[] = [{ name: '/', path: '/' }];

    let currentPath = '';
    for (const part of parts) {
      currentPath += '/' + part;
      result.push({ name: part, path: currentPath });
    }

    return result;
  }

  function navigateTo(targetPath: string) {
    dispatch('navigate', { path: targetPath });
  }

  function goBack() {
    dispatch('back');
  }

  function goForward() {
    dispatch('forward');
  }

  function goUp() {
    dispatch('up');
  }
</script>

<div class="path-breadcrumb" style="--theme-color: {themeColor}">
  <div class="nav-buttons">
    <button
      class="nav-btn"
      on:click={goBack}
      disabled={!canGoBack}
      title="Back"
    >
      &lt;
    </button>
    <button
      class="nav-btn"
      on:click={goForward}
      disabled={!canGoForward}
      title="Forward"
    >
      &gt;
    </button>
    <button
      class="nav-btn"
      on:click={goUp}
      disabled={pathParts.length <= 1}
      title="Up"
    >
      ^
    </button>
  </div>

  <div class="breadcrumb-path">
    {#each pathParts as part, i}
      {#if i > 0}
        <span class="separator">/</span>
      {/if}
      <button
        class="path-segment"
        class:current={i === pathParts.length - 1}
        on:click={() => navigateTo(part.path)}
      >
        {part.name === '/' ? 'ROOT' : part.name.toUpperCase()}
      </button>
    {/each}
  </div>
</div>

<style>
  .path-breadcrumb {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.4);
    border-bottom: 1px solid rgba(255, 170, 0, 0.2);
  }

  .nav-buttons {
    display: flex;
    gap: 4px;
  }

  .nav-btn {
    background: transparent;
    border: 1px solid rgba(255, 170, 0, 0.3);
    color: var(--theme-color);
    width: 24px;
    height: 24px;
    font-family: monospace;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .nav-btn:hover:not(:disabled) {
    background: rgba(255, 170, 0, 0.15);
    border-color: var(--theme-color);
  }

  .nav-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .breadcrumb-path {
    display: flex;
    align-items: center;
    gap: 4px;
    flex: 1;
    overflow-x: auto;
    scrollbar-width: none;
  }

  .breadcrumb-path::-webkit-scrollbar {
    display: none;
  }

  .separator {
    color: #6c7086;
    font-size: 11px;
  }

  .path-segment {
    background: transparent;
    border: none;
    color: var(--theme-color);
    font-family: inherit;
    font-size: 11px;
    letter-spacing: 1px;
    padding: 2px 6px;
    cursor: pointer;
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .path-segment:hover {
    background: rgba(255, 170, 0, 0.15);
  }

  .path-segment.current {
    color: #cdd6f4;
    cursor: default;
  }

  .path-segment.current:hover {
    background: transparent;
  }
</style>
