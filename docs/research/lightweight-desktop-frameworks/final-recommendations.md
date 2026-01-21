# Lightweight Desktop Frameworks: Final Recommendations

## Executive Summary

For building lightweight, fast desktop applications on Linux and macOS for home lab and personal projects, **Tauri** emerges as the top recommendation. It delivers the best balance of small binary size (2.5-10MB), low memory usage (30-40MB idle), fast startup (<500ms), and production maturity (102K GitHub stars, used by ~90 companies). For developers who prefer pure systems languages without web technologies, **Wails** (Go) offers comparable lightness (4-8MB binaries) with an easier learning curve, while **egui** (Rust) provides the fastest prototyping experience with immediate-mode rendering.

The research reveals a clear hierarchy: Rust frameworks lead in performance and binary size optimization, Go frameworks excel in developer experience and build simplicity, while traditional C++/native frameworks (Qt, wxWidgets) remain the gold standard for truly native look and production maturity but require more expertise. Electron alternatives like Neutralino.js achieve the smallest footprint (~2MB) but sacrifice ecosystem maturity. Swift and Zig frameworks are not production-ready for cross-platform desktop use in 2026.

**Key takeaway**: For a home lab project, choose Tauri if you know web technologies, Wails if you prefer Go, or egui/Iced if you want pure Rust without web dependencies.

---

## Comparison Matrix

### All Frameworks Compared

| Framework | Binary Size | Memory (Idle) | Startup Time | Linux | macOS | Learning Curve | Community | Production Ready |
|-----------|-------------|---------------|--------------|-------|-------|----------------|-----------|------------------|
| **Rust-Based** |
| Tauri | 2.5-10 MB | 30-40 MB | <500ms | Excellent | Excellent | Moderate | 102K stars | Yes |
| egui | ~18 MB (native) | 30-150 MB | 200-300ms | Excellent | Excellent | Low | 27.8K stars | Yes |
| Iced | 3-17 MB | Moderate | 217-333ms | Excellent | Excellent | Moderate | 29.2K stars | Yes |
| Dioxus | <5 MB | Moderate | Subsecond | Excellent | Excellent | Moderate | 34.3K stars | Maturing |
| Slint | Very small | <300KB runtime | Fast | Excellent | Excellent | Moderate | 21.5K stars | Yes (embedded) |
| **Go-Based** |
| Wails | 4-8 MB | Low | Fast | Excellent | Excellent | Moderate | 32.3K stars | Yes |
| Fyne | 30-103 MB | High (issues) | Slow build | Good | Poor perf | Easy | 27.8K stars | Yes (with issues) |
| Gio | 5-15 MB | Low | Fast | Excellent | Good | Steep | 2.1K stars | Approaching |
| gotk4 | 5-10 MB + deps | Medium | Slow build | Excellent | Limited | Steep | 646 stars | Linux only |
| **Cross-Platform** |
| Qt (C++) | 5-14 MB | 15-40 MB | <0.5-1s | Excellent | Excellent | Steep | Mature | Yes |
| PySide6 | Varies | 30-60 MB | 0.5-1.5s | Excellent | Excellent | Moderate | Mature | Yes |
| wxWidgets | 5-15 MB | 10-30 MB | <0.5s | Excellent | Excellent | Moderate | 7K stars | Yes |
| Flutter Desktop | ~18 MB | 96-200 MB | <2s | Good | Good | Moderate | 175K stars | Yes |
| Avalonia | 60-80 MB | 50-150 MB | 1-2.4s | Excellent | Excellent | Moderate | 29.8K stars | Yes |
| Neutralino.js | 0.5-2 MB | 30-50 MB | ~1s | Good | Good | Easy | 8.3K stars | Small apps |
| **Experimental** |
| Swift (cross-platform) | Varies | Low | Native | Limited | Excellent | Moderate | Small | No |
| Zig (Capy UI) | Small | Low | Native | Limited | WIP | Steep | 2.5K stars | No |

### Performance Tier Rankings

**Tier 1 - Smallest Footprint (<10MB binary, <50MB RAM)**
1. Neutralino.js (0.5-2 MB binary)
2. Tauri (2.5-10 MB binary, 30-40 MB RAM)
3. Wails (4-8 MB binary)
4. Qt static (5 MB binary)
5. wxWidgets (5-15 MB binary, 10-30 MB RAM)

**Tier 2 - Lightweight (<20MB binary, <100MB RAM)**
1. egui (~18 MB binary, 30-150 MB RAM)
2. Iced (3-17 MB binary)
3. Dioxus (<5 MB binary)
4. Gio (5-15 MB binary)
5. Flutter Desktop (~18 MB binary, 96 MB baseline)

