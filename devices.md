# Home Network Devices

## Network Info
- **Subnet:** 192.168.68.0/24
- **Gateway:** 192.168.68.1
- **Scan date:** 2026-01-21
- **Total devices:** 19

## Security Alerts

> **WARNING:** Network printer (192.168.68.107) has Telnet enabled (port 23) - credentials transmitted in plaintext. Disable immediately via printer web interface.

---

## Network Infrastructure

### 192.168.68.1 - TP-Link Mesh Router (Main Gateway)
- **MAC:** d8:07:b6:f3:9b:2c
- **Type:** TP-Link Deco Mesh Router (Primary Unit)
- **Services:** DNS (53), HTTP (80), HTTPS (443), UPnP (1900)
- **Actions:**
  - Web interface: http://192.168.68.1 or https://192.168.68.1
  - UPnP port forwarding and device discovery
  - Home Assistant integration: [tplink-router](https://github.com/AlexandrErohin/home-assistant-tplink-router)
  - Note: No official API - only web interface and unofficial integrations

### 192.168.68.246-250 - TP-Link Mesh Satellite Nodes
| IP | MAC |
|----|-----|
| 192.168.68.246 | d8:07:b6:f3:a0:20 |
| 192.168.68.247 | d8:07:b6:f3:2a:8c |
| 192.168.68.248 | d8:07:b6:f3:9e:fc |
| 192.168.68.249 | d8:07:b6:f3:2a:28 |
| 192.168.68.250 | d8:07:b6:f3:2c:5c |

- **Type:** TP-Link Deco Mesh Satellite Units
- **Services:** HTTP (80), HTTPS (443)
- **Actions:** Individual web interfaces for local management, mesh topology visualization

---

## Apple Devices

### 192.168.68.102 - Apple Time Capsule
- **MAC:** 6c:70:9f:d7:67:4d
- **Hostname:** borjas-airport-time-capsule
- **Type:** Network Storage / Time Machine Backup Device
- **Services:** NetBIOS (139), SMB (445), AFP (548), AirPort Admin (5009)
- **Actions:**
  - Time Machine backups from macOS devices
  - File sharing via AFP and SMB/CIFS
  - Network-attached storage (NAS)
  - Admin interface on port 5009

### 192.168.68.103 - Apple TV or HomePod
- **MAC:** c8:69:cd:04:d6:2f
- **Type:** AirPlay Device (Apple TV or HomePod)
- **Services:** RTSP/AirTunes (5000), AirPlay streaming (49152-49153)
- **Actions:**
  - AirPlay audio/video streaming from iOS/macOS
  - Screen mirroring
  - HomeKit Hub for home automation
  - Siri voice control

### 192.168.68.109 - iPhone or iPad
- **MAC:** f0:18:98:83:80:c5
- **Type:** iOS Mobile Device
- **Services:** None open (normal for iOS)
- **Actions:**
  - AirDrop file sharing
  - AirPlay streaming
  - HomeKit control via Home app
  - Continuity features (Handoff, Universal Clipboard)

---

## Entertainment Devices

### 192.168.68.104 - Roku Express
- **MAC:** 8c:49:62:d2:15:0c
- **Type:** Roku Express 3930MX Streaming Player
- **Name:** "Roku Express Marquez de la Plata" (Living room)
- **Services:** AirPlay (7000), Roku ECP API (8060), HTTP (9080), SSDP (1900/UDP)
- **Actions:**
  - **REST API (port 8060):**
    ```bash
    # Device info
    curl http://192.168.68.104:8060/query/device-info
    # Send remote commands
    curl -d '' http://192.168.68.104:8060/keypress/Home
    # Launch Netflix
    curl -d '' http://192.168.68.104:8060/launch/12
    ```
  - AirPlay streaming from Apple devices
  - Home Assistant integration available
  - Note: ECP currently in "limited mode" - enable in Settings > System > Advanced > Control by mobile apps

### 192.168.68.115 - Sony PlayStation
- **MAC:** bc:33:29:ab:b0:ee
- **Type:** PlayStation (PS4/PS5) - Currently in standby mode
- **Services:** None (device in rest mode)
- **Actions:**
  - Remote Play streaming (requires device powered on)
  - Wake-on-LAN potential for remote startup
  - Third-party tools: [pyremoteplay](https://github.com/ktnrg45/pyremoteplay), [Chiaki](https://sr.ht/~thestr4ng3r/chiaki/)
  - Rest mode features: auto-updates, controller charging, remote download

---

## Smart Home & Security

### 192.168.68.106 - Alarm.com Security Hub
- **MAC:** 00:1c:fa:56:a0:3c
- **Type:** Alarm.com Smart Hub / Security Panel
- **Services:** All ports filtered (strong security posture)
- **Actions:**
  - Alarm.com mobile app control
  - Arm/disarm security system
  - Z-Wave device integration
  - AI Deterrence features
  - No local API - cloud-managed only

### 192.168.68.108 - DoorBird Video Doorbell
- **MAC:** b8:3a:9d:70:9c:68
- **Type:** DoorBird IP Video Doorbell (via Alarm.com)
- **Services:** RTSP video streaming (554)
- **Actions:**
  - **RTSP stream:** `rtsp://user:pass@192.168.68.108:554/mpeg/media.amp`
  - **API - unlock door:** `curl -u user:pass "http://192.168.68.108/bha-api/open-door.cgi"`
  - Home Assistant official integration
  - Two-way audio, motion detection
  - RFID card reader support
  - Works with VLC, NVR systems
  - [Full API docs](https://www.doorbird.com/downloads/api_lan.pdf)

### 192.168.68.107 - Network Printer (Brother/HP)
- **MAC:** f8:da:0c:06:3f:f3
- **Vendor:** Hon Hai (Foxconn) - manufactures printer network cards
- **Type:** Brother or HP Network Printer
- **Services:** FTP (21), **Telnet (23) - SECURITY RISK**, HTTP (80), HTTPS (443), LPD (515), IPP (631), JetDirect (9100)
- **Actions:**
  - Web interface: http://192.168.68.107
  - Network printing via IPP, LPD, or JetDirect
  - **TODO: Disable Telnet and FTP for security**

---

## Mobile Devices (Randomized MACs)

### 192.168.68.105 - macOS Device
- **MAC:** 12:82:4b:92:c4:c7 (locally administered - randomized)
- **Type:** Apple macOS 11 (Big Sur) device
- **Services:** NetBIOS (137-138/UDP), WebSocket server (15611), unknown (51682)
- **Actions:**
  - File sharing via NetBIOS
  - Note: Unusual agar.io game server on port 15611 - investigate if unexpected

### 192.168.68.110 - Mobile Device (Offline)
- **MAC:** 7e:ef:9f:3c:25:4b (locally administered - randomized)
- **Type:** Mobile device (iPhone/Android) - currently offline or sleeping
- **Services:** None responding
- **Actions:** Check when device is active; likely has Privacy Wi-Fi Address enabled

### 192.168.68.113 - Mobile Device (Offline)
- **MAC:** 22:29:fb:fd:dc:6f (locally administered - randomized)
- **Type:** Mobile device - currently offline or sleeping
- **Services:** None responding
- **Actions:** Monitor for activity; may be guest or intermittent device

### 192.168.68.114 - iOS/Android Mobile Device
- **MAC:** f6:ba:ca:7c:06:8c (locally administered - randomized)
- **Type:** iOS or Android mobile device
- **Services:** Background services (49152, 62078) - typical iOS ports
- **Actions:** Likely iPhone with Private Wi-Fi Address; check Settings > Wi-Fi to identify

---

## This Machine

### 192.168.68.222 - marqness (This Computer)
- **MAC:** 00:0e:c8:9f:09:69
- **Type:** Linux Computer (Ubuntu 24.04 on MacBook Air 6,2)
- **Services:** SSH, Docker, Tailscale VPN
- **Actions:** Local machine - scanner host

---

## Integration Summary

| Device | Local API | Home Assistant | Cloud Control |
|--------|-----------|----------------|---------------|
| TP-Link Router | No | Unofficial | TP-Link Deco App |
| Apple Time Capsule | AFP/SMB | Limited | iCloud |
| Apple TV/HomePod | AirPlay | HomeKit | Home App |
| Roku | Yes (ECP) | Official | Roku App |
| PlayStation | Remote Play | Unofficial | PS App |
| Alarm.com Hub | No | Unofficial | Alarm.com App |
| DoorBird | Yes (REST) | Official | DoorBird App |
| Printer | CUPS/IPP | Unofficial | Vendor App |