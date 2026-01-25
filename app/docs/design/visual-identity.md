# Visual Identity: NASA Mission Control

The Control Center uses a retro NASA mission control aesthetic inspired by 1960s–1980s space program interfaces. This document defines the visual language to ensure consistency across all current and future UI work.

## Design Philosophy

### Core Principles

1. **Functional over decorative** — Every visual element serves a purpose. Glow effects indicate active/important states, not mere decoration.

2. **Information density** — Mission control rooms displayed vast amounts of data. The UI should feel data-rich without being cluttered.

3. **CRT authenticity** — Simulate the characteristics of phosphor displays: text glow, scanlines, and color bleeding at edges.

4. **Color as identity** — Each device has a unique color that persists across all interactions, creating instant recognition.

5. **Uppercase authority** — All-caps text evokes the teletype and early computer terminal aesthetic while improving scanability of labels.

6. **Readability over decoration** — Never sacrifice text legibility for aesthetic effect. Terminal text is always high-contrast amber on dark background, regardless of device color.

### Visual Metaphor

The interface represents a command center for home infrastructure. Users are "operators" monitoring and controlling remote systems. The aesthetic reinforces this through:

- Status indicators that glow like physical LEDs
- Bordered panels resembling hardware modules
- Monospace typography throughout
- Color-coded channels (each device is a "channel")

---

## Color System

### Background & Base

| Role | Hex | Usage |
|------|-----|-------|
| Background | `#000000` | Primary surface, pure black for maximum contrast |
| Panel background | `rgba(0,0,0,0.5)` | Semi-transparent for layered elements |
| Border default | `rgba(255,170,0,0.2)` | Subtle amber borders for structure |
| Border emphasis | `rgba(255,170,0,0.3)` | Stronger borders for headers/dividers |

### Primary Palette (Amber)

The default interface color is amber, evoking classic monochrome monitors.

| Role | Hex | Usage |
|------|-----|-------|
| Primary | `#FFAA00` | Main text, active elements, default device color |
| Primary dimmed | `#AA7700` | Secondary text, metadata, inactive labels |
| Primary glow | `rgba(255,170,0,0.5)` | Text shadow, hover states |
| Primary subtle | `rgba(255,170,0,0.15)` | Button backgrounds, hover fills |

### Status Colors

| Status | Hex | Glow | Usage |
|--------|-----|------|-------|
| Online/Success | `#00FF41` | `rgba(0,255,65,0.6)` | Connected states, success indicators |
| Offline/Inactive | `#882222` | `rgba(136,34,34,0.8)` | Disconnected, unavailable |
| Error | `#FF4444` | `rgba(255,68,68,0.4)` | Errors, destructive actions |

### Device Color Palette

Devices cycle through these 8 colors in order. Each color represents a unique "channel" for that device.

| Index | Name | Hex | Terminal BG (15%) |
|-------|------|-----|-------------------|
| 0 | Amber | `#FFAA00` | `#261a00` |
| 1 | Green | `#00FF41` | `#002609` |
| 2 | Cyan | `#00DDFF` | `#002126` |
| 3 | Orange | `#FF6600` | `#260f00` |
| 4 | Violet | `#AA88FF` | `#191326` |
| 5 | Magenta | `#FF0066` | `#26000f` |
| 6 | Lime | `#88FF00` | `#132600` |
| 7 | Teal | `#00FFAA` | `#002619` |

**Color assignment rule:** Device at index `i` gets color `DEVICE_COLORS[i % 8]`. This ensures consistent assignment based on array position.

### Darkening Algorithm

Terminal backgrounds use a darkened version of the device color at 15% brightness:

```
darkenColor(hex, factor = 0.85):
  r, g, b = parseHex(hex)
  return rgb(r * 0.15, g * 0.15, b * 0.15)
```

The factor `0.85` means we keep `1 - 0.85 = 0.15` (15%) of the original brightness.

---

## Typography

### Font Stack

```css
font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
```

Menlo is the primary typeface. The stack ensures fallback to other monospace fonts on systems without Menlo.

### Text Styles

| Element | Size | Weight | Letter-spacing | Transform |
|---------|------|--------|----------------|-----------|
| Page title | 18px | 600 | 3px | uppercase |
| Section headers | 13px | 600 | 1px | uppercase |
| Body text | 12px | 400 | 0 | none |
| Labels/metadata | 11px | 400 | 1px | uppercase |
| Buttons | 10–11px | 400 | 1px | uppercase |
| Badges | 9px | 600 | 1px | uppercase |

### Text Glow

Important text uses `text-shadow` for the phosphor glow effect:

```css
/* Primary glow (amber) */
text-shadow: 0 0 10px rgba(255, 170, 0, 0.5),
             0 0 20px rgba(255, 170, 0, 0.3);

/* Subtle glow (for smaller text) */
text-shadow: 0 0 8px rgba(255, 170, 0, 0.4);
```

