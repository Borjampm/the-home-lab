# Tailscale Architecture and How It Works

**Research Date:** 2026-01-20
**Category:** VPN Technology - Architecture
**Sources:** Official Tailscale documentation and technical blogs

---

## Overview

Tailscale is a mesh VPN built on top of WireGuard that automatically creates a private network between devices without manual configuration, port forwarding, or NAT traversal setup.

---

## Core Architecture

### Control Plane vs Data Plane Separation

**Control Plane (Hub-and-Spoke):**
- Centralized coordination server at login.tailscale.com
- Manages encryption keys, network changes, access policies
- Carries virtually no traffic - just exchanges metadata
- Maintains connections to all devices

**Data Plane (Mesh):**
- Direct peer-to-peer connections when possible
- Encrypted WireGuard tunnels between devices
- No central bottleneck for data transfer
- End-to-end encryption

> "The control plane is hub-and-spoke but carries virtually no traffic - it just exchanges encryption keys and sets policies, while the data plane is a mesh."

---

## The Coordination Server

### Role and Responsibilities

The coordination server functions as a **shared drop box** for:
- **Public keys** - Distribution of WireGuard public keys
- **Metadata** - Network topology information
- **Access policies** - Who can access what
- **Connection state** - Device online/offline status

### What It Does NOT Do

❌ Does not relay user traffic
❌ Does not decrypt communications
❌ Does not see content of data transfers
❌ Does not create performance bottlenecks

### Critical Design Choice

By separating control and data planes, Tailscale avoids the central server bottleneck common in traditional VPNs.

---

## DERP Relay System

### What is DERP?

**DERP** = Designated Encrypted Relay for Packets

DERP serves the same role as TURN servers in the ICE standard but uses:
- HTTPS streams instead of UDP
- WireGuard keys for encryption
- More efficient protocol design

### Primary Purposes

1. **Negotiating connections** - Help devices find each other
2. **Fallback relay** - When direct connections impossible
3. **NAT traversal** - Coordinate hole punching attempts

### How DERP Works

**Initial Connection:**
```
Device A → DERP Server ← Device B
    ↓
[Exchange disco messages]
    ↓
[Attempt direct connection]
    ↓
Success: Direct P2P connection
Failure: Continue through DERP
```

**Key Security Property:**
> "DERP servers cannot decrypt traffic. A DERP server blindly forwards already-encrypted traffic from one device to another."

All data sent through DERP is encrypted with WireGuard using keys that never leave local devices.

### DERP Server Selection

- Clients pick their "DERP home" (closest/fastest server)
- Report selection to coordination server
- Coordination server shares this info with peers
- When sending packets, use recipient's DERP home

---

## Connection Establishment Process

### Phase 1: Initial Connection (Always Relayed)

All connections start relayed through DERP:
```
Device A
    ↓ [WireGuard encrypted]
DERP Server
    ↓ [WireGuard encrypted]
Device B
```

This ensures connectivity even before direct connection attempts.

### Phase 2: Direct Connection Attempts

Tailscale tries to upgrade to direct connection:

1. **Send disco (discovery) messages** - Test NAT traversal
2. **Exchange endpoint information** - Share potential connection points
3. **Hole punching** - Coordinate simultaneous packets
4. **STUN discovery** - Determine NAT type and external IP

### Phase 3: Peer Relay (If Direct Fails)

If direct connection fails but DERP would be slow, try **peer relay**:
- Use another device as intermediary
- Still encrypted end-to-end
- Better latency than DERP in some cases

### Phase 4: Final State

Connection settles into one of three states:

| Connection Type | Description | Performance |
|-----------------|-------------|-------------|
| **Direct** | Peer-to-peer WireGuard tunnel | Excellent |
| **Peer Relay** | Through intermediate device | Good |
| **DERP Relay** | Through DERP server | Moderate |

> "All connections start as relayed through a DERP server, then Tailscale tries to upgrade them to a direct connection; if that fails, it tries peer relay; if that fails, the connection remains relayed through DERP."

---

## NAT Traversal Deep Dive

### The NAT Problem

Network Address Translation (NAT) prevents incoming connections to devices behind routers. Traditional solutions require:
- Port forwarding (manual configuration)
- UPnP (security risk)
- Static IP addresses (expensive)

### Tailscale's Solution

**Automatic hole punching** using coordination server:

1. **Both devices send packets simultaneously**
2. **Each NAT sees outbound traffic**
3. **Return flows are allowed**
4. **Direct connection established**

> "Tailscale's coordination layer normally breaks the deadlock by getting both devices to send packets at the same time, so each NAT sees outbound traffic and allows the return flow."

### NAT Traversal Techniques

Tailscale uses multiple techniques:

**STUN (Session Traversal Utilities for NAT):**
- Discover external IP and port
- Determine NAT type
- Test connectivity paths

**ICE (Interactive Connectivity Establishment):**
- Try multiple connection candidates
- Race different paths
- Pick fastest successful connection

**UDP Keepalives:**
- Maintain NAT mappings
- Prevent timeout of hole-punched connections

**Coordinated Sending:**
- Synchronize packet transmission
- Break NAT deadlocks

### Hard Cases

Even with advanced techniques, some scenarios are difficult:

**Symmetric NAT:**
- Changes port for each destination
- Harder to predict and punch through
- DERP relay often required

**Multiple NATs:**
- Carrier-grade NAT (CGNAT) + home NAT
- More layers to traverse
- Lower direct connection success rate

**Strict Firewalls:**
- Corporate networks
- Restrictive policies
- May block UDP or unusual traffic

### Success Rate

> "Even with a peer connection success rate above 90%, Tailscale has made significant efforts to improve NAT traversal."

