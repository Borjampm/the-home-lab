# Tailscale Deep Dive Research Scope

**Document Type:** Research Planning
**Created:** 2026-01-20
**Status:** Draft
**Context:** Follow-up from remote computing research where Tailscale emerged as a key solution

---

## 1. Executive Summary

Tailscale has emerged as the leading solution for home lab remote access due to its simplicity, WireGuard-based performance, and zero-configuration NAT traversal. This document outlines the comprehensive research needed to fully understand Tailscale for home lab implementation, covering architecture, features, comparisons, security implications, and practical deployment patterns.

### Why a Deep Dive on Tailscale?

From our remote computing research, Tailscale stood out for:
- "Dead simple" setup and management
- WireGuard-based excellent performance
- No port forwarding required
- Free tier sufficient for most home labs
- Cross-platform support

However, several questions remain unanswered:
- What exactly happens with the coordination server?
- How does it compare technically to raw WireGuard?
- What are the actual privacy implications?
- What advanced features exist beyond basic connectivity?
- How does it integrate with existing infrastructure?

---

## 2. Foundational Knowledge Areas

### 2.1 How Tailscale Actually Works

| Topic | Research Questions |
|-------|-------------------|
| **Architecture Overview** | What are the core components? How do they interact? |
| **Coordination Server** | What does it do? What data does it see? Can it be self-hosted? |
| **DERP Relays** | What are they? When are they used? Can they be self-hosted? |
| **Control Plane vs Data Plane** | How is control separated from actual traffic? |
| **Key Exchange** | How are WireGuard keys distributed and managed? |
| **Node Discovery** | How do devices find each other? |
| **Connection Establishment** | What's the handshake process? |

### 2.2 WireGuard Foundation

| Topic | Research Questions |
|-------|-------------------|
| **WireGuard Protocol Basics** | How does WireGuard work at a fundamental level? |
| **Cryptography Used** | What encryption, key exchange, and hashing algorithms? |
| **Tailscale's WireGuard Modifications** | What did Tailscale change or extend? |
| **Performance Characteristics** | Throughput, latency, CPU usage, battery impact? |
| **Protocol Limitations** | What can't WireGuard do that Tailscale addresses? |

### 2.3 NAT Traversal Deep Dive

| Topic | Research Questions |
|-------|-------------------|
| **NAT Types** | How does Tailscale handle different NAT types? |
| **STUN/TURN Equivalent** | What mechanisms are used for NAT traversal? |
| **Hole Punching** | How does UDP hole punching work in Tailscale? |
| **CGNAT Handling** | How does Tailscale work with carrier-grade NAT? |
| **Fallback Mechanisms** | What happens when direct connection fails? |
| **Connection Quality** | How to verify direct vs relayed connections? |

---

## 3. Feature Research Areas

### 3.1 Core Networking Features

| Feature | Research Questions |
|---------|-------------------|
| **MagicDNS** | How does internal DNS work? Custom domains? Integration with existing DNS? |
| **Subnet Routing** | How to expose entire subnets? Use cases? Performance implications? |
| **Exit Nodes** | How to route all traffic through a node? Use cases? Security implications? |
| **Split DNS** | How to use Tailscale DNS for some domains, regular DNS for others? |
| **IPv4/IPv6** | How does Tailscale handle both? Dual-stack support? |
| **4via6 Subnet Routing** | What is this feature? When is it useful? |

### 3.2 Access Control & Security

| Feature | Research Questions |
|---------|-------------------|
| **ACLs (Access Control Lists)** | Syntax and capabilities? How granular can controls be? |
| **Tags** | How do device tags work? Use cases for organization? |
| **Groups** | How to organize users and devices? |
| **SSH Integration** | How does Tailscale SSH work? Benefits over regular SSH? |
| **HTTPS Certificates** | How does automatic certificate provisioning work? |
| **Node Authorization** | Manual vs automatic approval? Expiration policies? |
| **Key Expiry** | How does key rotation work? Forced re-authentication? |

### 3.3 Identity & Authentication

