# Design System

Visual design documentation for the Control Center application.

## Overview

The Control Center uses a **NASA Mission Control** aesthetic — a retro interface inspired by 1960s–1980s space program control rooms. The design evokes phosphor CRT monitors, hardware control panels, and the functional density of real mission control interfaces.

Key characteristics:
- Pure black background with amber/green phosphor colors
- CRT effects (scanlines, text glow)
- Uppercase typography throughout
- Device-specific color coding that persists to terminal windows
- Industrial aesthetic with sharp corners and bordered panels

## Index

- [visual-identity.md](visual-identity.md) — Complete visual design specification
  - Color palette and usage
  - Typography guidelines
  - CRT effects implementation
  - Component patterns (buttons, cards, status indicators)
  - Terminal theming
  - Extension guidelines

## Quick Reference

### Primary Colors

| Role | Hex |
|------|-----|
| Background | `#000000` |
| Primary (Amber) | `#FFAA00` |
| Secondary | `#AA7700` |
| Online | `#00FF41` |
| Offline | `#882222` |
| Error | `#FF4444` |

### Device Colors (8-color rotation)

```
#FFAA00  #00FF41  #00DDFF  #FF6600
#AA88FF  #FF0066  #88FF00  #00FFAA
```

### Design Principles

1. Functional over decorative
2. Information density
3. CRT authenticity
4. Color as identity
5. Uppercase authority
