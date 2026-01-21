# Rust-Based Desktop Frameworks Deep Dive

## Abstract

This report provides a comprehensive analysis of five major Rust-based frameworks for building lightweight desktop applications: Tauri, Dioxus, Iced, egui, and Slint. Based on research conducted in January 2026, Tauri emerges as the most mature option with extensive production use and the largest community (102K GitHub stars), offering web-based UIs with extremely small binaries (2.5-3 MB) and minimal memory footprint (30-40 MB idle). For pure Rust developers, egui provides the fastest development experience with the smallest binaries (especially for WASM), while Iced offers a more structured architecture suitable for complex applications. Dioxus shows strong potential as a React-like framework with unified Rust codebases across platforms, and Slint stands out for embedded and resource-constrained environments with runtime memory usage under 300KB. All frameworks demonstrate production readiness as of 2025-2026, with each excelling in different use cases.

## Findings

### 1. Tauri - Web-Based UI with Rust Backend

#### Overview
Tauri enables developers to build smaller, faster, and more secure desktop and mobile applications with a web frontend using the system's native WebView. It is the most mature and widely adopted Rust desktop framework as of 2026.

#### Performance Metrics

**Binary Size:**
- Hello World: 2.5-3 MB
- Small apps: Under 10 MB
- Benchmark comparison: 5 MB (release build on Linux)
- Significantly smaller than Electron (80-120 MB typical)

**Memory Usage:**
- Idle RAM: 30-40 MB for basic apps
- ~180 MB for more complex applications
- 50% less memory than Electron equivalents
- Note: Only ~5MB is the actual Tauri/Rust process; remainder is WebView2/WebKit

**Startup Time:**
- Consistently under 500ms in production
- Benchmark measurements (Linux):
  - Window appearance: 100-150ms
  - Final render: 366-417ms
- Real-world production apps start in under 1 second

**Input Lag:**
- 2-3 frames (32-48ms at 60fps)

**Resize Performance:**
- 10-15 FPS during window resize
- UI can lag behind window changes

#### UI Approach
- Web-based frontend (HTML/CSS/JavaScript/TypeScript)
- Rust backend for system interactions
- Uses OS-native WebView (WebKit on macOS, WebView2 on Windows, WebKitGTK on Linux)
- No bundled browser engine (key difference from Electron)
- Requires IPC (Inter-Process Communication) between frontend and backend

#### Cross-Platform Support
- **Linux:** Excellent support
- **macOS:** Excellent support (10.15+)
- **Windows:** Excellent support (7+)
- **Mobile:** iOS 9+ and Android 7+ support in Tauri 2.0

#### Learning Curve

**For Frontend Developers:**
- Low barrier to entry if familiar with web technologies
- Existing frontend skills directly transferable
- Can start building immediately with HTML/CSS/JS knowledge

**Rust Component:**
- Moderate challenge for newcomers to Rust
- Must learn Rust for backend functionality
- Requires understanding of IPC patterns
- Two-language context switching (JS/TS + Rust)

**Documentation:**
- Very well-organized and comprehensive
- Official docs at v2.tauri.app
- Community tutorials and practical examples available
- Growing educational resources (17,700+ Discord members)

#### Community Health

**GitHub Statistics (January 2026):**
- Stars: 102,000
- Contributors: 513
- Commits: 5,848 on dev branch
- Latest release: tauri-cli v2.9.6 (December 9, 2025)

**Adoption:**
- Used by ~90 companies including Cloudflare, ABBYY, Cboe Global Markets
- 35% year-over-year growth in adoption
- 17,700+ Discord members
- Active GitHub Discussions community
- Partnership with CrabNebula for commercial support

#### Maturity & Production Readiness

**Status:** Production-ready and widely adopted

**Notable Production Applications:**
- **Ariadne** (Git Client): Starts in under 1 second, <40 MB on disk
- **LumenTrack** (Cryptocurrency Portfolio Tracker): Security-first approach
- **LangForge** (Educational Software): Privacy-sensitive learning applications
- Enterprise tools: Operations dashboards, internal tools, offline SaaS

**Version Status:**
- Tauri 2.0 released late 2024
- Stable API with regular maintenance releases
- Mobile support now production-ready

**Commercial Support:**
- CrabNebula offers enterprise support
- Commons Conservancy backing
- Active security audits and updates

### 2. Dioxus - React-Like Rust Framework

#### Overview
Dioxus is a fullstack app framework for Rust that provides a React-inspired development experience with a unified Rust codebase across web, desktop, and mobile platforms. It's the newest major framework among those reviewed but shows rapid maturity.

#### Performance Metrics

