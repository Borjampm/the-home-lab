# Go-Based Desktop GUI Frameworks Research

## Abstract

This report examines four major Go-based frameworks for building lightweight desktop applications on Linux and macOS: Fyne, Wails, Gio, and gotk4. Research findings indicate that Wails offers the smallest binary sizes (4-8MB) and lowest memory footprint by leveraging native webviews, while Fyne provides the most comprehensive documentation and largest community (27.8k GitHub stars) but suffers from larger binaries (30-103MB) and performance concerns. Gio delivers excellent performance through immediate-mode rendering with competitive binary sizes, but requires steeper learning curve and has an unstable API (pre-1.0). GTK4 bindings (gotk4) produce small binaries but require external GTK dependencies and have limited macOS support quality. All frameworks are production-capable, with varying trade-offs between developer experience, performance, and deployment complexity.

## Findings

### Framework Overview Comparison

| Framework | GitHub Stars | Type | Latest Release | Community Size |
|-----------|--------------|------|----------------|----------------|
| Wails | 32.3k | Web-based (native webview) | v2.11.0 (Nov 2025) | 315 contributors, 1.6k forks |
| Fyne | 27.8k | Native Go (OpenGL) | v2.7.2 (Jan 2026) | 202 contributors, 1.5k forks |
| Gio | 2.1k | Immediate-mode (OpenGL/D3D11) | v0.9.0 (2025) | 62 contributors, 156 forks |
| gotk4 | 646 | GTK4 bindings (C/CGo) | Active (branch 4) | 8 contributors, 29 forks |

### 1. Binary Size Analysis

#### Wails
- **Hello World:** 4-8MB (stripped)
- **Small Applications:** 4-8MB typical
- **Comparison:** ~25-50x smaller than Electron (100-200MB)
- **Key Factor:** Uses native OS webview instead of bundling browser

**Technical Details:** Wails compiles Go code to native binary and runs UI in the operating system's built-in WebView (WebKit on macOS/Linux, WebView2 on Windows), resulting in dramatically smaller binaries compared to Electron which bundles entire Chromium runtime.

#### Fyne
- **Hello World:** 16MB (Windows), size varies by platform
- **Typical Applications:** 30MB (v2.4.x), 60-103MB (v2.5+)
- **Platform Variance:** 103MB on Windows, 62MB on Linux (recent versions)
- **Reduction Technique:** Can be reduced using `go build -ldflags="-s -w"` to strip debug symbols

**Known Issues:** Binary size increased significantly from v2.4 to v2.5+, with reports showing triple the size for identical code. Users report concerns about bloat, with simple applications producing unexpectedly large binaries.

#### Gio
- **Typical Applications:** Comparable to standard Go binaries (estimated 5-15MB)
- **Design Philosophy:** Minimal dependencies - only platform libraries for window management, input, and GPU
- **Advantage:** Efficient vector renderer avoids texture baking, keeping binary size reasonable

**Note:** Specific benchmark data for Gio hello world binaries was not available in public documentation, but the minimal-dependency architecture suggests competitive sizes with other native Go frameworks.

#### gotk4
- **Binary Size:** Small standalone executable (estimated 5-10MB)
- **External Dependencies:** Requires GTK4 libraries installed on system
- **Trade-off:** Smaller binary but requires bundling GTK libraries for distribution
- **Total Footprint:** Binary + GTK4 runtime dependencies can be 50-100MB+

**Architectural Difference:** GTK bindings don't compile libraries into binary, requiring external dependencies which "ruins the simplicity of a Go program consisting of a single executable file."

### 2. Memory Usage

#### Wails
- **Idle Memory:** Low, comparable to native applications
- **Runtime:** Efficient due to native webview usage
- **Architecture:** Sub-millisecond overhead for Go-JS bridge communication
- **Comparison:** Simple Electron apps use 100-200MB RAM; Wails uses significantly less

**Performance Characteristic:** Because Wails uses the OS's native webview instead of embedding a browser, memory usage is substantially lower than Electron-based alternatives.

#### Fyne
- **Known Issues:** Significant memory concerns documented:
  - v2.3.0: Memory usage jumped from ~100MB to >1000MB (potentially font rendering related)
  - Entry widgets: 14.2MB baseline → 18MB with "Hello" → 22.5MB with 100 characters
  - Image viewing: Reports of needing 20GB RAM for viewing many images due to caching issues