| Feature | Research Questions |
|---------|-------------------|
| **Identity Providers** | Which IdPs are supported? OIDC? SAML? |
| **SSO Integration** | How does single sign-on work? |
| **Multi-User Tailnets** | How to share a tailnet with family/team? |
| **Device Sharing** | How to share specific devices without full network access? |
| **Guest Access** | How to provide temporary access? |
| **2FA/MFA** | Is additional MFA supported? How? |

### 3.4 Advanced Features

| Feature | Research Questions |
|---------|-------------------|
| **Tailscale Funnel** | What is it? How to expose services to internet? Security implications? |
| **Tailscale Serve** | What services can it proxy? HTTPS termination? |
| **Mullvad Integration** | How does the Mullvad exit node partnership work? |
| **App Connectors** | What are they? Use cases? |
| **Tailscale SSH** | How is it different from regular SSH over Tailscale? |
| **Taildrop** | File sharing feature - how does it work? Limitations? |

### 3.5 Monitoring & Observability

| Feature | Research Questions |
|---------|-------------------|
| **Admin Console** | What visibility does the web UI provide? |
| **Connection Status** | How to see if connections are direct or relayed? |
| **Logging** | What logs are available? Where are they stored? |
| **Metrics** | Does Tailscale expose Prometheus metrics? |
| **Network Flow Logs** | Can you see what traffic is flowing? |
| **Audit Logs** | What actions are logged? How long retained? |

---

## 4. Technical Comparisons

### 4.1 Tailscale vs Raw WireGuard

| Aspect | Research Questions |
|--------|-------------------|
| **Setup Complexity** | Exactly how much simpler is Tailscale? |
| **Performance Overhead** | Is there measurable overhead vs raw WireGuard? |
| **Feature Parity** | What can raw WireGuard do that Tailscale can't? |
| **Control & Flexibility** | What control do you lose with Tailscale? |
| **Key Management** | How does Tailscale's automatic key management compare? |
| **NAT Traversal** | How do manual WireGuard NAT solutions compare? |
| **Multi-Site** | How do hub-and-spoke WireGuard setups compare to mesh? |

### 4.2 Tailscale vs ZeroTier

| Aspect | Research Questions |
|--------|-------------------|
| **Protocol Differences** | How do the underlying protocols differ? |
| **Performance Comparison** | Speed, latency, resource usage differences? |
| **Feature Comparison** | What features does each have that the other lacks? |
| **Pricing Comparison** | How do free and paid tiers compare? |
| **Self-Hosting** | How do Headscale vs ZeroTier Moon compare? |
| **Community & Support** | Size and activity of communities? |
| **Platform Support** | Which platforms does each support? |

### 4.3 Tailscale vs Netbird

| Aspect | Research Questions |
|--------|-------------------|
| **Open Source Status** | How do open-source components compare? |
| **Self-Hosting Story** | How does Netbird's fully open-source approach compare? |
| **Feature Parity** | What features does Netbird offer vs Tailscale? |
| **Maturity & Stability** | How do the projects compare in maturity? |
| **Performance** | Any performance differences? |
| **Documentation Quality** | How does documentation compare? |

### 4.4 Tailscale vs Traditional VPN (OpenVPN, IPsec)

| Aspect | Research Questions |
|--------|-------------------|
| **Architecture Differences** | Hub-and-spoke vs mesh - practical implications? |
| **Performance Gap** | How much faster is Tailscale really? Benchmarks? |
| **Configuration Complexity** | Compare setup time and ongoing maintenance? |
| **Enterprise Features** | What enterprise features do traditional VPNs have? |
| **Audit & Compliance** | How do compliance capabilities compare? |

### 4.5 Tailscale vs Cloudflare Tunnel

| Aspect | Research Questions |
|--------|-------------------|
| **Use Case Differences** | When to use each? Can they complement each other? |
| **Traffic Visibility** | Privacy implications of each approach? |
| **Protocol Support** | What protocols does each support? |
| **Authentication Models** | How do auth approaches differ? |
| **Integration Patterns** | How to use both together effectively? |

---

## 5. Self-Hosting Research: Headscale

### 5.1 Headscale Fundamentals

