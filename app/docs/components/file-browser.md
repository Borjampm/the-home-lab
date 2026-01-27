# File Browser

An SFTP-based file browser for browsing, transferring, and managing files on remote Tailscale devices and the local machine.

## Architecture

```
Frontend (FileBrowser.svelte)
      |
      | invoke('sftp_connect', { deviceIp })
      v
Rust: sftp_connect()
      |
      | russh::client::connect()
      v
SSH Connection → Authentication (agent or key file)
      |
      | SftpSession::new(channel)
      v
SFTP Session stored in SftpState


File Operations Flow:

User action → invoke('sftp_*', { deviceIp, ... })
      |
      v
Rust: Lookup connection in SftpState
      |
      v
russh-sftp: Execute operation (read_dir, open, create, etc.)
      |
      v
Return result → Update frontend state


Transfer Flow (Download):

User selects file → invoke('sftp_download', { ..., onProgress })
      |
      v
Rust: Create CancellationToken, register in transfers
      |
      v
Open remote file → Read in 64KB chunks
      |   |
      |   v Progress: Channel<TransferEvent>::send(Progress { ... })
      v
Write to local file → Complete → send(Complete { ... })
      |
      v
Frontend: TransferQueue updates via Channel callback


Local File Operations (Self):

User browses self device → invoke('local_list_dir', { path })
      |
      v
Rust: std::fs operations (no SSH/SFTP)
      |
      v
Return DirectoryListing → Render in FileList
```

## Purpose

Provides visual file management across home lab devices without SSH/SCP command-line knowledge. Enables file transfers between local and remote machines, quick file previews, and navigation bookmarks for frequently accessed directories.

## Key Design Decision: russh + russh-sftp

The SFTP implementation uses `russh` and `russh-sftp` crates rather than shelling out to CLI tools or using OpenSSH libraries. This choice was made for several reasons:

1. **Pure Rust** - No external dependencies on OpenSSH binaries. Works consistently across macOS and Linux without system-specific library linking issues.

2. **Async-native** - Built on tokio, integrating naturally with Tauri's async command model. Enables non-blocking file transfers with progress streaming.

3. **SSH Agent Support** - Connects to the user's running SSH agent via `SSH_AUTH_SOCK`, allowing authentication with keys that require passphrases without prompting in the app.

4. **Fallback Key File Auth** - When SSH agent is unavailable, attempts passwordless authentication with `~/.ssh/id_ed25519` or `~/.ssh/id_rsa`.

5. **Connection Reuse** - Maintains persistent SFTP sessions per device, avoiding reconnection overhead for subsequent operations.

## Components

### Backend (`sftp.rs`)

**Connection Management:**
- `sftp_connect` - Establishes SSH connection, authenticates, opens SFTP subsystem
- `sftp_disconnect` - Closes connection and removes from state

**Directory Operations:**
- `sftp_list_dir` - Lists directory contents with metadata (size, permissions, timestamps)
- `sftp_mkdir` - Creates new directories

**File Operations:**
- `sftp_read_file` - Reads file content (up to 1MB by default) for Quick Look preview
- `sftp_delete` - Deletes files or directories (with recursive option)
- `sftp_rename` - Renames or moves files/directories

**Transfers:**
- `sftp_download` - Downloads file with progress streaming via Tauri Channel
- `sftp_upload` - Uploads file with progress streaming
- `sftp_cancel_transfer` - Cancels an in-progress transfer via CancellationToken

**Bookmarks:**
- `sftp_get_bookmarks` - Loads bookmarks from app data directory
- `sftp_save_bookmark` - Saves new or updates existing bookmark
- `sftp_delete_bookmark` - Removes a bookmark

**Local Operations (for self device):**
- `local_list_dir`, `local_read_file`, `local_mkdir`, `local_delete`, `local_rename`

### Frontend

**FileBrowser.svelte** - Main container, orchestrates all sub-components and handles:
- Connection lifecycle (connect on mount, disconnect on destroy)
- Navigation history (back/forward/up)
- File selection and multi-select
- Context menu actions (open, rename, delete, download, etc.)
- Dialog management (rename, new folder)

**FileList.svelte** - Renders file grid/list view with:
- Selection handling (click, shift-click, cmd-click)
- Native file drop support (upload from Finder)
- Keyboard navigation and spacebar Quick Look