- **Performance Issues:** Laggy window resizing, slow Select widget opening (100-200 entries)

**Critical Concern:** Multiple GitHub issues document memory leaks and excessive memory consumption, particularly in recent versions (v2.3.0+).

#### Gio
- **Memory Profile:** Efficient, with focus on avoiding dynamic heap allocations
- **Performance Priority:** "Performance is a major focus... lightning fast!"
- **Design:** Immediate-mode architecture minimizes retained state
- **Rendering:** Vector rendering without texture baking reduces memory overhead

**Optimization:** Gio's immediate-mode paradigm and careful memory management result in low memory footprint competitive with native applications.

#### gotk4
- **Memory Management:** GTK3 and GTK4 use proper Go garbage collector integration
- **Approach:** Runtime finalizers properly free memory when no longer needed
- **Concerns:** Potential memory leaks and crashes noted in certain API parts (gotk4 is autogenerated)

**Note:** GTK itself has mature memory management, but Go bindings add layer of complexity in managing memory across C/Go boundary.

### 3. Startup Time & Performance

#### Wails
- **Build Time:** Average 12,290ms on Windows x64 (vs Tauri's 343,135ms)
- **Startup:** Fast, due to native binary and lightweight webview
- **Bridge Overhead:** Sub-millisecond for Go-JS communication
- **Characteristics:** Small binaries, fast startup, low memory usage make applications "feel native"

#### Fyne
- **Compilation:** Slow, requires C compiler (CGo)
- **Build Cache:** Approximately 15 minutes without build cache for simple examples (similar to GTK)
- **Runtime Performance:** Multiple concerns documented:
  - "Really, really bad" performance on macOS
  - Excessive CPU consumption
  - Slower rendering than native window rendering on macOS/Windows 10
  - Laggy window resizing in v2.3.0+

**Developer Experience Impact:** Slow compilation and CGo requirement make development iteration cycles slower compared to pure Go alternatives.

#### Gio
- **Performance:** "Lightning fast" once familiar with the framework
- **Optimization:** Avoids dynamic heap allocations, immediate-mode efficiency
- **Rendering:** Efficient vector renderer based on Pathfinder project
- **GPU Backend:** OpenGL ES and Direct3D 11, migrating to compute-shader-based piet-gpu

**Technical Advantage:** Immediate-mode architecture and GPU-accelerated vector rendering provide excellent runtime performance.

#### gotk4
- **Compilation:** ~15 minutes to build simple example without cache (CGo overhead)
- **Alternative:** puregotk using purego achieves ~40 seconds (no CGo)
- **Cross-compilation:** Harder due to CGo involvement
- **Runtime:** Native GTK performance on Linux, less optimized on macOS

### 4. UI Approach

#### Wails (Web-Based)
- **Frontend:** HTML/CSS/JavaScript with modern frameworks (React, Vue, Svelte)
- **Backend:** Go
- **Rendering:** Native OS webview (WebKit/WebView2)
- **Look & Feel:** Web-based UI, can achieve modern appearances
- **Developer Experience:** Familiar for web developers

**Platform Webview Details:**
- macOS: Cocoa frameworks with WebKit
- Linux: GTK with WebKit2GTK
- Windows: Microsoft WebView2 (Blink/Edge)

#### Fyne (Custom Rendering)
- **Approach:** Material Design-inspired, custom OpenGL rendering
- **Philosophy:** Consistent appearance across platforms (not platform-native)
- **Graphics:** Vector graphics for resolution independence
- **Themes:** Built-in themes designed to "look native on all supported platforms"

**Design Trade-off:** Prioritizes consistency over platform-specific native look, which some developers view as a limitation compared to truly native UIs.

#### Gio (Custom Rendering - Immediate Mode)
- **Architecture:** Immediate-mode GUI paradigm
- **Rendering:** Custom vector renderer (Pathfinder-based)
- **Graphics Backend:** OpenGL ES, Direct3D 11, migrating to compute-shader piet-gpu
- **Text Rendering:** Outline-based without texture baking for efficient animations
- **Philosophy:** Resolution-independent, portable across platforms

**Technical Sophistication:** Efficient compute-shader-based rendering provides both performance and visual quality, with "bleeding-edge 2D graphics technology."

#### gotk4 (Native Widgets)
- **Approach:** True native GTK widgets via bindings
- **Look & Feel:** Native on Linux, less native on macOS/Windows
- **Integration:** Access to full GTK ecosystem and widgets
- **Limitation:** "Without a true native look and feel" on macOS and Windows

**Platform Considerations:** Excellent Linux integration as GTK's primary platform, but macOS/Windows support improved in GTK4 while still not fully native.

### 5. Cross-Platform Support Quality

#### Wails
**Supported Platforms:** Windows, macOS, Linux

**Quality Assessment:**
- **Linux:** Excellent, uses GTK + WebKit2GTK
- **macOS:** Excellent, uses Cocoa + WebKit (native)
- **Windows:** Excellent, uses WebView2 (Edge/Chromium-based)
- **Consistency:** Uses native webview on each platform for native feel

**Key Advantage:** Platform-specific webview implementations ensure good integration with each OS's rendering pipeline and behavior.

#### Fyne
**Supported Platforms:** Windows, macOS, Linux, iOS, Android, Web (WebAssembly)

**Quality Assessment:**
- **Cross-compilation:** Excellent tooling (fyne-cross)
- **Consistency:** Same appearance across all platforms (Material Design-inspired)
- **Performance Issues:** Documented problems specifically on macOS
- **Mobile:** Full support for iOS and Android from single codebase

**Critical Design Issue:** "Fundamental design flaw" attempting to use same code for desktop and mobile, according to some developers. Performance particularly problematic on macOS.

#### Gio
**Supported Platforms:** Linux, macOS, Windows, Android, iOS, FreeBSD, OpenBSD, WebAssembly

**Quality Assessment:**
- **Architecture:** Layered abstraction for platform details
- **Platform Layer:** Handles OS-specific interactions
- **GPU Backends:** Different graphics APIs per platform
- **Recent Improvements:** macOS improvements in 2025 updates (v0.9.0)
- **Portability:** Applications run on any platform without code changes

**2025-2026 Status:** Active development with "many small window implementation bugfixes and macOS improvements" demonstrating ongoing platform quality improvements.

#### gotk4
**Supported Platforms:** Linux (primary), macOS (improved in GTK4), Windows (limited)

**Quality Assessment:**
- **Linux:** Excellent native integration, GTK's primary platform
- **macOS:** GTK4 brings "better & faster macOS support" with new backend
  - Native display manager, keyboard, pointing device integration
  - Menu bar integration, standard keyboard shortcuts
  - Software (Cairo) or GPU (OpenGL) rendering
  - CI builds GTK on every commit for macOS testing
- **Windows:** Basic support, not native look/feel
- **Limitation:** "Without a true native look and feel" on non-Linux platforms

**Platform Dependencies:** Requires GTK libraries on system, which may not be available by default on macOS/Windows.

### 6. Learning Curve & Documentation

#### Wails
**Documentation Quality:**
- **Official Docs:** Comprehensive at v3alpha.wails.io and wails.io
- **Status:** v3 in ALPHA but API "reasonably stable"
- **Getting Started:** Clear tutorials for first application
- **Development Tools:** CLI tool (wails3), dev mode with hot reload
- **Build System:** Uses Task by default, "bring your own tooling" philosophy

**Learning Curve:** Moderate - requires understanding both Go and web technologies

**Strengths:**
- Familiar paradigm for web developers
- Good documentation and practical tutorials
- Active development and refinement of docs before final release

**Community Resources:** Multiple tutorials, blog posts, and examples available (awesome-wails list)

#### Fyne
**Documentation Quality:**
- **Official Docs:** Comprehensive at docs.fyne.io
- **Resources:** Written docs, video tutorials (YouTube), demo app
- **Coverage:** Beginner to advanced topics, cross-platform development
- **API:** Available at pkg.go.dev with extensive examples

**Learning Curve:** Easy - "Really easy to code with," designed for "any experience level"

**Strengths:**
- Gentlest learning curve of all frameworks reviewed
- Excellent documentation quality
- Multiple learning formats (written, video, demo app)
- Simple API: app.New() → window → widget tree → ShowAndRun()
- Active community (Fyne Conf 2025, Discord, Slack #fyne)

**Ecosystem:** Examples repository (fyne-io/examples), application showcase (apps.fyne.io)

#### Gio
**Documentation Quality:**
- **Official Docs:** gioui.org with "Get Started" and "Learn" sections
- **Perception:** "Official website is a bit advanced"
- **Resources:** Multiple community tutorials, example repository
- **API Stability:** Pre-1.0, "many breaking changes" between versions

**Learning Curve:** Steep - "After this step, the learning curve will not be so steep" (implying initial difficulty)

**Strengths:**
- Community-created tutorials (10-part "GUI with Gio" series from scratch)
- Architecture brain-dump document for understanding design
- Example programs (gio-example repository, "kitchen" widget demo)

**Challenges:**
- Immediate-mode paradigm unfamiliar to many developers
- "Unclear where to start" despite good examples
- API instability requires "rough time" when upgrading
- "Usage appears complex and cumbersome... decent amount of boilerplate"

**Community Feedback:** "Lightning fast once you get used to it" but initial learning investment is higher.

#### gotk4
**Documentation Quality:**
- **Official Docs:** Package documentation at pkg.go.dev
- **GTK4 Docs:** Can reference official GTK4 documentation (docs.gtk.org/gtk4)
- **Go-Specific:** diamondburned/gotk4-examples repository
- **Status:** "Most of pkg/gtk/v4 and pkg/gtk/v3 are ready to be used"

**Learning Curve:** Steep - requires knowledge of both Go and GTK architecture

**Challenges:**
- Autogenerated bindings may have incomplete/missing APIs
- Memory leaks and crashes possible in certain API parts
- Build times very slow initially (~15 minutes without cache)
- Limited Go-specific tutorials compared to other frameworks

**Strengths:**
- Can leverage extensive GTK documentation and community
- Examples repository provides basic patterns
- Matrix room for community support

**Requirement:** Must understand GTK widget model and Go's CGo interactions.

### 7. Community Health & Activity

#### Wails
**Metrics:**
- **Stars:** 32.3k (highest of all Go GUI frameworks)
- **Forks:** 1.6k
- **Contributors:** 315 (plus 301 additional)
- **Used By:** 5.3k repositories
- **Latest Release:** v2.11.0 (November 2025)
- **Commits:** 3,957 on master
- **Total Releases:** 279

**Health Indicators:**
- Most starred Go GUI framework
- Active development (v3 alpha ongoing)
- Large contributor base
- Substantial downstream adoption
- Featured in awesome-go curated list
- Regular releases and updates

**Community Activity:** Active discussions, tutorials, and blog posts. Multiple developers building production applications.

#### Fyne
**Metrics:**
- **Stars:** 27.8k (second-highest)
- **Forks:** 1.5k
- **Contributors:** 202 (plus ~100 volunteers)
- **Latest Release:** v2.7.2 (January 2026)
- **Commits:** 12,373
- **Total Releases:** 57
- **Imported By:** 3,465 projects

**Health Indicators:**
- Became most popular GUI toolkit for Go by stars (Dec 2019)
- Trending #1 GitHub project (Feb 2020)
- Annual conference (Fyne Conf 2025 in Edinburgh)
- Multiple communication channels (Slack #fyne, Discord)
- Application showcase site (apps.fyne.io)
- Commercial support available (Fyne Labs)

**Community Activity:** Very active, well-organized community with regular conferences, comprehensive ecosystem, and commercial backing.

#### Gio
**Metrics:**
- **Stars:** 2.1k
- **Forks:** 156
- **Contributors:** 62
- **Commits:** 3,093 on main
- **Latest Update:** January 7, 2026
- **Releases:** v0.9.0 (2025)

**Health Indicators:**
- Active development through 2025-2026 (newsletters Jan and Sep 2025)
- Primary repository on Sourcehut (git.sr.ht/~eliasnaur/gio), GitHub is mirror
- Mailing list (~eliasnaur/gio@lists.sr.ht)
- Issue tracker accepts email submissions
- OpenCollective for funding

**Concerns:**
- Small maintainer team ("mostly maintained by only two persons")
- Pre-1.0 status means API instability
- Lower community size compared to Fyne/Wails
- "Breaking API or behavior changes will increment minor version"

**Community Activity:** Smaller but dedicated community, active development, regular updates despite limited maintainer team.

#### gotk4
**Metrics:**
- **Stars:** 646
- **Forks:** 29
- **Contributors:** 8
- **Commits:** 531 on main
- **Status:** Active development
- **Multiple Branches:** Branch "4" (GTK 4.14), Branch "4.16" (GTK 4.16)

**Health Indicators:**
- Matrix room for community discussions
- Examples repository maintained
- Regular updates for GTK version compatibility
- Part of larger GTK ecosystem

**Concerns:**
- Small contributor base (8 contributors)
- Limited compared to pure-Go alternatives
- "Memory leaks and crashes may occur in certain parts"
- APIs "might be completely missing"

**Context:** Part of larger GTK community (30+ years development) but Go bindings have limited adoption compared to native Go frameworks.

### 8. Maturity & Production Readiness

#### Wails
**Maturity Level:** Production-ready

**Status:**
- v2: Stable, production deployments (v2.11.0, Nov 2025)
- v3: ALPHA, but "API reasonably stable, applications running in production"
- Used by multiple production applications

**Notable Applications:**
- Tiny RDM - Redis desktop manager (Mac, Windows, Linux)
- Kafka-King - Kafka GUI client
- ES-King - Elasticsearch GUI client
- RWKV-Runner - LLM management tool with OpenAI-compatible API
- Multiple game launchers with auto-updaters

**Assessment:**
- "Full framework for production-ready apps"
- "Use Wails if you want to ship a polished app fast"
- Mature v2 with extensive production usage
- v3 alpha showing stability despite pre-release status

**Limitations:**
- Reported lack of cookie support (WebView2 limitation)
- Smaller ecosystem than Electron
- Fewer third-party integrations

#### Fyne
**Maturity Level:** Production-ready

**Version:** v2.7.2 (January 2026), mature v2.x series

**Notable Applications:**
- FyneDesk - Complete Linux desktop environment (largest Fyne project)
- Supersonic - Popular music player
- Apptrix - Graphical app editor
- Used by Tuffnells, Tailscale, and other businesses

**Assessment:**
- Used in production by multiple companies for internal/utility apps
- 5+ years of development (since 2019)
- Comprehensive toolkit with mobile support
- Annual conference demonstrates ecosystem maturity

**Concerns:**
- Performance issues documented in v2.3.0+
- Memory leaks and consumption problems
- Binary size bloat in recent versions
- "Fundamental design flaws" per some developers
- Quality issues despite popularity

**Verdict:** Production-capable but with known issues requiring workarounds.

#### Gio
**Maturity Level:** Pre-production, approaching stability

**Version:** v0.9.0 (2025), pre-1.0

**Status:**
- Original 2019 announcement: "Very much experimental; don't expect production ready programs"
- 2025-2026: Significant maturation, v0.9.0 with many fixes
- Pre-1.0 warning: "Breaking API or behavior changes will increment minor version"
- "Not yet stable" but improving

**Known Applications:**
- Public transit application (Miles)
- Cross-platform GUI client for gost.plus
- Various community projects (launchers, date pickers, API clients)

**Assessment:**
- Actively developed, improving stability
- API instability is primary concern for production use
- "Many breaking changes... rough time" when upgrading
- Used in commercial application (gio-plugins reference)

**Verdict:** Suitable for adventurous production use with understanding of API instability risks. Best for projects that can absorb breaking changes.

#### gotk4
**Maturity Level:** Mixed - GTK4 is mature, Go bindings are evolving

**GTK4 Status:** Mature, production-ready (30+ years of GTK development)

**Go Bindings Status:**
- Autogenerated bindings, ongoing development
- "Most of pkg/gtk/v4 and pkg/gtk/v3 ready to be used for most purposes"
- Known issues: "Memory leaks and crashes may occur"
- APIs "might be completely missing" - need to file issues

**Platform Production Readiness:**
- Linux: Production-ready (GTK's primary platform)
- macOS: Improved in GTK4 but not fully native
- Windows: Limited, not production-recommended

**Assessment:**
- Leverages mature GTK4 library
- Go bindings have rough edges
- Best for Linux-primary applications
- Requires external GTK dependencies complicating deployment

**Verdict:** Production-ready on Linux for projects comfortable with GTK ecosystem and CGo dependencies. Not recommended for cross-platform applications requiring native look/feel on all platforms.

### Comparison Matrix

| Criterion | Wails | Fyne | Gio | gotk4 |
|-----------|-------|------|-----|-------|
| **Binary Size** | 4-8MB ⭐⭐⭐⭐⭐ | 30-103MB ⭐⭐ | ~5-15MB ⭐⭐⭐⭐ | 5-10MB + deps ⭐⭐⭐ |
| **Memory Usage** | Low ⭐⭐⭐⭐⭐ | High, issues ⭐⭐ | Low ⭐⭐⭐⭐⭐ | Medium ⭐⭐⭐⭐ |
| **Startup Time** | Fast ⭐⭐⭐⭐⭐ | Slow build ⭐⭐⭐ | Fast ⭐⭐⭐⭐⭐ | Slow build ⭐⭐ |
| **UI Quality** | Modern web ⭐⭐⭐⭐ | Material ⭐⭐⭐ | Custom/Clean ⭐⭐⭐⭐ | Native (Linux) ⭐⭐⭐ |
| **Learning Curve** | Moderate ⭐⭐⭐⭐ | Easy ⭐⭐⭐⭐⭐ | Steep ⭐⭐ | Steep ⭐⭐ |
| **Documentation** | Good ⭐⭐⭐⭐ | Excellent ⭐⭐⭐⭐⭐ | Fair ⭐⭐⭐ | Fair ⭐⭐⭐ |
| **Community** | 32.3k stars ⭐⭐⭐⭐⭐ | 27.8k stars ⭐⭐⭐⭐⭐ | 2.1k stars ⭐⭐⭐ | 646 stars ⭐⭐ |
| **macOS Quality** | Excellent ⭐⭐⭐⭐⭐ | Poor perf ⭐⭐ | Good ⭐⭐⭐⭐ | Improved ⭐⭐⭐ |
| **Linux Quality** | Excellent ⭐⭐⭐⭐⭐ | Good ⭐⭐⭐⭐ | Excellent ⭐⭐⭐⭐⭐ | Excellent ⭐⭐⭐⭐⭐ |
| **Production Ready** | Yes ⭐⭐⭐⭐⭐ | Yes (issues) ⭐⭐⭐ | Approaching ⭐⭐⭐ | Linux only ⭐⭐⭐ |
| **API Stability** | Stable ⭐⭐⭐⭐⭐ | Stable ⭐⭐⭐⭐⭐ | Unstable ⭐⭐ | Evolving ⭐⭐⭐ |

## Conflicts

### Binary Size Reporting

**Conflict:** Different sources report varying Fyne binary sizes
- Some sources cite 16-30MB for earlier versions
- Recent reports show 60-103MB for v2.5+ (platform-dependent)
- **Resolution:** Binary size increased significantly between v2.4 and v2.5+, with both measurements accurate for their respective versions

### Fyne Performance Assessment

**Conflict:** Contradictory performance reports
- Official documentation emphasizes GPU acceleration and high performance
- User reports document "really, really bad" performance on macOS, excessive CPU usage
- **Resolution:** Performance degraded in v2.3.0+, particularly affecting macOS. Earlier versions may have performed better. Context matters: newer versions have documented performance regressions.

### Gio Production Readiness

**Conflict:** Varying assessments of production readiness
- 2019 announcement: "Don't expect production ready programs"
- 2025-2026: Used in commercial applications, v0.9.0 with many fixes
- Pre-1.0 status with API instability warnings
- **Resolution:** Framework has matured significantly but remains pre-1.0. Can be used in production by teams comfortable with API instability and potential breaking changes. Not recommended for projects requiring long-term API stability.

### GTK macOS Support Quality

**Conflict:** Different perspectives on GTK4 macOS support
- GTK4 documentation promotes "better & faster macOS support"
- Community feedback: "Without a true native look and feel"
- **Resolution:** GTK4 improved macOS support significantly (new backend, better performance, menu integration) but still doesn't achieve fully native macOS appearance and behavior. Improvement is real but limitations remain.

## Follow-up Directions

### Performance Benchmarking
Conduct systematic performance benchmarks comparing:
- Cold start times across all frameworks
- Memory usage under typical workloads
- CPU usage during UI operations (scrolling, animations, window resizing)
- Build time comparisons with and without build cache
- Binary size measurements for identical applications across frameworks

**Value:** Empirical data would resolve conflicting performance claims and provide quantitative decision-making criteria.

### Fyne Performance Investigation
Deep dive into Fyne v2.3.0+ performance regressions:
- Identify specific changes causing memory bloat
- Test performance across different platforms (Linux vs macOS vs Windows)
- Evaluate workarounds and configuration options
- Compare v2.2.x vs v2.3.0+ performance profiles

**Value:** Understanding root causes could guide framework selection or inform contribution to fixing issues.

### Gio API Stability Roadmap
Research Gio's path to 1.0:
- Interview maintainers about timeline and remaining API changes
- Document breaking changes between recent versions
- Identify stable vs unstable API surfaces
- Create upgrade guides for version migrations

**Value:** Would help assess risk of Gio adoption for production projects and inform migration strategies.

### Cross-Platform UI Consistency Study
Compare actual application appearance and behavior:
- Build identical test application in all frameworks
- Document platform-specific rendering differences
- Evaluate native vs custom UI approaches
- Test accessibility features and integration

**Value:** Visual and behavioral comparisons would inform decisions about UI consistency vs native platform integration trade-offs.

### Deployment & Distribution Analysis
Investigate real-world deployment complexities:
- App store submission processes (macOS App Store, Linux package managers)
- Code signing and notarization requirements
- Dependency bundling strategies (especially gotk4)
- Auto-update mechanisms availability
- Installer size and installation experience

**Value:** Deployment complexity significantly impacts production viability beyond just development experience.

### Hybrid Approach Evaluation
Explore combining frameworks:
- Wails for complex UI with web technologies
- Gio for performance-critical components
- CGo considerations when mixing approaches
- Inter-process communication patterns

**Value:** Real-world applications may benefit from using different frameworks for different components based on requirements.

## Sources

[1]: https://github.com/fyne-io/fyne/issues/3499 (accessed 2026-01-20, company-blog, high) - v2.3.0 Performance and Memory issues
[2]: https://github.com/fyne-io/fyne/issues/5008 (accessed 2026-01-20, official, high) - Built binary is too large
[3]: https://github.com/fyne-io/fyne (accessed 2026-01-20, official, high) - Fyne GitHub repository with 27.8k stars
[4]: https://en.wikipedia.org/wiki/Fyne_(software) (accessed 2026-01-20, community, high) - Fyne Wikipedia overview
[5]: https://github.com/wailsapp/wails/discussions/1455 (accessed 2026-01-20, official, high) - Wails performance and build size benchmarks
[6]: https://github.com/wailsapp/wails (accessed 2026-01-20, official, high) - Wails GitHub repository with 32.3k stars
[7]: https://github.com/tauri-apps/tauri/discussions/3521 (accessed 2026-01-20, community, medium) - Comparison with Wails
[8]: https://wails.io/docs/introduction/ (accessed 2026-01-20, official, high) - Wails official documentation
[9]: https://gioui.org/ (accessed 2026-01-20, official, high) - Gio UI official website
[10]: https://github.com/gioui/gio (accessed 2026-01-20, official, high) - Gio GitHub repository with 2.1k stars
[11]: https://gioui.org/news/2025-09 (accessed 2026-01-20, official, high) - Gio Newsletter September 2025
[12]: https://gioui.org/news/2025-01 (accessed 2026-01-20, official, high) - Gio Newsletter January 2025
[13]: https://github.com/diamondburned/gotk4 (accessed 2026-01-20, official, high) - gotk4 GitHub repository
[14]: https://docs.gtk.org/gtk4/ (accessed 2026-01-20, official, high) - GTK4 official documentation
[15]: https://www.phoronix.com/news/GTK4-macOS-Improved (accessed 2026-01-20, community, high) - GTK4 macOS improvements
[16]: https://blog.logrocket.com/best-gui-frameworks-go/ (accessed 2026-01-20, community, medium) - Best GUI frameworks for Go comparison
[17]: https://medium.com/@balle.johannes/no-good-go-gui-9f54813e9bf (accessed 2026-01-20, community, medium) - Critical analysis of Go GUI frameworks
[18]: https://news.ycombinator.com/item?id=40399965 (accessed 2026-01-20, forum, medium) - HN discussion on Fyne and Gio limitations
[19]: https://docs.fyne.io/ (accessed 2026-01-20, official, high) - Fyne official documentation
[20]: https://v3alpha.wails.io/ (accessed 2026-01-20, official, high) - Wails v3 alpha documentation
[21]: https://jonegil.github.io/gui-with-gio/ (accessed 2026-01-20, community, high) - GUI with Gio tutorial series
[22]: https://github.com/diamondburned/gotk4-examples (accessed 2026-01-20, official, high) - gotk4 examples repository
[23]: https://github.com/wailsapp/awesome-wails (accessed 2026-01-20, official, high) - Curated list of Wails applications
[24]: https://conf.fyne.io/ (accessed 2026-01-20, official, high) - Fyne Conf 2025
[25]: https://apps.fyne.io/ (accessed 2026-01-20, official, high) - Fyne applications showcase
