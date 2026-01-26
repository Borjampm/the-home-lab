<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke, Channel } from '@tauri-apps/api/core';
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import { open as openDialog, save as saveDialog } from '@tauri-apps/plugin-dialog';
  import { copyFile } from '@tauri-apps/plugin-fs';
  import FileList from './FileList.svelte';
  import PathBreadcrumb from './PathBreadcrumb.svelte';
  import DeviceSidebar from './DeviceSidebar.svelte';
  import TransferQueue from './TransferQueue.svelte';
  import ContextMenu from './ContextMenu.svelte';
  import QuickLook from './QuickLook.svelte';
  import DualPane from './DualPane.svelte';

  interface FileEntry {
    name: string;
    path: string;
    isDir: boolean;
    size: number;
    modified: number | null;
    permissions: number;
    isSymlink: boolean;
  }

  interface DirectoryListing {
    path: string;
    entries: FileEntry[];
    parentPath: string | null;
  }

  interface Device {
    hostname: string;
    ips: string[];
    online: boolean;
    isSelf: boolean;
    os: string;
  }

  interface Bookmark {
    id: string;
    name: string;
    deviceIp: string;
    deviceHostname: string;
    path: string;
  }

  interface Transfer {
    id: string;
    filename: string;
    bytesTransferred: number;
    totalBytes: number;
    direction: 'upload' | 'download';
    status: 'pending' | 'in_progress' | 'complete' | 'error';
    error?: string;
  }

  interface TransferEvent {
    event: 'progress' | 'complete' | 'error';
    data: {
      bytesTransferred?: number;
      totalBytes?: number;
      filename: string;
      message?: string;
    };
  }

  export let deviceIp: string;
  export let deviceHostname: string;
  export let initialPath: string = '';
  export let themeColor: string = '#FFAA00';
  export let devices: Device[] = [];

  // State
  let currentPath = initialPath;
  let files: FileEntry[] = [];
  let selectedFiles: Set<string> = new Set();
  let loading = false;
  let error: string | null = null;

  let historyBack: string[] = [];
  let historyForward: string[] = [];

  let viewMode: 'grid' | 'list' = 'list';
  let splitEnabled = false;
  let showHidden = false;

  let rightPanePath = '';
  let rightPaneFiles: FileEntry[] = [];
  let rightPaneLoading = false;

  let bookmarks: Bookmark[] = [];
  let transfers: Transfer[] = [];

  // Context menu
  let contextMenuVisible = false;
  let contextMenuX = 0;
  let contextMenuY = 0;
  let contextMenuFile: FileEntry | null = null;

  // Quick look
  let quickLookVisible = false;
  let quickLookFile: FileEntry | null = null;
  let quickLookContent: string | null = null;
  let quickLookLoading = false;
  let quickLookError: string | null = null;

  // Rename dialog
  let renameDialogVisible = false;
  let renameFile: FileEntry | null = null;
  let renameNewName = '';

  // New folder dialog
  let newFolderDialogVisible = false;
  let newFolderName = '';

  $: isSelf = deviceIp === 'self';
  $: filteredFiles = showHidden ? files : files.filter(f => !f.name.startsWith('.'));
  $: filteredRightPaneFiles = showHidden ? rightPaneFiles : rightPaneFiles.filter(f => !f.name.startsWith('.'));

  onMount(async () => {
    await connect();
    await loadBookmarks();
  });

  onDestroy(() => {
    if (!isSelf) {
      invoke('sftp_disconnect', { deviceIp }).catch(() => {});
    }
  });

  async function connect() {
    if (isSelf) {
      await navigateTo(currentPath || '~');
    } else {
      loading = true;
      error = null;
      try {
        await invoke('sftp_connect', { deviceIp });
        await navigateTo(currentPath || '/home');
      } catch (e) {
        error = String(e);
      }
      loading = false;
    }
  }

  async function navigateTo(path: string, addToHistory = true) {
    if (addToHistory && currentPath) {
      historyBack = [...historyBack, currentPath];
      historyForward = [];
    }

    loading = true;
    error = null;
    selectedFiles = new Set();

    try {
      const listing: DirectoryListing = isSelf
        ? await invoke('local_list_dir', { path })
        : await invoke('sftp_list_dir', { deviceIp, path });

      currentPath = listing.path;
      files = listing.entries;
    } catch (e) {
      error = String(e);
      files = [];
    }

    loading = false;
  }

  async function navigateRightPane(path: string) {
    rightPaneLoading = true;

    try {
      const listing: DirectoryListing = isSelf
        ? await invoke('local_list_dir', { path })
        : await invoke('sftp_list_dir', { deviceIp, path });

      rightPanePath = listing.path;
      rightPaneFiles = listing.entries;
    } catch (e) {
      rightPaneFiles = [];
    }

    rightPaneLoading = false;
  }

  function goBack() {
    if (historyBack.length === 0) return;
    const prev = historyBack[historyBack.length - 1];
    historyBack = historyBack.slice(0, -1);
    historyForward = [...historyForward, currentPath];
    navigateTo(prev, false);
  }

  function goForward() {
    if (historyForward.length === 0) return;
    const next = historyForward[historyForward.length - 1];
    historyForward = historyForward.slice(0, -1);
    historyBack = [...historyBack, currentPath];
    navigateTo(next, false);
  }

  function goUp() {
    const parts = currentPath.split('/').filter(Boolean);
    if (parts.length <= 1) {
      navigateTo('/');
    } else {
      parts.pop();
      navigateTo('/' + parts.join('/'));
    }
  }

  function handleOpen(e: CustomEvent<{ file: FileEntry }>) {
    const { file } = e.detail;
    if (file.isDir) {
      navigateTo(file.path);
    } else {
      openQuickLook(file);
    }
  }

  function handleContextMenu(e: CustomEvent<{ file: FileEntry; x: number; y: number }>) {
    contextMenuFile = e.detail.file;
    contextMenuX = e.detail.x;
    contextMenuY = e.detail.y;
    contextMenuVisible = true;
  }

  async function handleContextAction(e: CustomEvent<{ action: string; file: FileEntry | null }>) {
    const { action, file } = e.detail;

    switch (action) {
      case 'open':
        if (file?.isDir) navigateTo(file.path);
        else if (file) openQuickLook(file);
        break;

      case 'openTerminal':
        if (file) openTerminalAt(file.path);
        break;

      case 'rename':
        if (file) {
          renameFile = file;
          renameNewName = file.name;
          renameDialogVisible = true;
        }
        break;

      case 'delete':
        await deleteSelected();
        break;

      case 'copyPath':
        if (file) {
          try {
            await writeText(file.path);
          } catch {
            // Clipboard not available
          }
        }
        break;

      case 'bookmark':
        if (file?.isDir) {
          await addBookmark(file.path, file.name);
        }
        break;

      case 'quicklook':
        if (file && !file.isDir) openQuickLook(file);
        break;

      case 'newFolder':
        newFolderName = '';
        newFolderDialogVisible = true;
        break;

      case 'refresh':
        await navigateTo(currentPath, false);
        break;

      case 'download':
        await downloadSelected();
        break;

      case 'upload':
        await uploadFromDialog();
        break;
    }
  }

  function openTerminalAt(path: string) {
    const label = `terminal-${Date.now()}`;
    let cmd: string;

    if (isSelf) {
      cmd = `cd "${path}" && exec $SHELL`;
    } else {
      cmd = `ssh ${deviceIp} -t "cd '${path}' && exec \\$SHELL"`;
    }

    new WebviewWindow(label, {
      url: `/terminal?id=${label}&color=${encodeURIComponent(themeColor)}&host=${encodeURIComponent(deviceHostname)}&cmd=${encodeURIComponent(cmd)}`,
      title: `${deviceHostname} - ${path}`,
      width: 800,
      height: 500,
      resizable: true,
      center: true,
    });
  }

  async function openQuickLook(file: FileEntry) {
    quickLookFile = file;
    quickLookContent = null;
    quickLookError = null;
    quickLookLoading = true;
    quickLookVisible = true;

    try {
      const content: string = isSelf
        ? await invoke('local_read_file', { path: file.path, maxBytes: 100000 })
        : await invoke('sftp_read_file', { deviceIp, path: file.path, maxBytes: 100000 });

      quickLookContent = content;
    } catch (e) {
      quickLookError = String(e);
    }

    quickLookLoading = false;
  }

  async function deleteSelected() {
    const paths = Array.from(selectedFiles);
    if (paths.length === 0) return;

    const confirmMsg = paths.length === 1
      ? `Delete "${paths[0].split('/').pop()}"?`
      : `Delete ${paths.length} items?`;

    if (!confirm(confirmMsg)) return;

    for (const path of paths) {
      try {
        if (isSelf) {
          await invoke('local_delete', { path, recursive: true });
        } else {
          await invoke('sftp_delete', { deviceIp, path, recursive: true });
        }
      } catch (e) {
        error = `Failed to delete: ${e}`;
        break;
      }
    }

    await navigateTo(currentPath, false);
  }

  async function doRename() {
    if (!renameFile || !renameNewName.trim()) return;

    const oldPath = renameFile.path;
    const newPath = oldPath.substring(0, oldPath.lastIndexOf('/') + 1) + renameNewName.trim();

    try {
      if (isSelf) {
        await invoke('local_rename', { oldPath, newPath });
      } else {
        await invoke('sftp_rename', { deviceIp, oldPath, newPath });
      }
      renameDialogVisible = false;
      await navigateTo(currentPath, false);
    } catch (e) {
      error = `Rename failed: ${e}`;
    }
  }

  async function doCreateFolder() {
    if (!newFolderName.trim()) return;

    const path = currentPath === '/'
      ? `/${newFolderName.trim()}`
      : `${currentPath}/${newFolderName.trim()}`;

    try {
      if (isSelf) {
        await invoke('local_mkdir', { path });
      } else {
        await invoke('sftp_mkdir', { deviceIp, path });
      }
      newFolderDialogVisible = false;
      await navigateTo(currentPath, false);
    } catch (e) {
      error = `Failed to create folder: ${e}`;
    }
  }

  async function loadBookmarks() {
    try {
      const result = await invoke<{ bookmarks: Bookmark[] }>('sftp_get_bookmarks');
      bookmarks = result.bookmarks.filter(b => b.deviceIp === deviceIp);
    } catch (e) {
      console.error('Failed to load bookmarks:', e);
    }
  }

  async function addBookmark(path: string, name: string) {
    const bookmark: Bookmark = {
      id: `${deviceIp}-${Date.now()}`,
      name,
      deviceIp,
      deviceHostname,
      path,
    };

    try {
      await invoke('sftp_save_bookmark', { bookmark });
      bookmarks = [...bookmarks, bookmark];
    } catch (e) {
      error = `Failed to save bookmark: ${e}`;
    }
  }

  async function deleteBookmark(bookmark: Bookmark) {
    try {
      await invoke('sftp_delete_bookmark', { bookmarkId: bookmark.id });
      bookmarks = bookmarks.filter(b => b.id !== bookmark.id);
    } catch (e) {
      error = `Failed to delete bookmark: ${e}`;
    }
  }

  function handleSelectDevice(e: CustomEvent<{ device: Device; ip: string }>) {
    const { device, ip } = e.detail;
    // Open new browser window for different device
    const label = `browser-${Date.now()}`;
    const color = themeColor;

    new WebviewWindow(label, {
      url: `/browser?id=${label}&device=${ip}&host=${encodeURIComponent(device.hostname)}&color=${encodeURIComponent(color)}`,
      title: `${device.hostname} - Files`,
      width: 900,
      height: 600,
      resizable: true,
      center: true,
    });
  }

  function handleSelectBookmark(e: CustomEvent<{ bookmark: Bookmark }>) {
    navigateTo(e.detail.bookmark.path);
  }

  function handleDeleteBookmark(e: CustomEvent<{ bookmark: Bookmark }>) {
    deleteBookmark(e.detail.bookmark);
  }

  function handleAddBookmark(e: CustomEvent<{ path: string; name: string }>) {
    addBookmark(e.detail.path, e.detail.name);
  }

  function handleSplitToggle(e: CustomEvent<{ splitEnabled: boolean }>) {
    splitEnabled = e.detail.splitEnabled;
    if (splitEnabled && !rightPanePath) {
      navigateRightPane(currentPath);
    }
  }

  function cancelTransfer(e: CustomEvent<{ transferId: string }>) {
    invoke('sftp_cancel_transfer', { transferId: e.detail.transferId }).catch(() => {});
  }

  function clearTransfers() {
    transfers = transfers.filter(t => t.status === 'in_progress' || t.status === 'pending');
  }

  // Handle native file drop (from Finder)
  async function handleNativeDrop(e: CustomEvent<{ paths: string[] }>) {
    const { paths } = e.detail;
    if (paths.length === 0) return;

    for (const localPath of paths) {
      const filename = localPath.split('/').pop() || 'file';
      const remotePath = currentPath === '/' ? `/${filename}` : `${currentPath}/${filename}`;

      await uploadFile(localPath, remotePath, filename);
    }
  }

  async function uploadFile(localPath: string, remotePath: string, filename: string) {
    const transferId = `upload-${Date.now()}-${Math.random().toString(36).slice(2)}`;

    const transfer: Transfer = {
      id: transferId,
      filename,
      bytesTransferred: 0,
      totalBytes: 0,
      direction: 'upload',
      status: 'in_progress',
    };
    transfers = [...transfers, transfer];

    try {
      if (isSelf) {
        // Local copy
        await copyFile(localPath, remotePath);
        updateTransfer(transferId, { status: 'complete', bytesTransferred: 1, totalBytes: 1 });
      } else {
        const onProgress = new Channel<TransferEvent>();
        onProgress.onmessage = (msg) => {
          if (msg.event === 'progress') {
            updateTransfer(transferId, {
              bytesTransferred: msg.data.bytesTransferred || 0,
              totalBytes: msg.data.totalBytes || 0,
            });
          } else if (msg.event === 'complete') {
            updateTransfer(transferId, { status: 'complete' });
          } else if (msg.event === 'error') {
            updateTransfer(transferId, { status: 'error', error: msg.data.message });
          }
        };

        await invoke('sftp_upload', {
          deviceIp,
          localPath,
          remotePath,
          transferId,
          onProgress,
        });
      }
    } catch (e) {
      updateTransfer(transferId, { status: 'error', error: String(e) });
    }

    await navigateTo(currentPath, false);
  }

  async function downloadFile(remotePath: string, localPath: string, filename: string) {
    const transferId = `download-${Date.now()}-${Math.random().toString(36).slice(2)}`;

    const transfer: Transfer = {
      id: transferId,
      filename,
      bytesTransferred: 0,
      totalBytes: 0,
      direction: 'download',
      status: 'in_progress',
    };
    transfers = [...transfers, transfer];

    try {
      if (isSelf) {
        await copyFile(remotePath, localPath);
        updateTransfer(transferId, { status: 'complete', bytesTransferred: 1, totalBytes: 1 });
      } else {
        const onProgress = new Channel<TransferEvent>();
        onProgress.onmessage = (msg) => {
          if (msg.event === 'progress') {
            updateTransfer(transferId, {
              bytesTransferred: msg.data.bytesTransferred || 0,
              totalBytes: msg.data.totalBytes || 0,
            });
          } else if (msg.event === 'complete') {
            updateTransfer(transferId, { status: 'complete' });
          } else if (msg.event === 'error') {
            updateTransfer(transferId, { status: 'error', error: msg.data.message });
          }
        };

        await invoke('sftp_download', {
          deviceIp,
          remotePath,
          localPath,
          transferId,
          onProgress,
        });
      }
    } catch (e) {
      updateTransfer(transferId, { status: 'error', error: String(e) });
    }
  }

  function updateTransfer(id: string, updates: Partial<Transfer>) {
    transfers = transfers.map((t) => (t.id === id ? { ...t, ...updates } : t));
  }

  // Download selected files to local
  async function downloadSelected() {
    const paths = Array.from(selectedFiles);
    if (paths.length === 0) return;

    // Ask user where to save
    const destPath = await openDialog({
      directory: true,
      multiple: false,
      title: 'Select download destination',
    });

    if (!destPath) return;

    for (const remotePath of paths) {
      const filename = remotePath.split('/').pop() || 'file';
      const localPath = `${destPath}/${filename}`;
      await downloadFile(remotePath, localPath, filename);
    }
  }

  // Upload files from local
  async function uploadFromDialog() {
    const files = await openDialog({
      directory: false,
      multiple: true,
      title: 'Select files to upload',
    });

    if (!files || files.length === 0) return;

    const filePaths = Array.isArray(files) ? files : [files];
    for (const localPath of filePaths) {
      const filename = localPath.split('/').pop() || 'file';
      const remotePath = currentPath === '/' ? `/${filename}` : `${currentPath}/${filename}`;
      await uploadFile(localPath, remotePath, filename);
    }
  }

  // Handle right-click on empty area
  function handleEmptyContextMenu(e: MouseEvent) {
    // Only trigger if clicking on the file list background, not on files
    const target = e.target as HTMLElement;
    if (target.closest('.file-item')) return;

    e.preventDefault();
    contextMenuFile = null;
    contextMenuX = e.clientX;
    contextMenuY = e.clientY;
    contextMenuVisible = true;
  }
