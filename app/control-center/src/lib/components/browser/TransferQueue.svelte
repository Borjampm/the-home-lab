<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  interface Transfer {
    id: string;
    filename: string;
    bytesTransferred: number;
    totalBytes: number;
    direction: 'upload' | 'download';
    status: 'pending' | 'in_progress' | 'complete' | 'error';
    error?: string;
  }

  export let transfers: Transfer[] = [];
  export let collapsed: boolean = false;
  export let themeColor: string = '#FFAA00';

  const dispatch = createEventDispatcher();

  $: activeTransfers = transfers.filter((t) => t.status === 'in_progress' || t.status === 'pending');
  $: completedTransfers = transfers.filter((t) => t.status === 'complete' || t.status === 'error');

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB'];
    let i = 0;
    while (bytes >= 1024 && i < units.length - 1) {
      bytes /= 1024;
      i++;
    }
    return `${bytes.toFixed(1)} ${units[i]}`;
  }

  function getProgress(transfer: Transfer): number {
    if (transfer.totalBytes === 0) return 0;
    return (transfer.bytesTransferred / transfer.totalBytes) * 100;
  }

  function cancelTransfer(transfer: Transfer) {
    dispatch('cancel', { transferId: transfer.id });
  }

  function clearCompleted() {
    dispatch('clear');
  }

  function toggleCollapsed() {
    collapsed = !collapsed;
    dispatch('toggle', { collapsed });
  }
</script>

{#if transfers.length > 0}
  <div class="transfer-queue" class:collapsed style="--theme-color: {themeColor}">
    <button class="queue-header" on:click={toggleCollapsed}>
      <span class="queue-title">
        TRANSFERS
        {#if activeTransfers.length > 0}
          <span class="active-count">({activeTransfers.length})</span>
        {/if}
      </span>
      <span class="toggle-icon">{collapsed ? '+' : '-'}</span>
    </button>

    {#if !collapsed}
      <div class="queue-content">
        {#each activeTransfers as transfer (transfer.id)}
          <div class="transfer-item in-progress">
            <div class="transfer-info">
              <span class="transfer-icon">{transfer.direction === 'upload' ? '^' : 'v'}</span>
              <span class="transfer-name">{transfer.filename}</span>
              <button class="cancel-btn" on:click={() => cancelTransfer(transfer)}>x</button>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {getProgress(transfer)}%"></div>
            </div>
            <div class="transfer-stats">
              <span>{formatBytes(transfer.bytesTransferred)} / {formatBytes(transfer.totalBytes)}</span>
              <span>{getProgress(transfer).toFixed(0)}%</span>
            </div>
          </div>
        {/each}

        {#if completedTransfers.length > 0}
          <div class="completed-section">
            <div class="completed-header">
              <span class="completed-title">COMPLETED ({completedTransfers.length})</span>
              <button class="clear-btn" on:click={clearCompleted}>CLEAR</button>
            </div>
            {#each completedTransfers.slice(0, 5) as transfer (transfer.id)}
              <div class="transfer-item" class:error={transfer.status === 'error'}>
                <span class="transfer-icon">{transfer.direction === 'upload' ? '^' : 'v'}</span>
                <span class="transfer-name">{transfer.filename}</span>
                <span class="status-icon">
                  {transfer.status === 'complete' ? 'OK' : 'ERR'}
                </span>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
{/if}

<style>
  .transfer-queue {
    background: rgba(0, 0, 0, 0.5);
    border-top: 1px solid rgba(255, 170, 0, 0.2);
    max-height: 200px;
    display: flex;
    flex-direction: column;
  }

  .transfer-queue.collapsed {
    max-height: none;
  }

  .queue-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.3);
    border: none;
    cursor: pointer;
    font-family: inherit;
    color: var(--theme-color);
  }

  .queue-header:hover {
    background: rgba(255, 170, 0, 0.1);
  }

  .queue-title {
    font-size: 10px;
    letter-spacing: 2px;
  }

  .active-count {
    color: #00FF41;
  }

  .toggle-icon {
    font-family: monospace;
    font-size: 12px;
  }

  .queue-content {
    overflow-y: auto;
    flex: 1;
  }

  .transfer-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 8px 12px;
    border-bottom: 1px solid rgba(255, 170, 0, 0.1);
  }

  .transfer-item:not(.in-progress) {
    flex-direction: row;
    align-items: center;
    gap: 8px;
  }

  .transfer-item.error {
    background: rgba(243, 139, 168, 0.1);
  }

  .transfer-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .transfer-icon {
    font-family: monospace;
    font-size: 10px;
    color: var(--theme-color);
  }

  .transfer-name {
    flex: 1;
    font-size: 11px;
    color: #cdd6f4;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .cancel-btn {
    background: transparent;
    border: none;
    color: #f38ba8;
    font-family: monospace;
    font-size: 10px;
    padding: 2px 6px;
    cursor: pointer;
  }

  .cancel-btn:hover {
    background: rgba(243, 139, 168, 0.2);
  }

  .progress-bar {
    height: 4px;
    background: rgba(255, 255, 255, 0.1);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--theme-color);
    transition: width 0.2s ease;
  }

  .transfer-stats {
    display: flex;
    justify-content: space-between;
    font-size: 9px;
    color: #6c7086;
  }

  .completed-section {
    border-top: 1px solid rgba(255, 170, 0, 0.2);
    margin-top: 8px;
  }

  .completed-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.2);
  }

  .completed-title {
    font-size: 9px;
    letter-spacing: 1px;
    color: #6c7086;
  }

  .clear-btn {
    background: transparent;
    border: 1px solid rgba(255, 170, 0, 0.3);
    color: var(--theme-color);
    font-family: inherit;
    font-size: 9px;
    padding: 2px 6px;
    cursor: pointer;
  }

  .clear-btn:hover {
    background: rgba(255, 170, 0, 0.1);
  }

  .status-icon {
    font-size: 9px;
    letter-spacing: 1px;
  }

  .transfer-item:not(.error) .status-icon {
    color: #00FF41;
  }

  .transfer-item.error .status-icon {
    color: #f38ba8;
  }
</style>