**DeviceSidebar.svelte** - Shows:
- Available Tailscale devices with online status
- Bookmarks for current device
- Drop zone for creating bookmarks

**DualPane.svelte** - Split view with:
- Resizable divider (20-80% range)
- Independent left/right directory navigation

**TransferQueue.svelte** - Shows active and completed transfers:
- Progress bars with byte counts
- Cancel button for active transfers
- Clear completed option

**QuickLook.svelte** - File preview modal:
- Text/code file content display
- File metadata (size, modified date)
- Keyboard dismiss (Space or Escape)

**PathBreadcrumb.svelte** - Navigation bar with:
- Clickable path segments
- Back/Forward/Up buttons

**ContextMenu.svelte** - Right-click menu with file operations

## Key Features

### Dual-Pane Browser
Toggle split view to see two directories simultaneously. Useful for comparing contents or organizing files. Each pane maintains independent navigation state.

### Dotfiles Toggle
Show or hide hidden files (names starting with `.`) via toolbar button. Preference is per-session; not persisted.

### File Transfers with Progress
Uploads and downloads stream progress events to the frontend. The transfer queue shows real-time progress bars, byte counts, and completion status. Transfers can be cancelled mid-operation.

### Bookmarks
Save frequently accessed directories for quick navigation. Bookmarks are stored per-device in `~/Library/Application Support/com.homelab.control-center/bookmarks.json` (macOS) or `~/.local/share/com.homelab.control-center/bookmarks.json` (Linux).

### Quick Look Preview
Spacebar or double-click opens a modal preview of text files (up to 100KB). Recognizes code files for syntax-appropriate display. Binary files show a placeholder.

### Open Terminal at Location
Context menu option opens a new terminal window at the selected directory. For remote devices, spawns an SSH session to that path.

### Multi-Device Support
Sidebar shows all Tailscale devices. Clicking a different device opens a new browser window for that host while keeping the current window intact.

## Authentication

Authentication follows this order:
1. **SSH Agent** - Connects to `SSH_AUTH_SOCK` if set, requests identities from agent, attempts authentication with each
2. **Key Files** - Falls back to `~/.ssh/id_ed25519` then `~/.ssh/id_rsa` without passphrase

If both methods fail, connection fails with an error message suggesting the user ensure SSH agent is running or unencrypted key files exist.

**Note:** Password authentication and passphrase-protected key files (without agent) are not supported.

## Error Handling

Errors surface at multiple levels:
- **Connection errors** - Displayed in error bar, prevents file listing
- **Operation errors** - Displayed in error bar, allows retry
- **Transfer errors** - Shown in transfer queue with `ERR` status

All SFTP commands return `Result<T, String>` where the error string contains the underlying error message from russh-sftp.

## State Management

**SftpState** (Rust):
- `connections: HashMap<String, SftpConnection>` - Active SFTP sessions keyed by device IP
- `transfers: HashMap<String, TransferJob>` - Active transfers with cancellation tokens

**Frontend State** (Svelte):
- `currentPath`, `files`, `selectedFiles` - Current directory state
- `historyBack`, `historyForward` - Navigation history
- `transfers` - Transfer queue with progress updates
- `bookmarks` - Loaded bookmarks for current device

## Dependencies

### Rust Crates
- `russh` - SSH client protocol implementation
- `russh-sftp` - SFTP protocol on top of russh
- `tokio` - Async runtime for non-blocking I/O
- `tokio-util` - CancellationToken for transfer cancellation
- `directories` - Cross-platform app data directory resolution
- `thiserror` - Error type derivation

### Frontend
- `@tauri-apps/api/core` - invoke, Channel for IPC
- `@tauri-apps/api/webviewWindow` - Open new windows
- `@tauri-apps/plugin-dialog` - Native file/folder dialogs
- `@tauri-apps/plugin-clipboard-manager` - Copy path to clipboard
- `@tauri-apps/plugin-fs` - Local file copy operations

## See Also

- [terminal.md](terminal.md) - Terminal emulator (can be opened from file browser context menu)
- [tailscale.md](tailscale.md) - Tailscale integration (provides device list)
- [../architecture.md](../architecture.md) - Overall system architecture
