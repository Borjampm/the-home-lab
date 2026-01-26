<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let splitEnabled: boolean = false;
  export let themeColor: string = '#FFAA00';

  const dispatch = createEventDispatcher();

  let dividerPosition = 50; // percentage
  let isDragging = false;
  let containerEl: HTMLDivElement;

  function toggleSplit() {
    splitEnabled = !splitEnabled;
    dispatch('toggle', { splitEnabled });
  }

  function startDrag(e: MouseEvent) {
    if (!splitEnabled) return;
    isDragging = true;
    document.addEventListener('mousemove', handleDrag);
    document.addEventListener('mouseup', stopDrag);
  }

  function handleDrag(e: MouseEvent) {
    if (!isDragging || !containerEl) return;
    const rect = containerEl.getBoundingClientRect();
    const newPosition = ((e.clientX - rect.left) / rect.width) * 100;
    dividerPosition = Math.min(80, Math.max(20, newPosition));
  }

  function stopDrag() {
    isDragging = false;
    document.removeEventListener('mousemove', handleDrag);
    document.removeEventListener('mouseup', stopDrag);
  }
</script>

<div
  bind:this={containerEl}
  class="dual-pane"
  class:split={splitEnabled}
  style="--theme-color: {themeColor}"
>
  <div class="pane left-pane" style={splitEnabled ? `width: ${dividerPosition}%` : 'width: 100%'}>
    <slot name="left" />
  </div>

  {#if splitEnabled}
    <div
      class="divider"
      on:mousedown={startDrag}
      class:dragging={isDragging}
      role="separator"
      aria-orientation="vertical"
    >
      <div class="divider-line"></div>
    </div>

    <div class="pane right-pane" style="width: {100 - dividerPosition}%">
      <slot name="right" />
    </div>
  {/if}

  <button
    class="split-toggle"
    on:click={toggleSplit}
    title={splitEnabled ? 'Close split view' : 'Open split view'}
  >
    {splitEnabled ? '[ | ]' : '[   ]'}
  </button>
</div>

<style>
  .dual-pane {
    display: flex;
    flex: 1;
    position: relative;
    min-height: 0;
    overflow: hidden;
  }

  .pane {
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }

  .left-pane {
    border-right: none;
  }

  .split .left-pane {
    border-right: 1px solid rgba(255, 170, 0, 0.2);
  }

  .right-pane {
    background: rgba(0, 0, 0, 0.1);
  }

  .divider {
    width: 8px;
    cursor: col-resize;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    flex-shrink: 0;
    margin: 0 -4px;
    z-index: 10;
  }

  .divider:hover,
  .divider.dragging {
    background: rgba(255, 170, 0, 0.1);
  }

  .divider-line {
    width: 2px;
    height: 40px;
    background: rgba(255, 170, 0, 0.3);
    border-radius: 1px;
  }

  .divider:hover .divider-line,
  .divider.dragging .divider-line {
    background: var(--theme-color);
  }

  .split-toggle {
    position: absolute;
    bottom: 8px;
    right: 8px;
    background: rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 170, 0, 0.3);
    color: var(--theme-color);
    font-family: monospace;
    font-size: 10px;
    padding: 4px 8px;
    cursor: pointer;
    z-index: 20;
    transition: all 0.15s ease;
  }

  .split-toggle:hover {
    background: rgba(255, 170, 0, 0.15);
    border-color: var(--theme-color);
  }
</style>
