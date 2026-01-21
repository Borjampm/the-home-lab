# Executive Summary: Remote Computing for Home Labs

**Document Type:** Research Synthesis
**Created:** 2026-01-20
**Status:** Final
**Sources:** Research documents 00-05 in this directory

---

## 1. Overview of Remote Computing Landscape

### The Core Challenge

The fundamental question for home lab remote access is: *How do you securely and efficiently access home computing resources from anywhere in the world?*

This research explored five interconnected domains that together form a complete remote computing strategy:

| Domain | Primary Concern | Key Trade-off |
|--------|-----------------|---------------|
| **File Synchronization** | Keeping files consistent across devices | Features vs. performance |
| **Remote Desktop** | Full graphical control of remote systems | Latency vs. security |
| **File Protocols** | Low-level file access methods | Speed vs. security |
| **NAS Remote Access** | Storage-centric remote access | Convenience vs. privacy |
| **Device Transfer** | Ad-hoc device-to-device sharing | Simplicity vs. integration |

### Paradigm Shift: VPN-First Architecture

A clear pattern emerged across all research: **the era of direct port forwarding is over**. Modern home labs should adopt a VPN-first or zero-trust architecture where:

1. No services are directly exposed to the internet
2. All remote access flows through encrypted tunnels (Tailscale, WireGuard)
3. Authentication happens at the network layer, not just the application layer
4. NAT traversal is handled automatically without router configuration

---

## 2. Key Findings Across Categories

### 2.1 File Synchronization

**Critical Insight:** No single solution handles all file access needs. The optimal approach combines multiple tools.

| Solution | Strength | Weakness | Best Use |
|----------|----------|----------|----------|
| **Nextcloud** | Full-featured cloud replacement | Resource-intensive, slower sync | Collaboration, sharing, web access |
| **Syncthing** | Fast P2P sync, privacy-first | No collaboration features | Personal device sync |
| **Seafile** | High performance, strong encryption | Less feature-rich | Performance-critical environments |

**Recommendation:** Deploy Nextcloud for external sharing and collaboration, Syncthing for fast inter-device sync.

### 2.2 Remote Desktop Solutions

**Critical Insight:** Use case determines tool selection. Gaming/creative work requires different solutions than general administration.

| Solution | Latency | Best For |
|----------|---------|----------|
| **Moonlight + Sunshine** | Ultra-low | Gaming, video editing, real-time interaction |
| **RustDesk** | Medium | General remote desktop, TeamViewer replacement |
| **Apache Guacamole** | Medium | Multi-server management, browser-based access |
| **RDP/VNC** | Low-High | Native Windows desktop, legacy systems |

**Recommendation:** Install Moonlight/Sunshine for any system requiring responsive interaction. Deploy Guacamole as a centralized access gateway for server management.

### 2.3 File Server Protocols

**Critical Insight:** Protocol selection depends entirely on network location. LAN and WAN require fundamentally different approaches.

| Protocol | LAN Performance | WAN Performance | Security |
|----------|-----------------|-----------------|----------|
| **NFS** | Excellent | Poor (latency-sensitive) | Low (no auth by default) |
| **SMB** | Excellent | Poor (latency-sensitive) | Moderate |
| **SFTP** | Good | Good | Excellent |
| **WebDAV** | Moderate | Moderate | Good (over HTTPS) |

**Critical Rule:** Never use SMB or NFS over WAN/VPN connections. These protocols are highly latency-sensitive and will provide poor user experience regardless of bandwidth.

**Recommendation:** Use SMB/NFS for local LAN access, switch to SFTP/WebDAV for remote access even through VPN.

### 2.4 NAS Remote Access

**Critical Insight:** Proprietary solutions (QuickConnect) trade performance for convenience. Tailscale offers the best balance.

| Method | Speed | Setup | Privacy | CGNAT Compatible |
|--------|-------|-------|---------|------------------|
| **Tailscale** | Fast | Easy | Good | Yes |
| **WireGuard (self-hosted)** | Fast | Moderate | Excellent | No |
| **QuickConnect** | Slow (~1 MB/s) | Trivial | Moderate | Yes |
| **Port Forwarding** | Fast | Easy | Poor | No |

**Recommendation:** Tailscale as primary access method, with QuickConnect as emergency fallback for Synology users.

### 2.5 Device-to-Device Transfer

**Critical Insight:** Three complementary tools cover all local transfer needs.

| Tool | Purpose | Key Capability |
|------|---------|----------------|
| **LocalSend** | Instant transfers | Directory support, 20-80 MB/s |
| **KDE Connect** | Device integration | Notifications, clipboard, remote control |
| **Syncthing** | Continuous sync | Background automation |

**Recommendation:** Install LocalSend on all devices for ad-hoc transfers. Add KDE Connect for Linux desktop integration. Use Syncthing for automated folder synchronization.

