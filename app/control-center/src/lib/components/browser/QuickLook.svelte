<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';

  interface FileEntry {
    name: string;
    path: string;
    isDir: boolean;
    size: number;
    modified: number | null;
  }

  export let visible: boolean = false;
  export let file: FileEntry | null = null;
  export let content: string | null = null;
  export let loading: boolean = false;
  export let error: string | null = null;
  export let themeColor: string = '#FFAA00';

  const dispatch = createEventDispatcher();

  $: fileExt = file?.name.split('.').pop()?.toLowerCase() || '';
  $: isImage = ['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp', 'ico'].includes(fileExt);
  $: isCode = ['js', 'ts', 'py', 'rs', 'go', 'java', 'c', 'cpp', 'h', 'svelte', 'vue', 'jsx', 'tsx', 'css', 'scss', 'html', 'json', 'yaml', 'yml', 'toml', 'xml', 'sh', 'bash', 'zsh'].includes(fileExt);
  $: isMarkdown = ['md', 'mdx'].includes(fileExt);
  $: isText = isCode || isMarkdown || ['txt', 'log', 'ini', 'conf', 'env'].includes(fileExt);

  function formatSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB'];
    let i = 0;
    while (bytes >= 1024 && i < units.length - 1) {
      bytes /= 1024;
      i++;
    }
    return `${bytes.toFixed(1)} ${units[i]}`;
  }

  function formatDate(timestamp: number | null): string {
    if (!timestamp) return '-';
    return new Date(timestamp * 1000).toLocaleString();
  }

  function close() {
    visible = false;
    dispatch('close');
  }

  function handleKeydown(e: KeyboardEvent) {
    if (visible) {
      if (e.key === 'Escape' || e.key === ' ') {
        e.preventDefault();
        close();
      }
    }
  }

  onMount(() => {
    document.addEventListener('keydown', handleKeydown);
  });

  onDestroy(() => {
    document.removeEventListener('keydown', handleKeydown);
  });
</script>

{#if visible && file}
  <div class="quicklook-overlay" on:click={close} on:keydown={handleKeydown} role="dialog" aria-modal="true">
    <div class="quicklook-modal" style="--theme-color: {themeColor}" on:click|stopPropagation role="document">
      <header class="modal-header">
        <div class="file-info">
          <span class="file-name">{file.name.toUpperCase()}</span>
          <span class="file-meta">{formatSize(file.size)} | {formatDate(file.modified)}</span>
        </div>
        <button class="close-btn" on:click={close}>X</button>
      </header>

      <div class="modal-content">
        {#if loading}
          <div class="loading-state">LOADING...</div>
        {:else if error}
          <div class="error-state">{error}</div>
        {:else if isImage}
          <div class="image-preview">
            <div class="image-placeholder">
              <span class="placeholder-icon">[IMAGE]</span>
              <span class="placeholder-text">IMAGE PREVIEW</span>
              <span class="placeholder-note">{file.name}</span>
            </div>
          </div>
        {:else if content !== null}
          <pre class="text-content" class:code={isCode}>{content}</pre>
        {:else}
          <div class="binary-state">
            <span class="binary-icon">[?]</span>
            <span class="binary-text">BINARY FILE</span>
            <span class="binary-note">Cannot preview this file type</span>
          </div>
        {/if}
      </div>

      <footer class="modal-footer">
        <span class="path-display">{file.path}</span>
        <span class="hint">PRESS SPACE OR ESC TO CLOSE</span>
      </footer>
    </div>
  </div>
{/if}

<style>
  .quicklook-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
    padding: 40px;
  }

  .quicklook-modal {
    background: #1e1e2e;
    border: 1px solid var(--theme-color);
    max-width: 800px;
    max-height: 80vh;
    width: 100%;
    display: flex;
    flex-direction: column;
    box-shadow: 0 0 40px rgba(0, 0, 0, 0.5), 0 0 20px color-mix(in srgb, var(--theme-color) 20%, transparent);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 12px 16px;
    border-bottom: 1px solid rgba(255, 170, 0, 0.2);
    background: rgba(0, 0, 0, 0.3);
  }

  .file-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .file-name {
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 1px;
    color: var(--theme-color);
  }

  .file-meta {
    font-size: 10px;
    color: #6c7086;
    letter-spacing: 1px;
  }

  .close-btn {
    background: transparent;
    border: 1px solid rgba(255, 170, 0, 0.3);
    color: var(--theme-color);
    width: 24px;
    height: 24px;
    font-family: monospace;
    font-size: 11px;
    cursor: pointer;
  }

  .close-btn:hover {
    background: rgba(255, 170, 0, 0.15);
    border-color: var(--theme-color);
  }

  .modal-content {
    flex: 1;
    overflow: auto;
    min-height: 200px;
  }

  .loading-state,
  .error-state,
  .binary-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: 200px;
    gap: 8px;
    color: #6c7086;
    font-size: 12px;
    letter-spacing: 2px;
  }

  .error-state {
    color: #f38ba8;
  }

  .binary-icon,
  .placeholder-icon {
    font-family: monospace;
    font-size: 24px;
    color: var(--theme-color);
  }

  .binary-text,
  .placeholder-text {
    font-size: 14px;
    color: #cdd6f4;
  }

  .binary-note,
  .placeholder-note {
    font-size: 10px;
    color: #6c7086;
  }

  .image-preview {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
    padding: 20px;
  }

  .image-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 40px;
    border: 1px dashed rgba(255, 170, 0, 0.3);
  }

  .text-content {
    margin: 0;
    padding: 16px;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.5;
    color: #cdd6f4;
    white-space: pre-wrap;
    word-break: break-all;
    background: rgba(0, 0, 0, 0.3);
  }

  .text-content.code {
    color: #89b4fa;
  }

  .modal-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    border-top: 1px solid rgba(255, 170, 0, 0.2);
    background: rgba(0, 0, 0, 0.3);
  }

  .path-display {
    font-size: 10px;
    color: #6c7086;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .hint {
    font-size: 9px;
    letter-spacing: 1px;
    color: #6c7086;
    flex-shrink: 0;
    margin-left: 16px;
  }
</style>
