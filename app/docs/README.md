# Documentation

Reference documentation for the Control Center application.

## What is the Control Center?

A desktop app for managing home lab systems. Displays Tailscale network status, provides terminal access, and enables file browsing across devices. Built with Tauri 2 (Rust backend, SvelteKit frontend).

## Index

- [guides/](guides/) — Step-by-step guides for setup and operations
- [architecture.md](architecture.md) — High-level system design, data flow, and technology decisions
- [components/](components/) — Component documentation (Terminal, Tailscale, File Browser)
- [design/](design/) — Visual design system (NASA Mission Control aesthetic)

## Quick Reference

### Core Systems
- **Dashboard** — Displays Tailscale network devices, connection state, and network controls
- **Terminal** — Embedded PTY-based shell with full interactive support
- **File Browser** — SFTP-based file management with dual-pane view and transfers

### Integration Points
- Tailscale CLI (shelled out for network management)
- portable-pty (PTY process spawning and I/O)
- xterm.js (terminal rendering in webview)
- russh + russh-sftp (SSH/SFTP for file operations)

### Target Platforms
macOS and Linux only (no Windows support)
