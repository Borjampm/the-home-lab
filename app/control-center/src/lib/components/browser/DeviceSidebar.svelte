<script lang="ts">
  import { createEventDispatcher } from 'svelte';

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

  export let devices: Device[] = [];
  export let bookmarks: Bookmark[] = [];
  export let currentDeviceIp: string | null = null;
  export let themeColor: string = '#FFAA00';
  export let collapsed: boolean = false;

  const dispatch = createEventDispatcher();

  function selectDevice(device: Device) {
    const ip = device.isSelf ? 'self' : device.ips[0];
    dispatch('selectdevice', { device, ip });
  }

  function selectBookmark(bookmark: Bookmark) {
    dispatch('selectbookmark', { bookmark });
  }

  function deleteBookmark(bookmark: Bookmark, e: MouseEvent) {
    e.stopPropagation();
    dispatch('deletebookmark', { bookmark });
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    const data = e.dataTransfer?.getData('application/x-sftp-files');
    if (data) {
      try {
        const files = JSON.parse(data);
        if (files.length === 1 && files[0].isDir) {
          dispatch('addbookmark', { path: files[0].path, name: files[0].name });
        }
      } catch {}
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'link';
    }
  }
</script>

<aside
  class="device-sidebar"
  class:collapsed
  style="--theme-color: {themeColor}"
  on:drop={handleDrop}
  on:dragover={handleDragOver}
>
  {#if !collapsed}
    <div class="sidebar-section">
      <h3 class="section-title">DEVICES</h3>
      <div class="device-list">
        {#each devices as device}
          {@const ip = device.isSelf ? 'self' : device.ips[0]}
          <button
            class="device-item"
            class:active={currentDeviceIp === ip}
            class:offline={!device.online}
            on:click={() => selectDevice(device)}
          >
            <span class="status-dot" class:online={device.online}></span>
            <span class="device-name">{device.hostname.toUpperCase()}</span>
            {#if device.isSelf}
              <span class="self-badge">SELF</span>
            {/if}
          </button>
        {/each}
      </div>
    </div>

    {#if bookmarks.length > 0}
      <div class="sidebar-section">
        <h3 class="section-title">BOOKMARKS</h3>
        <div class="bookmark-list">
          {#each bookmarks as bookmark}
            <div
              class="bookmark-item"
              on:click={() => selectBookmark(bookmark)}
              on:keydown={(e) => e.key === 'Enter' && selectBookmark(bookmark)}
              role="button"
              tabindex="0"
              title={bookmark.path}
            >
              <span class="bookmark-icon">[+]</span>
              <span class="bookmark-name">{bookmark.name.toUpperCase()}</span>
              <button
                class="delete-btn"
                on:click={(e) => deleteBookmark(bookmark, e)}
                title="Remove bookmark"
              >
                x
              </button>
            </div>
          {/each}
        </div>
      </div>
    {/if}

    <div class="sidebar-section drop-zone">
      <span class="drop-hint">DROP FOLDER HERE TO BOOKMARK</span>
    </div>
  {/if}
</aside>

<style>
  .device-sidebar {
    width: 180px;
    background: rgba(0, 0, 0, 0.4);
    border-right: 1px solid rgba(255, 170, 0, 0.2);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    flex-shrink: 0;
  }

  .device-sidebar.collapsed {
    width: 0;
    border-right: none;
  }

  .sidebar-section {
    padding: 12px;
    border-bottom: 1px solid rgba(255, 170, 0, 0.1);
  }

  .section-title {
    margin: 0 0 8px 0;
    font-size: 10px;
    letter-spacing: 2px;
    color: #6c7086;
  }

  .device-list,
  .bookmark-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .device-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    background: transparent;
    border: none;
    color: #cdd6f4;
    font-family: inherit;
    font-size: 11px;
    letter-spacing: 1px;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s ease;
  }

  .device-item:hover {
    background: rgba(255, 170, 0, 0.1);
  }

  .device-item.active {
    background: color-mix(in srgb, var(--theme-color) 20%, transparent);
    border-left: 2px solid var(--theme-color);
    padding-left: 6px;
  }

  .device-item.offline {
    opacity: 0.5;
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #444;
    flex-shrink: 0;
  }

  .status-dot.online {
    background: #00FF41;
    box-shadow: 0 0 6px rgba(0, 255, 65, 0.6);
  }

  .device-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .self-badge {
    font-size: 8px;
    background: var(--theme-color);
    color: #000;
    padding: 1px 4px;
    font-weight: 600;
  }

  .bookmark-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    background: transparent;
    border: none;
    color: #00DDFF;
    font-family: inherit;
    font-size: 11px;
    cursor: pointer;
    text-align: left;
    transition: all 0.15s ease;
  }

  .bookmark-item:hover {
    background: rgba(0, 221, 255, 0.1);
  }

  .bookmark-icon {
    color: #00DDFF;
    font-family: monospace;
    font-size: 10px;
  }

  .bookmark-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    letter-spacing: 1px;
  }

  .delete-btn {
    background: transparent;
    border: none;
    color: #6c7086;
    font-family: monospace;
    font-size: 10px;
    padding: 2px 4px;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .bookmark-item:hover .delete-btn {
    opacity: 1;
  }

  .delete-btn:hover {
    color: #f38ba8;
  }

  .drop-zone {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 60px;
    border: 1px dashed rgba(255, 170, 0, 0.2);
    margin: 12px;
    border-radius: 4px;
  }

  .drop-hint {
    font-size: 9px;
    letter-spacing: 1px;
    color: #6c7086;
    text-align: center;
    padding: 8px;
  }
</style>
