<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke, Channel } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { Terminal } from '@xterm/xterm';
  import { FitAddon } from '@xterm/addon-fit';
  import { WebglAddon } from '@xterm/addon-webgl';
  import '@xterm/xterm/css/xterm.css';

  export let terminalId: string;
  export let command: string | undefined = undefined;
  // themeColor is passed but only used by parent for chrome styling
  // Terminal text always uses amber for readability
  export let themeColor: string = '#FFAA00';

  // Fixed amber color for terminal text - ensures readability regardless of device color
  const TERMINAL_AMBER = '#FFAA00';
  const TERMINAL_BG = '#0a0a0a';

  let terminalEl: HTMLDivElement;
  let term: Terminal;
  let fitAddon: FitAddon;

  type PtyEvent =
    | { event: 'data'; data: string }
    | { event: 'exit'; data: number };

  onMount(async () => {
    // Always use amber text on near-black background for maximum readability
    // Device color is used for chrome (border, header) not terminal content
    term = new Terminal({
      cursorBlink: true,
      fontSize: 14,
      fontFamily: 'Menlo, Monaco, "Courier New", monospace',
      theme: {
        background: TERMINAL_BG,
        foreground: TERMINAL_AMBER,
        cursor: TERMINAL_AMBER,
        cursorAccent: TERMINAL_BG,
        selectionBackground: `${TERMINAL_AMBER}40`,
      },
    });

    fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(terminalEl);

    try {
      const webgl = new WebglAddon();
      webgl.onContextLoss(() => webgl.dispose());
      term.loadAddon(webgl);
    } catch (e) {
      console.warn('WebGL unavailable, using DOM renderer');
    }

    fitAddon.fit();

    const onData = new Channel<PtyEvent>();
    onData.onmessage = (msg: PtyEvent) => {
      if (msg.event === 'data') term.write(msg.data);
      if (msg.event === 'exit') term.write('\r\n[Process exited]\r\n');
    };

    await invoke('spawn_shell', { onData, rows: term.rows, cols: term.cols, terminalId, command: command ?? null });

    term.onData((data) => invoke('write_to_pty', { data, terminalId }));
    term.onResize(({ rows, cols }) => invoke('resize_pty', { rows, cols, terminalId }));

    window.addEventListener('resize', handleResize);

    const currentWindow = getCurrentWindow();
    currentWindow.onCloseRequested(() => {
      invoke('kill_pty', { terminalId }).catch(() => {});
    });
  });

  function handleResize() {
    fitAddon?.fit();
  }

  onDestroy(() => {
    window.removeEventListener('resize', handleResize);
    invoke('kill_pty', { terminalId }).catch(() => {});
    term?.dispose();
  });
</script>

<div bind:this={terminalEl} class="terminal-container"></div>

<style>
  .terminal-container {
    flex: 1;
    padding: 4px;
  }
  .terminal-container :global(.xterm) {
    height: 100%;
  }
</style>
