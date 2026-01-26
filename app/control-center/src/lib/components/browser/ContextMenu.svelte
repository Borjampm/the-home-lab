<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';

  interface FileEntry {
    name: string;
    path: string;
    isDir: boolean;
  }

  export let x: number = 0;
  export let y: number = 0;
  export let visible: boolean = false;
  export let file: FileEntry | null = null;
  export let selectedCount: number = 1;
  export let themeColor: string = '#FFAA00';

  const dispatch = createEventDispatcher();

  let menuEl: HTMLDivElement;

  onMount(() => {
    document.addEventListener('click', handleClickOutside);
    document.addEventListener('contextmenu', handleClickOutside);
  });

  onDestroy(() => {
    document.removeEventListener('click', handleClickOutside);
    document.removeEventListener('contextmenu', handleClickOutside);
  });

  function handleClickOutside(e: Event) {
    if (visible && menuEl && !menuEl.contains(e.target as Node)) {
      close();
    }
  }

  function close() {
    visible = false;
    dispatch('close');
  }

  function action(name: string) {
    dispatch('action', { action: name, file });
    close();
  }

  // Adjust position to stay within viewport
  $: if (visible && menuEl) {
    const rect = menuEl.getBoundingClientRect();
    if (x + rect.width > window.innerWidth) {
      x = window.innerWidth - rect.width - 10;
    }
    if (y + rect.height > window.innerHeight) {
      y = window.innerHeight - rect.height - 10;
    }
  }
</script>

{#if visible}
  <div
    bind:this={menuEl}
    class="context-menu"
    style="left: {x}px; top: {y}px; --theme-color: {themeColor}"
    role="menu"
  >
    {#if file}
      <button class="menu-item" on:click={() => action('open')} role="menuitem">
        <span class="item-icon">&gt;</span>
        <span class="item-label">OPEN</span>
      </button>

      {#if file.isDir}
        <button class="menu-item" on:click={() => action('openTerminal')} role="menuitem">
          <span class="item-icon">$</span>
          <span class="item-label">OPEN IN TERMINAL</span>
        </button>
      {/if}

      <div class="menu-divider"></div>

      <button class="menu-item" on:click={() => action('rename')} role="menuitem">
        <span class="item-icon">~</span>
        <span class="item-label">RENAME</span>
      </button>

      <button class="menu-item destructive" on:click={() => action('delete')} role="menuitem">
        <span class="item-icon">x</span>
        <span class="item-label">
          DELETE{#if selectedCount > 1} ({selectedCount}){/if}
        </span>
      </button>

      <div class="menu-divider"></div>

      <button class="menu-item" on:click={() => action('copyPath')} role="menuitem">
        <span class="item-icon">#</span>
        <span class="item-label">COPY PATH</span>
      </button>

      {#if file.isDir}
        <button class="menu-item" on:click={() => action('bookmark')} role="menuitem">
          <span class="item-icon">+</span>
          <span class="item-label">ADD TO BOOKMARKS</span>
        </button>
      {/if}

      {#if !file.isDir}
        <button class="menu-item" on:click={() => action('quicklook')} role="menuitem">
          <span class="item-icon">?</span>
          <span class="item-label">QUICK LOOK</span>
          <span class="item-shortcut">SPACE</span>
        </button>
      {/if}

      <div class="menu-divider"></div>

      <button class="menu-item" on:click={() => action('download')} role="menuitem">
        <span class="item-icon">v</span>
        <span class="item-label">
          DOWNLOAD{#if selectedCount > 1} ({selectedCount}){/if}
        </span>
      </button>
    {:else}
      <button class="menu-item" on:click={() => action('newFolder')} role="menuitem">
        <span class="item-icon">[+]</span>
        <span class="item-label">NEW FOLDER</span>
      </button>

      <button class="menu-item" on:click={() => action('upload')} role="menuitem">
        <span class="item-icon">^</span>
        <span class="item-label">UPLOAD FILES</span>
      </button>

      <div class="menu-divider"></div>

      <button class="menu-item" on:click={() => action('refresh')} role="menuitem">
        <span class="item-icon">@</span>
        <span class="item-label">REFRESH</span>
      </button>
    {/if}
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    z-index: 1000;
    background: #1e1e2e;
    border: 1px solid rgba(255, 170, 0, 0.4);
    min-width: 180px;
    padding: 4px 0;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    background: transparent;
    border: none;
    color: #cdd6f4;
    font-family: inherit;
    font-size: 11px;
    letter-spacing: 1px;
    cursor: pointer;
    text-align: left;
    transition: background 0.1s ease;
  }

  .menu-item:hover {
    background: color-mix(in srgb, var(--theme-color) 20%, transparent);
  }

  .menu-item.destructive {
    color: #f38ba8;
  }

  .menu-item.destructive:hover {
    background: rgba(243, 139, 168, 0.2);
  }

  .item-icon {
    font-family: monospace;
    font-size: 10px;
    color: var(--theme-color);
    width: 16px;
    text-align: center;
  }

  .menu-item.destructive .item-icon {
    color: #f38ba8;
  }

  .item-label {
    flex: 1;
  }

  .item-shortcut {
    font-size: 9px;
    color: #6c7086;
    padding: 2px 4px;
    background: rgba(255, 255, 255, 0.05);
  }

  .menu-divider {
    height: 1px;
    background: rgba(255, 170, 0, 0.2);
    margin: 4px 8px;
  }
</style>