| Topic | Research Questions |
|-------|-------------------|
| **What is Headscale?** | Open-source Tailscale control server - how complete is it? |
| **Compatibility** | Does it work with official Tailscale clients? |
| **Feature Support** | Which Tailscale features does Headscale support? |
| **Missing Features** | What's not available in Headscale? |
| **Stability & Maturity** | Is it production-ready? |
| **Community & Development** | How active is development? |

### 5.2 Headscale Deployment

| Topic | Research Questions |
|-------|-------------------|
| **Installation Options** | Docker, bare metal, Kubernetes? |
| **Database Requirements** | What databases are supported? |
| **DERP Configuration** | How to set up self-hosted DERP relays? |
| **High Availability** | Can Headscale be clustered? |
| **Integration with IdPs** | How does authentication work? |
| **Migration Path** | Can you migrate from Tailscale to Headscale? |

### 5.3 Trade-offs Analysis

| Topic | Research Questions |
|-------|-------------------|
| **When to Self-Host?** | What scenarios justify running Headscale? |
| **Operational Burden** | What's the maintenance overhead? |
| **Security Responsibility** | What security burden do you take on? |
| **Feature Lag** | How far behind official Tailscale is Headscale? |
| **Support Implications** | What support options exist for Headscale? |

---

## 6. Privacy & Security Analysis

### 6.1 Data Flow Analysis

| Topic | Research Questions |
|-------|-------------------|
| **What Data Reaches Tailscale?** | Exactly what metadata does Tailscale see? |
| **Encryption Guarantees** | What is end-to-end encrypted? What isn't? |
| **Coordination Server Data** | What does the coordination server store? |
| **DERP Relay Traffic** | When traffic is relayed, can Tailscale see content? |
| **Logging & Retention** | What logs does Tailscale keep? For how long? |
| **Legal Jurisdiction** | Where is data stored? What laws apply? |

### 6.2 Security Model

| Topic | Research Questions |
|-------|-------------------|
| **Trust Model** | What do you have to trust Tailscale for? |
| **Attack Surface** | What are potential attack vectors? |
| **Key Security** | How are private keys protected? |
| **Compromise Scenarios** | What if Tailscale is breached? What if your IdP is? |
| **Security Audits** | Has Tailscale been independently audited? |
| **Vulnerability History** | What security issues have been found and fixed? |

### 6.3 Privacy Considerations

| Topic | Research Questions |
|-------|-------------------|
| **Metadata Exposure** | What can Tailscale learn about your activity? |
| **Network Topology** | Does Tailscale know your internal network structure? |
| **Traffic Analysis** | Could Tailscale perform traffic analysis? |
| **Third-Party Sharing** | What's Tailscale's data sharing policy? |
| **Privacy-Focused Alternatives** | How to maximize privacy while using Tailscale? |

---

## 7. Practical Considerations

### 7.1 Pricing & Plans

| Topic | Research Questions |
|-------|-------------------|
| **Free Tier Details** | Exact limits? Number of devices, users, features? |
| **Personal vs Business** | Feature differences between tiers? |
| **Pricing Structure** | Per-user? Per-device? What counts as what? |
| **Free Tier Sustainability** | Is the free tier generous enough long-term? |
| **Upgrade Triggers** | What would force an upgrade to paid? |
| **Enterprise Features** | What do you get at higher tiers? |

### 7.2 Platform Support

| Topic | Research Questions |
|-------|-------------------|
| **Desktop Clients** | Windows, macOS, Linux - feature parity? |
| **Mobile Clients** | iOS, Android - limitations? Always-on VPN? |
| **Linux Variations** | Which distros? ARM support? Headless operation? |
| **Container Support** | Docker, Kubernetes integration? Sidecar patterns? |
| **NAS Integration** | Synology, QNAP, TrueNAS support? |
| **Router Support** | OpenWrt, pfSense, OPNsense, Ubiquiti? |
| **Embedded/IoT** | Raspberry Pi, embedded Linux support? |

### 7.3 Limitations & Constraints

| Topic | Research Questions |
|-------|-------------------|
| **Device Limits** | What are actual device limits per plan? |
| **Bandwidth Limits** | Are there bandwidth caps? Fair use policies? |
| **Connection Limits** | Concurrent connection limits? |
| **Feature Restrictions** | What features are paid-only? |
| **Geographic Restrictions** | Any regional limitations? |
| **ToS Restrictions** | What uses are prohibited? |

