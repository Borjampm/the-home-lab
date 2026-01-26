<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let file: {
    name: string;
    path: string;
    isDir: boolean;
    size: number;
    modified: number | null;
    permissions: number;
    isSymlink: boolean;
  };
  export let selected: boolean = false;
  export let viewMode: 'grid' | 'list' = 'list';
  export let themeColor: string = '#FFAA00';

  const dispatch = createEventDispatcher();

  function formatSize(bytes: number): string {
    if (bytes === 0) return '-';
    const units = ['B', 'KB', 'MB', 'GB'];
    let i = 0;
    while (bytes >= 1024 && i < units.length - 1) {
      bytes /= 1024;
      i++;
    }
    return `${bytes.toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
  }

  function formatDate(timestamp: number | null): string {
    if (!timestamp) return '-';
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    });
  }

  function getFileIcon(file: { name: string; isDir: boolean; isSymlink: boolean }): string {
    if (file.isDir) return file.isSymlink ? '[ ]' : '[+]';

    const ext = file.name.split('.').pop()?.toLowerCase();
    const codeExts = ['js', 'ts', 'py', 'rs', 'go', 'java', 'c', 'cpp', 'h', 'svelte', 'vue', 'jsx', 'tsx'];
    const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'svg', 'webp', 'ico'];
    const docExts = ['md', 'txt', 'pdf', 'doc', 'docx'];
    const configExts = ['json', 'yaml', 'yml', 'toml', 'xml', 'ini', 'conf'];

    if (ext && codeExts.includes(ext)) return '</>';
    if (ext && imageExts.includes(ext)) return '[*]';
    if (ext && docExts.includes(ext)) return '[=]';
    if (ext && configExts.includes(ext)) return '[:]';
    return '[-]';
  }

  function handleClick(e: MouseEvent) {
    dispatch('select', { file, shiftKey: e.shiftKey, metaKey: e.metaKey || e.ctrlKey });
  }

  function handleDoubleClick() {
    dispatch('open', { file });
  }

  function handleContextMenu(e: MouseEvent) {
    e.preventDefault();
    dispatch('contextmenu', { file, x: e.clientX, y: e.clientY });
  }

  function handleDragStart(e: DragEvent) {
    if (e.dataTransfer) {
      e.dataTransfer.setData('application/x-sftp-files', JSON.stringify([file]));
      e.dataTransfer.effectAllowed = 'copyMove';
    }
    dispatch('dragstart', { file });
  }
</script>

<div
  class="file-item"
  class:selected
  class:is-dir={file.isDir}
  class:grid-view={viewMode === 'grid'}
  class:list-view={viewMode === 'list'}
  style="--theme-color: {themeColor}"
  on:click={handleClick}
  on:dblclick={handleDoubleClick}
  on:contextmenu={handleContextMenu}
  on:keydown={(e) => e.key === 'Enter' && handleDoubleClick()}
  draggable="true"
  on:dragstart={handleDragStart}
  role="button"
  tabindex="0"
>
  <span class="file-icon">{getFileIcon(file)}</span>
  <span class="file-name" title={file.path}>
    {file.name}
    {#if file.isSymlink}<span class="symlink-badge">@</span>{/if}
  </span>
  {#if viewMode === 'list'}
    <span class="file-size">{file.isDir ? '-' : formatSize(file.size)}</span>
    <span class="file-date">{formatDate(file.modified)}</span>
  {/if}
</div>

<style>
  .file-item {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    user-select: none;
    font-size: 12px;
    transition: background 0.1s ease;
  }

  .file-item:focus {
    outline: none;
  }

  /* List View */
  .list-view {
    padding: 6px 12px;
    border-bottom: 1px solid rgba(255, 170, 0, 0.1);
  }

  .list-view:hover {
    background: rgba(255, 170, 0, 0.08);
  }

  .list-view.selected {
    background: color-mix(in srgb, var(--theme-color) 20%, transparent);
    border-left: 2px solid var(--theme-color);
    padding-left: 10px;
  }

  /* Grid View */
  .grid-view {
    flex-direction: column;
    justify-content: center;
    padding: 12px 8px;
    width: 80px;
    text-align: center;
    border: 1px solid transparent;
  }

  .grid-view:hover {
    background: rgba(255, 170, 0, 0.08);
    border-color: rgba(255, 170, 0, 0.2);
  }

  .grid-view.selected {
    background: color-mix(in srgb, var(--theme-color) 20%, transparent);
    border-color: var(--theme-color);
  }

  .grid-view .file-icon {
    font-size: 20px;
    margin-bottom: 4px;
  }

  .grid-view .file-name {
    font-size: 10px;
    word-break: break-all;
    line-height: 1.2;
    max-height: 2.4em;
    overflow: hidden;
  }

  .file-icon {
    color: var(--theme-color);
    font-family: monospace;
    flex-shrink: 0;
    text-shadow: 0 0 8px color-mix(in srgb, var(--theme-color) 40%, transparent);
  }

  .is-dir .file-icon {
    color: #00DDFF;
    text-shadow: 0 0 8px rgba(0, 221, 255, 0.4);
  }

  .file-name {
    flex: 1;
    color: #cdd6f4;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .is-dir .file-name {
    color: #00DDFF;
  }

  .symlink-badge {
    color: #AA88FF;
    margin-left: 4px;
    font-size: 10px;
  }

  .file-size,
  .file-date {
    color: #6c7086;
    font-size: 11px;
    flex-shrink: 0;
  }

  .file-size {
    width: 70px;
    text-align: right;
  }

  .file-date {
    width: 100px;
    text-align: right;
  }
</style>
