# Research Plan: Tailscale Home Lab Monitoring and File Management

## Investigation Goals

Research optimal solutions for monitoring and managing a Tailscale-connected home lab consisting of:
- MacBook laptop (client)
- Ubuntu server (server)

## Core Requirements

### Hardware Monitoring
- Real-time CPU, RAM, disk usage/space monitoring
- Access from MacBook to Ubuntu server
- Lightweight on server resources
- Self-hosted, privacy-focused

### File Browsing
- Finder-like UI for remote file management
- View, navigate, and manage server files from MacBook
- Seamless integration preferred

### Cross-cutting Concerns
- Security over Tailscale (leverage encrypted tunnel)
- Ease of setup and maintenance
- Resource efficiency on home server
- Self-hosted/privacy-focused solutions preferred
- Consider unified vs separate tools

## Research Tasks

### Task R1: Web-based Hardware Monitoring Solutions
Priority: P0
Question: What are the best web-based dashboards for hardware monitoring on Ubuntu servers accessible via Tailscale?
Scope: Focus on Netdata, Glances web mode, Grafana+Prometheus, Cockpit, and other popular solutions. Evaluate setup complexity, resource usage, feature set, and security considerations.
Output: reports/r1-web-monitoring-dashboards.md
Status: complete
Completed: 2026-01-21

### Task R2: Native/Desktop Hardware Monitoring Solutions
Priority: P1
Question: What desktop applications or native macOS solutions can monitor remote Linux servers?
Scope: Include terminal-based UIs (htop over SSH, btop, etc.) and native macOS apps that can connect remotely. Consider ease of use vs feature depth.
Output: reports/r2-native-monitoring-apps.md
Status: complete
Completed: 2026-01-21

### Task R3: Web-based File Management Solutions
Priority: P0
Question: What are the best self-hosted web-based file managers for browsing Ubuntu server files from macOS?
Scope: Research FileBrowser, Cockpit file manager, Nextcloud, File Browser alternatives. Evaluate UI/UX, setup complexity, feature set (upload, download, edit, preview), and security.
Output: reports/r3-web-file-managers.md
Status: complete
Completed: 2026-01-21

### Task R4: Native macOS File Browsing Solutions
Priority: P0
Question: What are the best ways to mount and browse remote server files natively in macOS Finder or dedicated apps?
Scope: Cover SSHFS/macFUSE, SMB mounting, SFTP clients (Transmit, Cyberduck, FileZilla), and native Finder integration. Include performance and reliability considerations over Tailscale.
Output: reports/r4-native-file-browsing.md
Status: complete
Completed: 2026-01-21

### Task R5: Unified Solutions Analysis
Priority: P1
Question: Which tools provide both hardware monitoring and file management in a single platform?
Scope: Focus on Cockpit, Webmin, similar all-in-one solutions. Analyze trade-offs between unified platforms vs best-of-breed separate tools.
Output: reports/r5-unified-solutions.md
Status: complete
Completed: 2026-01-21

### Task R6: Security and Performance Over Tailscale
Priority: P1
Question: What are the specific security and performance considerations for running these solutions over Tailscale?
Scope: Research whether additional authentication is needed given Tailscale's encryption, performance implications of web UIs vs native protocols, and best practices for exposing services only on Tailscale network.
Output: reports/r6-tailscale-security-performance.md
Status: complete
Completed: 2026-01-21

### Task R7: Setup and Configuration Guides
Priority: P2
Question: What are the detailed setup steps for the top 3 recommended solutions?
Scope: Create practical installation guides for Ubuntu server setup, macOS client configuration, and Tailscale-specific networking considerations. Include troubleshooting common issues.
Output: reports/r7-setup-guides.md
Status: pending

### Task R8: Comparative Analysis and Recommendations
Priority: P2
Question: Based on all research, what is the optimal solution stack for this specific use case?
Scope: Synthesize findings into clear recommendations with pros/cons matrices, decision trees for different user priorities (simplicity vs features), and migration paths.
Output: reports/r8-final-recommendations.md
Status: complete
Completed: 2026-01-21

## Success Criteria

- All P0 questions answered with 2+ corroborating sources
- Practical recommendations with clear trade-offs
- Step-by-step setup guidance for top solutions
- Security considerations documented
- Resource usage benchmarks for server solutions

## Research Constraints

- Maximum 8 researcher subagents
- Maximum 3 parallel researchers
- Stop if 2 consecutive researchers provide <20% novel information
- Checkpoint review after P0 tasks complete
