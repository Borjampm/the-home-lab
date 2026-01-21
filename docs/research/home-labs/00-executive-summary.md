# Executive Summary: Home Lab Research

**Date:** 2026-01-20
**Author:** AI Research Assistant (Opus)
**Status:** Initial Research Complete

---

## Purpose

This document synthesizes findings from five research documents covering home lab ecosystems in 2025-2026. It provides curated insights, identifies patterns, highlights research gaps, and proposes actionable next steps for the-home-lab project.

---

## Key Findings

### 1. Technology Landscape Has Converged

The home lab community has largely converged on a standard technology stack:

| Layer | Dominant Choice | Runner-Up |
|-------|-----------------|-----------|
| Hypervisor | **Proxmox VE 9.1** | None (VMware lost free tier) |
| Container Orchestration | **K3s** (lightweight K8s) | Docker Compose |
| Container Management UI | **Komodo** (emerging) | Portainer (established) |
| Media Server | **Jellyfin** | Plex (declining) |
| VPN | **WireGuard** | OpenVPN (legacy) |
| Mesh VPN | **Tailscale** | Self-hosted WireGuard |
| Reverse Proxy | **Caddy** (advanced) | Nginx Proxy Manager (beginner) |
| IaC Provisioning | **Terraform** | Manual setup |
| Configuration Management | **Ansible** | Manual scripts |
| Local AI | **Ollama** | None dominant |

**Implication:** Technology selection is more straightforward than in previous years. The community has battle-tested these tools.

### 2. Hardware Sweet Spots Are Clear

| RAM | Use Case | Complexity Level |
|-----|----------|------------------|
| **16GB** | Basic self-hosting (3-5 services) | Beginner |
| **32GB** | Standard home lab (media, automation, dev) | Intermediate |
| **64GB+** | Full Kubernetes, multiple VMs, heavy workloads | Advanced |

**Hardware preference:** Mini PCs are favored for power efficiency, noise, and capability. Used enterprise gear remains viable for budget-conscious setups.

### 3. Network Segmentation Is Essential

VLANs are no longer optional for serious home labs:

| VLAN | Purpose | Why It Matters |
|------|---------|----------------|
| Management | Hypervisors, switches | Protect infrastructure |
| Trusted | Personal devices | Privacy from IoT/lab |
| Lab | Services, containers | Isolation for experiments |
| IoT | Smart devices | Contain insecure devices |
| Guest | Visitor WiFi | Zero trust for guests |
| DMZ | Public-facing services | Expose without risk |

**Critical insight:** Default-deny firewall rules between VLANs are mandatory. Without them, segmentation provides no real security.

### 4. Remote Access Has Three Tiers

| Solution | Privacy | Ease | Best For |
|----------|---------|------|----------|
| Self-hosted WireGuard | Excellent | Moderate | Privacy-focused users |
| Tailscale | Good | Very Easy | Most home labs |
| Cloudflare Tunnel | Fair | Easy | Public web services only |

**Recommendation:** Use Tailscale or WireGuard for private access; reserve Cloudflare Tunnel for intentionally public services. Many advanced users run both.

### 5. Use Cases Drive Architecture

The most common home lab purposes are:

1. **Media hosting** (Jellyfin, photos, music) - Storage-focused
2. **Learning platform** (K8s, IaC, networking) - Compute-focused
3. **Home automation** (Home Assistant) - Reliability-focused
4. **Self-hosted cloud** (Nextcloud, passwords) - Security-focused
5. **Development environment** (CI/CD, databases) - Flexibility-focused

**Key insight:** "A smaller stack you maintain well beats an impressive stack that falls apart." Focus on actual use cases, not technology collection.

---

## Common Patterns and Best Practices

### Architecture Patterns

**Pattern 1: Beginner-Friendly**
```
Used PC/Mini PC
    -> Proxmox VE
        -> Docker + Portainer
            -> Docker Compose stacks
```
- Minimal complexity
- Good for 3-10 services
- Easy to understand and maintain

**Pattern 2: Standard Home Lab**
```
Mini PC (32GB RAM)
    -> Proxmox VE
        -> Terraform (VM provisioning)
            -> K3s cluster
                -> Services via Helm/ArgoCD
```
- Production-like experience
- IaC skills development
- Scalable foundation

**Pattern 3: Multi-Node Cluster**
```
2-3 Mini PCs
    -> Proxmox Cluster
        -> K3s cluster (containers)
        -> Traditional VMs (non-containerized)
        -> Storage VMs (NAS, backup)
```
- High availability possible
- Distributed workloads
- More complex, more capable

### Operational Best Practices

1. **Documentation is non-negotiable**
   - Version control all configurations
   - Document VLAN schemes, IP allocations
   - Write runbooks for recovery

2. **Start small, scale gradually**
   - Begin with Docker Compose
   - Add Proxmox when needing VMs
   - Adopt Kubernetes when understanding containers
   - Add distributed storage when needing resilience

3. **Security layers**
   - Never expose services directly to internet
   - Use VPN or tunnel for all remote access
   - Segment networks with VLANs
   - Default-deny firewall rules
   - Key-based authentication over passwords