---

## 3. Decision Frameworks

### 3.1 Remote Access Method Selection

```
START: What do you need to do remotely?
  │
  ├── Browse/transfer files only?
  │     ├── Large files or streaming? → Tailscale + SMB/NFS
  │     ├── Occasional small files? → QuickConnect / Web interface
  │     └── Automated backups? → SFTP
  │
  ├── Full desktop control?
  │     ├── Gaming or video editing? → Moonlight + Sunshine
  │     ├── General use? → RustDesk or RDP
  │     └── Managing many servers? → Apache Guacamole
  │
  └── Access home network services?
        └── Always use: Tailscale or WireGuard VPN
```

### 3.2 File Access Protocol Selection

```
Where is the client connecting FROM?
  │
  ├── Same LAN (trusted network)?
  │     ├── Windows environment → SMB/CIFS
  │     ├── Linux environment → NFS
  │     └── Mixed environment → SMB/CIFS
  │
  ├── Remote via VPN?
  │     ├── Need native file manager? → WebDAV
  │     ├── Scripted/automated? → SFTP
  │     └── Acceptable latency on browsing → WebDAV + SMB/NFS backup
  │
  └── Direct internet (not recommended)?
        └── SFTP or WebDAV over HTTPS only
```

### 3.3 File Sync Solution Selection

```
What type of file access do you need?
  │
  ├── Collaboration with others?
  │     └── Nextcloud (or Seafile for speed)
  │
  ├── Personal device sync only?
  │     └── Syncthing (fastest, most private)
  │
  ├── Web-based file browsing?
  │     └── Nextcloud or FileRun
  │
  └── All of the above?
        └── Nextcloud + Syncthing combined
```

---

## 4. Recommended Technology Stacks

### 4.1 Home Lab Starter (Single User, Simple Needs)

**Use Case:** Individual accessing files and occasionally controlling home PC

| Layer | Solution | Purpose |
|-------|----------|---------|
| **Network** | Tailscale | Secure remote access |
| **File Sync** | Syncthing | Cross-device sync |
| **Remote Desktop** | RDP (Windows) or RustDesk | Occasional desktop access |
| **Local Transfer** | LocalSend | Device-to-device |

**Advantages:**
- Minimal complexity
- All free/open-source options
- Quick to set up (under 2 hours)
- Works behind CGNAT

### 4.2 Home Lab Standard (Power User, Multiple Devices)

**Use Case:** Tech enthusiast with NAS, multiple computers, media streaming needs

| Layer | Solution | Purpose |
|-------|----------|---------|
| **Network** | Tailscale + optional WireGuard | Primary + backup access |
| **File Cloud** | Nextcloud | Web access, sharing, collaboration |
| **File Sync** | Syncthing | Fast inter-device sync |
| **Remote Desktop** | Moonlight/Sunshine + Guacamole | Gaming PC + server management |
| **NAS** | Synology/TrueNAS with SMB/NFS | Central storage |
| **Local Transfer** | LocalSend + KDE Connect | Quick transfers + device integration |

**Advantages:**
- Covers all common use cases
- Performance-optimized choices
- Maintains privacy (self-hosted)
- Scalable architecture

### 4.3 Home Lab Advanced (Family/Multi-User, High Security)

**Use Case:** Family access to shared resources, multiple security levels, media streaming

| Layer | Solution | Purpose |
|-------|----------|---------|
| **Network** | WireGuard (self-hosted) + Tailscale | Maximum control + convenience |
| **Access Gateway** | Apache Guacamole | Centralized, audited access |
| **File Cloud** | Nextcloud with LDAP | Multi-user with permissions |
| **File Sync** | Syncthing | Personal device sync |
| **Remote Desktop** | Moonlight/Sunshine + RDP | Performance + compatibility |
| **NAS** | TrueNAS with VLAN isolation | Segmented storage |
| **Media** | Jellyfin/Plex | Family media streaming |
| **Local Transfer** | LocalSend | Cross-platform sharing |

**Advantages:**
- Enterprise-grade security posture
- Multi-user access control
- Audit capabilities
- Complete self-hosting option

---

## 5. Security and Performance Considerations

### 5.1 Security Hierarchy (Most to Least Secure)

**Remote Access Methods:**
1. Self-hosted WireGuard VPN (full control, no third parties)
2. Tailscale (zero-trust, E2E encrypted, third-party coordination)
3. Cloudflare Tunnel (encrypted, but traffic through Cloudflare)
4. QuickConnect-style relays (encrypted, vendor-dependent)
5. Direct port forwarding (exposed, not recommended)

**File Transfer Protocols:**
1. SFTP with key authentication
2. WebDAV over HTTPS with strong auth
3. SMB 3.0+ with encryption
4. NFS with Kerberos
5. SMB (older versions) or NFS (default) - avoid