**Binary Size:**
- Desktop: <3 MB (portable binaries)
- Desktop optimized: <5 MB typical
- Web (WASM): 50-70KB for hello world (compressed)
- Web (WASM): ~275KB hello world with full optimizations
- Significantly smaller than Electron (~100MB)

**Memory Usage:**
- Not extensively documented
- Uses native WebView for desktop, benefiting from WebView efficiency
- Virtual DOM adds slight overhead compared to pure Tauri

**Startup Time:**
- Subsecond rebuilds during development
- Specific cold start metrics not widely published
- Development features hot-reloading with millisecond updates

**Rendering Performance:**
- 30% faster than Yew in WebAssembly
- 50% faster than React for large UIs
- Benefits from Rust's zero-cost abstractions

#### UI Approach
- React-like virtual DOM architecture
- Pure Rust UI definitions
- JSX-like RSX syntax for component markup
- Component-based architecture
- Uses Tauri under the hood for desktop (via WRY and Tao crates)
- Native rendering on mobile platforms

#### Cross-Platform Support
- **Linux:** Excellent support (portable binaries)
- **macOS:** Excellent support (portable binaries)
- **Windows:** Excellent support (portable binaries)
- **Mobile:**
  - Android: Native support (simply run `dx serve --platform android`)
  - iOS: Native support with device/simulator deployment
- **Web:** WebAssembly with strong performance
- **Server-Side Rendering (SSR):** Built-in support

#### Learning Curve

**For React Developers:**
- Familiar component model and patterns
- RSX syntax similar to JSX
- State management concepts translate well

**For Rust Developers:**
- Unified Rust codebase (no JS/TS required)
- Fewer context switches than Tauri
- Must learn React-like patterns if unfamiliar

**Documentation:**
- Comprehensive official documentation
- Active Discord community
- Growing tutorial ecosystem
- Examples across all platforms

**Development Experience:**
- Subsecond rebuilds
- Zero-config setup
- Hot reloading support
- Integrated backend functions (Axum integration)

#### Community Health

**GitHub Statistics (January 2026):**
- Stars: 34,300
- Contributors: 392
- Commits: 7,052 on main branch
- Latest release: v0.7.3 (January 17, 2026)
- Total releases: 33
- Used by 3,800+ projects as a dependency

**Backing:**
- Full-time core team
- Supported by FutureWei, Satellite.im
- GitHub Accelerator program participant
- Active Discord community

#### Maturity & Production Readiness

**Status:** Rapidly maturing; production-ready for many use cases

**Strengths:**
- Active development with frequent releases
- Growing adoption (3,800+ dependent projects)
- Strong momentum in the Rust web/desktop space
- Comprehensive platform support

**Considerations:**
- Younger than Tauri (less proven at scale)
- API still evolving (v0.7.x as of January 2026)
- Smaller ecosystem of plugins compared to Tauri
- Less extensive production case studies

**Best For:**
- Teams wanting unified Rust codebases
- Cross-platform apps (web + desktop + mobile)
- Developers comfortable with React patterns
- Projects prioritizing Rust throughout the stack

### 3. Iced - Elm-Inspired GUI Library

#### Overview
Iced is a cross-platform GUI library for Rust focused on simplicity and type-safety, inspired by The Elm Architecture. It uses a declarative, retained-mode approach with a model-update-view pattern suitable for structured applications.

#### Performance Metrics

**Binary Size:**
- Debug build: 14 MB
- Release build: 6 MB
- Optimized (opt-level="z", lto=true): 3.1 MB
- Benchmark comparison: 17 MB (Linux release)
- Larger than egui due to backend rendering layers

**Memory Usage:**
- Specific idle metrics vary by backend (wgpu vs glow)
- Generally moderate memory footprint
- Predictable memory usage patterns

**Startup Time:**
- Window appearance: 33-50ms (very fast)
- Final render: 217-333ms
- Faster window appearance than Tauri
- Known issues: Vulkan instance creation can take 500-600ms on Windows
- OpenGL backend shows faster startup

**Input Lag:**
- 3 frames (48ms at 60fps)
- Consistent and predictable

**Resize Performance:**
- 12-30 FPS
- UI synchronized with window changes
- Better than Tauri for resize operations

#### UI Approach
- Declarative, retained-mode architecture
- Elm-inspired Model-Update-View pattern
- Type-safe component system
- Renderer-agnostic design
- Multiple backend options:
  - wgpu (modern, GPU-accelerated)
  - glow (OpenGL-based)
  - tiny-skia (software rendering)
- Native rendering (no web technologies)

