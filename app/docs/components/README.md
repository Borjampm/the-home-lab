# Components

Documentation for Control Center components — the functional subsystems that make up the application.

## Overview

The Control Center is composed of two main components that handle distinct responsibilities:

- **Terminal** — Interactive shell emulator embedded in the desktop app
- **Tailscale** — Network management integration via CLI

Each component operates independently and exposes commands to the frontend via Tauri's IPC layer.

## Component Index

- [terminal.md](terminal.md) — PTY-based terminal emulator with bidirectional streaming
- [tailscale.md](tailscale.md) — Tailscale VPN status monitoring and connection management

## Architecture Context

For high-level system design and data flow diagrams, see [architecture.md](../architecture.md).
