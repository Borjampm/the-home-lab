# Tailscale Integration

A CLI-based integration layer for monitoring Tailscale VPN network status and managing connections from within the Control Center.

## Architecture

```
Frontend Dashboard
      |
      | invoke('get_tailscale_status')
      v
get_tailscale_status()
      |
      | Command::new("tailscale")
      |   .args(["status", "--json"])
      v
Parse JSON → Transform → TailscaleStatus
      |
      v
Return { connected, devices[], error }
      |
      v
Render device list in UI


Connection Toggle Flow:

User clicks Connect/Disconnect
      |
      | invoke('tailscale_up') or invoke('tailscale_down')
      v
Command::new("tailscale").args(["up"]) or .args(["down"])
      |
      v
Return Result<(), String>
      |
      v
Refresh status → Update UI
```

## Purpose

Provides visibility into which home lab devices are currently connected to the Tailscale network and enables users to connect or disconnect without opening a terminal. This integration surfaces network state directly in the Control Center dashboard.

## Key Design Decision: CLI over Library

The integration shells out to the `tailscale` command-line tool rather than using the `tailscale` Rust crate or Unix socket API. This choice was made because the Mac App Store version of Tailscale does not expose a Unix socket, making the CLI the only universally available integration point across distribution methods (standalone installer vs. App Store).

## Components

### Status Query (`get_tailscale_status`)

**File**: `control-center/src-tauri/src/tailscale.rs`

Executes `tailscale status --json` and transforms the output into a frontend-friendly structure.

**Usage**:
```typescript
const status = await invoke<TailscaleStatus>('get_tailscale_status');
// Returns: { connected: boolean, devices: TailscaleDevice[], error: string | null }
```

**Returned Data**:
- `connected` — Whether Tailscale is running (backend state == "Running")
- `devices` — Array of network peers including self
  - `hostname` — Device name
  - `dnsName` — Tailscale DNS name (e.g., `device.tailnet-name.ts.net`)
  - `os` — Operating system
  - `ips` — Tailscale IP addresses
  - `online` — Current connection status
  - `isSelf` — Whether this is the current device
  - `relay` — DERP relay server if not directly connected
  - `lastSeen` — Timestamp of last activity (if offline)
- `error` — Error message if the command fails or cannot be parsed

**Device Sorting**:
Devices are sorted to display:
1. Self first
2. Online devices before offline
3. Alphabetically by hostname within each group

### Connection Control

**File**: `control-center/src-tauri/src/tailscale.rs`

Two commands for managing connection state:

**`tailscale_up`**:
```typescript
await invoke('tailscale_up');
// Executes: tailscale up
// Returns: Result<(), String>
```

**`tailscale_down`**:
```typescript
await invoke('tailscale_down');
// Executes: tailscale down
// Returns: Result<(), String>
```

Both commands return a Result type. On success, they return `Ok(())`. On failure, they return `Err(message)` with the stderr output from the CLI command.

## Error Handling

Errors are captured at three levels:
1. **Command execution failure** — `tailscale` binary not found or not executable
2. **Non-zero exit code** — `tailscale` command failed (e.g., not logged in, network error)
3. **JSON parsing failure** — Unexpected output format from `tailscale status --json`

All errors are returned as strings in the `error` field of `TailscaleStatus` or as `Result<(), String>` for connection commands. The frontend displays these errors to the user.

## Dependencies

### External
- **Tailscale CLI** — Must be installed and available in `PATH`. The app does not function without it.

### Rust Crates
- `serde`, `serde_json` — JSON parsing and serialization
- `std::process::Command` — CLI execution
- `tauri::command` — IPC command registration

## See Also

- [terminal.md](terminal.md) — Terminal emulator component
- [../architecture.md](../architecture.md) — Overall system architecture and design decisions
