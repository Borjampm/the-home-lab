# Modern Cross-Platform Desktop Frameworks (Non-Rust, Non-Go)

## Abstract

This report examines cross-platform desktop frameworks suitable for building lightweight applications on Linux and macOS, excluding Rust and Go-based solutions. We evaluated eight framework categories: Flutter Desktop, Qt (C++ and Python bindings), wxWidgets, Avalonia (.NET), Electron alternatives (Tauri, Neutralino.js), webview libraries, Swift cross-platform options, and Zig-based GUI toolkits. Key findings indicate that Qt remains the most mature and production-ready option with excellent performance characteristics, while Flutter Desktop has rapidly matured for production use despite higher memory overhead. Avalonia emerges as the strongest .NET option with 29.8k GitHub stars and enterprise adoption. Among Electron alternatives, Tauri (stable 2.0 release) offers the best balance of features and maturity, while Neutralino.js provides the smallest footprint (~2MB vs ~85MB for Electron). Swift and Zig frameworks remain experimental but show promise for future development.

## Findings

### 1. Flutter Desktop

**Overview**: Google's UI toolkit using the Dart language with Skia rendering engine for consistent cross-platform UI.

#### Binary Size
- Default release application: ~18MB on Linux
- Includes Flutter engine and framework
- Significantly larger than native frameworks but smaller than Electron

#### Memory Usage
- Runtime memory: 96MB for default applications on Linux
- Reported consumption: 150-200MB idle, up to 3-4GB for larger projects
- High memory usage compared to native alternatives due to Skia rendering engine
- Memory issues reported in versions 0.10.0 but improvements made in recent releases

#### Startup Time
- Target: Sub-2 seconds for cold launch (industry standard)
- Frame rendering: 16ms per frame (60 FPS target)
- Desktop-specific benchmarks not widely published, but mobile targets apply

#### UI Approach
- Custom rendering via Skia (not native widgets)
- Consistent 60 FPS performance across platforms
- UI doesn't look native by default but provides pixel-perfect consistency

#### Cross-Platform Support
- Windows, macOS, and Linux officially supported (stable since ~2023)
- Desktop support is "no longer experimental" as of 2025
- Quality: Windows and macOS support is mature; Linux support improving
- Platform-specific issues: Cannot use Firebase SDK on Windows
- Thousands of plugins now support desktop platforms

#### Learning Curve
- Well-documented with extensive official resources
- 175k GitHub stars (most popular in this survey)
- Flutter Academy: 54,000+ learners, 120+ courses
- Intuitive for those familiar with reactive/declarative UI patterns
- Requires learning Dart language

#### Community Health
- GitHub: 175k stars, 87,816 commits, 1,668 contributors
- Extremely active development by Google
- Large ecosystem with active plugin development
- Google continues active development for desktop and web in 2025-2026

#### Maturity
- Production-ready for desktop as of 2025
- Major organizations shipping production desktop apps
- Used for: dashboards, internal tools, admin panels, POS systems, AI-powered dashboards
- "Mature doesn't mean perfect" - gaps in deep platform integrations
- Some developers still consider web/desktop "work in progress" compared to mobile

**Production Users**: Thousands of internal tools, admin panels, and cross-platform applications; increasingly used by startups for SaaS MVPs.

---

### 2. Qt Framework

**Overview**: Comprehensive C++ framework with bindings for Python (PySide6/PyQt6) and other languages, using native widgets or Qt Quick (QML).

#### Binary Size
- Static "Hello World": 5MB (release), 14MB (debug)
- Android "Hello World": 7-11MB APK
- Size heavily dependent on:
  - Static vs dynamic linking
  - Included Qt modules
  - Platform (Windows/Linux/macOS)
- Can be optimized significantly with selective module inclusion

#### Memory Usage
- Superior performance with smaller memory footprint than Flutter/Electron
- Native widget rendering provides efficient memory management
- Qt C++ uses optimized memory management
- **PySide6/PyQt6**: PySide created Shiboken specifically to reduce binaries and memory footprint
- PyQt loads entire Qt framework, increasing memory footprint with consolidated module approach

#### Startup Time
- Native performance with fast cold starts
- Significantly faster than Electron (sub-second typical)
- Specific benchmarks not widely published but consistently reported as excellent

#### UI Approach
- **Native widgets** (Qt Widgets) - platform-native look and feel
- **Qt Quick/QML** - custom rendered, modern declarative UI
- Flexible approach allows choosing between native appearance and custom design
- ModelViews for high-performance GUIs with millions of items
- Graphics View Framework for fast, efficient scenes

#### Cross-Platform Support
- Windows, macOS, Linux, Android, iOS, embedded systems
- Excellent quality across all platforms
- Industry-standard for cross-platform development since 1990s
- Deep native platform integration
- Same codebase with minimal platform-specific code needed