### 5.2 Security Best Practices

**Never Do:**
- Expose RDP, VNC, or SSH directly to internet without VPN
- Use default ports without additional security
- Rely solely on password authentication
- Use SMB/NFS over untrusted networks

**Always Do:**
- Access through VPN (Tailscale/WireGuard) first
- Use key-based authentication where possible
- Enable multi-factor authentication
- Monitor access logs
- Keep software updated
- Segment networks (VLANs) for sensitive resources

### 5.3 Performance Optimization

**For Maximum Speed:**
- Use wired connections on critical devices
- Deploy Syncthing for inter-device sync (faster than Nextcloud)
- Use Moonlight/Sunshine for desktop streaming
- Access via direct Tailscale connections (not relayed)
- Use NFS/SMB only on LAN, never over WAN

**For Minimum Latency:**
- Moonlight/Sunshine for real-time interaction
- WireGuard over OpenVPN (faster handshakes)
- Tailscale with direct connections
- Avoid relay services for interactive work

**Bandwidth Considerations:**
- Remote desktop: 5-20 Mbps for good experience
- HD video streaming: 10-25 Mbps
- File sync: Depends on sync frequency and file sizes
- Guacamole: More efficient than native VNC

---

## 6. Implementation Roadmap

### Phase 1: Foundation (Week 1)

**Objective:** Establish secure remote access

| Task | Tool | Time | Priority |
|------|------|------|----------|
| Install Tailscale on primary PC | Tailscale | 15 min | Critical |
| Install Tailscale on phone/laptop | Tailscale | 15 min | Critical |
| Enable RDP on Windows machines | Windows Settings | 10 min | High |
| Install SSH on Linux machines | OpenSSH | 10 min | High |
| Test remote connectivity | - | 30 min | Critical |

**Outcome:** Secure access to all home devices from anywhere

### Phase 2: File Access (Week 2)

**Objective:** Set up file synchronization and access

| Task | Tool | Time | Priority |
|------|------|------|----------|
| Install Syncthing on all personal devices | Syncthing | 1 hr | High |
| Configure sync folders | Syncthing | 30 min | High |
| Deploy Nextcloud (Docker) | Nextcloud | 2 hr | Medium |
| Configure Nextcloud users and shares | Nextcloud | 1 hr | Medium |
| Install LocalSend on all devices | LocalSend | 30 min | Medium |

**Outcome:** Files accessible and synced across all devices

### Phase 3: Enhanced Desktop Access (Week 3)

**Objective:** Optimize remote desktop experience

| Task | Tool | Time | Priority |
|------|------|------|----------|
| Install Sunshine on gaming/primary PC | Sunshine | 30 min | High |
| Install Moonlight on client devices | Moonlight | 30 min | High |
| Deploy Guacamole (Docker) | Guacamole | 2 hr | Medium |
| Configure Guacamole connections | Guacamole | 1 hr | Medium |
| Test all remote desktop scenarios | - | 1 hr | High |

**Outcome:** Optimized remote desktop for all use cases

### Phase 4: NAS Integration (Week 4)

**Objective:** Integrate NAS with remote access infrastructure

| Task | Tool | Time | Priority |
|------|------|------|----------|
| Install Tailscale on NAS | Tailscale | 30 min | Critical |
| Configure SMB/NFS shares for LAN | NAS OS | 1 hr | High |
| Set up WebDAV for remote access | Nextcloud/NAS | 1 hr | Medium |
| Configure Syncthing NAS integration | Syncthing | 30 min | Medium |
| Test mobile access | DS File/FE Explorer | 30 min | High |

**Outcome:** NAS fully accessible locally and remotely

### Phase 5: Optimization (Week 5+)

**Objective:** Fine-tune and document

| Task | Tool | Time | Priority |
|------|------|------|----------|
| Configure VLAN segmentation | Router | 2 hr | Medium |
| Set up monitoring/logging | Varies | 2 hr | Low |
| Document all configurations | - | 2 hr | Medium |
| Create family user guides | - | 1 hr | Low |
| Test failure scenarios | - | 1 hr | Medium |

**Outcome:** Production-ready, documented home lab

---

## 7. Proposed Further Research Topics

### 7.1 High Priority

| Topic | Rationale |
|-------|-----------|
| **VPN Performance Comparison** | Quantitative comparison of Tailscale, WireGuard, OpenVPN speeds and latency in home lab scenarios |
| **Docker Deployment Patterns** | Best practices for containerizing Nextcloud, Guacamole, and other services |
| **Reverse Proxy Architecture** | Traefik/Caddy/Nginx Proxy Manager comparison for service exposure |
| **Backup Strategies** | Integration of remote access with backup workflows (3-2-1 rule implementation) |
| **Mobile Optimization** | Deep dive into best mobile clients and workflows for iOS and Android |