---

## 8. Implementation Patterns

### 8.1 Home Lab Topologies

| Pattern | Research Questions |
|---------|-------------------|
| **Single Node Access** | Simplest setup - one device on Tailscale, access from anywhere |
| **Full Network Access** | Subnet router exposing entire home network |
| **Segmented Access** | Different access levels for different services |
| **Multi-Site** | Connecting home lab with cloud resources |
| **Exit Node Pattern** | Using home as exit node for privacy |
| **Hybrid Pattern** | Combining Tailscale with other solutions |

### 8.2 Service Integration Patterns

| Pattern | Research Questions |
|---------|-------------------|
| **Direct Access** | Services directly on Tailscale network |
| **Reverse Proxy Integration** | Tailscale + Traefik/Nginx/Caddy |
| **Container Networking** | Docker/Kubernetes with Tailscale |
| **Database Access** | Secure database access patterns |
| **Development Workflows** | Remote development with Tailscale |
| **CI/CD Integration** | Using Tailscale in automation pipelines |

### 8.3 Specific Use Cases

| Use Case | Research Questions |
|----------|-------------------|
| **Remote SSH Access** | Best practices for SSH over Tailscale |
| **File Server Access** | SMB/NFS over Tailscale - performance? |
| **Media Server Access** | Plex/Jellyfin over Tailscale |
| **Home Assistant Access** | Smart home remote access |
| **Proxmox/VMware Access** | Hypervisor management access |
| **Pi-hole/DNS Access** | DNS services over Tailscale |
| **Backup Systems** | Remote backup over Tailscale |
| **Surveillance Systems** | Camera/NVR access |

---

## 9. Operational Considerations

### 9.1 Day-to-Day Operations

| Topic | Research Questions |
|-------|-------------------|
| **Device Onboarding** | How to add new devices efficiently? |
| **User Management** | Adding/removing users and devices |
| **Troubleshooting** | Common issues and diagnostics |
| **Performance Monitoring** | How to monitor Tailscale health? |
| **Updates & Maintenance** | Client update strategies |
| **Documentation** | What to document for your setup? |

### 9.2 Reliability & Availability

| Topic | Research Questions |
|-------|-------------------|
| **Tailscale Uptime** | What's Tailscale's SLA? Historical uptime? |
| **Outage Scenarios** | What if Tailscale coordination server is down? |
| **Offline Operation** | Do existing connections survive outages? |
| **Redundancy Options** | Can you have redundant paths? |
| **Failover Strategies** | Backup access if Tailscale fails? |

### 9.3 Migration & Evolution

| Topic | Research Questions |
|-------|-------------------|
| **Migration from Other VPNs** | How to migrate from OpenVPN/WireGuard? |
| **Gradual Adoption** | Can you run Tailscale alongside existing setup? |
| **Rollback Plan** | How to revert if Tailscale doesn't work out? |
| **Scale Evolution** | How does management change as network grows? |
| **Exit Strategy** | How to move away from Tailscale if needed? |

---

## 10. Research Questions Summary

### 10.1 Architecture & Technology Questions
- [ ] How does the control plane work and what data does it process?
- [ ] What exactly is the DERP relay system and when is it used?
- [ ] How does Tailscale extend/modify the WireGuard protocol?
- [ ] What's the actual NAT traversal success rate in practice?
- [ ] How does MagicDNS work internally?

### 10.2 Security & Privacy Questions
- [ ] What data does Tailscale have access to, exactly?
- [ ] What are the implications if Tailscale is compromised?
- [ ] How does the trust model compare to self-hosted solutions?
- [ ] Has Tailscale had any security incidents? How were they handled?
- [ ] What's the cryptographic key lifecycle?

### 10.3 Practical Implementation Questions
- [ ] What's the real-world performance for file transfers? Latency-sensitive apps?
- [ ] How do ACLs work in practice for home lab scenarios?
- [ ] What's the best approach for subnet routing in a home lab?
- [ ] How does Tailscale integrate with Docker/containers?
- [ ] What's the experience on mobile? Battery impact?