#### Learning Curve
- **C++**: Requires solid C++ knowledge, though Qt simplifies many aspects
- Learning curve depends on C++ expertise and what you want to achieve
- Extensive documentation: Qt Wiki, 54,000+ Qt Academy learners, 120+ courses
- Updated CMake course (January 2026)
- Step-by-step tutorials for beginners
- **Python (PySide6/PyQt6)**: Easier entry point, excellent documentation at pythonguis.com
- PySide6 recommended for 2026 professional applications

#### Community Health
- Mature, stable project with decades of development
- Strong commercial backing (The Qt Company)
- Active open-source community
- Extensive third-party resources and commercial support
- Regular releases and updates

#### Maturity
- Extremely mature and production-ready
- Used by thousands of commercial applications
- Industry standard for desktop development
- Proven in mission-critical applications
- **PySide6/PyQt6**: Top choice for professional, native look and feel in 2026

**Production Users**: Autodesk Maya, VirtualBox, OBS Studio, KDE Plasma Desktop, countless commercial and open-source applications.

**Licensing Note**:
- Qt C++: GPL/LGPL or commercial license
- PySide6: LGPL (free for commercial use)
- PyQt6: GPL or commercial license required for closed-source

---

### 3. wxWidgets

**Overview**: C++ library providing native look and feel by wrapping platform-native APIs (Win32, GTK+, Cocoa).

#### Binary Size
- Minimal overhead - leverages system libraries
- No bundled runtime beyond OS dependencies
- Exact "Hello World" sizes not published but significantly smaller than Qt static builds
- Libraries built and compiled in C++ for high performance

#### Memory Usage
- Lightweight footprint
- Minimal overhead as it's a thin wrapper over native toolkits
- Reference counting and smart pointers minimize memory overhead
- Comparative measurements show minimal overhead "at best"
- Specific numeric values for minimal apps not readily available

#### Startup Time
- Fast - nearly as fast as native toolkit development
- No significant runtime initialization overhead
- Native performance characteristics

#### UI Approach
- True native widgets on each platform
- Uses platform's native API rather than emulation
- Windows: Win32 API
- Linux: GTK+ 2.6 or higher (GTK 3 recommended)
- macOS: Cocoa (macOS 10.10+)
- Native look and feel guaranteed

#### Cross-Platform Support
- Windows, macOS, Linux, Unix variants
- High-quality native appearance on all platforms
- Single codebase with little to no platform-specific code
- Truly native applications with native capabilities and speed

#### Learning Curve
- Requires C++ knowledge
- API is more traditional/imperative than modern declarative approaches
- Comprehensive documentation and tutorials
- Active community forums
- Qt Wiki provides step-by-step tutorials
- Steeper initial learning curve than higher-level frameworks

#### Community Health
- GitHub: 7k stars, 79,403 commits
- Very active development in 2025
  - Version 3.2.9: December 12, 2025 (stable)
  - Version 3.3.0: June 6, 2025 (development)
  - Version 3.3.1: July 21, 2025 (development)
- 180 unique committers for 3.3.0 release
- Strong community support and ongoing development
- Multiple maintained branches (stable 3.2.x and development 3.3.x)

#### Maturity
- Extremely mature (first released 1992)
- Production-ready and battle-tested
- Used in countless commercial and open-source applications
- Stable API with long-term backward compatibility
- Reliable for mission-critical applications

**Production Users**: Audacity, FileZilla, Code::Blocks, numerous scientific and commercial applications requiring native appearance.

---

### 4. Avalonia UI

**Overview**: .NET cross-platform UI framework using XAML, inspired by WPF, with Skia rendering for consistency.

#### Binary Size
- Compiled binaries: 60-80MB (considered "really large" by some developers)
- Includes .NET runtime and Avalonia framework
- Alternative ModernForms stripped down to "much smaller" sizes
- Can be optimized with Native AOT compilation

#### Memory Usage
- Historical issues (v0.10.0): 150MB idle, up to 500MB+ after resizing (5x growth)
- .NET Core: 3x more memory than .NET Framework in v0.10.0
- Current status: "highly optimized compositional rendering engine delivers exceptional performance while using minimal resources"
- Memory issues appear resolved in recent versions
- Native AOT reduces memory usage significantly

#### Startup Time
- Cold start: Under 1 second on Windows (typical)
- Simple "Hello World": 2.4 seconds baseline, 1.4 seconds with PublishReadyToRun=true
- Compared to Windows Forms: <0.5 seconds (WinForms is faster)
- Native AOT: Significantly faster startup times
- Avalonia 11.1: Improved startup times through better resource management

#### UI Approach
- Skia-based rendering (like Flutter)
- XAML for declarative UI definition
- Eliminates cross-platform headaches with consistent rendering
- Modern, flexible UI paradigm familiar to WPF developers
- Custom rendering (not native widgets)

#### Cross-Platform Support
- Desktop: Windows, macOS, Linux (excellent quality)
- Mobile: iOS, Android (introduced 2023, still maturing)
- WebAssembly support
- Browser: Experimental
- Embedded systems
- Desktop is mature; mobile support evolving

