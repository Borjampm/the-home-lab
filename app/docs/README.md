# Documentation

Reference documentation for the Control Center application.

## What is the Control Center?

A desktop app for managing home lab systems. Displays Tailscale network status and provides terminal access. Built with Tauri 2 (Rust backend, SvelteKit frontend).

## Index

- [guides/](guides/) — Step-by-step guides for setup and operations
- [architecture.md](architecture.md) — High-level system design, data flow, and technology decisions
- [components/](components/) — Component documentation (Terminal, Tailscale)
- [design/](design/) — Visual design system (NASA Mission Control aesthetic)

## Quick Reference

### Core Systems
- **Dashboard** — Displays Tailscale network devices, connection state, and network controls
- **Terminal** — Embedded PTY-based shell with full interactive support

### Integration Points
- Tailscale CLI (shelled out for network management)
- portable-pty (PTY process spawning and I/O)
- xterm.js (terminal rendering in webview)

### Target Platforms
macOS and Linux only (no Windows support)