4. **Maintenance matters**
   - Regular updates prevent vulnerabilities
   - Automated backups are mandatory
   - Monitor system health proactively

---

## Technology Recommendations for This Project

Based on the research, here are specific recommendations for the-home-lab:

### Recommended Initial Stack

| Component | Recommendation | Rationale |
|-----------|----------------|-----------|
| Hypervisor | Proxmox VE 9.1 | Industry standard, free, excellent docs |
| IaC | Terraform + Ansible | Learn professional skills, reproducibility |
| Container Platform | K3s | Lightweight K8s, production-ready |
| Container UI | Start with Portainer, migrate to code | Learn progressively |
| Reverse Proxy | Caddy | Simple config, automatic HTTPS |
| Remote Access | Tailscale (initial) + WireGuard (later) | Easy start, then learn self-hosted |
| Media | Jellyfin | Open source, community winner |
| DNS/Ads | Pi-hole or AdGuard Home | Network-wide benefits |
| Monitoring | Prometheus + Grafana | Industry standard |
| Backup | Proxmox Backup Server | Integrated, reliable |

### Recommended VLAN Scheme

| VLAN ID | Name | Subnet | Purpose |
|---------|------|--------|---------|
| 10 | Management | 10.0.10.0/24 | Hypervisors, switches, IPMI |
| 20 | Trusted | 10.0.20.0/24 | Personal devices, workstations |
| 30 | Lab | 10.0.30.0/24 | Containers, services, experiments |
| 40 | IoT | 10.0.40.0/24 | Smart home devices (isolated) |
| 50 | Guest | 10.0.50.0/24 | Visitor WiFi (internet only) |

### Recommended Progression Path

```
Phase 1: Foundation (Month 1-2)
    - Single mini PC with Proxmox
    - Docker Compose for services
    - Basic VLAN setup (Management, Lab)
    - Tailscale for remote access
    - Pi-hole for DNS/ad blocking

Phase 2: Automation (Month 3-4)
    - Add Terraform for VM provisioning
    - Ansible for configuration
    - Expand VLANs (IoT, Trusted)
    - Basic monitoring (Uptime Kuma)

Phase 3: Orchestration (Month 5-6)
    - Deploy K3s cluster
    - Migrate services to Kubernetes
    - Add Prometheus/Grafana monitoring
    - Implement backup strategy

Phase 4: Advanced (Month 7+)
    - Self-hosted WireGuard
    - GitOps with ArgoCD
    - Distributed storage (Longhorn/Ceph)
    - High availability for critical services
```

---

## Critical Decisions Required

Before implementation, the following decisions must be made:

### 1. Hardware Selection
- **Question:** Mini PC vs used enterprise server vs custom build?
- **Trade-offs:** Power consumption vs raw performance vs cost
- **Recommendation:** Mini PC for most users (power efficient, quiet)
- **Action needed:** Research specific models, compare specs/prices

### 2. Primary Use Case Priority
- **Question:** What is the primary goal?
  - Learning (maximize technology exposure)
  - Practical self-hosting (minimize complexity)
  - Media hosting (maximize storage)
  - Development (maximize flexibility)
- **Action needed:** Define 1-2 primary objectives

### 3. Network Infrastructure
- **Question:** What router/firewall for VLAN support?
- **Options:** pfSense, OPNsense, OpenWrt, Unifi
- **Dependency:** Requires managed switch with 802.1Q support
- **Action needed:** Evaluate current network hardware capabilities

### 4. Storage Strategy
- **Question:** How to handle storage for media and backups?
- **Options:** Local disks, NAS, distributed storage (Ceph/Longhorn)
- **Trade-offs:** Cost vs redundancy vs complexity
- **Action needed:** Calculate storage requirements, evaluate options

### 5. Security Model
- **Question:** How paranoid should the security posture be?
- **Options:** Basic (VPN only) vs Strict (full segmentation + monitoring)
- **Trade-offs:** Complexity vs security assurance
- **Action needed:** Threat model assessment

---

## Identified Research Gaps

The following topics were insufficiently covered and require additional research:

### High Priority Gaps

| Gap | Why It Matters | Current Coverage |
|-----|----------------|------------------|
| **Hardware comparison** | Affects cost, power, noise, capability | Mentioned but not detailed |
| **Security hardening** | Prevents breaches, protects data | Scattered across documents |
| **Backup strategy** | Disaster recovery is critical | Brief mentions only |
| **Storage architecture** | ZFS, RAID, NAS options | Not covered |
| **Power management** | UPS sizing, energy costs | Not covered |

### Medium Priority Gaps

| Gap | Why It Matters | Current Coverage |
|-----|----------------|------------------|
| **Certificate management** | HTTPS for internal services | Brief mention |
| **Internal DNS** | Service discovery, clean URLs | Not covered |
| **Secrets management** | Secure credential handling | Not covered |
| **High availability** | Uptime for critical services | Minimal |
| **Cost analysis** | ROI of self-hosting | Anecdotal only |

### Lower Priority Gaps