#### Learning Curve
- Familiar to WPF/XAML developers (easy transition)
- Excellent documentation
- C# and XAML knowledge required
- Modern, declarative approach easier than traditional UI frameworks
- Growing educational resources and community tutorials

#### Community Health
- GitHub: 29.8k stars, 27,455 commits, 532 contributors
- Most-starred cross-platform UI solution for .NET (25% increase in 2024)
- 117 releases, actively maintained
- 24,000+ dependent repositories
- Month-over-month growth: 14.29% average (down from 30% peak)
- 35 million total package downloads approaching
- Latest release: 11.3.11 (January 13, 2026)

#### Maturity
- Mature and production-ready (over a decade of development)
- Enterprise adoption by major companies:
  - Schneider Electric
  - Unity
  - JetBrains (modernizing WPF tools used by 170,000+ companies, 431 Fortune 500)
  - GitHub
- Used in Formula 1 racing teams, airlines, financial institutions
- Drag-and-drop designer coming in 2025 (Avalonia Accelerate)
- Stable for desktop, mobile support maturing

**Production Users**: JetBrains IDE tools, Schneider Electric applications, Unity tools, high-performance environments (F1, airlines, financial institutions).

---

### 5. Electron Alternatives

#### 5.1 Tauri

**Overview**: Rust-based backend with system WebView frontend, focusing on security and efficiency.

##### Binary Size
- Application installers: ~2.5-10MB
- Dramatically smaller than Electron (~85MB)
- Uses system WebView (no Chromium bundling)

##### Memory Usage
- Idle: 30-40MB
- Significantly lower than Electron (hundreds of MB)
- No full Chromium instance

##### Startup Time
- Cold start: Under 0.5 seconds
- Electron: 1-2 seconds typical
- Faster startup due to no browser engine initialization

##### UI Approach
- Web technologies (HTML, CSS, JavaScript/TypeScript)
- System WebView for rendering:
  - Windows: WebView2 (Chromium Edge)
  - macOS: WKWebView (Safari)
  - Linux: WebKitGTK+
- Frontend framework agnostic (React, Vue, Svelte, etc.)

##### Cross-Platform Support
- Desktop: Windows, macOS, Linux (excellent)
- Mobile: iOS, Android (stable in 2.0)
- High-quality implementation across platforms
- Better native integration than Electron

##### Learning Curve
- Requires Rust knowledge for backend
- Web technologies for frontend (familiar to web developers)
- Steeper learning curve than pure web approaches
- Excellent documentation and growing resources

##### Community Health
- Tauri 2.0: Stable release in 2024
- Very active development
- Growing ecosystem
- Strong community support
- Regular updates and improvements

##### Maturity
- Tauri 2.0: Production-ready (stable release 2024)
- Used in production by early adopters during beta
- Rapidly maturing with strong backing
- Suitable for production desktop and mobile applications

**Production Users**: Various desktop applications, particularly by developers prioritizing security and efficiency over Electron.

#### 5.2 Neutralino.js

**Overview**: Lightweight alternative using system JavaScript engine and WebView, no Node.js bundling.

##### Binary Size
- Uncompressed: ~2MB
- Compressed: ~0.5MB
- Smallest of all frameworks surveyed
- Electron equivalent: 180MB → Neutralino: 2MB

##### Memory Usage
- Very low footprint (< 3MB overhead)
- Note: Some 2024 reports suggest memory usage comparable to or higher than Electron in certain scenarios
- Uses system libraries rather than bundling

##### Startup Time
- Fast - no heavy runtime initialization
- Specific benchmarks not published

##### UI Approach
- Web technologies (JavaScript, HTML, CSS)
- System WebView:
  - Linux: WebKitGTK+ 2
  - macOS: WKWebView
  - Windows: Browser controls
- WebSocket connection for native operations
- Embedded static web server

##### Cross-Platform Support
- Windows, macOS, Linux
- Web support
- Quality varies by system JavaScript engine
- Potential compatibility issues across different systems

##### Learning Curve
- Easy for web developers
- JavaScript/HTML/CSS knowledge sufficient
- Simple API
- Good documentation

##### Community Health
- GitHub: 8.3k stars, 1,931 commits, 84 contributors
- Latest release: v6.4.0 (November 29, 2025)
- Active development
- Google Summer of Code 2026 participant
- GitHub Discussions and Discord community

##### Maturity
- Stable for small to medium applications
- Not recommended for very large applications
- Production-ready for appropriate use cases
- Ongoing development and improvements

**Limitations**:
- Depends on system JavaScript engine (compatibility concerns)
- Not suitable for very large applications
- Some memory usage concerns reported in 2024

**Production Users**: Smaller desktop applications, utilities, tools where minimal footprint is critical.

#### 5.3 Other Webview Options

**Photino**: .NET-based with system WebView (WebView2/WKWebView/WebKitGTK+), no Chromium/Node.js bundling. Good for .NET developers.

**DeskGap**: Node.js runtime with system WebView, minimal bundling. Less active than Tauri/Neutralino.