#### Cross-Platform Support
- **Linux:** Excellent support
- **macOS:** Excellent support
- **Windows:** Excellent support
- **Web:** WebAssembly support via canvas
- Used by 6,000+ projects
- Notable organizational adopters: auto-stack, playtron-os

#### Learning Curve

**For Elm Developers:**
- Very familiar architecture and patterns
- Natural progression from Elm to Rust

**For General Developers:**
- Moderate learning curve
- Must learn The Elm Architecture pattern
- Strong type system provides excellent compile-time guarantees
- More structured than immediate mode libraries

**Documentation:**
- Official book at book.iced.rs
- Active community forums (Discourse)
- Discord server for support
- Growing collection of examples

**Development Experience:**
- Predictable behavior
- Type-safe refactoring
- Clear separation of concerns
- Good error messages

#### Community Health

**GitHub Statistics (January 2026):**
- Stars: 29,200
- Contributors: 318
- Commits: 6,589 on master branch
- Latest release: v0.14.0 (December 7, 2025)
- Used by 6,000+ projects

**Backing:**
- Sponsored by Cryptowatch team at Kraken.com
- Active maintenance and development
- Regular feature releases

**Recent Developments (v0.14.0):**
- Reactive rendering
- Time-travel debugging support
- Headless testing capabilities
- Input methods support
- Concurrent image decoding and uploading
- Hot reloading support
- Smart scrollbars
- New widgets

#### Maturity & Production Readiness

**Status:** Production-ready with established patterns

**Strengths:**
- Mature architecture (Elm-proven patterns)
- Strong type safety
- Predictable performance
- Good for complex applications with significant state
- Excellent for maintainability and scalability

**Production Use:**
- 6,000+ projects depend on it
- Cryptowatch (Kraken.com) uses it
- Various commercial and open-source projects

**Best For:**
- Structured, complex desktop applications
- Teams valuing type safety and maintainability
- Applications with complex state management
- Developers familiar with Elm or similar architectures

### 4. egui - Immediate Mode GUI

#### Overview
egui is an easy-to-use immediate mode GUI library in Rust that runs on both web and native platforms. It prioritizes simplicity, fast iteration, and portability, making it ideal for developer tools and prototypes.

#### Performance Metrics

**Binary Size:**
- Produces the smallest binaries among frameworks reviewed
- Especially optimized for WASM targets
- Benchmark comparison: 18 MB (Linux release)
- Trade-off: Can increase with complex applications

**Memory Usage:**
- Simple apps: 30-150 MB RAM (varies by backend and complexity)
- Very simple apps (few buttons/labels): ~30 MB with strip enabled
- Apps with textures/images: 150-300+ MB
- Known issues:
  - Version 0.29.1 uses ~2x memory vs 0.28.1 for texture-heavy apps
  - Memory leaks possible with wgpu backend (up to 200MB/s in severe cases)
  - macOS maximized windows: ~160MB
  - Memory continues to grow when minimized (buffer allocation issue)

**Startup Time:**
- Window appearance: Immediate (fastest among benchmarked frameworks)
- Final render: 200-300ms
- Fastest startup in comparison tests

**Input Lag:**
- 2 frames (32ms at 60fps)
- Lowest input lag among benchmarked frameworks

**Frame Time:**
- 1-2ms per frame for most cases
- Extremely fast for simple UIs
- Can become CPU-intensive with many redraws
- Only repaints on interaction or animation (idle = 0% CPU)

#### UI Approach
- Immediate mode paradigm
- UI rebuilt from scratch every frame
- No retained UI tree or virtual DOM
- Direct imperative code (no DSLs or macros)
- Backends:
  - eframe (official framework, cross-platform)
  - Integration options for game engines (Bevy, etc.)
  - wgpu or glow rendering backends
- Custom rendering (looks consistent across platforms)

#### Cross-Platform Support
- **Linux:** Excellent support
- **macOS:** Excellent support
- **Windows:** Excellent support
- **Web:** Excellent WASM support
- **Game Engines:** Integration available for Bevy, macroquad, etc.
- Most portable among frameworks reviewed

#### Learning Curve

**For Any Developer:**
- Very low barrier to entry
- Immediate mode is intuitive (code runs top-to-bottom)
- No special patterns or architecture required
- Fast prototyping and experimentation
- Plain Rust code without macros or DSLs

**Documentation:**
- Comprehensive docs at docs.rs
- Official website with examples
- Discord server for community support
- GitHub Discussions for Q&A

**Development Experience:**
- Fastest iteration cycle
- Live editing and hot reloading
- Great for interactive tools and dashboards
- Simple debugging (code executes linearly)

#### Community Health

