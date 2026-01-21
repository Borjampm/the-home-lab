# Home Lab Networking and VLANs

**Research Date:** 2026-01-20
**Sources:** Networking guides, tutorials, community best practices

---

## Overview

How home labs connect multiple computers and segment networks using VLANs for security, performance, and organization.

---

## VLAN Fundamentals

### What Are VLANs?

**Purpose:**
- Segment traffic at Layer 2
- Devices on different segments don't see each other's broadcasts
- Improves security, reduces noise, isolates traffic

**Key Concepts:**
- Virtual LANs allow multiple logical networks on same physical infrastructure
- 802.1Q tagging identifies which VLAN a frame belongs to
- VLANs provide isolation without separate switches

---

## Common Home Lab VLAN Use Cases

### Network Segmentation Scenarios

1. **Work-from-Home Isolation**
   - Separate work devices from personal devices
   - Compliance with corporate policies
   - Data protection

2. **Lab Environment Isolation**
   - Production servers vs staging/development
   - Testing without impacting production
   - Safe experimentation

3. **IoT Device Isolation**
   - Untrusted smart home devices
   - Can't see internal network
   - Limited internet-only access

4. **Guest Network**
   - Visitor WiFi access
   - No access to internal resources
   - Internet-only connectivity

5. **Security Zones**
   - DMZ for public-facing services
   - Management network for admin access
   - Trusted internal network

---

## VLAN Configuration Components

### Port Types

**Access Ports:**
- Carry a single VLAN
- Send/receive untagged frames
- Connect to end devices (PCs, printers, etc.)
- Device doesn't need to be VLAN-aware

**Trunk Ports:**
- Carry multiple VLANs
- Use 802.1Q tags to identify frames
- Connect switches to switches
- Connect switches to hypervisor hosts

### Hypervisor Integration

**Proxmox/ESXi Setup:**
- Physical NIC connects to switch trunk port
- Virtual switches serve multiple VLANs over single physical NIC
- VMs can be assigned to specific VLANs
- Allows multiple network segments per host

---

## Typical Home Lab VLAN Design

### Example VLAN Scheme

| VLAN ID | Name | Purpose | Subnet |
|---------|------|---------|--------|
| 1 | Default | Management (avoid using) | N/A |
| 10 | Management | Admin access, hypervisors | 10.0.10.0/24 |
| 20 | Trusted | Personal devices, workstations | 10.0.20.0/24 |
| 30 | Lab | Servers, services, containers | 10.0.30.0/24 |
| 40 | IoT | Smart home devices | 10.0.40.0/24 |
| 50 | Guest | Visitor WiFi | 10.0.50.0/24 |
| 99 | DMZ | Public-facing services | 10.0.99.0/24 |

---

## Router/Firewall Configuration

### Inter-VLAN Routing

**On pfSense/OPNsense/OpenWrt/Unifi:**
- Create logical interface per VLAN tag
- Enable DHCP scope per network
- Specify DNS per network for correct resolution
- Configure firewall rules between VLANs

### Firewall Rules Best Practices

**Default Deny:**
- Block all traffic between VLANs by default
- Explicitly allow only necessary traffic
- Log denied connections for troubleshooting

