# Components

Documentation for Control Center components — the functional subsystems that make up the application.

## Overview

The Control Center is composed of three main components that handle distinct responsibilities:

- **Terminal** — Interactive shell emulator embedded in the desktop app
- **Tailscale** — Network management integration via CLI
- **File Browser** — SFTP-based file management for remote and local devices

Each component operates independently and exposes commands to the frontend via Tauri's IPC layer.

## Component Index

- [terminal.md](terminal.md) — PTY-based terminal emulator with bidirectional streaming
- [tailscale.md](tailscale.md) — Tailscale VPN status monitoring and connection management
- [file-browser.md](file-browser.md) — SFTP file browser with dual-pane view, transfers, and bookmarks

## Architecture Context

For high-level system design and data flow diagrams, see [architecture.md](../architecture.md).