**GitHub Statistics (January 2026):**
- Stars: 27,800
- Forks: 1,900
- Commits: 4,167
- Active development with regular releases

**Backing:**
- Sponsored by Rerun (data visualization startup)
- Author Emil Ernerfeldt (emilk) actively maintains
- Strong community engagement
- Dual-licensed: Apache-2.0 / MIT

**Notable Projects:**
- **Rerun Viewer**: Professional data visualization application
- Various developer tools and debuggers
- Scientific visualization applications
- Game development tools

#### Maturity & Production Readiness

**Status:** Production-ready for appropriate use cases

**Strengths:**
- Extremely fast prototyping
- Minimal code required
- Excellent for tools and dashboards
- Strong web support
- Proven in professional applications (Rerun Viewer)

**Limitations:**
- Not ideal for very complex UIs
- CPU usage increases with large scroll areas
- Full layout recalculation every frame
- May lack advanced layout engines of structured frameworks
- Memory leak issues with certain backends (being addressed)

**Best For:**
- Developer tools and debuggers
- Data visualization dashboards
- Scientific applications
- Rapid prototyping
- Tools needing web + native support
- Game development UI overlays

### 5. Slint - Declarative UI Toolkit

#### Overview
Slint is an open-source declarative GUI toolkit for building native user interfaces for embedded systems, desktop, and mobile platforms. It originated as SixtyFPS and focuses on resource efficiency and multi-language support (Rust, C++, JavaScript, Python).

#### Performance Metrics

**Binary Size:**
- Very small binaries
- Emphasis on resource efficiency
- Not extensively benchmarked in public comparisons

**Memory Usage:**
- Runtime: <300KB RAM
- Extremely efficient for resource-constrained environments
- Can run on Raspberry Pi Pico (264KB internal RAM)
- One of the most memory-efficient frameworks

**Startup Time:**
- Optimized for embedded systems
- Fast startup typical
- Specific benchmarks not widely published

**Rendering Performance:**
- GPU accelerated rendering
- Alternative backends: DMA2D, Framebuffer, Line-by-line
- Adaptive rendering strategy based on hardware
- Ahead-of-time compilation optimization

#### UI Approach
- Declarative .slint markup language
- Separate UI description from business logic
- Compiles to machine code (not interpreted)
- Reactive property system
- Tooling includes:
  - Live preview
  - Code completion
  - Material Design 3 components
- Native rendering (no web technologies)

#### Cross-Platform Support
- **Linux:** Excellent (desktop and embedded)
- **macOS:** Excellent
- **Windows:** Excellent with native styles
- **Web:** WebAssembly support
- **Mobile:**
  - Android: Initial port available (v1.3+)
  - iOS: Supported via NLnet funding
- **Embedded:**
  - QNX (BlackBerry)
  - Bare metal (ARM Cortex-M/A, RISC-V, x86, Cadence Tensilica)
  - MCU support: Raspberry Pi, STM32, RP2040

#### Learning Curve

**For Multi-Language Teams:**
- Can use Rust, C++, JavaScript, or Python
- Lowers barrier for polyglot teams
- .slint markup is declarative and readable

**For Rust Developers:**
- Moderate learning curve
- Must learn .slint markup syntax
- Clear separation between UI and logic
- Good documentation and tooling

**Documentation:**
- Official documentation at slint.dev
- Getting started guides
- API documentation for all supported languages
- Examples gallery ("Made with Slint")

**Development Experience:**
- Live preview during development
- Code completion support
- Design tools integration potential
- Clear compile-time errors

#### Community Health

**GitHub Statistics (January 2026):**
- Stars: 21,500
- Contributors: 232
- Commits: 15,996 on master branch
- Latest release: v1.14.1 (October 23, 2025)
- Total releases: 51
- Watchers: 115

**Company Backing:**
- Developed by SixtyFPS GmbH
- Commercial support available
- Over 50 contributors to v1.0
- Active at embedded industry conferences

#### Maturity & Production Readiness

**Status:** Production-ready, especially for embedded systems

**Platform-Specific Maturity:**

**Embedded Systems:**
- Most mature use case
- Five years of development focus
- Preferred option for Rust-based embedded GUIs
- Customers using in production on embedded Linux and Windows

**Desktop:**
- Version 1.0 released in 2023 marked production readiness
- WesAudio and other commercial applications built with Slint
- Ongoing improvements for desktop-specific features
- Less proven than embedded use cases but rapidly improving

**Production Examples:**
- Industrial HMIs (Human-Machine Interfaces)
- Automotive dashboards
- Medical device interfaces
- Consumer electronics
- Replacing Qt QML implementations with reported improvements