### 10.4 Comparison Questions
- [ ] When should you use raw WireGuard instead of Tailscale?
- [ ] What does Headscale sacrifice for self-hosting?
- [ ] How does ZeroTier compare for similar use cases?
- [ ] When should Cloudflare Tunnel be used instead of or with Tailscale?
- [ ] Is Netbird a viable fully-open-source alternative?

### 10.5 Long-Term Considerations
- [ ] What happens if Tailscale changes pricing or terms?
- [ ] Is the free tier likely to remain sufficient?
- [ ] What's Tailscale's business model and sustainability?
- [ ] How does the company handle feature requests and community?
- [ ] What's the Headscale project's long-term viability?

---

## 11. Proposed Research Documents

### 11.1 Foundation Documents

| Document | Description | Priority |
|----------|-------------|----------|
| `01-architecture-deep-dive.md` | How Tailscale works internally | High |
| `02-wireguard-foundation.md` | WireGuard fundamentals and Tailscale's extensions | High |
| `03-nat-traversal-explained.md` | How Tailscale achieves connectivity | Medium |

### 11.2 Feature Documentation

| Document | Description | Priority |
|----------|-------------|----------|
| `04-networking-features.md` | MagicDNS, subnet routing, exit nodes | High |
| `05-access-control.md` | ACLs, tags, groups, policies | High |
| `06-authentication-identity.md` | IdPs, SSO, multi-user setups | Medium |
| `07-advanced-features.md` | Funnel, Serve, Taildrop, SSH | Medium |

### 11.3 Comparison Documents

| Document | Description | Priority |
|----------|-------------|----------|
| `08-tailscale-vs-wireguard.md` | Detailed technical comparison | High |
| `09-tailscale-vs-alternatives.md` | ZeroTier, Netbird, Cloudflare comparisons | Medium |
| `10-headscale-analysis.md` | Self-hosting option deep dive | Medium |

### 11.4 Security & Privacy Documents

| Document | Description | Priority |
|----------|-------------|----------|
| `11-security-model.md` | Trust model, attack surface, audit results | High |
| `12-privacy-analysis.md` | Data exposure, metadata, privacy implications | High |

### 11.5 Implementation Documents

| Document | Description | Priority |
|----------|-------------|----------|
| `13-home-lab-patterns.md` | Deployment patterns for home labs | High |
| `14-service-integration.md` | Docker, proxies, specific services | High |
| `15-operational-guide.md` | Day-to-day operations and troubleshooting | Medium |

### 11.6 Summary Documents

| Document | Description | Priority |
|----------|-------------|----------|
| `20-executive-summary.md` | Key findings and recommendations | Final |
| `21-decision-guide.md` | When to use Tailscale vs alternatives | Final |
| `22-implementation-plan.md` | Specific plan for this home lab | Final |

---

## 12. Research Methodology

### 12.1 Primary Sources

| Source Type | Examples |
|-------------|----------|
| **Official Documentation** | Tailscale docs, knowledge base, blog |
| **Source Code** | Tailscale client code (Go), Headscale code |
| **Technical Papers** | WireGuard whitepaper, Tailscale security writeups |
| **API Documentation** | Tailscale API reference |
| **Release Notes** | Changelogs and release announcements |

### 12.2 Community Sources

| Source Type | Examples |
|-------------|----------|
| **Reddit** | r/Tailscale, r/selfhosted, r/homelab |
| **Forums** | Tailscale community forums |
| **GitHub** | Issues, discussions, PRs |
| **Blogs** | User experiences, tutorials, benchmarks |
| **YouTube** | Tutorial videos, reviews, comparisons |

### 12.3 Hands-On Testing

| Test Type | Description |
|-----------|-------------|
| **Basic Setup** | Install and configure Tailscale |
| **Feature Testing** | Test specific features (ACLs, subnet routing, etc.) |
| **Performance Testing** | Benchmark throughput, latency |
| **Integration Testing** | Docker, services, NAS integration |
| **Failure Testing** | What happens when things go wrong |

---

## 13. Success Criteria

The Tailscale research phase will be considered complete when we can confidently answer:

