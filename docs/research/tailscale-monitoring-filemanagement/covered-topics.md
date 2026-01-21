# Covered Topics

This file tracks topics already researched to prevent duplicate work.

## Topics Covered

### Hardware Monitoring - Web Solutions (R1)
- Netdata: Real-time monitoring, zero-config, comprehensive metrics
- Glances Web Mode: Lightweight Python-based monitoring
- Grafana + Prometheus: Enterprise monitoring stack
- Cockpit: Red Hat web console with system management
- Uptime Kuma: Service uptime monitoring (not suitable for hardware metrics)

Key findings:
- Netdata recommended for most users (best balance)
- Glances for minimal resource usage
- Cockpit for unified management
- All work well over Tailscale with proper binding

---

### Hardware Monitoring - Native/Desktop Solutions (R2)
- btop: Modern beautiful TUI, zero overhead, best terminal option
- htop: Classic process viewer, reliable
- glances TUI: Python-based terminal monitoring
- gotop: Go-based terminal dashboard
- ServerCat: Premium macOS menu bar app ($5.99)
- iStatistica: Mac-focused, limited Linux support
- Custom scripts: DIY with macOS notifications

Key findings:
- btop recommended for terminal monitoring (SSH-based, zero overhead)
- ServerCat best native macOS experience (paid)
- glances client/server for persistent monitoring with alerts
- All SSH-based solutions work perfectly over Tailscale

---

### File Management - Web Solutions (R3)
- File Browser: Modern Go-based file manager, single binary
- Cockpit File Manager: Built-in file management in Cockpit
- Nextcloud: Full cloud platform (overkill for basic needs)
- Tiny File Manager: Single PHP file solution
- Miniserve: Rust-based file serving (not full management)

Key findings:
- File Browser recommended (modern UI, comprehensive features)
- Cockpit built-in option for unified solution
- Nextcloud too heavy for simple file browsing
- All solutions secure over Tailscale

---

### File Browsing - Native macOS Solutions (R4)
- SSHFS (macFUSE): Mount remote filesystems in Finder
- Cyberduck: Free GUI SFTP client with excellent UI
- SMB/Samba: Native network file protocol
- Mountain Duck: Commercial SFTP mounting ($39)
- Transmit: Premium file transfer app ($45)
- ForkLift: Dual-pane file manager with remote support

Key findings:
- SSHFS recommended for native Finder integration (free)
- Cyberduck for GUI without system extensions
- SMB for best performance and stability
- Commercial options not necessary for home lab use
- All leverage Tailscale's secure tunnel effectively

---

### Unified Solutions (R5)
- Cockpit: Best traditional unified solution, mature and stable
- Webmin: Veteran admin panel, powerful but dated UI
- Ajenti: Modern but inconsistent development
- CasaOS: Modern home lab platform, beautiful UI, app store
- Umbrel: Bitcoin-focused personal server OS
- Yacht: Lightweight Docker management
- Portainer: Professional Docker/Kubernetes platform
- TrueNAS/OpenMediaVault: NAS-focused (not suitable for existing Ubuntu)

Key findings:
- Cockpit recommended for unified approach (70% of best-of-breed features)
- CasaOS excellent for home lab platform with app ecosystem
- Unified solutions trade some features for convenience
- Best-of-breed (Netdata + File Browser) provides 20-30% better features
- Resource overhead: Cockpit (60MB) vs Netdata+File Browser (180MB) vs CasaOS (200-400MB)

---

### Tailscale Security and Performance (R6)
- WireGuard encryption: Automatic, no configuration needed
- Service binding: Must bind to Tailscale IP (100.x.x.x), not 0.0.0.0
- Firewall configuration: ufw rules for defense in depth
- Authentication: Application-level auth still recommended for write operations
- HTTPS: Optional (already encrypted), but useful for browser warnings
- Tailscale certificates: Available via Let's Encrypt
- ACLs: Control device/user access to services
- Performance: Direct connections 95-99% native speed, DERP relay adds 20-100ms latency

Key findings:
- Always bind services to Tailscale IP and configure firewall
- Application authentication required for file management, optional for read-only monitoring
- Direct connections excellent performance, DERP relay acceptable
- SMB fastest for file transfers (80-120 MB/s), SSHFS adequate for editing (20-50 MB/s)
- Web dashboards perform excellently over Tailscale (<500ms load times)

---

### Final Synthesis and Recommendations (R8)
- Evaluated 25+ total solutions across all categories
- Decision matrix by use case: minimal, balanced, full-featured, unified, native
- Resource comparison: 0MB (SSH-only) to 400MB+ (CasaOS platform)
- Cost analysis: $0 (all free solutions) to $50+ (optional paid enhancements)
- Implementation roadmap: 4-phase approach
- Security checklist: Network, Tailscale, and application layers
- Performance benchmarks: Real-world tests across protocols
- Migration paths between configurations

**Top Recommendations**:
1. Best Overall: Netdata + File Browser + SSHFS (180MB RAM, best features)
2. Runner-Up: Cockpit (60MB RAM, unified simplicity)
3. Alternative: CasaOS platform for multi-app home labs (200-400MB RAM)

**Quick Start Winner**: Netdata + File Browser
- 30-minute setup
- Best-in-class tools
- Reasonable resources
- Excellent security when properly configured
- Great performance over Tailscale

---

## Research Statistics

- Total solutions evaluated: 25+
- Research reports completed: 7 (R1-R6, R8)
- Novel information rate: 90-100% across all reports
- Total research depth: Comprehensive
- Confidence level: High

---

## What Was NOT Covered (Out of Scope)

- Windows server monitoring (focus was Ubuntu)
- iOS/Android mobile clients (focus was macOS)
- Physical hardware monitoring (IPMI, BMC)
- Application-specific monitoring (databases, web servers)
- Automated provisioning tools (Ansible, Terraform)
- Backup solutions (separate topic)
- VPN alternatives to Tailscale (Tailscale was given)

---

## Update Log

- 2026-01-20: Investigation initialized
- 2026-01-21: Completed P0 tasks (R1, R3, R4) - web monitoring, web files, native files
- 2026-01-21: Completed P1 tasks (R2, R5, R6) - native monitoring, unified solutions, Tailscale security
- 2026-01-21: Completed R8 final synthesis and recommendations
- Investigation status: COMPLETE (all P0/P1 priorities met)

---

## Recommendations for Future Research

If expanding this investigation:
1. R7 (Setup Guides): Detailed step-by-step configuration for top 3 solutions
2. Automated deployment scripts (Ansible playbooks)
3. Backup strategies for monitored systems
4. Alerting and notification deep dive
5. Multi-server monitoring federation
6. Application performance monitoring (APM)
7. Log aggregation and analysis
8. Incident response workflows