**Licensing:**
- Open source with flexible licensing models
- Commercial licensing available for proprietary projects
- Dual licensing strategy for business sustainability

**Best For:**
- Embedded systems with resource constraints
- Industrial automation and HMI
- Automotive applications
- Multi-language teams (Rust/C++/JS/Python)
- Applications requiring <300KB RAM
- Hardware with minimal resources

## Comparison Tables

### Size and Performance Comparison

| Framework | Hello World Binary | Typical App Binary | Idle Memory | Startup Time | Input Lag |
|-----------|-------------------|-------------------|-------------|--------------|-----------|
| **Tauri** | 2.5-3 MB | <10 MB | 30-40 MB | <500ms | 2-3 frames (32-48ms) |
| **Dioxus** | <5 MB | <5 MB | Moderate | Subsecond | Not benchmarked |
| **Iced** | 3.1 MB (optimized) | 6 MB (release) | Moderate | 217-333ms | 3 frames (48ms) |
| **egui** | Smallest (WASM) | 18 MB (native) | 30-150 MB | 200-300ms | 2 frames (32ms) |
| **Slint** | Very small | Very small | <300KB runtime | Fast | Not benchmarked |

### UI Approach and Architecture

| Framework | UI Technology | Architecture | Rendering |
|-----------|--------------|--------------|-----------|
| **Tauri** | Web (HTML/CSS/JS) | Web frontend + Rust backend | Native WebView |
| **Dioxus** | RSX (React-like) | Virtual DOM | Native WebView (desktop) / Native (mobile) |
| **Iced** | Declarative Rust | Elm architecture | Native (wgpu/glow/tiny-skia) |
| **egui** | Immediate mode Rust | No retained tree | Custom (wgpu/glow) |
| **Slint** | .slint markup | Declarative | Native (GPU/software) |

### Platform Support Matrix

| Framework | Linux | macOS | Windows | iOS | Android | Web | Embedded |
|-----------|-------|-------|---------|-----|---------|-----|----------|
| **Tauri** | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓✓ | ✓✓ | ✗ | ✗ |
| **Dioxus** | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓✓ | ✓✓ | ✓✓✓ | ✗ |
| **Iced** | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✗ | ✗ | ✓✓ | ✗ |
| **egui** | ✓✓✓ | ✓✓✓ | ✓✓✓ | via game engines | via game engines | ✓✓✓ | Limited |
| **Slint** | ✓✓✓ | ✓✓✓ | ✓✓✓ | ✓ | ✓ | ✓✓ | ✓✓✓ |

Legend: ✓✓✓ = Excellent, ✓✓ = Good, ✓ = Basic/Emerging, ✗ = Not supported

### Community and Maturity

| Framework | GitHub Stars | Contributors | Latest Release | Production Readiness | Notable Users |
|-----------|--------------|--------------|----------------|---------------------|---------------|
| **Tauri** | 102,000 | 513 | v2.9.6 (Dec 2025) | ✓✓✓ Highly mature | ~90 companies, Cloudflare, ABBYY |
| **Dioxus** | 34,300 | 392 | v0.7.3 (Jan 2026) | ✓✓ Rapidly maturing | 3,800+ dependent projects |
| **Iced** | 29,200 | 318 | v0.14.0 (Dec 2025) | ✓✓✓ Mature | 6,000+ projects, Kraken/Cryptowatch |
| **egui** | 27,800 | Not specified | Active | ✓✓✓ Mature | Rerun Viewer, dev tools |
| **Slint** | 21,500 | 232 | v1.14.1 (Oct 2025) | ✓✓✓ Production (embedded) | Industrial HMI, automotive |

### Learning Curve Assessment

| Framework | Frontend Devs | Rust Devs | Learning Curve | Documentation Quality | Time to First App |
|-----------|---------------|-----------|----------------|-----------------------|-------------------|
| **Tauri** | ✓✓✓ Easy | ✓✓ Moderate | Low (web) / Moderate (Rust) | ✓✓✓ Excellent | <1 hour |
| **Dioxus** | ✓✓ Moderate (React) | ✓✓✓ Easy | Moderate | ✓✓ Good | <2 hours |
| **Iced** | ✓ Challenging | ✓✓ Moderate | Moderate | ✓✓ Good | 2-4 hours |
| **egui** | ✓✓ Moderate | ✓✓✓ Easy | Low | ✓✓✓ Excellent | <1 hour |
| **Slint** | ✓✓ Moderate | ✓✓ Moderate | Moderate | ✓✓ Good | 2-3 hours |

### Use Case Recommendations