| Gap | Why It Matters | Current Coverage |
|-----|----------------|------------------|
| **Physical setup** | Rack mounting, cooling, cables | Not covered |
| **Container registry** | Private image hosting | Brief mention |
| **Log management** | Centralized logging strategy | Tools mentioned, no strategy |
| **Update strategy** | How to keep systems current | Mentioned, not detailed |

---

## Proposed Further Research

### Immediate Research Needs (Pre-Implementation)

#### 1. Hardware Selection Guide
**Objective:** Compare specific hardware options for home lab use
**Scope:**
- Mini PC comparison (Intel NUC alternatives, ASUS, Beelink, etc.)
- Used enterprise server evaluation (Dell R720, HP DL380, etc.)
- Power consumption benchmarks
- Noise level considerations
- Cost-per-GB-RAM analysis
- Upgrade paths

**Deliverable:** Hardware decision matrix with recommendations

#### 2. Security Hardening Checklist
**Objective:** Comprehensive security baseline for home labs
**Scope:**
- OS hardening (SSH, firewalls, updates)
- Proxmox security configuration
- Container security best practices
- Network security (VLANs, firewall rules)
- Authentication standards
- Secrets management (Vault, SOPS)
- Intrusion detection options

**Deliverable:** Security checklist with implementation guides

#### 3. Backup and Disaster Recovery Strategy
**Objective:** Design reliable backup architecture
**Scope:**
- 3-2-1 backup rule implementation
- Proxmox Backup Server configuration
- Kubernetes backup with Velero
- Off-site backup options (cloud, remote location)
- Recovery testing procedures
- RTO/RPO targets for different service tiers

**Deliverable:** Backup architecture document with runbooks

### Secondary Research Needs (During Implementation)

#### 4. Storage Architecture Deep Dive
**Objective:** Evaluate storage solutions for home lab
**Scope:**
- ZFS fundamentals and configuration
- TrueNAS vs Proxmox storage
- RAID level comparison
- SSD vs HDD strategy
- Distributed storage (Ceph, Longhorn)
- Storage sizing methodology

**Deliverable:** Storage architecture recommendation

#### 5. Network Hardware Evaluation
**Objective:** Select router/firewall and switch hardware
**Scope:**
- pfSense vs OPNsense vs OpenWrt comparison
- Managed switch options (budget to prosumer)
- WiFi access point integration
- 2.5GbE vs 10GbE considerations
- Future expansion planning

**Deliverable:** Network hardware purchasing guide

#### 6. Certificate and DNS Management
**Objective:** Design internal DNS and HTTPS strategy
**Scope:**
- Internal DNS options (Pi-hole, AdGuard, CoreDNS)
- Let's Encrypt automation
- Wildcard certificates
- Split-horizon DNS
- Service discovery patterns

**Deliverable:** DNS/certificate configuration guide

### Future Research Needs (Optimization Phase)

#### 7. High Availability Patterns
**Objective:** HA design for critical home lab services
**Scope:**
- Proxmox clustering
- Kubernetes HA configurations
- Database HA (PostgreSQL, MariaDB)
- Load balancer options
- Failover testing

**Deliverable:** HA architecture patterns for home labs

#### 8. Cost-Benefit Analysis
**Objective:** Quantify value of self-hosting vs cloud
**Scope:**
- Hardware TCO calculation
- Power cost estimation
- Time investment valuation
- Equivalent cloud service costs
- Break-even analysis

**Deliverable:** Cost model spreadsheet with scenarios

#### 9. Monitoring and Alerting Strategy
**Objective:** Design comprehensive observability
**Scope:**
- Prometheus configuration patterns
- Grafana dashboard templates
- Alert routing (email, Slack, push)
- Log aggregation with Loki
- Uptime monitoring integration
- On-call considerations for home use

**Deliverable:** Monitoring stack configuration and dashboards

---

## Summary

This research provides a solid foundation for building a modern home lab. The technology landscape has matured significantly, with clear community consensus on the best tools for each layer of the stack.

**Key takeaways:**

1. **Proxmox + K3s + Terraform/Ansible** is the modern standard stack
2. **32GB RAM** is the sweet spot for a capable home lab
3. **VLANs are essential** for security and organization
4. **Tailscale simplifies remote access** (WireGuard for advanced users)
5. **Start simple, grow gradually** - Docker Compose before Kubernetes
6. **Documentation and backups** are non-negotiable

**Next steps for the-home-lab project:**

1. Complete hardware selection research (Gap #1)
2. Define primary use case objectives
3. Evaluate current network infrastructure
4. Create security hardening baseline (Gap #2)
5. Design backup strategy (Gap #3)
6. Begin Phase 1 implementation

---

## Document References

| Doc # | Title | Key Topic |
|-------|-------|-----------|
| 01 | Popular Projects 2025-2026 | Current technology choices |
| 02 | Technology Stacks | Architecture patterns and IaC |
| 03 | Networking and VLANs | Network segmentation |
| 04 | Remote Access Methods | VPN and tunnel solutions |
| 05 | Use Cases and Applications | What to run and why |

---

*This executive summary will be updated as additional research is completed and decisions are made.*