</script>

<div class="file-browser" style="--theme-color: {themeColor}">
  <DeviceSidebar
    {devices}
    {bookmarks}
    currentDeviceIp={deviceIp}
    {themeColor}
    on:selectdevice={handleSelectDevice}
    on:selectbookmark={handleSelectBookmark}
    on:deletebookmark={handleDeleteBookmark}
    on:addbookmark={handleAddBookmark}
  />

  <div class="browser-main">
    <div class="browser-toolbar">
      <PathBreadcrumb
        path={currentPath}
        canGoBack={historyBack.length > 0}
        canGoForward={historyForward.length > 0}
        {themeColor}
        on:navigate={(e) => navigateTo(e.detail.path)}
        on:back={goBack}
        on:forward={goForward}
        on:up={goUp}
      />

      <div class="toolbar-actions">
        <button
          class="action-btn"
          on:click={uploadFromDialog}
          title="Upload files"
        >
          ^ UPLOAD
        </button>
        <button
          class="action-btn"
          on:click={downloadSelected}
          disabled={selectedFiles.size === 0}
          title="Download selected"
        >
          v DOWNLOAD
        </button>
        <span class="separator"></span>
        <button
          class="view-btn"
          class:active={showHidden}
          on:click={() => (showHidden = !showHidden)}
          title={showHidden ? "Hide dotfiles" : "Show dotfiles"}
        >
          .*
        </button>
        <span class="separator"></span>
        <button
          class="view-btn"
          class:active={viewMode === 'list'}
          on:click={() => (viewMode = 'list')}
          title="List view"
        >
          =
        </button>
        <button
          class="view-btn"
          class:active={viewMode === 'grid'}
          on:click={() => (viewMode = 'grid')}
          title="Grid view"
        >
          #
        </button>
      </div>
    </div>

    {#if error}
      <div class="error-bar">{error}</div>
    {/if}

    <DualPane {splitEnabled} {themeColor} on:toggle={handleSplitToggle}>
      <div slot="left" class="pane-content" on:contextmenu={handleEmptyContextMenu}>
        <FileList
          files={filteredFiles}
          {selectedFiles}
          {viewMode}
          {themeColor}
          {loading}
          error={null}
          on:selectionchange={(e) => (selectedFiles = e.detail.selected)}
          on:open={handleOpen}
          on:contextmenu={handleContextMenu}
          on:quicklook={(e) => openQuickLook(e.detail.file)}
          on:goback={goUp}
          on:delete={deleteSelected}
          on:nativedrop={handleNativeDrop}
        />
      </div>

      <div slot="right" class="pane-content">
        {#if splitEnabled}
          <FileList
            files={filteredRightPaneFiles}
            selectedFiles={new Set()}
            {viewMode}
            {themeColor}
            loading={rightPaneLoading}
            error={null}
            on:open={(e) => {
              if (e.detail.file.isDir) navigateRightPane(e.detail.file.path);
            }}
          />
        {/if}
      </div>
    </DualPane>

    <TransferQueue
      {transfers}
      {themeColor}
      on:cancel={cancelTransfer}
      on:clear={clearTransfers}
    />
  </div>

  <ContextMenu
    bind:visible={contextMenuVisible}
    x={contextMenuX}
    y={contextMenuY}
    file={contextMenuFile}
    selectedCount={selectedFiles.size}
    {themeColor}
    on:action={handleContextAction}
  />

  <QuickLook
    bind:visible={quickLookVisible}
    file={quickLookFile}
    content={quickLookContent}
    loading={quickLookLoading}
    error={quickLookError}
    {themeColor}
  />

  <!-- Rename Dialog -->
  {#if renameDialogVisible}
    <div class="dialog-overlay" on:click={() => (renameDialogVisible = false)} role="dialog" aria-modal="true">
      <div class="dialog" on:click|stopPropagation role="document">
        <h3 class="dialog-title">RENAME</h3>
        <input
          type="text"
          class="dialog-input"
          bind:value={renameNewName}
          on:keydown={(e) => e.key === 'Enter' && doRename()}
          autofocus
        />
        <div class="dialog-actions">
          <button class="dialog-btn" on:click={() => (renameDialogVisible = false)}>CANCEL</button>
          <button class="dialog-btn primary" on:click={doRename}>RENAME</button>
        </div>
      </div>
    </div>
  {/if}

  <!-- New Folder Dialog -->
  {#if newFolderDialogVisible}
    <div class="dialog-overlay" on:click={() => (newFolderDialogVisible = false)} role="dialog" aria-modal="true">
      <div class="dialog" on:click|stopPropagation role="document">
        <h3 class="dialog-title">NEW FOLDER</h3>
        <input
          type="text"
          class="dialog-input"
          bind:value={newFolderName}
          placeholder="Folder name"
          on:keydown={(e) => e.key === 'Enter' && doCreateFolder()}
          autofocus
        />
        <div class="dialog-actions">
          <button class="dialog-btn" on:click={() => (newFolderDialogVisible = false)}>CANCEL</button>
          <button class="dialog-btn primary" on:click={doCreateFolder}>CREATE</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .file-browser {
    display: flex;
    flex: 1;
    min-height: 0;
    width: 100%;
    background: #1e1e2e;
    color: #cdd6f4;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
    overflow: hidden;
  }

  .browser-main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 0;
    overflow: hidden;
  }

  .browser-toolbar {
    display: flex;
    align-items: center;
    border-bottom: 1px solid rgba(255, 170, 0, 0.2);
  }

  .toolbar-actions {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 0 8px;
  }

  .action-btn {
    background: transparent;
    border: 1px solid rgba(255, 170, 0, 0.3);
    color: var(--theme-color);
    padding: 4px 8px;
    font-family: inherit;
    font-size: 9px;
    letter-spacing: 1px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .action-btn:hover:not(:disabled) {
    background: rgba(255, 170, 0, 0.15);
    border-color: var(--theme-color);
  }

  .action-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .separator {
    width: 1px;
    height: 16px;
    background: rgba(255, 170, 0, 0.2);
    margin: 0 4px;
  }

  .view-btn {
    background: transparent;
    border: 1px solid transparent;
    color: #6c7086;
    width: 24px;
    height: 24px;
    font-family: monospace;
    font-size: 12px;
    cursor: pointer;
  }

  .view-btn:hover {
    color: var(--theme-color);
  }

  .view-btn.active {
    color: var(--theme-color);
    border-color: rgba(255, 170, 0, 0.3);
    background: rgba(255, 170, 0, 0.1);
  }

  .error-bar {
    padding: 8px 12px;
    background: rgba(243, 139, 168, 0.15);
    color: #f38ba8;
    font-size: 11px;
    letter-spacing: 1px;
    border-bottom: 1px solid rgba(243, 139, 168, 0.3);
  }

  .pane-content {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  /* Dialogs */
  .dialog-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 3000;
  }

  .dialog {
    background: #1e1e2e;
    border: 1px solid var(--theme-color);
    padding: 20px;
    min-width: 300px;
  }

  .dialog-title {
    margin: 0 0 16px 0;
    font-size: 12px;
    letter-spacing: 2px;
    color: var(--theme-color);
  }

  .dialog-input {
    width: 100%;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 170, 0, 0.3);
    color: #cdd6f4;
    font-family: inherit;
    font-size: 12px;
    margin-bottom: 16px;
    box-sizing: border-box;
  }

  .dialog-input:focus {
    outline: none;
    border-color: var(--theme-color);
  }

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .dialog-btn {
    background: transparent;
    border: 1px solid rgba(255, 170, 0, 0.3);
    color: #cdd6f4;
    padding: 6px 16px;
    font-family: inherit;
    font-size: 10px;
    letter-spacing: 1px;
    cursor: pointer;
  }

  .dialog-btn:hover {
    background: rgba(255, 170, 0, 0.1);
    border-color: var(--theme-color);
  }

  .dialog-btn.primary {
    background: rgba(255, 170, 0, 0.2);
    color: var(--theme-color);
    border-color: var(--theme-color);
  }

  .dialog-btn.primary:hover {
    background: rgba(255, 170, 0, 0.3);
  }
</style>