| Use Case | Best Choice | Alternative | Rationale |
|----------|-------------|-------------|-----------|
| **Web developers building desktop apps** | Tauri | Dioxus | Leverage existing HTML/CSS/JS skills |
| **Pure Rust cross-platform apps** | Dioxus | Tauri | Unified Rust codebase, no JS required |
| **Developer tools / dashboards** | egui | Tauri | Fast prototyping, minimal code |
| **Complex desktop applications** | Iced | Tauri | Structured architecture, type safety |
| **Embedded systems / IoT** | Slint | - | Resource efficiency, <300KB RAM |
| **Mobile-first apps** | Dioxus | Tauri | Native mobile rendering |
| **Data visualization tools** | egui | Dioxus | Immediate mode, fast iteration |
| **Enterprise desktop software** | Tauri | Iced | Proven at scale, commercial support |
| **Multi-language teams** | Slint | Tauri | Rust/C++/JS/Python support |
| **Rapid prototyping** | egui | Dioxus | Fastest iteration cycle |

## Conflicts

### Binary Size Discrepancies

**Tauri**: One benchmark showed 5 MB while general documentation claims 2.5-3 MB for hello world applications. The difference likely stems from different platforms (Linux vs macOS/Windows), optimization levels, and whether the WebView runtime is included in measurements.

**egui**: Reported as having the "smallest binary especially for WASM" in general comparisons, but showed 18 MB in the Linux native benchmark. This discrepancy is explained by WASM vs native builds - WASM builds are indeed smaller while native builds include rendering backends (wgpu/glow) that increase size.

### Memory Usage Evolution

**egui**: Multiple sources report varying memory usage. GitHub issues note that v0.29.1 uses approximately 2x the memory of v0.28.1 for texture-heavy applications (300MB vs 150MB). Simple applications show 30-150MB range depending on backend and complexity. This indicates memory usage is highly dependent on application type, egui version, and rendering backend choice.

### Startup Time Variations

**Iced**: One benchmark showed 217-333ms for final render, but Windows users reported 2-3 second hangs during startup related to Vulkan instance creation (500-600ms). The OpenGL backend shows better startup performance. This demonstrates platform and backend-specific performance characteristics that aren't captured in single-platform benchmarks.

### Production Readiness Claims

**Dioxus**: While the framework shows strong technical capabilities and growing adoption (3,800+ dependent projects), it's still at v0.7.x (pre-1.0), indicating API stability is not yet guaranteed. Some sources describe it as "rapidly maturing" while others suggest it could "outshine Electron, Flutter, and even Tauri" - the latter being aspirational rather than current reality. The framework is production-ready for many use cases but less battle-tested than Tauri or Iced.

**Slint Desktop**: Sources indicate that while Slint is "production-ready" overall (v1.0+ released), there's a distinction between embedded (very mature) and desktop (improving but less proven) use cases. The team explicitly stated they're working to make it "production-ready for desktop application development," suggesting desktop maturity is still evolving compared to embedded.

### Performance Benchmark Context

The detailed Tauri vs Iced vs egui benchmark (lukaskalbertodt.github.io, 2023) includes an important caveat: "Do not interpret these performance metrics as a universal statement about these GUI libraries." The author notes all tests were on a specific Linux configuration. This is critical context as performance varies significantly by:
- Operating system and version
- Hardware specifications
- Rendering backend choice (wgpu vs glow vs others)
- Build optimization levels
- Application complexity

Many performance claims in framework marketing materials represent optimal conditions rather than guaranteed real-world performance.

## Follow-up Directions

### Performance Benchmarking Gaps

1. **Comprehensive Multi-Platform Benchmarks**: The most detailed benchmark found (2023) only covers Linux. A thorough investigation should include Windows and macOS benchmarks across all frameworks to identify platform-specific performance characteristics.

2. **Real-World Application Profiling**: Beyond "hello world" benchmarks, profiling actual production applications (chat clients, IDEs, media players) would provide more actionable data for framework selection.

3. **Memory Profiling Deep Dive**: Given the reported memory issues with egui and varying consumption patterns, detailed memory profiling across frameworks using tools like valgrind or heaptrack would be valuable.

4. **Battery Life / Power Consumption**: No sources covered power consumption metrics, which are critical for laptop and mobile applications. Testing idle power draw and under-load consumption would fill this gap.

### Emerging Framework Investigation

1. **Newer Alternatives**: Several frameworks mentioned but not deeply researched:
   - **Makepad**: VR, web, and native-rendering UI framework
   - **Floem**: UI framework developed for the Lapce IDE
   - **GPUI**: Used by Zed editor, showing promise
   - **Druid**: Data-driven native GUI (development status unclear)