Tailscale constantly works to improve NAT traversal for hard cases.

---

## WireGuard Foundation

### Why WireGuard?

Tailscale built on WireGuard because:
- **Modern cryptography** - ChaCha20, Poly1305, Curve25519
- **High performance** - Minimal protocol overhead
- **Simple design** - Less than 4,000 lines of code
- **Audited** - Well-reviewed security
- **Fast** - Can achieve 8+ Gbps in ideal conditions

### How Tailscale Uses WireGuard

**Core Protocol:**
- WireGuard handles all encryption
- WireGuard manages tunnel communication
- WireGuard provides cryptographic identity

**Tailscale Additions:**
- Automatic key distribution
- Dynamic peer discovery
- NAT traversal coordination
- Access control layer
- User authentication

### WireGuard Packet Flow

```
Application Data
    ↓
[WireGuard Encryption]
    ↓
Encrypted Packet
    ↓
[Direct connection or DERP]
    ↓
Recipient Device
    ↓
[WireGuard Decryption]
    ↓
Application Data
```

### Cryptographic Guarantees

**Encryption:** ChaCha20 with Poly1305 authentication
**Key Exchange:** Noise protocol framework with Curve25519
**Forward Secrecy:** Regular key rotation
**Authentication:** Public key cryptography

---

## Key Exchange and Distribution

### Key Generation

**On Each Device:**
1. Generate private key locally
2. Derive public key from private key
3. **Private key NEVER leaves device**
4. Public key sent to coordination server

### Key Distribution

**Coordination Server Role:**
```
Device A → [Public Key A] → Coordination Server
                                ↓
                        [Stores in database]
                                ↓
Device B ← [Public Key A] ← Coordination Server
```

**What Gets Shared:**
- Public keys only
- Node metadata
- Endpoint information
- Access permissions

**What Never Leaves Device:**
- Private keys
- Decryption capability
- Data traffic content

### Trust Model

You must trust:
- Coordination server to distribute correct public keys
- Tailscale not to tamper with key distribution

You do NOT need to trust:
- Coordination server with traffic (can't decrypt)
- DERP servers with traffic (can't decrypt)
- Tailscale with data content (end-to-end encrypted)

**Tailnet Lock** provides additional protection:
- Cryptographically sign permitted public keys
- Nodes verify signatures before trusting
- Protection against coordination server compromise

---

## Connection Types in Detail

### Direct Connections

**Characteristics:**
- Peer-to-peer WireGuard tunnel
- UDP port 41641 (source)
- Best performance and lowest latency
- No intermediate hops

**When Achieved:**
- Both devices have public IPs
- NAT traversal succeeds
- No restrictive firewalls blocking UDP
- Direct routing path exists

### DERP Relayed Connections

**Characteristics:**
- Traffic flows through DERP server
- Still end-to-end encrypted
- Higher latency than direct
- Guaranteed connectivity

**When Used:**
- Direct connection impossible
- Symmetric NAT or CGNAT
- Strict firewalls blocking direct UDP
- Initial connection before upgrade attempt

**Performance:**
- Can drop to ~35.6 Mbps in worst case
- Added latency from relay hop
- Still encrypted and secure

### Peer Relay Connections

**Characteristics:**
- Use another Tailscale node as relay
- Better latency than DERP in some cases
- Still encrypted end-to-end
- Transparent to applications

**When Used:**
- Direct connection failed
- But another node has better path
- DERP would be geographically far
- Optimization for latency

---

## Performance Characteristics

### Direct Connection Performance

**Best Case:**
- Near-native WireGuard performance
- Up to 8 Gbps theoretically
- Latency = Network latency only
- No added overhead

**Typical:**
- 100-1000 Mbps on consumer hardware
- Sub-millisecond added latency
- CPU encryption overhead minimal

### DERP Relay Performance

**Bandwidth:**
- Can vary significantly
- 35-100+ Mbps typical
- Depends on DERP server load and location

**Latency:**
- Additional hop to DERP server
- Geographic distance matters
- 10-100ms added latency

### Optimization Strategy

Tailscale constantly works to maximize direct connections:
- Improved NAT traversal algorithms
- Better DERP server placement
- Smarter fallback decisions
- Peer relay optimization

---

## What Happens if Coordination Server is Down?

### Existing Connections

✅ **Continue working** - Data plane independent of control plane
✅ **Direct connections unaffected**
✅ **DERP relays continue** (with caveats)

### New Connections

❌ Cannot establish new device connections
❌ Cannot update access policies
❌ Cannot add/remove devices

### Fallback Behavior

**Last known good state:**
- Devices cache peer information
- Can reconnect to known peers
- Access policies cached locally
- Continue with stored keys

**Duration:**
- Can operate offline for hours/days
- Eventually may need reauthorization
- Long outages require manual intervention

---

## Sources

- [Tailscale: How it works](https://tailscale.com/blog/how-tailscale-works)
- [DERP servers · Tailscale Docs](https://tailscale.com/kb/1232/derp-servers)
- [Connection types · Tailscale Docs](https://tailscale.com/kb/1257/connection-types)
- [Tailscale Peer Relays · Tailscale Docs](https://tailscale.com/kb/1591/peer-relays)
- [How Tailscale is improving NAT traversal (part 1)](https://tailscale.com/blog/nat-traversal-improvements-pt-1)
- [How NAT traversal works](https://tailscale.com/blog/how-nat-traversal-works)
- [What happens if the coordination server is down?](https://tailscale.com/kb/1091/what-happens-if-the-coordination-server-is-down)
- [Terminology and concepts · Tailscale Docs](https://tailscale.com/kb/1155/terminology-and-concepts)
- [Now with more DERP](https://tailscale.com/blog/more-derp)
