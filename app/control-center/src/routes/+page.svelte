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

  const DEVICE_COLORS = [
    '#FFAA00', '#00FF41', '#00DDFF', '#FF6600',
    '#AA88FF', '#FF0066', '#88FF00', '#00FFAA'
  ];

  function getDeviceColor(index: number): string {
    return DEVICE_COLORS[index % DEVICE_COLORS.length];
  }

  let status: TailscaleStatus | null = null;
  let loading = true;
  let toggling = false;

  function openTerminal() {
    const label = `terminal-${Date.now()}`;
    const color = DEVICE_COLORS[0]; // Default amber
    new WebviewWindow(label, {
      url: `/terminal?id=${label}&color=${encodeURIComponent(color)}&host=TERMINAL`,
      title: 'Terminal',
      width: 800,
      height: 500,
      resizable: true,
      center: true,
    });
  }

  function takeControl(device: TailscaleDevice, color: string) {
    const label = `terminal-${Date.now()}`;
    let url = `/terminal?id=${label}&color=${encodeURIComponent(color)}&host=${encodeURIComponent(device.hostname)}`;
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
  <div class="scanlines"></div>
  <div class="dashboard">
    <header>
      <h1>CONTROL CENTER</h1>
      <div class="actions">
        <button class="btn" on:click={fetchStatus} disabled={loading}>
          {loading ? 'LOADING...' : 'REFRESH'}
        </button>
        <button class="btn btn-primary" on:click={openTerminal}>
          TERMINAL
        </button>
      </div>
    </header>

    {#if status?.error}
      <div class="error">
        <strong>ERROR:</strong> {status.error}
      </div>
    {/if}

    {#if loading && !status}
      <p class="loading-text">FETCHING TAILSCALE STATUS...</p>
    {:else if status}
      <div class="status-bar">
        <span class="status-indicator" class:online={status.connected}></span>
        <span class="status-label">TAILSCALE: {status.connected ? 'CONNECTED' : 'DISCONNECTED'}</span>
        <button
          class="btn btn-toggle"
          class:btn-disconnect={status.connected}
          on:click={toggleTailscale}
          disabled={toggling}
        >
          {toggling ? '...' : status.connected ? 'DISCONNECT' : 'CONNECT'}
        </button>
        <span class="device-count">{status.devices.length} NODES</span>
      </div>

      <div class="device-list">
        {#each status.devices as device, i}
          {@const color = getDeviceColor(i)}
          <div class="device-card" style="--device-color: {color}">
            <div class="device-header">
              <span class="status-dot" class:online={device.online}></span>
              <span class="hostname">
                {device.hostname.toUpperCase()}
                {#if device.isSelf}<span class="self-badge">SELF</span>{/if}
              </span>
              <span class="os">{device.os.toUpperCase()}</span>
            </div>
            <div class="device-details">
              <span class="ips">{device.ips.join(', ')}</span>
              {#if device.relay}
                <span class="relay">RELAY: {device.relay.toUpperCase()}</span>
              {/if}
              {#if device.online}
                <button class="btn btn-control" style="--device-color: {color}" on:click={() => takeControl(device, color)}>
                  TAKE CONTROL
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
    background: #000000;
    color: #FFAA00;
    font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
  }

  main {
    width: 100vw;
    height: 100vh;
    display: flex;
    position: relative;
  }

  /* Scanlines overlay */
  .scanlines {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 1000;
    background: repeating-linear-gradient(
      0deg,
      rgba(0, 0, 0, 0.15) 0px,
      rgba(0, 0, 0, 0.15) 1px,
      transparent 1px,
      transparent 2px
    );
  }

  .dashboard {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
    position: relative;
    z-index: 1;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    padding-bottom: 12px;
    border-bottom: 1px solid rgba(255, 170, 0, 0.3);
  }

  h1 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    letter-spacing: 3px;
    color: #FFAA00;
    text-shadow: 0 0 10px rgba(255, 170, 0, 0.5), 0 0 20px rgba(255, 170, 0, 0.3);
  }

  .actions {
    display: flex;
    gap: 8px;
  }

  .btn {
    background: transparent;
    color: #FFAA00;
    border: 1px solid rgba(255, 170, 0, 0.5);
    padding: 6px 12px;
    border-radius: 0;
    cursor: pointer;
    font-family: inherit;
    font-size: 11px;
    letter-spacing: 1px;
    text-shadow: 0 0 8px rgba(255, 170, 0, 0.4);
    transition: all 0.15s ease;
  }

  .btn:hover {
    background: rgba(255, 170, 0, 0.15);
    border-color: #FFAA00;
    text-shadow: 0 0 12px rgba(255, 170, 0, 0.6);
  }

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .btn-primary {
    background: rgba(255, 170, 0, 0.2);
    border-color: #FFAA00;
  }

  .btn-primary:hover {
    background: rgba(255, 170, 0, 0.35);
  }

  .btn-toggle {
    padding: 3px 10px;
    font-size: 10px;
    background: rgba(0, 255, 65, 0.15);
    color: #00FF41;
    border-color: rgba(0, 255, 65, 0.5);
    text-shadow: 0 0 8px rgba(0, 255, 65, 0.4);
  }

  .btn-toggle:hover {
    background: rgba(0, 255, 65, 0.25);
    border-color: #00FF41;
  }

  .btn-disconnect {
    background: rgba(136, 34, 34, 0.3);
    color: #FF4444;
    border-color: rgba(255, 68, 68, 0.5);
    text-shadow: 0 0 8px rgba(255, 68, 68, 0.4);
  }

  .btn-disconnect:hover {
    background: rgba(136, 34, 34, 0.5);
    border-color: #FF4444;
  }

  .error {
    background: rgba(136, 34, 34, 0.2);
    border: 1px solid #FF4444;
    color: #FF4444;
    padding: 10px 14px;
    margin-bottom: 16px;
    font-size: 11px;
    letter-spacing: 1px;
    text-shadow: 0 0 8px rgba(255, 68, 68, 0.4);
  }

  .loading-text {
    color: #AA7700;
    font-size: 12px;
    letter-spacing: 1px;
  }

  .status-bar {
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 11px;
    margin-bottom: 20px;
    padding: 10px 14px;
    background: rgba(255, 170, 0, 0.05);
    border: 1px solid rgba(255, 170, 0, 0.2);
  }

  .status-label {
    letter-spacing: 1px;
  }

  .device-count {
    margin-left: auto;
    color: #AA7700;
    letter-spacing: 1px;
  }

  .status-indicator {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #882222;
    box-shadow: 0 0 6px rgba(136, 34, 34, 0.8);
  }

  .status-indicator.online {
    background: #00FF41;
    box-shadow: 0 0 8px rgba(0, 255, 65, 0.6), 0 0 16px rgba(0, 255, 65, 0.3);
  }

  .device-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .device-card {
    background: rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 170, 0, 0.2);
    border-left: 3px solid var(--device-color, #FFAA00);
    padding: 12px 16px;
  }

  .device-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 6px;
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #444444;
    flex-shrink: 0;
  }

  .status-dot.online {
    background: #00FF41;
    box-shadow: 0 0 6px rgba(0, 255, 65, 0.6);
  }

  .hostname {
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 1px;
    color: var(--device-color, #FFAA00);
    text-shadow: 0 0 8px color-mix(in srgb, var(--device-color, #FFAA00) 50%, transparent);
  }

  .self-badge {
    font-size: 9px;
    background: var(--device-color, #FFAA00);
    color: #000000;
    padding: 2px 6px;
    margin-left: 8px;
    font-weight: 600;
    letter-spacing: 1px;
  }

  .os {
    font-size: 10px;
    color: #AA7700;
    margin-left: auto;
    letter-spacing: 1px;
  }

  .device-details {
    display: flex;
    gap: 16px;
    font-size: 11px;
    color: #AA7700;
    padding-left: 16px;
  }

  .ips {
    font-family: inherit;
  }

  .relay {
    color: #666666;
  }

  .btn-control {
    margin-left: auto;
    padding: 4px 12px;
    font-size: 10px;
    background: rgba(0, 0, 0, 0.5);
    color: var(--device-color, #FFAA00);
    border: 1px solid var(--device-color, #FFAA00);
    text-shadow: 0 0 8px color-mix(in srgb, var(--device-color, #FFAA00) 40%, transparent);
  }

  .btn-control:hover {
    background: color-mix(in srgb, var(--device-color, #FFAA00) 20%, transparent);
  }
</style>