**Common Rules:**
- Allow trusted → lab (access to services)
- Block IoT → trusted (IoT can't access internal)
- Allow all → internet (with restrictions per VLAN)
- Allow management → all (for admin access)

---

## Common Mistakes to Avoid

### Configuration Errors

1. **Same subnet on multiple VLANs**
   - Causes routing confusion
   - Each VLAN needs unique subnet

2. **Forgetting to tag trunk ports**
   - VLANs won't traverse switches
   - Inter-switch communication fails

3. **No firewall between VLANs**
   - Defeats purpose of segmentation
   - Default routing allows all traffic

4. **Inconsistent VLAN numbering**
   - Makes troubleshooting difficult
   - Document your VLAN scheme

5. **Mixing IoT with lab traffic**
   - Security risk
   - IoT devices can be vulnerable

---

## Switch Configuration

### Managed Switch Requirements

**Minimum Features Needed:**
- 802.1Q VLAN support
- Trunk port configuration
- Access port assignment
- Web UI or CLI for management

**Popular Options:**
- TP-Link managed switches (budget-friendly)
- Ubiquiti UniFi switches (ecosystem integration)
- Cisco small business switches (enterprise-like)
- MikroTik switches (powerful, complex)

### Configuration Steps

1. **Create VLANs on switch**
   - Assign VLAN IDs
   - Name each VLAN

2. **Configure trunk ports**
   - Uplink to router/firewall
   - Links between switches
   - Hypervisor connections

3. **Configure access ports**
   - Assign devices to appropriate VLAN
   - Tag appropriately

4. **Verify configuration**
   - Test connectivity within VLANs
   - Test isolation between VLANs
   - Confirm firewall rules working

---

## Hypervisor VLAN Setup

### Proxmox Example

```
Physical NIC (trunk to switch)
↓
Linux Bridge (vmbr0)
├── VLAN 10 (vmbr0.10) → Management VMs
├── VLAN 20 (vmbr0.20) → Trusted network VMs
├── VLAN 30 (vmbr0.30) → Lab services VMs
└── VLAN 40 (vmbr0.40) → IoT controller VMs
```

### ESXi Example

```
Physical NIC (trunk to switch)
↓
vSwitch
├── Port Group VLAN 10 → Management
├── Port Group VLAN 20 → Trusted
├── Port Group VLAN 30 → Lab
└── Port Group VLAN 40 → IoT
```

---

## Troubleshooting VLANs

### Common Issues

**Can't communicate between VLANs:**
- Check firewall rules on router
- Verify VLAN routing is enabled
- Confirm correct gateway on clients

**VLAN traffic not passing:**
- Verify trunk port configuration
- Check VLAN membership on switches
- Confirm 802.1Q tagging enabled

**Devices can't get DHCP:**
- DHCP server configured for that VLAN?
- DHCP relay configured if needed?
- VLAN correctly assigned to port?

---

## Advanced Topics

### VLAN Best Practices

1. **Document everything**
   - VLAN IDs and purposes
   - IP schemes
   - Firewall rules

2. **Use consistent numbering**
   - Same VLAN IDs across all switches
   - Logical grouping (10-19 infrastructure, 20-29 user networks, etc.)

3. **Native VLAN security**
   - Don't use VLAN 1
   - Set unused native VLAN on trunks
   - Prevents VLAN hopping attacks

4. **Regular audits**
   - Review which devices are on which VLANs
   - Check firewall rule effectiveness
   - Remove unused VLANs

---

## Sources

- [Home Lab Networking 101: VLANs, Subnets, and Segmentation - Virtualization Howto](https://www.virtualizationhowto.com/2025/10/home-lab-networking-101-vlans-subnets-and-segmentation-for-beginners/)
- [Setup VLANs in Your Home Lab](https://phb-crystal-ball.org/setup-vlans-in-your-home-lab/)
- [Setting up and leveling up your homelab - Chris Kirby](https://medium.com/@sirkirby/setting-up-and-leveling-up-your-homelab-a-comprehensive-guide-ce47ef6fa21c)
- [VLANs for the Homelab: A beginner's guide](https://liore.com/vlans-for-the-homelab-a-beginners-guide-to-segmenting-networks/)
- [VLANs and Subnets: Organize Your Home Lab Like a Pro](https://mattadam.com/2025/09/17/how-to-use-vlans-and-subnets-in-home-lab-environments-without-losing-your-mind/)
- [HomeLab Networking Part 1 - The VLANS](https://thelastguardian.me/posts/2019-12-20-homelab-networking-part-1-the-vlans-there-and-back-again/)