For device-specific colors, use CSS `color-mix()`:

```css
text-shadow: 0 0 8px color-mix(in srgb, var(--device-color) 50%, transparent);
```

---

## CRT Effects

### Scanlines

A fixed overlay that simulates CRT scanlines:

```css
.scanlines {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 1000;
  background: repeating-linear-gradient(
    0deg,
    rgba(0, 0, 0, 0.15) 0px,
    rgba(0, 0, 0, 0.15) 1px,
    transparent 1px,
    transparent 2px
  );
}
```

**Key properties:**
- `pointer-events: none` — Allows clicking through the overlay
- `z-index: 1000` — Sits above all content
- 2px repeating pattern — Creates subtle horizontal lines

### Glow Effects

Glow is applied via `box-shadow` for elements and `text-shadow` for text:

```css
/* Status LED glow */
box-shadow: 0 0 8px rgba(0, 255, 65, 0.6),
            0 0 16px rgba(0, 255, 65, 0.3);

/* Button hover glow */
text-shadow: 0 0 12px rgba(255, 170, 0, 0.6);
```

**Glow intensity guidelines:**
- Active/online states: Strong glow (0.6 opacity inner, 0.3 outer)
- Hover states: Medium glow (0.4–0.6 opacity)
- Static elements: Subtle or no glow

---

## Component Patterns

### Buttons

Buttons are transparent with colored borders, filling on hover.

**Default button:**
```css
.btn {
  background: transparent;
  color: #FFAA00;
  border: 1px solid rgba(255, 170, 0, 0.5);
  border-radius: 0;  /* Sharp corners */
  padding: 6px 12px;
  font-size: 11px;
  letter-spacing: 1px;
  text-shadow: 0 0 8px rgba(255, 170, 0, 0.4);
}

.btn:hover {
  background: rgba(255, 170, 0, 0.15);
  border-color: #FFAA00;
}
```

**Button variants:**

| Variant | Background | Border | Text color |
|---------|------------|--------|------------|
| Default | transparent | `rgba(primary, 0.5)` | primary |
| Primary | `rgba(primary, 0.2)` | primary | primary |
| Success/Connect | `rgba(green, 0.15)` | `rgba(green, 0.5)` | green |
| Danger/Disconnect | `rgba(882222, 0.3)` | `rgba(error, 0.5)` | error |
| Device-colored | `rgba(0,0,0,0.5)` | device color | device color |

### Status Indicators

Small circular LEDs that indicate online/offline state.

```css
.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #882222;  /* Offline: dim red */
  box-shadow: 0 0 6px rgba(136, 34, 34, 0.8);
}

.status-indicator.online {
  background: #00FF41;  /* Online: bright green */
  box-shadow: 0 0 8px rgba(0, 255, 65, 0.6),
              0 0 16px rgba(0, 255, 65, 0.3);
}
```

Smaller dots (6px) used inline with device names follow the same pattern.

### Cards

Device cards use a left-border accent in the device's assigned color:

```css
.device-card {
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(255, 170, 0, 0.2);
  border-left: 3px solid var(--device-color);
  padding: 12px 16px;
}
```

The `--device-color` CSS variable is set inline via the `style` attribute.

### Panels & Containers

Panels use subtle amber borders to create structure:

```css
.panel {
  background: rgba(255, 170, 0, 0.05);
  border: 1px solid rgba(255, 170, 0, 0.2);
  padding: 10px 14px;
}
```

Headers/dividers use slightly stronger borders:
```css
border-bottom: 1px solid rgba(255, 170, 0, 0.3);
```

### Badges

Small labels for metadata (like "SELF" badge):

```css
.badge {
  font-size: 9px;
  font-weight: 600;
  letter-spacing: 1px;
  background: var(--device-color);
  color: #000000;
  padding: 2px 6px;
}
```

Badges always use the device color as background with black text for contrast.

### Terminal Chrome

Terminal windows use a "chrome" wrapper that separates device identity (colored border/header) from terminal content (always amber text). This ensures readability while maintaining device recognition.

**Structure:**
```
┌─ HOSTNAME ────────────────────────── ● ACTIVE ─┐  ← Colored border + header
│                                                 │
│   Terminal content (always amber on black)     │
│                                                 │
└─────────────────────────────────────────────────┘
```

**Chrome container:**
```css
.terminal-chrome {
  border: 1px solid var(--theme-color);
  background: var(--bg-color);  /* Darkened device color */
}
```

**Header bar:**
```css
header {
  padding: 8px 12px;
  border-bottom: 1px solid color-mix(in srgb, var(--theme-color) 30%, transparent);
  background: color-mix(in srgb, var(--theme-color) 10%, transparent);
}

.terminal-title {
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 2px;
  color: var(--theme-color);
  text-shadow: 0 0 8px color-mix(in srgb, var(--theme-color) 50%, transparent);
}
```