**Saucer**: Modern C++ webview library supporting Windows, Linux, macOS with multiple backend implementations.

---

### 6. Swift Cross-Platform

**Overview**: Swift language with various GUI toolkit options for cross-platform development beyond Apple's ecosystem.

#### Binary Size
- Not well-documented for cross-platform GUI applications
- Varies significantly by toolkit and dependencies
- Generally competitive with C/C++ applications

#### Memory Usage
- Swift uses ARC (Automatic Reference Counting) for efficient memory management
- Shaft framework: "uses as little memory as possible"
- Specific benchmarks for desktop GUIs not published

#### Startup Time
- Native performance characteristics
- Specific desktop GUI benchmarks not available

#### UI Approach

**Available Frameworks**:

1. **SwiftCrossUI** (Gtk backend)
   - Declarative, inspired by SwiftUI
   - GTK 4 on Linux, macOS, Windows
   - Native look via GTK

2. **Adwaita for Swift**
   - GNOME/GTK integration
   - Linux, macOS, Windows
   - Native GNOME design language

3. **Shaft**
   - Flutter concepts ported to Swift
   - Cross-platform hot reload
   - Custom rendering (Skia-like)

4. **VertexGUI**
   - SDL2 for windowing
   - Skia for 2D graphics
   - Custom widget rendering

5. **Suit**
   - Pure Swift GUI toolkit
   - macOS and Linux support

#### Cross-Platform Support
- **Major Challenge**: Linux has no first-party Swift GUI toolkit
- macOS: Native (AppKit/SwiftUI)
- Linux: Via GTK bindings or custom toolkits
- Windows: Limited support
- Quality varies significantly by framework

#### Learning Curve
- Swift language: Moderate learning curve
- SwiftUI patterns: Easy for iOS/macOS developers
- GTK: Requires learning different widget model and layout system
- Documentation: Sparse for cross-platform options, often need to reference C docs
- SDL: Steeper curve requiring graphics programming knowledge

#### Community Health
- SwiftCrossUI: Active development, recent official releases starting
- Shaft: Newer project, active development
- Adwaita for Swift: Official support, active
- Most projects: Small communities, experimental phase
- Swift language itself: Backed by Apple, strong ecosystem

#### Maturity
- **macOS/iOS (SwiftUI)**: Production-ready, mature
- **Cross-platform options**: Experimental to early-stage
- SwiftCrossUI: Maturing, starting official releases
- Most frameworks: Not production-ready for critical applications
- Best for: 94% shared business logic, 4% platform-specific UI code

**Recommendation**: For production cross-platform work in 2026, consider using Swift for business logic with platform-specific UI code, or wait for frameworks like SwiftCrossUI to mature further.

---

### 7. Zig GUI Frameworks

**Overview**: Zig is a systems programming language with emerging GUI framework options for cross-platform development.

#### Binary Size
- Expected to be competitive with C/C++ (Zig compiles to native code)
- Specific measurements for GUI applications not published
- No runtime bundling

#### Memory Usage
- Low-level memory control like C
- Manual memory management with safety features
- Efficient resource usage expected
- No garbage collection overhead

#### Startup Time
- Native performance expected
- Specific GUI benchmarks not available

#### UI Approach

**Available Frameworks**:

1. **Capy UI** (Primary option)
   - Native controls from OS
   - Declarative API
   - Windows: Native widgets
   - Linux: GTK 4
   - Web: HTML/CSS
   - macOS: Work in progress
   - Android: Work in progress

2. **IUP for Zig**
   - Bindings for IUP (Portable User Interface)
   - Cross-platform GUI toolkit from PUC-RIO
   - Native controls

3. **Mach Engine**
   - Game engine focus but includes GUI plans
   - Cross-compilation capabilities
   - Future GUI editor and UI library planned

4. **Zylix**
   - Virtual DOM approach
   - Cross-platform UI framework

#### Cross-Platform Support
- **Capy**: Windows and Linux working, macOS WIP, Web supported
- Zig's cross-compilation: Major advantage (x86_64-windows, x86_64-linux-gnu, x86_64-macos, aarch64-macos)
- Quality varies - most projects in early development

#### Learning Curve
- Zig language: Moderate to steep (systems programming)
- Similar to C but with modern safety features
- Zig 1.0 expected in 2026 (language still stabilizing)
- GUI framework docs: Limited, projects are young
- Good for: Systems programmers familiar with C/C++

#### Community Health
- Capy UI: 2.5k stars, 658 commits, actively maintained
- Capy targets: Zig 0.14.1
- **Critical note**: "NOT ready for use in production as I'm still making breaking changes"
- Small but growing community
- Zig itself: Approaching 1.0 in 2026

#### Maturity
- **Zig language**: Approaching 1.0, still experimental
- **Capy UI**: Explicitly NOT production-ready
- **Other frameworks**: Early development stage
- Breaking changes still occurring
- Best approach: Shared Zig business logic (94%) + platform-specific UI (6%)

