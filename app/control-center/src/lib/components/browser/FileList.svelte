<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import FileItem from './FileItem.svelte';

  interface FileEntry {
    name: string;
    path: string;
    isDir: boolean;
    size: number;
    modified: number | null;
    permissions: number;
    isSymlink: boolean;
  }

  export let files: FileEntry[] = [];
  export let selectedFiles: Set<string> = new Set();
  export let viewMode: 'grid' | 'list' = 'list';
  export let themeColor: string = '#FFAA00';
  export let loading: boolean = false;
  export let error: string | null = null;

  const dispatch = createEventDispatcher();

  let lastSelectedIndex: number = -1;

  function handleSelect(e: CustomEvent<{ file: FileEntry; shiftKey: boolean; metaKey: boolean }>) {
    const { file, shiftKey, metaKey } = e.detail;
    const fileIndex = files.findIndex((f) => f.path === file.path);

    if (shiftKey && lastSelectedIndex !== -1) {
      // Range selection
      const start = Math.min(lastSelectedIndex, fileIndex);
      const end = Math.max(lastSelectedIndex, fileIndex);
      const newSelection = new Set(selectedFiles);
      for (let i = start; i <= end; i++) {
        newSelection.add(files[i].path);
      }
      selectedFiles = newSelection;
    } else if (metaKey) {
      // Toggle selection
      const newSelection = new Set(selectedFiles);
      if (newSelection.has(file.path)) {
        newSelection.delete(file.path);
      } else {
        newSelection.add(file.path);
      }
      selectedFiles = newSelection;
      lastSelectedIndex = fileIndex;
    } else {
      // Single selection
      selectedFiles = new Set([file.path]);
      lastSelectedIndex = fileIndex;
    }

    dispatch('selectionchange', { selected: selectedFiles });
  }

  function handleOpen(e: CustomEvent<{ file: FileEntry }>) {
    dispatch('open', e.detail);
  }

  function handleContextMenu(e: CustomEvent<{ file: FileEntry; x: number; y: number }>) {
    // Ensure the file is selected when right-clicking
    if (!selectedFiles.has(e.detail.file.path)) {
      selectedFiles = new Set([e.detail.file.path]);
      dispatch('selectionchange', { selected: selectedFiles });
    }
    dispatch('contextmenu', e.detail);
  }

  function handleDragStart(e: CustomEvent<{ file: FileEntry }>) {
    dispatch('dragstart', e.detail);
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    const data = e.dataTransfer?.getData('application/x-sftp-files');
    if (data) {
      try {
        const draggedFiles = JSON.parse(data);
        dispatch('drop', { files: draggedFiles });
      } catch {}
    }
    // Handle native file drop (Tauri provides path property on File objects)
    if (e.dataTransfer?.files?.length) {
      const nativeFiles = Array.from(e.dataTransfer.files).map((f) => (f as unknown as { path: string }).path).filter(Boolean);
      if (nativeFiles.length > 0) {
        dispatch('nativedrop', { paths: nativeFiles });
      }
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'copy';
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (files.length === 0) return;

    const currentIndex = lastSelectedIndex >= 0 ? lastSelectedIndex : 0;
    let newIndex = currentIndex;

    switch (e.key) {
      case 'ArrowUp':
        e.preventDefault();
        newIndex = Math.max(0, currentIndex - 1);
        break;
      case 'ArrowDown':
        e.preventDefault();
        newIndex = Math.min(files.length - 1, currentIndex + 1);
        break;
      case 'Enter':
        e.preventDefault();
        if (selectedFiles.size === 1) {
          const selectedPath = Array.from(selectedFiles)[0];
          const file = files.find((f) => f.path === selectedPath);
          if (file) dispatch('open', { file });
        }
        return;
      case ' ':
        e.preventDefault();
        if (selectedFiles.size === 1) {
          const selectedPath = Array.from(selectedFiles)[0];
          const file = files.find((f) => f.path === selectedPath);
          if (file) dispatch('quicklook', { file });
        }
        return;
      case 'Backspace':
        e.preventDefault();
        dispatch('goback');
        return;
      case 'Delete':
        e.preventDefault();
        dispatch('delete', { paths: Array.from(selectedFiles) });
        return;
      case 'a':
        if (e.metaKey || e.ctrlKey) {
          e.preventDefault();
          selectedFiles = new Set(files.map((f) => f.path));
          dispatch('selectionchange', { selected: selectedFiles });
        }
        return;
      default:
        return;
    }

    if (newIndex !== currentIndex) {
      lastSelectedIndex = newIndex;
      if (e.shiftKey) {
        const start = Math.min(currentIndex, newIndex);
        const end = Math.max(currentIndex, newIndex);
        const newSelection = new Set(selectedFiles);
        for (let i = start; i <= end; i++) {
          newSelection.add(files[i].path);
        }
        selectedFiles = newSelection;
      } else {
        selectedFiles = new Set([files[newIndex].path]);
      }
      dispatch('selectionchange', { selected: selectedFiles });
    }
  }
</script>

<div
  class="file-list"
  class:grid-view={viewMode === 'grid'}
  class:list-view={viewMode === 'list'}
  on:drop={handleDrop}
  on:dragover={handleDragOver}
  on:keydown={handleKeydown}
  role="listbox"
  tabindex="0"
  aria-label="File list"
>
  {#if loading}
    <div class="loading">LOADING...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if files.length === 0}
    <div class="empty">EMPTY DIRECTORY</div>
  {:else}
    {#if viewMode === 'list'}
      <div class="list-header">
        <span class="header-name">NAME</span>
        <span class="header-size">SIZE</span>
        <span class="header-date">MODIFIED</span>
      </div>
    {/if}
    <div class="file-container" class:grid-container={viewMode === 'grid'}>
      {#each files as file (file.path)}
        <FileItem
          {file}
          {viewMode}
          {themeColor}
          selected={selectedFiles.has(file.path)}
          on:select={handleSelect}
          on:open={handleOpen}
          on:contextmenu={handleContextMenu}
          on:dragstart={handleDragStart}
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .file-list {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: rgba(0, 0, 0, 0.3);
    min-height: 0;
    overflow: hidden;
  }

  .file-list:focus {
    outline: none;
  }

  .list-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    font-size: 10px;
    letter-spacing: 1px;
    color: #6c7086;
    border-bottom: 1px solid rgba(255, 170, 0, 0.2);
    background: rgba(0, 0, 0, 0.3);
    flex-shrink: 0;
  }

  .header-name {
    flex: 1;
    padding-left: 24px;
  }

  .header-size {
    width: 70px;
    text-align: right;
  }

  .header-date {
    width: 100px;
    text-align: right;
  }

  .file-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    min-height: 0;
  }

  .grid-container {
    display: flex;
    flex-wrap: wrap;
    padding: 12px;
    gap: 8px;
    align-content: flex-start;
  }

  .loading,
  .empty,
  .error {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    font-size: 12px;
    letter-spacing: 2px;
    color: #6c7086;
  }

  .error {
    color: #f38ba8;
  }
</style>