**Tier 3 - Moderate (20-80MB binary)**
1. Avalonia (60-80 MB binary)
2. Fyne (30-103 MB binary)
3. PySide6/PyQt6 (varies by modules)

**Tier 4 - Heavy (>80MB binary)**
1. Electron (85-180 MB binary, 200-500+ MB RAM)

---

## Recommendations by Use Case

### For Web Developers Wanting Native Apps

**Best Choice: Tauri**
- Use your existing HTML/CSS/JavaScript/TypeScript skills
- React, Vue, Svelte, or any frontend framework
- Rust backend provides security and performance
- Binary size 25-50x smaller than Electron
- Memory usage 50% less than Electron

**Alternative: Wails**
- Same web frontend approach but with Go backend
- Easier to learn Go than Rust
- Slightly larger binaries but still excellent
- Best if you prefer Go ecosystem

### For Systems Programmers Wanting GUI

**Best Choice: egui (Rust)**
- Immediate-mode paradigm is intuitive for systems programmers
- Pure Rust, no web technologies
- Fastest iteration cycle
- Great for tools, dashboards, and debuggers
- 1-2ms frame time for simple UIs

**Alternative: Iced (Rust)**
- Elm-inspired architecture for complex state management
- Better for larger, more structured applications
- Type-safe refactoring
- More predictable behavior than immediate-mode

**Alternative: Gio (Go)**
- Immediate-mode in Go
- Low memory footprint
- Pre-1.0 but actively developed

### For Rapid Prototyping

**Best Choice: egui**
- Fastest from idea to working UI
- No DSLs or macros required
- Live editing and hot reloading
- Minimal code for basic interfaces

**Alternative: Fyne**
- Easiest learning curve of all frameworks
- Excellent documentation with video tutorials
- Simple API: app.New() -> window -> widgets
- Caveat: Performance and memory issues in recent versions

### For Production-Ready Apps

**Best Choice: Tauri**
- 102K GitHub stars, used by ~90 companies
- Commercial support available (CrabNebula)
- Regular releases, stable API
- Proven in enterprise tools and consumer apps

**Alternative: Qt (C++ or PySide6)**
- Industry standard for 30+ years
- Used in mission-critical applications
- Extensive widget library
- Commercial and open-source options

**Alternative: wxWidgets**
- True native widgets on every platform
- Used by Audacity, FileZilla, Code::Blocks
- Permissive licensing
- Minimal overhead

### For Smallest Possible Footprint

**Best Choice: Neutralino.js**
- 0.5-2 MB binaries (smallest available)
- Uses system JavaScript engine
- Good for simple tools and utilities
- Caveat: Not suitable for large applications

**Alternative: Tauri**
- 2.5-10 MB binaries
- Much larger ecosystem than Neutralino
- Better for apps that may grow in complexity

**Alternative: Slint**
- <300KB runtime memory
- Designed for embedded systems
- Excellent if you need minimal RAM usage

### For Home Lab / Personal Projects

**Best Choice: Tauri**
- Hits the sweet spot of lightweight + mature + well-documented
- Web frontend means rapid UI iteration
- Rust backend for system integration
- Active community for support

**Alternative: Wails**
- If you prefer Go over Rust
- Simpler build process
- 4-8 MB binaries
- Growing ecosystem

**Alternative: egui**
- If you want pure Rust without web tech
- Great for developer tools
- Fastest prototyping
- Can be embedded in game engines

---

## Language Recommendations

### Choose Rust When:
- Binary size and memory efficiency are critical
- You need maximum performance
- Security is a high priority (memory safety)
- You want the most mature lightweight frameworks (Tauri, egui, Iced)
- You are building for the long term and can invest in learning curve

**Rust Framework Selection:**
- **Tauri**: Web frontend + Rust backend, most mature
- **egui**: Immediate mode, fastest prototyping
- **Iced**: Elm architecture, complex state management
- **Dioxus**: React-like, unified Rust codebase
- **Slint**: Embedded/resource-constrained environments

### Choose Go When:
- Developer experience and simplicity matter most
- Faster compilation times are important
- Your team knows Go better than Rust
- You want a single-binary deployment story

**Go Framework Selection:**
- **Wails**: Web frontend + Go backend, most mature Go option
- **Gio**: Immediate mode, best performance, pre-1.0
- **Fyne**: Easiest learning curve but has performance issues
- **gotk4**: Only for Linux-primary applications

### Choose C++ (Qt/wxWidgets) When:
- You need truly native look and feel
- Maximum control over performance
- Building for embedded or resource-constrained systems
- Long-term maintainability with stable APIs
- Commercial support is required

