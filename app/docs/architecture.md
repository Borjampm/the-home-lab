# Architecture

## Overview

The Control Center is a Tauri 2 desktop application that provides a terminal interface for managing home lab systems. It uses a Rust backend to manage PTY (pseudo-terminal) processes and a SvelteKit frontend to render the terminal UI via xterm.js.

## Data Flow

```
User input → xterm.js → invoke('write_to_pty') → Rust → PTY master fd
PTY output → Rust reader thread → Channel<PtyEvent> → xterm.js → screen
Window resize → FitAddon.fit() → term.onResize → invoke('resize_pty') → master.resize()
```

## Key Decisions

- **Tauri over Electron** — Smaller binary (~5MB vs ~100MB), lower memory usage, Rust backend safety
- **portable-pty over raw forkpty** — Clean API, handles platform differences (macOS vs Linux)
- **No Windows support** — Simplifies PTY handling (POSIX only, no ConPTY)
- **WebGL renderer with DOM fallback** — GPU-accelerated rendering where available, graceful degradation on older WebKit
- **IPC Channels for streaming** — Ordered, high-throughput data transfer from PTY to frontend without polling