2. **Framework Evolution Tracking**: Dioxus (v0.7.x) and Iced (v0.14.0) are actively evolving. Tracking their API stabilization and new features would be valuable for future selection.

### Specialized Use Case Research

1. **Accessibility (a11y) Support**: Limited information found about screen reader support, keyboard navigation, and WCAG compliance across frameworks. This is critical for many applications.

2. **Internationalization (i18n) and Localization (l10n)**: How well does each framework support right-to-left languages, CJK text rendering, and translation workflows?

3. **Custom Graphics Integration**: For applications needing custom 2D/3D graphics (CAD, games with UI, data visualization), research integration patterns with graphics libraries.

4. **Plugin/Extension Architectures**: Investigation of how each framework supports plugin systems for extensible applications.

### Commercial Viability Assessment

1. **License Compatibility Analysis**: While all frameworks are open source, detailed analysis of license implications (GPL, Apache, MIT) for commercial products.

2. **Support and Maintenance Costs**: Research into commercial support options, typical maintenance burdens, and long-term sustainability of each project.

3. **Hiring and Talent Pool**: Assess availability of developers familiar with each framework and associated learning/onboarding costs.

### Build and Distribution

1. **CI/CD Integration**: Best practices for automated testing and building across platforms for each framework.

2. **Code Signing and Notarization**: macOS notarization and Windows code signing workflows for each framework.

3. **Auto-Update Mechanisms**: How each framework handles application updates in production.

4. **Distribution Package Sizes**: Installer sizes for different platforms and optimization strategies.

### Framework Ecosystem Analysis

1. **Plugin/Component Libraries**: Availability of ready-made UI components, database connectors, and other libraries.

2. **IDE and Tooling Support**: Integration with VS Code, IntelliJ Rust, and other development environments.

3. **Testing Frameworks**: Unit testing, integration testing, and UI testing approaches for each framework.

## Sources

