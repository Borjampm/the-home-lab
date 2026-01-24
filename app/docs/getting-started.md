# Getting Started

This guide walks you through setting up and running the Control Center desktop application.

## Prerequisites

Before you begin, ensure you have the following installed:

### Required

- **Node.js 18+** and npm
- **Rust** (latest stable) — Install via [rustup.rs](https://rustup.rs)
- **Tailscale CLI** — Must be installed and available in PATH
  - macOS: `brew install tailscale` or download from [tailscale.com](https://tailscale.com/download)
  - Linux: Follow [Tailscale installation instructions](https://tailscale.com/download/linux)

### Platform Support

- **macOS** (Intel and Apple Silicon)
- **Linux** (x86_64)
- **Windows is NOT supported** (PTY implementation is POSIX-only)

## Installation

Clone the repository and install dependencies:

```bash
git clone https://github.com/yourusername/the-home-lab.git
cd the-home-lab/app/control-center

# Install frontend dependencies
npm install
```

The Rust dependencies will be fetched automatically when you build or run the app.

## Development

Run the app in development mode with hot-reload:

```bash
npm run tauri dev
```

This will:
1. Start the Vite dev server on `http://localhost:1420`
2. Compile the Rust backend
3. Launch the desktop app with the frontend loaded in a native webview
4. Watch for changes and hot-reload the frontend automatically
5. Recompile Rust code when changes are detected in `src-tauri/src/`

## Production Build

Build a production bundle:

```bash
npm run tauri build
```

This creates:
- **macOS**: `.dmg` installer in `src-tauri/target/release/bundle/dmg/`
- **Linux**: `.deb`, `.AppImage`, or other formats in `src-tauri/target/release/bundle/`

The bundled app is a standalone executable that does not require Node.js or Rust toolchains.

## First Launch

When you launch the Control Center for the first time:

### Dashboard View

The default view shows:
- **Tailscale Status** — Current connection state (Connected/Disconnected)
- **Device List** — All devices in your Tailscale network with hostname, IPs, and connection status
- **Network Controls** — Connect/disconnect button and refresh

### Terminal

Click the Terminal button to switch to an embedded terminal:
- Opens your default shell (`$SHELL` environment variable, defaults to `/bin/bash`)
- Full interactive terminal with PTY support
- Click "Dashboard" to switch back

## Troubleshooting

### Tailscale CLI Not Found

**Symptom**: Error message about failing to run `tailscale status --json`.

**Solution**:
1. Verify Tailscale is installed: `which tailscale`
2. If not found, install Tailscale CLI (see Prerequisites)
3. Ensure the `tailscale` binary is in your PATH
4. Restart the app

**Note**: The Mac App Store version of Tailscale includes the CLI.

### No Devices Showing

**Symptom**: Dashboard loads but shows zero devices.

**Solution**:
1. Verify you're connected to Tailscale: `tailscale status`
2. If not connected, run: `tailscale up`
3. Click the Refresh button in the dashboard

### Build Errors

**Symptom**: `npm run tauri build` fails with Rust compilation errors.

**Solution**:
1. Update Rust to the latest stable: `rustup update`
2. Clear the build cache: `cd src-tauri && cargo clean`
3. Rebuild: `npm run tauri build`

If you encounter errors related to `portable-pty`, ensure you're on a supported platform (macOS or Linux).

## Next Steps

- Read [architecture.md](architecture.md) to understand the high-level system design
- Explore [components/](components/) for detailed documentation on the Terminal and Tailscale integrations