### Choose Python (PySide6) When:
- Rapid development is priority
- Team expertise is in Python
- Native appearance with Qt widgets
- Acceptable startup time (0.5-1.5s)

### Choose Dart (Flutter) When:
- Cross-platform including mobile (iOS/Android)
- Consistent appearance across platforms
- Hot reload for rapid iteration
- Google ecosystem integration

### Choose .NET (Avalonia) When:
- Existing .NET codebase or team expertise
- WPF/XAML familiarity
- Enterprise environment with .NET standardization

### Avoid for Now:
- **Swift cross-platform**: Only mature on macOS, Linux options experimental
- **Zig GUI frameworks**: Explicitly not production-ready, breaking changes ongoing

---

## Frameworks to Avoid

### Critical Issues or Not Ready

| Framework | Issue | Recommendation |
|-----------|-------|----------------|
| **Fyne (Go)** | Memory usage jumped from ~100MB to >1000MB in v2.3.0+; "really, really bad" macOS performance; binary size tripled from v2.4 to v2.5+ | Use Wails or Gio instead |
| **Electron** | 85-180 MB binaries, 200-500+ MB RAM, slow startup | Use Tauri, Wails, or Neutralino.js instead |
| **Swift cross-platform** | No first-party Linux GUI toolkit; all options experimental | Wait for maturity or use Swift for macOS-only |
| **Zig (Capy UI)** | Explicitly "NOT ready for use in production"; breaking changes ongoing | Wait for Zig 1.0 and framework stability |
| **gotk4** | Requires external GTK dependencies; limited macOS support; only 8 contributors; potential memory leaks | Use for Linux-only apps, otherwise choose alternatives |
| **Gio** | Pre-1.0 with breaking API changes between versions; small maintainer team (2 people); steeper learning curve | Use if comfortable with instability, otherwise wait or use Wails |
| **Dioxus** | Still v0.7.x (pre-1.0); API evolving; less proven than Tauri | Acceptable risk for some projects; prefer Tauri for stability |

### Framework-Specific Warnings

**Tauri**: Uses different WebView engines per platform (WebKit on macOS/Linux, WebView2 on Windows), which can cause subtle cross-platform differences. Test on all target platforms.

**egui**: Memory issues reported in v0.29.1 (2x usage vs v0.28.1 for texture-heavy apps); potential memory leaks with wgpu backend. Monitor memory carefully.

**Iced**: Windows startup can have 2-3 second hangs due to Vulkan instance creation. Consider OpenGL backend for faster startup.

**Neutralino.js**: Some 2024 reports show memory usage comparable to Electron in certain scenarios. Binary size advantage is clear, but runtime memory varies.

**Avalonia**: Historical memory issues in v0.10.0 (150MB idle, 500MB+ after operations). Reportedly fixed in v11.x but verify for your use case.

**Flutter Desktop**: Still higher memory (96-200 MB) than native alternatives; some developers report gaps in platform integrations. Best for projects also targeting mobile.

---

## Final Verdict

### The Single Best Recommendation for Home Lab Projects

**Tauri** is the recommended framework for building lightweight, fast desktop applications on Linux and macOS for home lab and personal projects.

**Why Tauri Wins:**

1. **Right-sized**: 2.5-10 MB binaries, 30-40 MB RAM - lightweight without being too minimal
2. **Fast**: Sub-500ms startup, responsive UI
3. **Mature**: 102K GitHub stars, 513 contributors, used by ~90 companies
4. **Documented**: Excellent documentation at v2.tauri.app
5. **Flexible UI**: Use any web framework (React, Vue, Svelte, plain HTML/CSS)
6. **Rust Backend**: Type safety, memory safety, performance for system integration
7. **Active Development**: Tauri 2.0 released late 2024, regular updates
8. **Commercial Support**: Available through CrabNebula if needed
9. **Future-Proof**: Mobile support (iOS/Android) now production-ready

**When to Choose Something Else:**

- If you strongly prefer Go and its ecosystem: **Wails**
- If you want pure Rust without web technologies: **egui** (prototyping) or **Iced** (complex apps)
- If you need truly native widgets: **Qt** or **wxWidgets**
- If you need the absolute smallest binary: **Neutralino.js** (for simple apps only)
- If you are building for embedded systems: **Slint**

### Recommended Starting Configuration

For a typical home lab desktop tool:

```
Framework: Tauri 2.x
Frontend: Svelte or Vue 3 (lighter than React)
Styling: Tailwind CSS
Build: Vite
Backend: Rust with Tauri commands
Distribution: AppImage (Linux) + DMG (macOS)
```

