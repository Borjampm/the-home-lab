<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

  interface TailscaleDevice {
    hostname: string;
    dnsName: string;
    os: string;
    ips: string[];
    online: boolean;
    isSelf: boolean;
    relay: string;
    lastSeen: string | null;
  }

  interface TailscaleStatus {
    connected: boolean;
    devices: TailscaleDevice[];
    error: string | null;
  }

  let status: TailscaleStatus | null = null;
  let loading = true;
  let toggling = false;

  function openTerminal() {
    const label = `terminal-${Date.now()}`;
    new WebviewWindow(label, {
      url: `/terminal?id=${label}`,
      title: 'Terminal',
      width: 800,
      height: 500,
      resizable: true,
      center: true,
    });
  }

  function takeControl(device: TailscaleDevice) {
    const label = `terminal-${Date.now()}`;
    let url = `/terminal?id=${label}`;
    if (!device.isSelf) {
      const cmd = `ssh ${device.ips[0]}`;
      url += `&cmd=${encodeURIComponent(cmd)}`;
    }
    new WebviewWindow(label, {
      url,
      title: device.hostname,
      width: 800,
      height: 500,
      resizable: true,
      center: true,
    });
  }

  async function toggleTailscale() {
    if (!status) return;
    toggling = true;
    try {
      if (status.connected) {
        await invoke('tailscale_down');
      } else {
        await invoke('tailscale_up');
      }
      await fetchStatus();
    } catch (e) {
      status = { ...status, error: String(e) };
    }
    toggling = false;
  }

  async function fetchStatus() {
    loading = true;
    try {
      status = await invoke<TailscaleStatus>('get_tailscale_status');
    } catch (e) {
      status = { connected: false, devices: [], error: String(e) };
    }
    loading = false;
  }

  onMount(() => {
    fetchStatus();
  });
</script>

<main>
  <div class="dashboard">
    <header>
      <h1>Control Center</h1>
      <div class="actions">
        <button class="btn" on:click={fetchStatus} disabled={loading}>
          {loading ? 'Loading...' : 'Refresh'}
        </button>
        <button class="btn btn-primary" on:click={openTerminal}>
          Terminal
        </button>
      </div>
    </header>

    {#if status?.error}
      <div class="error">
        <strong>Error:</strong> {status.error}
      </div>
    {/if}

    {#if loading && !status}
      <p class="loading-text">Fetching Tailscale status...</p>
    {:else if status}
      <div class="status-bar">
        <span class="status-indicator" class:online={status.connected}></span>
        Tailscale: {status.connected ? 'Connected' : 'Disconnected'}
        <button
          class="btn btn-toggle"
          class:btn-danger={status.connected}
          on:click={toggleTailscale}
          disabled={toggling}
        >
          {toggling ? '...' : status.connected ? 'Disconnect' : 'Connect'}
        </button>
        <span class="device-count">{status.devices.length} devices</span>
      </div>

      <div class="device-list">
        {#each status.devices as device}
          <div class="device-card" class:self={device.isSelf}>
            <div class="device-header">
              <span class="status-dot" class:online={device.online}></span>
              <span class="hostname">
                {device.hostname}
                {#if device.isSelf}<span class="self-badge">self</span>{/if}
              </span>
              <span class="os">{device.os}</span>
            </div>
            <div class="device-details">
              <span class="ips">{device.ips.join(', ')}</span>
              {#if device.relay}
                <span class="relay">relay: {device.relay}</span>
              {/if}
              {#if device.online}
                <button class="btn btn-control" on:click={() => takeControl(device)}>
                  Take control
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: #1e1e2e;
    color: #cdd6f4;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
  }

  main {
    width: 100vw;
    height: 100vh;
    display: flex;
  }

  .dashboard {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  h1 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
  }

  .actions {
    display: flex;
    gap: 8px;
  }

  .btn {
    background: #313244;
    color: #cdd6f4;
    border: 1px solid #45475a;
    padding: 6px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-family: inherit;
    font-size: 12px;
  }

  .btn:hover {
    background: #45475a;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-primary {
    background: #89b4fa;
    color: #1e1e2e;
    border-color: #89b4fa;
  }

  .btn-primary:hover {
    background: #74c7ec;
    border-color: #74c7ec;
  }

  .btn-toggle {
    padding: 3px 8px;
    font-size: 11px;
    background: #a6e3a1;
    color: #1e1e2e;
    border-color: #a6e3a1;
  }

  .btn-toggle:hover {
    background: #94e2d5;
    border-color: #94e2d5;
  }

  .btn-danger {
    background: #f38ba8;
    border-color: #f38ba8;
  }

  .btn-danger:hover {
    background: #eba0ac;
    border-color: #eba0ac;
  }

  .error {
    background: #45273a;
    border: 1px solid #f38ba8;
    color: #f38ba8;
    padding: 10px 14px;
    border-radius: 4px;
    margin-bottom: 16px;
    font-size: 12px;
  }

  .loading-text {
    color: #6c7086;
    font-size: 13px;
  }

  .status-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    margin-bottom: 16px;
    color: #a6adc8;
  }

  .device-count {
    margin-left: auto;
    color: #6c7086;
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #f38ba8;
  }

  .status-indicator.online {
    background: #a6e3a1;
  }

  .device-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .device-card {
    background: #313244;
    border: 1px solid #45475a;
    border-radius: 4px;
    padding: 10px 14px;
  }

  .device-card.self {
    border-color: #89b4fa;
  }

  .device-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 4px;
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #6c7086;
    flex-shrink: 0;
  }

  .status-dot.online {
    background: #a6e3a1;
  }

  .hostname {
    font-size: 13px;
    font-weight: 600;
  }

  .self-badge {
    font-size: 10px;
    background: #89b4fa;
    color: #1e1e2e;
    padding: 1px 5px;
    border-radius: 3px;
    margin-left: 6px;
    font-weight: 400;
  }

  .os {
    font-size: 11px;
    color: #6c7086;
    margin-left: auto;
  }

  .device-details {
    display: flex;
    gap: 12px;
    font-size: 11px;
    color: #a6adc8;
    padding-left: 14px;
  }

  .relay {
    color: #6c7086;
  }

  .btn-control {
    margin-left: auto;
    padding: 2px 8px;
    font-size: 11px;
    background: #a6e3a1;
    color: #1e1e2e;
    border-color: #a6e3a1;
  }

  .btn-control:hover {
    background: #94e2d5;
    border-color: #94e2d5;
  }
</style>
