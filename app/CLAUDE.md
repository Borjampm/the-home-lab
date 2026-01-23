# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The Home Lab is a personal home lab project for connecting computers and utilities. This `app/` directory contains the **Control Center** — a desktop terminal application for managing the home lab.

## Repository Structure

The parent repository (`the-home-lab/`) contains:
- `app/` — Application code (this directory)
- `research/` — Research agents and reports (has its own CLAUDE.md with research methodology guidelines)

The application lives in `control-center/`:
- `control-center/src/` — SvelteKit frontend (xterm.js terminal UI)
- `control-center/src-tauri/` — Rust backend (PTY management, Tauri commands)

## Tech Stack

- **Tauri 2** — Desktop app framework (Rust backend + native webview)
- **SvelteKit** — Frontend framework (static adapter, SSR disabled)
- **xterm.js** — Terminal emulator in the webview (`@xterm/xterm`, `@xterm/addon-fit`, `@xterm/addon-webgl`)
- **portable-pty** — Rust PTY library for spawning and managing shell processes

Target platforms: **macOS and Linux only** (no Windows support).

## Architecture

The app spawns a PTY (pseudo-terminal) on the Rust side and streams data to xterm.js via Tauri IPC channels:

- `spawn_shell` — Creates a PTY, spawns the user's `$SHELL`, streams output via `Channel<PtyEvent>`
- `write_to_pty` — Forwards user keystrokes from xterm.js to the PTY master
- `resize_pty` — Syncs terminal dimensions when the window resizes

## Commands

```bash
cd control-center

# Development (hot-reload frontend, recompiles Rust on change)
npm run tauri dev

# Production build
npm run tauri build

# Install frontend dependencies
npm install
```

## Documentation Convention

Documentation uses an index-based architecture:
- Each folder contains a `README.md` that serves as an index for that directory
- The README links to files in the same folder and/or routes to subdirectories
- Each README includes a brief description of what the folder contains
- Subdirectories follow the same pattern with their own `README.md`

When starting work on a task, check `docs/` for existing documentation that may provide relevant context.

## Maintaining This File

As we work together, proactively propose updates to this CLAUDE.md when you notice:
- Project conventions or patterns emerging from the code
- User preferences for tooling, style, or workflow
- Architecture decisions worth preserving for future sessions
- New build/test/lint commands as they are introduced
- Any recurring context that would help future instances ramp up faster