[1]: https://www.gethopp.app/blog/tauri-vs-electron (accessed 2026-01-20, company-blog, high)
[2]: https://github.com/tauri-apps/tauri/issues/5889 (accessed 2026-01-20, official, high)
[3]: https://medium.com/@srish5945/tauri-rust-speed-but-heres-where-it-breaks-under-pressure-fef3e8e2dcb3 (accessed 2026-01-20, community, medium)
[4]: https://codeology.co.nz/articles/tauri-vs-electron-2025-desktop-development.html (accessed 2026-01-20, company-blog, medium)
[5]: https://edana.ch/en/2025/12/23/advantages-and-limitations-of-the-tauri-application-framework-in-the-enterprise/ (accessed 2026-01-20, company-blog, high)
[6]: https://github.com/tauri-apps/benchmark_results (accessed 2026-01-20, official, high)
[7]: https://raftlabs.medium.com/tauri-vs-electron-a-practical-guide-to-picking-the-right-framework-5df80e360f26 (accessed 2026-01-20, community, medium)
[8]: https://tibicle.com/blog/best-framework-for-desktop-application-in-2026 (accessed 2026-01-20, community, medium)
[9]: https://www.raftlabs.com/blog/tauri-vs-electron-pros-cons/ (accessed 2026-01-20, company-blog, medium)
[10]: https://github.com/tauri-apps/tauri/discussions/3162 (accessed 2026-01-20, official, high)
[11]: https://dioxuslabs.com/ (accessed 2026-01-20, official, high)
[12]: https://github.com/DioxusLabs/dioxus (accessed 2026-01-20, official, high)
[13]: https://medevel.com/dioxus/ (accessed 2026-01-20, community, medium)
[14]: https://www.geeky-gadgets.com/dioxus-rust-framework/ (accessed 2026-01-20, community, medium)
[15]: https://freedium.cfd/89fab2775b35 (accessed 2026-01-20, community, medium)
[16]: https://medium.com/solo-devs/why-cross-platform-development-with-dioxus-in-rust-is-amazing-web-desktop-ios-android-4489b74b7087 (accessed 2026-01-20, community, medium)
[17]: https://github.com/flosse/rust-web-framework-comparison (accessed 2026-01-20, community, medium)
[18]: https://www.blog.brightcoding.dev/2025/06/24/dioxus-a-rust-framework-for-building-cross-platform-apps/ (accessed 2026-01-20, community, medium)
[19]: https://medium.com/@vignarajj/rusts-cross-platform-frontier-guiding-mobile-devs-through-tauri-and-dioxus-in-2025-538917385064 (accessed 2026-01-20, community, medium)
[20]: http://lukaskalbertodt.github.io/2023/02/03/tauri-iced-egui-performance-comparison.html (accessed 2026-01-20, community, high)
[21]: https://www.phoronix.com/news/Iced-0.14-Rust-GUI-LIbrary (accessed 2026-01-20, community, high)
[22]: https://github.com/iced-rs/iced (accessed 2026-01-20, official, high)
[23]: https://iced.rs/ (accessed 2026-01-20, official, high)
[24]: https://book.iced.rs/ (accessed 2026-01-20, official, high)
[25]: https://an4t.com/rust-gui-libraries-compared/ (accessed 2026-01-20, community, medium)
[26]: https://www.webpronews.com/iced-0-14-elm-inspired-rust-gui-library-gets-major-upgrades/ (accessed 2026-01-20, community, medium)
[27]: https://news.ycombinator.com/item?id=46185323 (accessed 2026-01-20, community, medium)
[28]: https://github.com/emilk/egui (accessed 2026-01-20, official, high)
[29]: https://docs.rs/egui/latest/egui/ (accessed 2026-01-20, official, high)
[30]: https://www.egui.rs/ (accessed 2026-01-20, official, high)
[31]: https://github.com/slint-ui/slint (accessed 2026-01-20, official, high)
[32]: https://slint.dev/ (accessed 2026-01-20, official, high)
[33]: https://slint.dev/declarative-rust-gui (accessed 2026-01-20, official, high)
[34]: https://slint.dev/blog/announcing-slint-1.0 (accessed 2026-01-20, official, high)
[35]: https://dev.to/giuliano1993/learn-tauri-by-doing-part-1-introduction-and-structure-1gde (accessed 2026-01-20, community, medium)
[36]: https://blog.logrocket.com/tauri-adoption-guide/ (accessed 2026-01-20, company-blog, medium)
[37]: https://v2.tauri.app/learn/ (accessed 2026-01-20, official, high)
[38]: https://medium.com/solo-devs/tauri-vs-dioxus-the-ultimate-rust-showdown-5d8d305497d6 (accessed 2026-01-20, community, medium)
[39]: https://users.rust-lang.org/t/tauri-or-dioxus/109931 (accessed 2026-01-20, community, medium)
[40]: https://discourse.iced.rs/t/how-is-performance-compared-to-egui/161 (accessed 2026-01-20, official, medium)
[41]: https://github.com/iced-rs/iced/discussions/1478 (accessed 2026-01-20, official, medium)
[42]: https://www.boringcactus.com/2025/04/13/2025-survey-of-rust-gui-libraries.html (accessed 2026-01-20, community, high)
[43]: https://libs.tech/rust/gui-frameworks (accessed 2026-01-20, community, medium)
[44]: https://areweguiyet.com/ (accessed 2026-01-20, community, high)
[45]: https://github.com/DioxusLabs/dioxus/issues/732 (accessed 2026-01-20, official, medium)
[46]: https://github.com/DioxusLabs/dioxus/discussions/1119 (accessed 2026-01-20, official, medium)
[47]: https://github.com/iced-rs/iced/discussions/1531 (accessed 2026-01-20, official, medium)
[48]: https://github.com/iced-rs/iced/issues/615 (accessed 2026-01-20, official, medium)
[49]: https://github.com/emilk/egui/issues/3689 (accessed 2026-01-20, official, medium)
[50]: https://github.com/emilk/egui/issues/2044 (accessed 2026-01-20, official, medium)
[51]: https://github.com/emilk/egui/issues/5245 (accessed 2026-01-20, official, medium)
[52]: https://library.noroff.dev/frameworks/tauri/tauri-case-study/ (accessed 2026-01-20, community, medium)
[53]: https://blog.erikhorton.com/2025/10/05/4-mobile-apps-with-tauri-a-retrospective.html (accessed 2026-01-20, community, medium)
[54]: https://spyro-soft.com/blog/hmi/embedded-ui-frameworks (accessed 2026-01-20, company-blog, medium)
[55]: https://blog.cranksoftware.com/embedded-devices-and-hmi-trends-to-look-out-for-in-2025 (accessed 2026-01-20, company-blog, medium)
[56]: https://slint.dev/blog/making-slint-desktop-ready (accessed 2026-01-20, official, high)
[57]: https://thenewstack.io/dev-news-rust-based-slint-matures-and-shopify-cleans-up/ (accessed 2026-01-20, community, medium)
[58]: https://devclass.com/2023/04/06/interview-the-story-behind-slint-1-0-a-new-cross-platform-gui-toolkit-coded-in-rust/ (accessed 2026-01-20, community, high)
