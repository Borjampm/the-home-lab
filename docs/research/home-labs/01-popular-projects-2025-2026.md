# Popular Home Lab Projects & Setups (2025-2026)

**Research Date:** 2026-01-20
**Sources:** Web search across homelab communities, blogs, and forums

---

## Overview

This document captures what the homelab community is currently running in 2025-2026, based on recent articles, Reddit discussions (r/selfhosted, r/homelab), and technology blogs.

---

## Popular Software & Tools

### Hypervisors/Platforms

**Proxmox VE 9.1** - Top choice for fresh home lab builds in 2026
- Free and open-source
- Software-defined storage with Ceph integration
- Proxmox Backup Server (PBS) available
- VMware no longer offers free version, pushing users to Proxmox
- Tooling like Terraform and Ansible for automation

### Container Management

**Komodo** - Newest option gaining traction
- Free and fast
- Modern interface with great management features
- Community leaning toward this in late 2025/early 2026

**Portainer** - Still popular for beginners
- User-friendly UI
- Good for getting started
- Experienced homelabbers increasingly prefer managing through code

### Media Servers

**Jellyfin** - Winner of the "media server wars"
- Fully open-source
- Plex's aggressive monetization pushed users toward Jellyfin
- Growing ecosystem and community support

### Reverse Proxies

**Nginx Proxy Manager** - Beginner-friendly
- Friendly web UI
- Easy SSL certificate management
- Good starting point for new homelabbers

**Caddy** - Preferred by experienced users
- Automatic HTTPS
- Simple configuration
- Modern and elegant

### AI/LLM Integration

**Ollama** - Making local LLMs accessible
- Trivially easy to run local language models
- Used office PC with 32GB RAM can run 7B-13B parameter models
- Homelabbers integrating AI into everything

### VPN/Remote Access

**WireGuard** - Replaced OpenVPN as standard
- Noticeable speed improvements
- Dramatically simpler configuration
- Modern cryptography

---

## Hardware Recommendations

### Mini PCs
- Still popular choice even with high RAM prices
- Incredibly powerful for their size
- Can run enterprise-grade applications
- **32GB RAM** - Sweet spot for starter systems
- **64GB RAM** - Overhead for more dense stacks (may be expensive)

### Server Options
- Tower vs Rack vs Renewed equipment
- Many starting with renewed enterprise gear (Dell, HP)
- Raspberry Pi still popular for specific use cases

---

## Common Lessons for 2026

### Documentation is Critical
- "Documentation fixes all issues with forgetting where you left off"
- Being able to reference configuration files is a "godsend"
- Version control for configurations

### Quality Over Quantity
- "A smaller stack you maintain well beats an impressive stack that falls apart"
- Forgetting to update components causes issues
- Focus on what you'll actually use and maintain

### Start Small, Scale Gradually
- Don't over-invest initially
- Learn the basics before expanding
- Add complexity as needs grow

---

## Sources

- [Home Lab Projects 2025 | The Virtual Horizon](https://thevirtualhorizon.com/2025/01/03/home-lab-projects-2025/)
- [My Favorite Home Lab Tools in 2025 (So Far) - Virtualization Howto](https://www.virtualizationhowto.com/2025/06/my-favorite-home-lab-tools-in-2025-so-far-projects-i-keep-coming-back-to/)
- [The 2026 Homelab Stack: What Self-Hosters Are Actually Running This Year](https://blog.elest.io/the-2026-homelab-stack-what-self-hosters-are-actually-running-this-year/)
- [4 homelab mistakes I'll never make again in 2026](https://www.xda-developers.com/4-homelab-mistakes-ill-never-make-again-in-2026/)
- [Ultimate Home Lab Starter Stack for 2026 - Virtualization Howto](https://www.virtualizationhowto.com/2025/12/ultimate-home-lab-starter-stack-for-2026-key-recommendations/)
- [7 Raspberry Pi HomeLab Projects You Will Actually Use in 2025](https://berkem.xyz/blog/raspberry-pi-homelab-projects/)
- [Best Server for Home Lab 2026](https://edywerder.ch/best-server-for-home-lab/)
- [7 Homelab Ideas | Why You Should Have A Homelab](https://b3n.org/homelab-ideas/)
