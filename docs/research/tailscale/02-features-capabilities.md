# Tailscale Features and Capabilities

**Research Date:** 2026-01-20
**Category:** VPN Technology - Features
**Sources:** Official Tailscale documentation 2026

---

## Overview

Tailscale provides a comprehensive set of features beyond basic VPN connectivity, including automatic DNS, subnet routing, access control, and service exposure capabilities.

---

## MagicDNS

### What is MagicDNS?

Automatic DNS name assignment for devices in your Tailscale network (tailnet).

**Key Characteristics:**
- **Enabled by default** - No configuration required
- **Human-readable names** - Better discoverability
- **Automatic registration** - Names appear immediately
- **Private DNS** - Only accessible within tailnet

### How It Works

**Automatic Naming:**
```
Device hostname: my-laptop
Tailscale name: my-laptop.tail-scale.ts.net
```

Each device gets:
- Machine name based on hostname
- Fully qualified domain name (FQDN)
- Automatic DNS resolution
- Private to your tailnet

### Use Cases

**Instead of IP addresses:**
```bash
# Without MagicDNS
ssh 100.64.1.5

# With MagicDNS
ssh my-server
```

**Service Discovery:**
- Access services by name
- No manual DNS configuration
- Automatic updates when devices change
- Works across all devices in tailnet

### Custom DNS Configuration

MagicDNS also supports:
- Split DNS - Different nameservers for different domains
- Global nameservers - Override default DNS
- Custom search domains - Shorter names
- DNS-over-HTTPS - Encrypted DNS queries

---

## Subnet Routing

### Concept

Expose entire subnets through a Tailscale device, allowing access to non-Tailscale devices on that network.

### How It Works

**Architecture:**
```
Remote Device (Tailscale)
    ↓ [Tailscale mesh]
Subnet Router (Tailscale device on LAN)
    ↓ [Normal routing]
Non-Tailscale devices (192.168.1.0/24)
```

The subnet router acts as a gateway, forwarding traffic between the Tailscale mesh and local network.

### Configuration

**On the subnet router:**
1. Enable IP forwarding
2. Advertise subnet routes
3. Approve routes in admin console

**Command example:**
```bash
tailscale up --advertise-routes=192.168.1.0/24,10.0.0.0/24
```

### Use Cases

**Home Lab Access:**
- Access NAS without installing Tailscale on it
- Reach IoT devices that can't run Tailscale
- Manage network equipment (routers, switches)
- Access VMs and containers

**Multi-Site Networking:**
- Connect office networks together
- Site-to-site VPN replacement
- Branch office connectivity
- Hybrid cloud access

### Advanced Features

**BGP Advertisement:**
- Dynamic route updates
- Integration with existing routing
- Enterprise networking

**4via6 Subnet Routers:**
- IPv4 over IPv6 tunneling
- Future-proofing networks
- Address space flexibility

---

## Exit Nodes

### Concept

Route ALL internet traffic through a specific Tailscale device, similar to traditional VPN.

### Difference from Subnet Routing

| Feature | Subnet Router | Exit Node |
|---------|---------------|-----------|
| **Purpose** | Access specific subnets | Route all internet traffic |
| **Scope** | Limited to advertised routes | Everything (0.0.0.0/0) |
| **Use case** | Access home lab | Browse internet securely |

### How It Works

**Traffic Flow:**
```
Your Device
    ↓ [All internet traffic]
Exit Node
    ↓ [To internet]
Websites see exit node's IP
```

### Configuration

**On exit node:**
```bash
tailscale up --advertise-exit-node
```

**On client:**
```bash
tailscale up --exit-node=exit-node-name
```

### Use Cases

**Public WiFi:**
- Secure browsing on untrusted networks
- Hide traffic from local network
- Encrypt all internet traffic

**Geo-Shifting:**
- Access services through home IP
- Appear in different location
- Bypass regional restrictions

**Privacy:**
- Hide real IP address
- Route through trusted device
- Avoid ISP logging at remote location

### Types of Exit Nodes

**Recommended Exit Nodes:**
- Tailscale suggests best options
- Based on location and performance
- Simplified selection

**Mandatory Exit Nodes:**
- Admins can require exit node use
- Enforce security policies
- Control internet access

---

## Access Control (ACLs and Grants)

### Evolution

**Old System:** ACLs (Access Control Lists)
**New System:** Grants (2026 recommendation)

> "Tailscale now secures access to resources using grants, a next-generation access control policy syntax that provides all original ACL functionality plus additional capabilities."

### Policy File Format

Written in **HuJSON** (Human JSON):
- JSON with comments
- More readable
- Easier to maintain
- Version controllable

### Key Concepts