### 13.1 Understanding Questions
1. **How does Tailscale work?** - Complete understanding of architecture
2. **What are the security implications?** - Full trust model analysis
3. **What are the privacy trade-offs?** - Clear data exposure understanding

### 13.2 Decision Questions
4. **Should we use Tailscale for this home lab?** - Informed recommendation
5. **Tailscale vs Headscale?** - Clear criteria for choice
6. **What features do we need?** - Feature requirement mapping

### 13.3 Implementation Questions
7. **How should we configure Tailscale?** - Specific setup plan
8. **What ACL policies do we need?** - Access control design
9. **How do we integrate with existing services?** - Integration patterns

### 13.4 Operational Questions
10. **How do we manage it day-to-day?** - Operational procedures
11. **What can go wrong and how do we fix it?** - Troubleshooting guide
12. **What's our fallback plan?** - Redundancy and backup strategy

---

## 14. Research Timeline Estimate

| Phase | Duration | Focus |
|-------|----------|-------|
| **Phase 1: Foundations** | 2-3 hours | Architecture, WireGuard, NAT traversal |
| **Phase 2: Features** | 2-3 hours | Core features, access control, advanced features |
| **Phase 3: Comparisons** | 2-3 hours | WireGuard, alternatives, Headscale |
| **Phase 4: Security/Privacy** | 1-2 hours | Security model, privacy analysis |
| **Phase 5: Implementation** | 2-3 hours | Patterns, integration, operations |
| **Phase 6: Synthesis** | 1-2 hours | Summaries, decisions, plans |

**Total Estimated: 10-16 hours of research**

---

## 15. Key Terminology

| Term | Definition |
|------|------------|
| **Tailnet** | Your personal Tailscale network containing all your devices |
| **Coordination Server** | Tailscale server that manages key exchange and device discovery |
| **DERP** | Designated Encrypted Relay for Packets - fallback relay servers |
| **MagicDNS** | Tailscale's automatic DNS for device names |
| **ACL** | Access Control List - policies defining who can access what |
| **Subnet Router** | Device that advertises routes to a local subnet |
| **Exit Node** | Device that routes all traffic for other devices |
| **Tailscale Funnel** | Feature to expose services to the public internet |
| **Tailscale Serve** | Feature to serve content over Tailscale or Funnel |
| **Headscale** | Open-source, self-hosted Tailscale control server |
| **Taildrop** | Tailscale's secure file sharing feature |
| **WireGuard** | The underlying VPN protocol Tailscale is built on |
| **NAT Traversal** | Techniques to establish connections through NAT devices |
| **Mesh VPN** | VPN topology where all nodes can connect directly |

---

## 16. References & Resources

### Official Resources
- [Tailscale Documentation](https://tailscale.com/kb/)
- [Tailscale Blog](https://tailscale.com/blog/)
- [Tailscale GitHub](https://github.com/tailscale/tailscale)
- [WireGuard Whitepaper](https://www.wireguard.com/papers/wireguard.pdf)

### Self-Hosting Resources
- [Headscale GitHub](https://github.com/juanfont/headscale)
- [Headscale Documentation](https://headscale.net/)

### Community Resources
- [r/Tailscale](https://www.reddit.com/r/Tailscale/)
- [r/selfhosted](https://www.reddit.com/r/selfhosted/)
- [r/homelab](https://www.reddit.com/r/homelab/)

### Comparison Resources
- [ZeroTier Documentation](https://docs.zerotier.com/)
- [Netbird Documentation](https://docs.netbird.io/)
- [WireGuard Documentation](https://www.wireguard.com/)

---

## 17. Next Steps

1. [ ] Begin with `01-architecture-deep-dive.md` - understand how Tailscale works
2. [ ] Research `08-tailscale-vs-wireguard.md` - key comparison for decision making
3. [ ] Analyze `11-security-model.md` - critical for trust decisions
4. [ ] Explore `13-home-lab-patterns.md` - practical implementation guidance
5. [ ] Create `20-executive-summary.md` - synthesize findings for decision

---

**Document Status:** Ready for research execution
**Estimated Completion:** After 10-16 hours of focused research
**Next Review:** After initial foundation documents completed