**Recommendation**: Wait for Zig 1.0 release and framework maturation. Not suitable for production in 2026 unless you're comfortable with breaking changes and contributing to development.

---

## Comparison Tables

### Binary Size Comparison

| Framework | Hello World / Minimal App | Notes |
|-----------|---------------------------|-------|
| Neutralino.js | 0.5-2 MB | Smallest, compressed |
| Tauri | 2.5-10 MB | System WebView |
| wxWidgets | 5-15 MB | Native, varies by linking |
| Qt (static) | 5 MB | C++, optimizable |
| Flutter Desktop | ~18 MB | Includes engine |
| Avalonia | 60-80 MB | .NET runtime included |
| Electron | ~85-180 MB | Full Chromium bundle |
| PySide6/PyQt6 | Varies | Dependent on included Qt modules |

### Memory Usage Comparison

| Framework | Idle RAM | Runtime Notes |
|-----------|----------|---------------|
| wxWidgets | 10-30 MB | Minimal overhead, native |
| Qt C++ | 15-40 MB | Native widgets, efficient |
| Tauri | 30-40 MB | System WebView |
| Neutralino.js | ~30-50 MB | Lightweight, some concerns |
| PySide6 | 30-60 MB | Optimized for memory |
| Avalonia | 50-150 MB | Improved from early versions |
| Flutter Desktop | 96-200 MB | Skia rendering overhead |
| Electron | 200-500+ MB | Full Chromium instance |

### Startup Time Comparison

| Framework | Cold Start | Notes |
|-----------|------------|-------|
| wxWidgets | <0.5s | Native performance |
| Qt | <0.5-1s | Native/near-native |
| Tauri | <0.5s | Fast WebView init |
| PySide6/PyQt6 | 0.5-1.5s | Python overhead |
| Avalonia | 1-2.4s | Can optimize with AOT |
| Neutralino.js | ~1s | Minimal runtime |
| Flutter Desktop | <2s target | Custom rendering |
| Electron | 1-2s+ | Heavy initialization |

### Production Readiness Matrix (2026)

| Framework | Desktop Maturity | Enterprise Use | Breaking Changes | Recommendation |
|-----------|-----------------|----------------|------------------|----------------|
| Qt C++ | Mature | Extensive | Rare | Production-ready |
| wxWidgets | Mature | Extensive | Rare | Production-ready |
| PySide6/PyQt6 | Mature | Common | Occasional | Production-ready |
| Avalonia | Mature | Growing | Stable API | Production-ready |
| Flutter Desktop | Mature | Growing | Stable since 2023 | Production-ready |
| Tauri | Stable (2.0) | Early adoption | Post-2.0 stable | Production-ready |
| Neutralino.js | Stable | Niche use | Occasional | Use for small apps |
| Swift Cross-platform | Experimental | Research only | Frequent | Wait for maturity |
| Zig (Capy UI) | Pre-production | Not recommended | Frequent | Not ready |

### Community Health Snapshot (GitHub Stars)

| Framework | Stars | Notable Metrics |
|-----------|-------|-----------------|
| Flutter | 175k | Most popular, Google-backed |
| Avalonia | 29.8k | Most popular .NET UI |
| Neutralino.js | 8.3k | Active development |
| wxWidgets | 7k | Long history, stable |
| Capy UI (Zig) | 2.5k | Young, experimental |
| Qt* | N/A | Commercial + Open Source |

*Qt's community spans multiple repositories and commercial users, not fully reflected in GitHub metrics.

---

## Framework Selection Guide

### Choose **Qt (C++)** if you need:
- Maximum performance and efficiency
- True native look and feel
- Extensive widget library
- Commercial support available
- Mature, battle-tested framework
- Desktop, mobile, and embedded support

### Choose **PySide6/PyQt6** if you need:
- Qt framework with Python ease-of-use
- Rapid development with native appearance
- Extensive Python ecosystem integration
- Good performance with managed complexity
- LGPL licensing (PySide6)

### Choose **Flutter Desktop** if you need:
- Cross-platform including mobile (iOS/Android)
- Modern, reactive UI paradigm
- Consistent appearance across platforms
- Rich animation and custom UI capabilities
- Google ecosystem integration
- Hot reload for rapid development

### Choose **Avalonia** if you need:
- .NET ecosystem integration
- WPF-like development experience
- XAML familiarity
- Cross-platform .NET applications
- Growing enterprise support
- Modern declarative UI

### Choose **wxWidgets** if you need:
- Native widgets with minimal overhead
- Permissive licensing (wxWindows License)
- Lightweight footprint
- True native appearance guaranteed
- C++ control without framework complexity

### Choose **Tauri** if you need:
- Web technologies with desktop capabilities
- Small binary size and low memory usage
- Modern web framework integration
- Security-focused architecture
- Rust backend performance
- Mobile support (iOS/Android)

### Choose **Neutralino.js** if you need:
- Absolute minimal footprint
- Simple desktop wrapper for web apps
- Easy web developer entry point
- Small-scale applications
- Quick prototyping