### 7.2 Medium Priority

| Topic | Rationale |
|-------|-----------|
| **Media Server Integration** | Plex vs. Jellyfin remote streaming performance and features |
| **Multi-User Access Control** | LDAP/SSO integration for family access management |
| **Monitoring and Alerting** | Prometheus/Grafana setup for home lab health monitoring |
| **Power Management** | Wake-on-LAN and remote power management strategies |
| **IPv6 Transition** | Impact of IPv6 adoption on home lab remote access |

### 7.3 Specialized Topics

| Topic | Rationale |
|-------|-----------|
| **Gaming-Specific Optimization** | Moonlight/Parsec tuning for cloud gaming setups |
| **macOS Integration** | Apple ecosystem challenges and solutions |
| **Hybrid Cloud Architecture** | Using cloud VPS as jump host or relay server |
| **Disaster Recovery** | What happens when home internet fails? Fallback strategies |
| **Security Hardening** | Intrusion detection, fail2ban, certificate management |

### 7.4 Implementation Guides Needed

| Guide | Purpose |
|-------|---------|
| **Tailscale Complete Setup** | Step-by-step from install to subnet routing |
| **Nextcloud Docker Deployment** | Production-ready Nextcloud with best practices |
| **Guacamole Gateway Setup** | Multi-target Guacamole with 2FA |
| **Syncthing Network Architecture** | Optimal node topology for multi-device sync |
| **NAS Security Hardening** | Synology/TrueNAS security configuration guide |

---

## 8. Conclusions

### Key Takeaways

1. **Tailscale is the cornerstone** - For most home labs, Tailscale provides the optimal balance of security, ease of use, and performance. Start here.

2. **Layer your solutions** - No single tool does everything well. Combine Tailscale (network), Syncthing (sync), Nextcloud (cloud), and purpose-specific remote desktop tools.

3. **Respect protocol limitations** - SMB and NFS are excellent on LAN but terrible over WAN. Don't fight physics; use the right protocol for the right network.

4. **Security is non-negotiable** - Never expose services directly to the internet. The convenience gain is not worth the security risk.

5. **Start simple, add complexity** - Begin with Tailscale + RDP/SSH. Add Nextcloud when you need sharing. Add Guacamole when managing multiple servers. Evolve as needs grow.

### The Recommended Stack

For a typical home lab user, the following stack provides comprehensive coverage:

```
┌─────────────────────────────────────────────────────────┐
│                    NETWORK LAYER                         │
│  Tailscale (primary) + WireGuard (advanced/backup)       │
└─────────────────────────────────────────────────────────┘
                           │
          ┌───────────────┼───────────────┐
          │               │               │
          ▼               ▼               ▼
    ┌───────────┐   ┌───────────┐   ┌───────────┐
    │FILE ACCESS│   │ DESKTOP   │   │ DEVICE    │
    │           │   │ ACCESS    │   │ TRANSFER  │
    │Syncthing  │   │Moonlight/ │   │LocalSend  │
    │Nextcloud  │   │Sunshine   │   │KDE Connect│
    │SFTP/WebDAV│   │Guacamole  │   │           │
    └───────────┘   └───────────┘   └───────────┘
          │               │               │
          └───────────────┼───────────────┘
                          │
                          ▼
              ┌───────────────────────┐
              │   NAS / FILE SERVER   │
              │  Synology / TrueNAS   │
              │  SMB (LAN) + WebDAV   │
              └───────────────────────┘
```

This architecture provides:
- Secure access from anywhere (Tailscale)
- Fast file sync (Syncthing)
- Collaboration and sharing (Nextcloud)
- High-performance remote desktop (Moonlight/Sunshine)
- Centralized server management (Guacamole)
- Quick device-to-device transfers (LocalSend)
- Comprehensive NAS integration

### Next Steps for This Home Lab

1. **Immediate:** Install Tailscale on all devices and test connectivity
2. **Short-term:** Deploy Syncthing for file synchronization
3. **Medium-term:** Set up Nextcloud for cloud features
4. **Ongoing:** Document configurations and iterate based on actual usage

---

## Document References

| Document | Content |
|----------|---------|
| `00-research-scope.md` | Research framework and evaluation criteria |
| `01-file-sync-cloud-solutions.md` | Nextcloud, Syncthing, Seafile analysis |
| `02-remote-desktop-solutions.md` | RustDesk, Moonlight/Sunshine, Guacamole |
| `03-file-server-protocols.md` | SMB, NFS, SFTP, WebDAV comparison |
| `04-nas-remote-access.md` | Synology, TrueNAS remote access methods |
| `05-device-file-transfer.md` | LocalSend, KDE Connect, Syncthing |

---

**Document Status:** Complete
**Next Review:** After initial implementation phase
**Author:** AI Research Assistant