**Tags:**
- Label devices with roles
- Example: `tag:server`, `tag:dev`, `tag:prod`
- Used in access rules

**Groups:**
- Collections of users
- Example: `group:admins`, `group:developers`
- Simplify policy management

**Autogroups:**
- Built-in groups
- `autogroup:members` - All users
- `autogroup:admin` - Admins only
- `autogroup:internet` - Exit nodes

### Grant Examples

**Allow group to access tagged servers:**
```json
"grants": [
  {
    "src": ["group:developers"],
    "dst": ["tag:dev-servers"],
    "app": {
      "tailscale.com/cap/ssh": [
        {
          "users": ["autogroup:members"]
        }
      ]
    }
  }
]
```

**Device-specific access:**
```json
"grants": [
  {
    "src": ["user:alice@example.com"],
    "dst": ["tag:production"],
    "ports": ["tcp:443", "tcp:80"]
  }
]
```

### Access Control Best Practices

1. **Deny by default** - Only allow what's needed
2. **Use tags** - Group devices by function
3. **Minimize permissions** - Least privilege principle
4. **Regular audits** - Review and update policies
5. **Test changes** - Validate before deploying

---

## Tailscale SSH

### Concept

Use Tailscale to manage SSH authentication and authorization without managing SSH keys.

### How It Works

**Authentication:**
- Tailscale manages authentication
- Uses WireGuard node keys
- No SSH keys to distribute
- Access controlled by Tailscale ACLs/grants

**Connection Flow:**
```
ssh user@machine-name
    ↓
Tailscale authenticates with WireGuard
    ↓
Tailscale checks ACL/grant permissions
    ↓
SSH connection established
```

### Benefits

✅ **No SSH key management** - Tailscale handles it
✅ **Centralized access control** - ACLs/grants apply
✅ **Automatic encryption** - WireGuard + SSH
✅ **Audit logging** - Who accessed what
✅ **Easier onboarding** - No key distribution

### Configuration

**In policy file:**
```json
"grants": [
  {
    "src": ["group:admins"],
    "dst": ["tag:servers"],
    "app": {
      "tailscale.com/cap/ssh": [
        {
          "users": ["root", "admin"]
        }
      ]
    }
  }
]
```

### Limitations

- Cannot use SSH certificates (feature request exists)
- Requires Tailscale running on both ends
- Still uses underlying SSH daemon
- Some advanced SSH features may not work

---

## Tailscale Serve

### Purpose

Share local services securely **within your tailnet only**.

### How It Works

**Expose local service:**
```bash
tailscale serve 3000
```

This makes `http://machine-name:3000` accessible to anyone in your tailnet.

### Features

- **HTTPS automatically** - Tailscale provides certs
- **Tailnet-only** - Not exposed to internet
- **Simple command** - One line to expose service
- **Multiple ports** - Can serve multiple services

### Use Cases

**Development:**
- Share local dev server with team
- Test on other devices
- Mobile app testing

**Internal Tools:**
- Admin interfaces
- Monitoring dashboards
- Internal applications

**File Sharing:**
- Quick file server
- Share build artifacts
- Temporary access

---

## Tailscale Funnel

### Purpose

Expose local services **to the public internet** through Tailscale's infrastructure.

### How It Works

**One command exposure:**
```bash
tailscale funnel 3000
```

This creates a public HTTPS URL accessible to anyone on the internet.

### Key Characteristics

**Security:**
- **Ports:** Only 443, 8443, and 10000
- **Encryption:** TLS-only (no plain HTTP)
- **Certificates:** Automatic HTTPS certs
- **Tailscale 1.52+:** Single command simplicity

**URL Format:**
```
https://machine-name.ts.net
```

### Serve vs Funnel

| Feature | Serve | Funnel |
|---------|-------|--------|
| **Visibility** | Tailnet only | Public internet |
| **Use case** | Internal tools | Public demos, webhooks |
| **Security** | Tailnet auth | Public access |
| **Ports** | Any | 443, 8443, 10000 only |

### Use Cases

**Development:**
- Share work-in-progress
- Demo to clients
- Test webhooks

**Temporary Hosting:**
- Quick file sharing
- Event landing pages
- Personal blog

**Integration Testing:**
- Receive webhook calls
- OAuth callbacks
- API testing

### Security Considerations

⚠️ **Public access** - Anyone can reach your service
⚠️ **Add authentication** - Funnel doesn't provide it
⚠️ **Monitor usage** - Check for abuse
⚠️ **Temporary use** - Don't rely for production

---

## Taildrop

### Purpose

Quick file sharing between devices in your tailnet.

### How It Works

**Send files:**
- Drag and drop in Tailscale UI
- Command line: `tailscale file cp file.txt machine:`
- Mobile apps: Share menu integration