### Avoid for Production (2026):
- **Swift cross-platform**: Still maturing, limited options
- **Zig GUI frameworks**: Explicitly not production-ready
- **Electron**: Unless ecosystem lock-in or complexity justifies overhead

---

## Conflicts

### Flutter Desktop Memory Usage
- Early reports (2020-2021) showed 96MB baseline and scaling issues
- Recent documentation claims optimization improvements
- Reality: Still higher than native alternatives but improved from initial releases
- Context: Acceptable for modern hardware but concerning for resource-constrained environments

### Neutralino.js Memory Claims
- Marketed as ultra-lightweight alternative
- 2024 GitHub issue reports memory usage equal to or higher than Electron in some cases
- Resolution: Size advantage is clear (0.5MB vs 85MB), but runtime memory behavior varies by application

### Avalonia Memory Performance
- Version 0.10.0 (2021): Severe memory issues (150MB idle, 500MB+ after operations)
- Current marketing: "Minimal resources"
- Resolution: Issues resolved in recent versions, likely fixed in v11.x series

### Flutter Desktop Production Readiness
- Google/official sources: "No longer experimental," "production-ready"
- Some developers: "Still feels like work in progress"
- Resolution: Desktop support is stable enough for production but not as polished as mobile; suitable for internal tools, dashboards, and MVPs but may have gaps in platform integrations

### Swift Cross-Platform Viability
- Multiple frameworks exist (SwiftCrossUI, Shaft, Adwaita)
- All are experimental or early-stage
- Resolution: Not production-ready in 2026; best used for business logic with platform-specific UIs

---

## Follow-up Directions

### Performance Benchmarking
Conduct hands-on benchmarks with standardized test applications:
- Binary size for equivalent "TODO app" across all frameworks
- Memory profiling under consistent workloads
- Startup time measurements (cold/warm/hot start)
- Frame rate and rendering performance
- CPU usage during idle and active states

### Deep-Dive Framework Testing
Build reference implementations in top candidates:
- Qt (C++ and PySide6)
- Flutter Desktop
- Avalonia
- Tauri
- wxWidgets

Test areas:
- Native OS integration (notifications, system tray, file dialogs)
- Deployment and distribution complexity
- Developer experience and iteration speed
- Accessibility support
- Internationalization/localization

### Emerging Technology Monitoring
Track maturation of promising but not-yet-ready options:
- **Swift cross-platform**: SwiftCrossUI official releases and stability
- **Zig GUI**: Capy UI post-1.0 readiness
- **Shaft**: Flutter concepts in Swift maturity
- Next-generation webview frameworks

### Platform-Specific Optimizations
Investigate platform-specific considerations:
- macOS: Swift native vs. cross-platform frameworks
- Linux: GTK integration quality across frameworks
- Distribution formats: AppImage, Flatpak, Snap, DMG, etc.

### Licensing and Commercial Viability
- Qt commercial vs. LGPL implications
- PyQt vs. PySide for commercial applications
- Patent and IP considerations for production use

### Accessibility and Compliance
- WCAG compliance capabilities
- Screen reader support
- Keyboard navigation
- High contrast and theming support

---

## Sources

