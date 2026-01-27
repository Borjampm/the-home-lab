# Architecture

## Overview

The Control Center is a Tauri 2 desktop application that provides network monitoring, terminal access, and file management for home lab systems. The app combines a dashboard view showing Tailscale network status with a toggleable terminal emulator and an SFTP-based file browser. All features are backed by Rust commands invoked from a SvelteKit frontend.

## Core Systems

The application is built around three independent components:

- **Tailscale Integration** — Monitors VPN network status and manages connections via CLI integration
- **Terminal Emulator** — Provides an embedded interactive shell using PTY-based process management
- **File Browser** — Enables file browsing and transfers over SFTP to remote devices

For detailed documentation on each component, see [components/](components/).

## Data Flow

### Terminal Data Flow
```
User keypress → xterm.js → invoke('write_to_pty') → Rust writer → PTY master fd → shell
Shell output → PTY reader thread → Channel<PtyEvent> → xterm.js → screen
Window resize → FitAddon.fit() → term.onResize → invoke('resize_pty') → master.resize() → shell
```

See [components/terminal.md](components/terminal.md) for details on PTY management, bidirectional streaming, and WebGL rendering.

### Tailscale Data Flow
```
User clicks Refresh → invoke('get_tailscale_status') → Rust spawns `tailscale status --json`
  → Parse JSON → Transform to TailscaleStatus → Return to frontend → Render device list

User toggles connection → invoke('tailscale_up' | 'tailscale_down') → Rust spawns `tailscale up/down`
  → Refresh status → Update UI
```

See [components/tailscale.md](components/tailscale.md) for details on CLI integration and why this approach was chosen over library crates.

### SFTP Data Flow
```
Browser opens → invoke('sftp_connect', { deviceIp }) → russh SSH connect → Authenticate
  → Open SFTP subsystem → Store session in SftpState

Navigate to directory → invoke('sftp_list_dir', { deviceIp, path })
  → Lookup connection → sftp.read_dir() → Return DirectoryListing → Render FileList

Download file → invoke('sftp_download', { ..., onProgress }) → Register CancellationToken
  → sftp.open() → Read 64KB chunks → Channel<TransferEvent>::send(Progress)
  → Write to local file → send(Complete) → Update TransferQueue

Upload file → invoke('sftp_upload', { ..., onProgress }) → Same pattern as download
  → Local file → sftp.create() → Write chunks with progress → Complete
```

See [components/file-browser.md](components/file-browser.md) for details on russh integration, authentication, and feature breakdown.

## Key Design Decisions

### Tauri over Electron
Smaller binary size (~5MB vs ~100MB), significantly lower memory footprint, and Rust backend provides memory safety and better performance for process management. The tradeoff is a smaller ecosystem compared to Electron.

### CLI Integration over Library Crates
The Tailscale integration shells out to the `tailscale` CLI rather than using library crates or Unix sockets. This decision was made because the Mac App Store version of Tailscale does not expose a Unix socket, making CLI the only universally available integration point across distribution methods.

### portable-pty over raw forkpty
Provides a clean, cross-platform API for PTY management. Handles platform differences between macOS and Linux without conditional compilation. Saves implementation effort compared to raw POSIX calls.

### No Windows Support
Simplifies PTY handling by targeting POSIX-only systems. Windows ConPTY has different semantics and would require significant additional code. The home lab runs exclusively on macOS and Linux, so Windows support adds complexity without value.

### WebGL Terminal Renderer with DOM Fallback
The xterm.js terminal uses WebGL for GPU-accelerated rendering when available, providing better performance for high-throughput shell output. Falls back gracefully to DOM rendering on older WebKit versions. Users don't notice the difference except in performance.

### IPC Channels for Streaming
Tauri's Channel API provides ordered, high-throughput data transfer from the PTY reader thread to the frontend without polling. Events are serialized as an enum (`Data(String)` or `Exit(i32)`), allowing the frontend to handle output and process termination through a single message handler.

### Dashboard-Terminal Toggle Pattern
The app presents either the dashboard or terminal in fullscreen, toggled via a button. This keeps the UI simple and focused, avoiding split-pane complexity. The terminal is ephemeral — switching back to the dashboard leaves the shell running in the background until the app closes.

### Pure Rust SSH/SFTP via russh
The file browser uses `russh` and `russh-sftp` crates rather than shelling out to `scp`/`sftp` CLI tools or linking against system OpenSSH libraries. This provides a consistent implementation across platforms without external binary dependencies. The async-native design integrates naturally with Tauri's command model, enabling non-blocking file transfers with real-time progress streaming via Channels.

## Capabilities

The Control Center can:
- Display all Tailscale network devices with their connection state
- Connect/disconnect from the Tailscale network
- Spawn an interactive shell session in a terminal emulator
- Handle terminal resizing and streaming I/O
- Browse files on remote devices via SFTP
- Transfer files between local and remote machines with progress tracking
- Preview text files without downloading
- Open terminal sessions at specific directory locations
- Present network status, terminal, and file management in a unified interface

## Dependencies

### External
- **Tailscale CLI** — Must be installed and available in PATH for network management features
- **User's shell** — Reads `$SHELL` environment variable, defaults to `/bin/bash` if unset
- **SSH Agent** — Optional but recommended for key-based authentication to remote devices

### Rust Crates
- **portable-pty** — PTY process management
- **tauri** — Desktop app framework and IPC
- **russh** — SSH client protocol implementation
- **russh-sftp** — SFTP protocol layer
- **tokio** — Async runtime for non-blocking I/O

### Frontend
- **SvelteKit** — UI framework (static adapter, no SSR)
- **xterm.js** — Terminal emulator with WebGL and fit addons

## See Also

- [components/terminal.md](components/terminal.md) — Terminal emulator implementation details
- [components/tailscale.md](components/tailscale.md) — Tailscale integration implementation details
- [components/file-browser.md](components/file-browser.md) — SFTP file browser implementation details