Expected results:
- Binary size: 5-10 MB
- Memory usage: 30-50 MB
- Startup time: <500ms
- Development experience: Hot reload, familiar web tooling

---

## Confidence and Gaps

### High Confidence Findings

- **Binary size rankings**: Well-documented across multiple sources
- **Tauri maturity**: Corroborated by GitHub stats, production usage, company adoption
- **Fyne performance issues**: Multiple GitHub issues document memory and performance regressions
- **Rust vs Go framework landscape**: Clear differentiation in approaches and trade-offs
- **Production readiness assessments**: Based on official documentation, version numbers, and adoption data

### Moderate Confidence

- **Startup time comparisons**: Based on limited benchmarks (mostly Linux, 2023 data)
- **Memory usage for newer frameworks**: Fewer published benchmarks for Dioxus, Gio
- **macOS-specific performance**: Less thoroughly benchmarked than Linux

### Gaps and Unknowns

- **Power consumption**: No sources covered battery life impact
- **Accessibility (a11y)**: Limited information about screen reader support across frameworks
- **Internationalization**: How well frameworks handle RTL languages, CJK text
- **Real-world benchmarks**: Most data is "hello world" - complex app comparisons needed
- **Auto-update mechanisms**: Distribution and update story varies significantly
- **Platform-specific quirks**: WebView differences between platforms in Tauri/Wails

### Recommended Follow-up Research

1. Build identical test application across top 3 candidates (Tauri, Wails, egui) and measure:
   - Actual memory usage under workload
   - CPU usage during UI operations
   - Binary size with typical dependencies

2. Evaluate macOS notarization and Linux distribution (AppImage, Flatpak) workflows

3. Test accessibility features if required for your use case

---

## Source Index

### Official Documentation (High Reliability)
- Tauri: v2.tauri.app, github.com/tauri-apps/tauri (102K stars)
- Dioxus: dioxuslabs.com, github.com/DioxusLabs/dioxus (34.3K stars)
- Iced: iced.rs, book.iced.rs, github.com/iced-rs/iced (29.2K stars)
- egui: egui.rs, docs.rs/egui, github.com/emilk/egui (27.8K stars)
- Slint: slint.dev, github.com/slint-ui/slint (21.5K stars)
- Wails: wails.io, v3alpha.wails.io, github.com/wailsapp/wails (32.3K stars)
- Fyne: docs.fyne.io, github.com/fyne-io/fyne (27.8K stars)
- Gio: gioui.org, github.com/gioui/gio (2.1K stars)
- gotk4: github.com/diamondburned/gotk4 (646 stars)
- Flutter: docs.flutter.dev, github.com/flutter/flutter (175K stars)
- Qt: qt.io, wiki.qt.io
- wxWidgets: wxwidgets.org, github.com/wxWidgets/wxWidgets (7K stars)
- Avalonia: avaloniaui.net, github.com/AvaloniaUI/Avalonia (29.8K stars)
- Neutralino.js: neutralino.js.org, github.com/neutralinojs/neutralinojs (8.3K stars)

### Performance Benchmarks
- lukaskalbertodt.github.io/2023/02/03/tauri-iced-egui-performance-comparison.html (detailed Linux benchmarks)
- github.com/tauri-apps/benchmark_results (official Tauri benchmarks)
- github.com/neutralinojs/evaluation (Neutralino vs Electron comparison)
- github.com/Elanis/web-to-desktop-framework-comparison (comprehensive web-to-desktop comparison)

### Community Analysis
- boringcactus.com/2025/04/13/2025-survey-of-rust-gui-libraries.html
- areweguiyet.com (Rust GUI ecosystem tracker)
- blog.logrocket.com/best-gui-frameworks-go/ (Go framework comparison)
- pythonguis.com (Python GUI tutorials and comparisons)

### GitHub Issues (Performance/Bugs)
- github.com/fyne-io/fyne/issues/3499 (Fyne v2.3.0 memory issues)
- github.com/fyne-io/fyne/issues/5008 (Fyne binary size)
- github.com/emilk/egui/issues/3689 (egui memory concerns)
- github.com/AvaloniaUI/Avalonia/issues/5646 (Avalonia memory issues)

### Release Information (as of January 2026)
- Tauri: v2.9.6 (December 2025)
- Dioxus: v0.7.3 (January 2026)
- Iced: v0.14.0 (December 2025)
- Slint: v1.14.1 (October 2025)
- Wails: v2.11.0 (November 2025)
- Fyne: v2.7.2 (January 2026)
- Gio: v0.9.0 (2025)
- wxWidgets: v3.2.9 (December 2025)
- Avalonia: v11.3.11 (January 2026)
- Neutralino.js: v6.4.0 (November 2025)