**Status indicator:** Green "ACTIVE" label with glowing dot, always green regardless of device color.

### Vignette Effect

Terminal windows use a radial gradient overlay to darken edges, creating a CRT tube effect:

```css
.vignette {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 999;
  background: radial-gradient(
    ellipse at center,
    transparent 50%,
    rgba(0, 0, 0, 0.4) 100%
  );
}
```

---

## Terminal Theming

When opening a terminal window, the device's color is used for chrome (border, header) while terminal content always uses amber for readability.

### URL Parameters

The terminal page accepts these query parameters:
```
/terminal?id={id}&color={encodeURIComponent(color)}&host={encodeURIComponent(hostname)}
```

| Parameter | Description |
|-----------|-------------|
| `id` | Unique terminal identifier for PTY management |
| `color` | Device color (hex) — used for chrome styling |
| `host` | Device hostname — displayed in header bar |

### xterm.js Theme

Terminal text always uses fixed amber for maximum readability:

```javascript
{
  background: '#0a0a0a',                    // Near-black, neutral
  foreground: '#FFAA00',                    // Always amber
  cursor: '#FFAA00',                        // Amber cursor
  cursorAccent: '#0a0a0a',                 // Cursor text color
  selectionBackground: '#FFAA0040',        // 25% opacity selection
}
```

**Why fixed amber?** Using device color for text creates poor contrast for some colors (e.g., green text on dark green background). Separating identity (chrome) from content (text) ensures all terminals are readable.

### Chrome Styling

The chrome wrapper uses the device color via CSS variables:
```svelte
<main style="--theme-color: {themeColor}; --bg-color: {backgroundColor}">
```

- Border: `1px solid var(--theme-color)`
- Header background: `color-mix(in srgb, var(--theme-color) 10%, transparent)`
- Title text: `var(--theme-color)` with glow

---

## Implementation Reference

### CSS Custom Properties

Use CSS variables for device-specific theming:

```css
/* Set on parent element */
style="--device-color: #00FF41"

/* Use in children */
color: var(--device-color);
border-color: var(--device-color);
text-shadow: 0 0 8px color-mix(in srgb, var(--device-color) 50%, transparent);
```

### Color Utilities

The `darkenColor` function is used in both `terminal/+page.svelte` and `Terminal.svelte`:

```typescript
function darkenColor(hex: string, factor: number = 0.85): string {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  const darkenedR = Math.round(r * (1 - factor));
  const darkenedG = Math.round(g * (1 - factor));
  const darkenedB = Math.round(b * (1 - factor));
  return `#${darkenedR.toString(16).padStart(2, '0')}${darkenedG.toString(16).padStart(2, '0')}${darkenedB.toString(16).padStart(2, '0')}`;
}
```

### Device Color Array

Located in `src/routes/+page.svelte`:

```typescript
const DEVICE_COLORS = [
  '#FFAA00', '#00FF41', '#00DDFF', '#FF6600',
  '#AA88FF', '#FF0066', '#88FF00', '#00FFAA'
];

function getDeviceColor(index: number): string {
  return DEVICE_COLORS[index % DEVICE_COLORS.length];
}
```

---

## Extending the Design

### Adding New Components

When creating new UI elements:

1. **Use the existing color palette** — Don't introduce new colors without updating this document
2. **Apply glow appropriately** — Active states glow, static states don't
3. **Maintain uppercase convention** — All labels and headers are uppercase
4. **Use sharp corners** — `border-radius: 0` for the industrial aesthetic
5. **Keep backgrounds dark** — Pure black or near-black with low-opacity tints

### Adding New Device Types

If adding device types that need visual distinction beyond color:

- Consider icon shapes (future enhancement)
- Use the existing 8-color palette
- Document any new visual patterns here

### Adding New Status States

For states beyond online/offline:

| State | Suggested Color | Hex |
|-------|-----------------|-----|
| Warning | Amber | `#FFAA00` |
| Updating | Cyan | `#00DDFF` |
| Connecting | Amber pulse | `#FFAA00` (animated) |

### Animation Guidelines

Animations should be subtle and functional:

- **Transitions:** 0.15s ease for hover states
- **Pulse:** Reserved for "connecting" or "processing" states
- **No decorative animations** — Motion should indicate state change

---

## File Locations

| File | Contains |
|------|----------|
| `src/routes/+page.svelte` | Dashboard styles, device colors, scanlines |
| `src/routes/terminal/+page.svelte` | Terminal chrome, scanlines, vignette, header bar |
| `src/lib/components/Terminal.svelte` | xterm.js configuration, fixed amber theme |

All visual styling is currently co-located with components. If the design system grows, consider extracting to:
- `src/lib/styles/theme.css` — Shared CSS variables
- `src/lib/utils/colors.ts` — Color utilities (darken, lighten, etc.)