[1]: https://github.com/flutter/flutter/issues/92318 (accessed 2026-01-20, community, medium)
[2]: https://github.com/flutter/flutter/issues/90868 (accessed 2026-01-20, community, medium)
[3]: https://github.com/flutter/flutter/issues/95092 (accessed 2026-01-20, community, medium)
[4]: https://docs.flutter.dev/perf/app-size (accessed 2026-01-20, official, high)
[5]: https://tibicle.com/blog/best-framework-for-desktop-application-in-2026 (accessed 2026-01-20, community, medium)
[6]: https://www.qt.io/development/qt-framework (accessed 2026-01-20, official, high)
[7]: https://markaicode.com/flutter-desktop-vs-qt-performance-comparison/ (accessed 2026-01-20, community, medium)
[8]: https://www.qt.io/development/desktop-app-development (accessed 2026-01-20, official, high)
[9]: https://wxwidgets.org/ (accessed 2026-01-20, official, high)
[10]: https://github.com/wxWidgets/wxWidgets (accessed 2026-01-20, official, high)
[11]: https://wiki.wxwidgets.org/WxWidgets_Compared_To_Other_Toolkits (accessed 2026-01-20, official, medium)
[12]: https://wxwidgets.org/about/ (accessed 2026-01-20, official, high)
[13]: https://docs.wxwidgets.org/3.2/overview_install.html (accessed 2026-01-20, official, high)
[14]: https://dev.to/biozal/the-net-cross-platform-showdown-maui-vs-uno-vs-avalonia-and-why-avalonia-won-ian (accessed 2026-01-20, community, medium)
[15]: https://github.com/AvaloniaUI/Avalonia/issues/5646 (accessed 2026-01-20, community, medium)
[16]: https://avaloniaui.net/platforms (accessed 2026-01-20, official, high)
[17]: https://docs.avaloniaui.net/docs/guides/development-guides/improving-performance (accessed 2026-01-20, official, high)
[18]: https://github.com/AvaloniaUI/Avalonia (accessed 2026-01-20, official, high)
[19]: https://github.com/AvaloniaUI/Avalonia/discussions/9217 (accessed 2026-01-20, community, medium)
[20]: https://github.com/neutralinojs/evaluation (accessed 2026-01-20, official, high)
[21]: https://neutralino.js.org/ (accessed 2026-01-20, official, high)
[22]: https://github.com/neutralinojs/neutralinojs/issues/1226 (accessed 2026-01-20, community, medium)
[23]: https://dev.to/byteslash/what-is-neutralinojs-alternative-to-electron-5fjc (accessed 2026-01-20, community, medium)
[24]: https://blog.logrocket.com/why-use-electron-alternative/ (accessed 2026-01-20, community, medium)
[25]: https://github.com/neutralinojs/neutralinojs (accessed 2026-01-20, official, high)
[26]: https://www.tryphotino.io/ (accessed 2026-01-20, official, medium)
[27]: https://deskgap.com/ (accessed 2026-01-20, official, medium)
[28]: https://blog.scottlogic.com/2023/02/01/webview2-electron-challengers-and-slightly-lighter-desktop-web-applications.html (accessed 2026-01-20, community, medium)
[29]: https://github.com/saucer/saucer (accessed 2026-01-20, official, medium)
[30]: https://github.com/stackotter/swift-cross-ui (accessed 2026-01-20, official, medium)
[31]: https://forums.swift.org/t/preview-cross-platform-ide-and-gui-framework/24361 (accessed 2026-01-20, community, medium)
[32]: https://github.com/muizidn/Suit (accessed 2026-01-20, official, low)
[33]: https://www.swift.org/blog/adwaita-swift/ (accessed 2026-01-20, official, high)
[34]: https://forums.swift.org/t/new-framework-for-gui-apps-on-linux-and-macos/41528 (accessed 2026-01-20, community, medium)
[35]: https://github.com/VertexUI/VertexGUI (accessed 2026-01-20, official, low)
[36]: https://github.com/ShaftUI/Shaft (accessed 2026-01-20, official, medium)
[37]: https://linuxvox.com/blog/can-i-use-swift-code-to-create-a-desktop-linux-app/ (accessed 2026-01-20, community, medium)
[38]: https://capy-ui.org/ (accessed 2026-01-20, official, medium)
[39]: https://machengine.org/ (accessed 2026-01-20, official, medium)
[40]: https://mitchellh.com/writing/zig-and-swiftui (accessed 2026-01-20, community, medium)
[41]: https://github.com/capy-ui/capy (accessed 2026-01-20, official, medium)
[42]: https://zig.news/batiati/iup-for-zig-4ah (accessed 2026-01-20, community, medium)
[43]: https://github.com/Elanis/web-to-desktop-framework-comparison (accessed 2026-01-20, community, high)
[44]: https://raftlabs.medium.com/tauri-vs-electron-a-practical-guide-to-picking-the-right-framework-5df80e360f26 (accessed 2026-01-20, community, medium)
[45]: https://www.gethopp.app/blog/tauri-vs-electron (accessed 2026-01-20, community, medium)
[46]: https://www.levminer.com/blog/tauri-vs-electron (accessed 2026-01-20, community, medium)
[47]: https://blog.notesnook.com/neutralinojs-next-best-alternative-to-electron-and-tauri/ (accessed 2026-01-20, community, medium)
[48]: https://github.com/MichalStrehovsky/sizegame (accessed 2026-01-20, community, medium)
[49]: https://voxturrlabs.com/blog/can-flutter-build-enterprise-desktop-applications-2025/ (accessed 2026-01-20, community, medium)
[50]: https://kitrum.com/blog/why-flutter-isnt-ideal-for-cross-platform-development/ (accessed 2026-01-20, community, low)
[51]: https://shahzaibabid.com/flutter-roadmap-2026/ (accessed 2026-01-20, community, medium)
[52]: https://tomasrepcik.dev/blog/2025/2025-12-14-flutter-2026/ (accessed 2026-01-20, community, medium)
[53]: https://docs.flutter.dev/platform-integration/desktop (accessed 2026-01-20, official, high)
[54]: https://tomas-repcik.medium.com/my-take-on-flutter-in-2026-71905872fd2d (accessed 2026-01-20, community, medium)
[55]: https://github.com/topics/wxwidgets?o=desc&s=stars (accessed 2026-01-20, official, medium)
[56]: https://wxwidgets.org/news/2025/12/wxwidgets-3.2.9-released/ (accessed 2026-01-20, official, high)
[57]: https://wxwidgets.org/news/2025/06/wxwidgets-3.3.0-released/ (accessed 2026-01-20, official, high)
[58]: https://wxwidgets.org/news/2025/07/wxwidgets-3.3.1-released/ (accessed 2026-01-20, official, high)
[59]: https://visualstudiomagazine.com/articles/2023/07/07/avalonia-11.aspx (accessed 2026-01-20, company-blog, medium)
[60]: https://avaloniaui.net/blog/avalonia-ui-in-2024-growth-challenges-and-the-road-ahead (accessed 2026-01-20, official, high)
[61]: https://avaloniaui.net/ (accessed 2026-01-20, official, high)
[62]: https://www.tekrevol.com/blogs/which-framework-should-you-choose-maui-flutter-or-avalonia/ (accessed 2026-01-20, community, medium)
[63]: https://docs.avaloniaui.net/docs/faq (accessed 2026-01-20, official, high)
[64]: https://wiki.qt.io/Qt_for_Beginners (accessed 2026-01-20, official, high)
[65]: https://www.qt.io/blog/how-to-learn-qt-in-2026 (accessed 2026-01-20, official, high)
[66]: https://www.learnqt.guide/courses/ (accessed 2026-01-20, commercial, medium)
[67]: https://www.qt.io/academy (accessed 2026-01-20, official, high)
[68]: https://github.com/neutralinojs-community/awesome-neutralino (accessed 2026-01-20, community, medium)
[69]: https://github.com/neutralinojs/gsoc2026 (accessed 2026-01-20, official, medium)
[70]: https://v2.tauri.app/blog/tauri-20/ (accessed 2026-01-20, official, high)
[71]: https://v2.tauri.app/blog/tauri-2-0-0-release-candidate/ (accessed 2026-01-20, official, high)
[72]: https://v2.tauri.app/ (accessed 2026-01-20, official, high)
[73]: https://devnewsletter.com/p/state-of-swift-2026 (accessed 2026-01-20, community, medium)
[74]: https://rhx.github.io/SwiftGtk/ (accessed 2026-01-20, official, medium)
[75]: https://liuliu.me/eyes/write-cross-platform-gui-in-swift-like-it-is-1998/ (accessed 2026-01-20, community, low)
[76]: https://www.pythonguis.com/pyside6-tutorial/ (accessed 2026-01-20, community, high)
[77]: https://doc.qt.io/qtforpython-6/release_notes/pyside6_release_notes.html (accessed 2026-01-20, official, high)
[78]: https://www.pythonguis.com/faq/pyqt6-vs-pyside6/ (accessed 2026-01-20, community, high)
[79]: https://www.pythonguis.com/faq/which-python-gui-library/ (accessed 2026-01-20, community, high)
[80]: https://charleswan111.medium.com/pyqt-vs-pyside-a-comprehensive-comparison-for-python-qt-development-4f525f879cc4 (accessed 2026-01-20, community, medium)
[81]: https://doc.qt.io/qtforpython-6/deployment/deployment-pyside6-deploy.html (accessed 2026-01-20, official, high)
[82]: https://www.pythonguis.com/tutorials/packaging-pyside6-applications-windows-pyinstaller-installforge/ (accessed 2026-01-20, community, high)
[83]: https://www.pythonguis.com/tutorials/packaging-pyside6-applications-pyinstaller-macos-dmg/ (accessed 2026-01-20, community, high)
[84]: https://www.qtcentre.org/threads/45723-Hello-World-program-5-MB! (accessed 2026-01-20, community, medium)
[85]: https://www.qt.io/blog/reducing-binary-size-of-qt-applications-part-3-more-platforms (accessed 2026-01-20, official, medium)
[86]: https://medium.com/@reach.subhanu/optimizing-flutter-app-startup-cold-launch-to-ready-in-2-seconds-4ed32fa7a95f (accessed 2026-01-20, community, medium)
[87]: https://blog.sentry.io/4-mobile-vitals-to-keep-a-pulse-on-your-flutter-applications/ (accessed 2026-01-20, company-blog, medium)
[88]: https://github.com/AvaloniaUI/Avalonia/issues/5242 (accessed 2026-01-20, community, medium)
[89]: https://github.com/AvaloniaUI/Avalonia/discussions/11046 (accessed 2026-01-20, community, medium)
[90]: https://docs.avaloniaui.net/docs/deployment/native-aot (accessed 2026-01-20, official, high)
[91]: https://avaloniaui.net/blog/10-avalonia-performance-tips-to-supercharge-your-app (accessed 2026-01-20, official, medium)
[92]: https://szibele.com/memory-footprint-of-gui-toolkits/ (accessed 2026-01-20, community, low)
[93]: https://forums.swift.org/t/swift-gui-apps-on-macos-linux-windows/53230 (accessed 2026-01-20, community, medium)
[94]: https://www.swift.org/getting-started/ (accessed 2026-01-20, official, high)
[95]: https://siytek.com/an-easy-beginners-guide-to-making-a-macos-gui-using-swift/ (accessed 2026-01-20, community, medium)
[96]: https://techpreneurr.medium.com/zig-1-0-drops-in-2026-why-c-developers-are-secretly-learning-it-now-3188f8bcfedf (accessed 2026-01-20, community, low)
