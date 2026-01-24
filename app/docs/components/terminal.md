# Terminal Emulator

A PTY-based shell emulator embedded in the Control Center desktop app, providing full interactive terminal access with bidirectional streaming between Rust and the webview.

## Architecture

```
Frontend (xterm.js)
      |
      | User types → term.onData()
      v
invoke('write_to_pty', { data })
      |
      v
Rust: write_to_pty() → writer.write_all()
      |
      v
PTY master fd → Shell process


Shell Output Flow:

Shell stdout/stderr
      |
      v
PTY master fd → reader thread
      |
      | Read loop: buf[4096]
      v
Channel<PtyEvent>::send(Data(string))
      |
      v
Frontend: onData.onmessage
      |
      v
xterm.js: term.write(data) → Screen


Resize Flow:

Window resize → FitAddon.fit()
      |
      v
term.onResize({ rows, cols })
      |
      v
invoke('resize_pty', { rows, cols })
      |
      v
master.resize(PtySize) → SIGWINCH to shell
```

## Purpose

Provides quick CLI access for running commands on local or remote systems without leaving the Control Center or switching to an external terminal app. Useful for SSH sessions to home lab devices, running maintenance commands, or checking system status.

## Key Concepts

- **PTY (Pseudo-Terminal)** — A kernel-level construct that emulates a terminal device. The master side is held by the Control Center, the slave side is connected to the shell's stdin/stdout/stderr.
- **Bidirectional Streaming** — Output flows from PTY reader thread through Tauri IPC channel to xterm.js. Input flows from xterm.js keyboard events through Tauri commands to PTY writer.
- **WebGL Rendering** — xterm.js uses GPU-accelerated WebGL rendering when available, with automatic fallback to DOM rendering.

## Components

### Shell Spawner (`spawn_shell`)

**File**: `control-center/src-tauri/src/lib.rs`

Spawns the user's default shell in a PTY and establishes bidirectional I/O streaming.

**Usage**:
```typescript
const onData = new Channel<PtyEvent>();
onData.onmessage = (msg) => {
  if (msg.event === 'data') term.write(msg.data);
  if (msg.event === 'exit') term.write('[Process exited]');
};

await invoke('spawn_shell', {
  onData,
  rows: term.rows,
  cols: term.cols
});
```

**Behavior**:
1. Opens a PTY with the specified size
2. Spawns the shell from `$SHELL` environment variable (defaults to `/bin/bash`)
3. Sets `TERM=xterm-256color` for proper color support
4. Splits PTY master into reader and writer
5. Stores writer in global state for input handling
6. Spawns reader thread that streams output to frontend via channel
7. Sends `PtyEvent::Exit` when shell process terminates

**Events**:
- `PtyEvent::Data(string)` — Shell output chunk (up to 4KB per event)
- `PtyEvent::Exit(i32)` — Shell process exited (0 = success, 1 = failure)

### Input Handler (`write_to_pty`)

**File**: `control-center/src-tauri/src/lib.rs`

Writes user input from the frontend to the PTY master.

**Usage**:
```typescript
term.onData((data) => invoke('write_to_pty', { data }));
```

**Behavior**:
- Retrieves the PTY writer from global state
- Writes the input string as bytes to the PTY master
- Flushes the writer to ensure immediate delivery
- Returns an error if the PTY is not open or writing fails

### Resize Handler (`resize_pty`)

**File**: `control-center/src-tauri/src/lib.rs`

Updates the PTY size when the terminal window is resized.

**Usage**:
```typescript
term.onResize(({ rows, cols }) => {
  invoke('resize_pty', { rows, cols });
});
```

**Behavior**:
- Updates the PTY's internal size metadata
- Sends `SIGWINCH` signal to the shell process
- Allows terminal apps (vim, less, htop) to redraw at the new size

### Frontend Terminal (`Terminal.svelte`)

**File**: `control-center/src/lib/components/Terminal.svelte`

Renders the xterm.js terminal and wires up IPC communication.

**Features**:
- **WebGL Rendering** — Uses `@xterm/addon-webgl` for GPU acceleration, falls back to DOM on failure
- **Auto-fit** — `@xterm/addon-fit` sizes the terminal to fill available space and handles window resize
- **Cursor Blink** — Visual feedback for terminal input
- **Color Theme** — Catppuccin Mocha palette matching the app's design

**Lifecycle**:
- `onMount` — Creates terminal, loads addons, spawns shell, wires input/output handlers
- `onDestroy` — Cleans up resize listener and disposes terminal instance

## Key Features

### Streaming Output
Output is streamed in chunks (up to 4KB) as the shell produces it, rather than buffering until newline. This provides real-time feedback for long-running commands and interactive applications.

### Process Lifecycle Management
The shell process runs independently of the terminal view. Switching from terminal to dashboard leaves the shell running in the background. Closing the app terminates the shell process.

### Ephemeral Sessions
Each time the terminal is opened, a new shell session is spawned. There is no session persistence across app restarts.

### Full Interactive Support
The terminal supports interactive applications (vim, nano, htop, ssh) with proper keyboard input, control sequences (Ctrl-C, Ctrl-D), and terminal resizing.

## Error Handling

All three Tauri commands (`spawn_shell`, `write_to_pty`, `resize_pty`) return `Result<(), String>`. Errors are logged to the console but not displayed to the user, as terminal errors are typically visible in the terminal output itself (e.g., "command not found", "permission denied").

## Configuration

### Shell Selection
The terminal spawns the user's default shell from the `$SHELL` environment variable. If `$SHELL` is not set, it defaults to `/bin/bash`.

### Terminal Size
Initial size is determined by xterm.js based on available screen space and font metrics. The size is passed to `spawn_shell` when creating the PTY.

### Environment Variables
The only environment variable explicitly set is `TERM=xterm-256color`, which enables color support in the shell and terminal applications.

## Dependencies

### External
- **User's shell** — Typically `zsh` on modern macOS, `bash` on Linux. Must be available at the path specified by `$SHELL`.

### Rust Crates
- `portable-pty` — Cross-platform PTY creation and process spawning
- `tauri` — IPC layer (commands, channels, state management)

### Frontend
- `@xterm/xterm` — Core terminal emulator
- `@xterm/addon-fit` — Automatic terminal sizing
- `@xterm/addon-webgl` — GPU-accelerated rendering

## See Also

- [tailscale.md](tailscale.md) — Tailscale integration component
- [../architecture.md](../architecture.md) — Overall system architecture and design decisions