**Receive files:**
- Automatic save to configured folder
- Notification on receipt
- No manual acceptance needed

### Features

- Cross-platform (Windows, Mac, Linux, iOS, Android)
- No file size limits (within reason)
- Direct peer-to-peer transfer
- Encrypted with WireGuard
- No intermediary storage

### Comparison to Alternatives

| Solution | Speed | Setup | Internet Required |
|----------|-------|-------|-------------------|
| **Taildrop** | Fast (direct P2P) | None | No (for LAN) |
| **LocalSend** | Very fast | Install app | No |
| **Cloud storage** | Moderate | Account needed | Yes |
| **Email** | Slow | Email account | Yes |

---

## Monitoring and Observability

### Built-in Metrics

Tailscale provides:
- **Connection status** - Direct vs relayed
- **Bandwidth usage** - Per-device stats
- **Latency measurements** - Round-trip times
- **Packet loss** - Connection quality

### Access Information

**Web UI:**
- Admin console shows all devices
- Connection types visible
- Health checks

**CLI:**
```bash
tailscale status      # Device status
tailscale ping peer   # Test connectivity
tailscale netcheck    # Network diagnostics
```

### Integration

**External monitoring:**
- Export metrics for Prometheus
- Send logs to SIEM
- API access for custom tools
- Webhooks for events

---

## Platform Support

### Desktop Operating Systems

✅ **Windows** - Full support, GUI client
✅ **macOS** - Full support, menu bar app
✅ **Linux** - Full support, CLI and GUI options
✅ **ChromeOS** - Android app compatibility

### Mobile Devices

✅ **iOS** - Native app, system VPN integration
✅ **Android** - Native app, system VPN integration

### Servers and Containers

✅ **Linux servers** - Systemd integration
✅ **Docker** - Official container image
✅ **Kubernetes** - Operator available
✅ **LXC** - Supported with configuration

### Network Equipment

✅ **Synology** - Official package
✅ **QNAP** - Community support
✅ **UniFi** - Third-party integration
✅ **OpenWrt** - Community packages
✅ **pfSense/OPNsense** - Third-party packages

### Embedded Devices

✅ **Raspberry Pi** - Full support
✅ **ARM devices** - Various architectures
✅ **IoT devices** - Depends on OS

---

## Feature Comparison by Plan

### Free Personal Plan

✅ Core mesh VPN
✅ Up to 100 devices
✅ 3 users max
✅ MagicDNS
✅ Subnet routing
✅ Exit nodes
✅ Basic ACLs
✅ Taildrop
✅ Serve and Funnel

### Personal Plus ($5/month)

Everything in Free plus:
✅ Up to 6 users
✅ Better support

### Business Plans (Starter/Premium/Enterprise)

Everything in Personal Plus plus:
✅ Unlimited users
✅ Advanced ACLs/grants
✅ Audit logs
✅ SSO integration
✅ Custom DERP servers
✅ Dedicated support
✅ SLA guarantees
✅ Compliance features

---

## Sources

- [Features · Tailscale](https://tailscale.com/features)
- [DNS in Tailscale · Tailscale Docs](https://tailscale.com/kb/1054/dns)
- [Secure Your Network with Tailscale DNS and MagicDNS](https://tailscale.com/blog/2021-09-private-dns-with-magicdns)
- [Route traffic · Tailscale Docs](https://tailscale.com/kb/1351/route)
- [Subnet routers · Tailscale Docs](https://tailscale.com/kb/1019/subnets)
- [Exit nodes (route all traffic) · Tailscale Docs](https://tailscale.com/kb/1103/exit-nodes)
- [Use exit nodes · Tailscale Docs](https://tailscale.com/kb/1408/quick-guide-exit-nodes)
- [Syntax reference for the tailnet policy file · Tailscale Docs](https://tailscale.com/kb/1337/policy-syntax)
- [Tailscale SSH · Tailscale Docs](https://tailscale.com/kb/1193/tailscale-ssh)
- [Manage SSH Keys Securely with Tailscale](https://tailscale.com/tailscale-ssh)
- [tailscale serve command · Tailscale Docs](https://tailscale.com/kb/1242/tailscale-serve)
- [Tailscale Serve · Tailscale Docs](https://tailscale.com/kb/1312/serve)
- [Tailscale Funnel · Tailscale Docs](https://tailscale.com/kb/1223/funnel)
- [tailscale funnel command · Tailscale Docs](https://tailscale.com/kb/1311/tailscale-funnel)
- [Tailscale Funnel examples · Tailscale Docs](https://tailscale.com/kb/1247/funnel-examples)
- [Reintroducing Serve and Funnel](https://tailscale.com/blog/reintroducing-serve-funnel)
