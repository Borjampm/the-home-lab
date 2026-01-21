# R2: Native/Desktop Hardware Monitoring Solutions

## Executive Summary

This report evaluates native macOS and desktop solutions for monitoring remote Ubuntu servers, complementing the web-based solutions covered in R1. Key findings:

- **Best Overall**: SSH + btop/htop offers zero server overhead and excellent UX
- **Best Menu Bar**: ServerCat provides beautiful native UI with comprehensive metrics
- **Best Terminal Dashboard**: btop offers rich visuals and no installation overhead
- **Best for Notifications**: Custom shell scripts with macOS notifications

Unlike web dashboards, native solutions fall into two categories: zero-footprint SSH-based tools and lightweight agent-based apps. The former is recommended for minimal home labs.

---

## 1. Terminal-Based Solutions

### 1.1 htop over SSH

**Overview**: Classic process viewer, the de facto standard for terminal monitoring.

**Installation**:
```bash
# Ubuntu server
sudo apt install htop

# macOS - connect via SSH
ssh user@ubuntu-server.tail-scale.ts.net
htop
```

**Features**:
- Real-time CPU, RAM, swap, process list
- Process management (kill, nice, renice)
- Tree view for process hierarchy
- Color-coded meters
- Keyboard-driven navigation

**Pros**:
- Zero server overhead (only runs when you SSH in)
- Universally available
- Fast, responsive
- Works perfectly over Tailscale SSH
- No installation on macOS needed

**Cons**:
- Not persistent (closes when SSH session ends)
- No historical data
- Basic compared to modern alternatives
- No alerting or notifications

**Resource Usage**: <5MB RAM, negligible CPU (only when viewing)

**Tailscale Compatibility**: Perfect - just SSH to Tailscale hostname

**Use Case**: Quick checks, process debugging, when you want classic reliability

---

### 1.2 btop/bpytop

**Overview**: Modern, beautiful terminal monitoring with rich visualizations.

**Installation**:
```bash
# Ubuntu server - btop (C++, faster)
sudo apt install btop

# Or bpytop (Python, more compatible)
sudo apt install python3-bpytop

# macOS - connect via SSH
ssh user@ubuntu-server.tail-scale.ts.net
btop
```

**Features**:
- Gorgeous TUI with graphs and charts
- CPU, RAM, disk, network, process monitoring
- Historical graphs (during session)
- Mouse support
- Customizable themes
- Real-time network bandwidth per process
- Disk I/O monitoring

**Pros**:
- Best-in-class terminal UI
- More informative than htop (graphs, detailed metrics)
- Still zero persistent overhead
- Works flawlessly over SSH/Tailscale
- Active development
- Intuitive keyboard shortcuts

**Cons**:
- Slightly higher CPU usage than htop (still minimal)
- Requires modern terminal (iTerm2, Terminal.app work fine)
- No persistent monitoring
- No notifications

**Resource Usage**: ~10-20MB RAM, <1% CPU when viewing

**Tailscale Compatibility**: Excellent - tested extensively over Tailscale SSH

**Recommendation**: **Top choice** for interactive terminal monitoring

**Configuration**: Edit `~/.config/btop/btop.conf` on server for themes/preferences

---

### 1.3 Glances (Terminal Mode)

**Overview**: Python-based monitoring that can run in terminal, web, or API mode.

**Installation**:
```bash
# Ubuntu server
sudo apt install glances

# macOS - connect and run
ssh user@ubuntu-server.tail-scale.ts.net
glances
```

**Features**:
- CPU, RAM, swap, disk, network monitoring
- Docker container monitoring
- Plugin system (RAID, sensors, GPU)
- Export to various backends (InfluxDB, CSV)
- Alerts and thresholds
- Client/server mode for remote monitoring

**Pros**:
- Can run as persistent service with remote TUI client
- Extensive plugin ecosystem
- Alert capabilities
- Can export metrics for historical analysis
- Unified across terminal and web modes

**Cons**:
- Python dependency (heavier than btop)
- Less visually appealing than btop in terminal
- More complex to configure

**Client/Server Mode**:
```bash
# Server (runs persistently)
glances -s -B 0.0.0.0

# macOS client (connects to server)
glances -c ubuntu-server.tail-scale.ts.net
```

**Resource Usage**:
- Terminal only: ~50MB RAM
- Persistent server: ~80MB RAM, ~1% CPU

**Tailscale Compatibility**: Excellent - bind to Tailscale IP for security

**Use Case**: When you need alerts/plugins or want persistent monitoring daemon

---

### 1.4 gotop

**Overview**: Go-based terminal dashboard, simpler alternative to btop.

**Installation**:
```bash
# Ubuntu server (manual install)
wget https://github.com/xxxserxxx/gotop/releases/download/v4.2.0/gotop_v4.2.0_linux_amd64.deb
sudo dpkg -i gotop_v4.2.0_linux_amd64.deb
```

**Features**:
- Real-time CPU, memory, disk, network, temperature
- Process list with management
- Battery monitoring (for laptops)
- Colorful graphs

**Pros**:
- Single binary, no dependencies
- Fast and lightweight
- Nice visual design
- Active development

**Cons**:
- Not in default repos (manual install)
- Fewer features than btop
- No historical data
- Less mature than htop

**Resource Usage**: ~8MB RAM, <1% CPU

**Recommendation**: Consider if btop unavailable, but btop is generally better

---

## 2. Menu Bar Apps (macOS Native)

### 2.1 ServerCat

**Overview**: Premium iOS/macOS app for monitoring Linux/Unix servers via SSH.

**Platform**: macOS 12+, iOS/iPadOS

**Features**:
- Beautiful native macOS menu bar widget
- Real-time CPU, RAM, disk, network stats
- Process list with management
- File browser (bonus!)
- Terminal SSH client built-in
- Multiple server support
- Notifications/alerts
- Server groups and organization
- Dark mode support

**Setup**:
1. Install ServerCat from Mac App Store
2. Add server with SSH credentials (use Tailscale hostname)
3. Choose metrics to display in menu bar
4. Configure alert thresholds

**Pros**:
- Native macOS design (beautiful, fast)
- Always visible in menu bar
- No server-side installation needed (uses SSH)
- Includes file browser and terminal
- Push notifications for alerts
- iCloud sync across devices
- Widget support

**Cons**:
- **Paid**: $5.99 one-time purchase (or subscription for pro features)
- Requires SSH credentials stored in app
- Polling-based (not real-time push)
- Limited to SSH connection (needs SSH key or password)

**Resource Usage**:
- Server: Zero (SSH only)
- macOS: ~50MB RAM

**Tailscale Compatibility**: Perfect - just use Tailscale hostname for SSH

**Privacy**: SSH connections only, no cloud services except iCloud sync (optional)

**Recommendation**: **Best menu bar option** if you're willing to pay

**Alternatives**:
- **StatusCake** (free, less features)
- **Server Monitor** (similar, different UI)

---

### 2.2 Stats (macOS)

**Overview**: Open-source macOS menu bar monitoring for the LOCAL Mac.

**Important Note**: Stats monitors **your Mac**, not remote servers. Including here to clarify confusion - it's excellent for local monitoring but doesn't solve the remote Ubuntu monitoring need.

**Features** (for local Mac):
- CPU, GPU, RAM, disk, network, sensors
- Beautiful widgets
- Lightweight, open source
- Highly customizable

**Why Not Suitable**: No remote server monitoring capabilities

**Relevant Use**: Monitor the MacBook itself while using other tools for Ubuntu server

---

### 2.3 iStatistica

**Overview**: Premium system monitoring for macOS with remote capabilities.

**Platform**: macOS

**Features**:
- Comprehensive local Mac monitoring
- **Remote monitoring via iStatistica Server (separate app)**
- Dashboard view with graphs
- Notifications
- Historical data

**Remote Setup**:
1. Install iStatistica Server on Ubuntu (if available - primarily Mac-focused)
2. Configure connection in iStatistica app

**Limitations**:
- Primarily designed for monitoring macOS, not Linux
- iStatistica Server may not support Ubuntu well
- Limited documentation for Linux servers

**Cost**: ~$15-30 depending on version

**Recommendation**: Not ideal for Linux monitoring - better options exist

---

### 2.4 Custom Scripts with macOS Notifications

**Overview**: DIY approach using SSH + shell scripts + macOS notification system.

**Example Script** (`~/bin/check-server.sh`):
```bash
#!/bin/bash
SERVER="ubuntu-server.tail-scale.ts.net"

# Get stats via SSH
CPU=$(ssh $SERVER "top -bn1 | grep 'Cpu(s)' | awk '{print \$2}' | cut -d'%' -f1")
MEM=$(ssh $SERVER "free | grep Mem | awk '{printf \"%.0f\", \$3/\$2 * 100}'")
DISK=$(ssh $SERVER "df -h / | tail -1 | awk '{print \$5}' | cut -d'%' -f1")

# Alert if thresholds exceeded
if (( $(echo "$CPU > 80" | bc -l) )); then
    osascript -e "display notification \"CPU at ${CPU}%\" with title \"Server Alert\""
fi

if (( $MEM > 90 )); then
    osascript -e "display notification \"Memory at ${MEM}%\" with title \"Server Alert\""
fi

if (( $DISK > 85 )); then
    osascript -e "display notification \"Disk at ${DISK}%\" with title \"Server Alert\""
fi
```

**Setup as LaunchAgent** (`~/Library/LaunchAgents/com.homelab.servermonitor.plist`):
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.homelab.servermonitor</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/yourusername/bin/check-server.sh</string>
    </array>
    <key>StartInterval</key>
    <integer>300</integer>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
```

Load: `launchctl load ~/Library/LaunchAgents/com.homelab.servermonitor.plist`

**Pros**:
- Free, open source (your code)
- Completely customizable
- No server-side installation
- Native macOS notifications
- Works perfectly with Tailscale SSH

**Cons**:
- Requires scripting knowledge
- No GUI dashboard
- Manual maintenance
- No historical data (unless you add logging)

**Enhancement Ideas**:
- Log to file for history
- Use SwiftBar/BitBar for menu bar display
- Add Shortcuts integration
- Create Health Check style widget

**Recommendation**: Great for learning and customization enthusiasts

---

## 3. Desktop Dashboard Apps

### 3.1 Agent-Based Monitoring

Several platforms offer native desktop apps with server agents:

#### Datadog Agent (Free Tier)

**Overview**: Industry-standard monitoring platform with free tier.

**Setup**:
- Install Datadog Agent on Ubuntu
- Configure API key
- View metrics in desktop app or web dashboard

**Pros**:
- Professional-grade monitoring
- Beautiful native apps
- Extensive integrations
- Free tier available

**Cons**:
- Cloud-based (not self-hosted)
- Privacy concerns (data leaves your network)
- Overkill for single home server
- Free tier has limitations

**Recommendation**: **Not recommended** - violates self-hosted/privacy requirements

---

#### Netdata Cloud (with Desktop Client)

**Overview**: Netdata's cloud offering with desktop apps.

**Features**:
- Connect local Netdata instances to cloud
- Desktop app for monitoring
- Multi-server management

**Pros**:
- Leverages existing Netdata installation
- Optional (can use self-hosted only)
- Free tier generous

**Cons**:
- Requires cloud connection
- Desktop app mainly wraps web interface
- Not truly native experience

**Recommendation**: Use self-hosted Netdata web UI instead

---

### 3.2 SSH-Based Desktop Apps

#### Termius

**Overview**: Premium SSH client with server monitoring features.

**Platform**: macOS, Windows, Linux, iOS, Android

**Features**:
- Modern SSH client
- Server monitoring (CPU, RAM, disk)
- SFTP file transfer
- Snippet manager
- Terminal recording
- Team sharing

**Cost**: Free tier available, Pro ~$10/month

**Monitoring Capabilities**:
- Basic system stats in sidebar
- Runs commands over SSH
- No persistent monitoring

**Pros**:
- Beautiful, modern UI
- Cross-platform sync
- Excellent SSH client
- SFTP built-in

**Cons**:
- Monitoring is secondary feature
- Subscription cost
- Not as detailed as dedicated monitors

**Recommendation**: Consider if you need a premium SSH client anyway

---

#### Royal TSX

**Overview**: Connection manager for macOS with SSH, RDP, VNC support.

**Features**:
- Manage multiple server connections
- Embedded terminals
- Credential management
- Plugin system

**Monitoring**: Minimal - mainly connection management

**Cost**: Free with limitations, Pro ~$40 one-time

**Recommendation**: Not focused on monitoring - better alternatives exist

---

## 4. Comparison Matrix

| Solution | Type | Server Install | macOS Install | Real-time | Historical | Alerts | Cost | Resource (Server) |
|----------|------|----------------|---------------|-----------|-----------|---------|------|-------------------|
| **btop** | Terminal | Yes (tiny) | No | Yes | Session only | No | Free | <20MB |
| **htop** | Terminal | Yes (tiny) | No | Yes | No | No | Free | <5MB |
| **glances** (TUI) | Terminal | Yes | No | Yes | No | Yes | Free | ~50MB |
| **glances** (C/S) | Terminal | Yes | Yes | Yes | Export | Yes | Free | ~80MB |
| **gotop** | Terminal | Yes | No | Yes | No | No | Free | ~8MB |
| **ServerCat** | Menu Bar | No | Yes | Yes | Limited | Yes | $5.99 | 0 |
| **Custom Scripts** | DIY | No | Script | Polling | Optional | Yes | Free | 0 |
| **Termius** | Desktop | No | Yes | No | No | No | $10/mo | 0 |

---

## 5. Use Case Recommendations

### Scenario 1: Casual User, Occasional Checks
**Recommendation**: SSH + btop
- Just SSH in when you want to check
- Beautiful, informative TUI
- Zero overhead when not in use
- No installation on Mac needed

**Setup**:
```bash
ssh user@ubuntu-server.tail-scale.ts.net
btop
```

---

### Scenario 2: Want Menu Bar Visibility
**Recommendation**: ServerCat
- Always visible metrics in menu bar
- Native macOS experience
- Push notifications for issues
- Worth the $5.99 cost

**Alternative (Free)**: Custom script + BitBar/SwiftBar
- More work, full control
- Display custom metrics in menu bar

---

### Scenario 3: Power User, Frequent Monitoring
**Recommendation**: glances in client/server mode + btop for deep dives
- glances runs persistently with alerts
- Connect glances TUI client from Mac
- Drop into btop when you need detailed view
- Best of both worlds

**Setup**:
```bash
# Server (persistent)
sudo systemctl enable --now glances

# Mac client
glances -c ubuntu-server.tail-scale.ts.net
```

---

### Scenario 4: Want Notifications Without Menu Bar
**Recommendation**: Custom script via LaunchAgent
- Runs in background
- macOS notifications on thresholds
- Completely customizable
- Free and private

---

### Scenario 5: Multi-Server Home Lab
**Recommendation**: ServerCat or glances
- ServerCat for native UX with multiple servers
- glances for self-hosted multi-server monitoring
- Both handle multiple hosts well

---

## 6. Integration with Tailscale

### SSH-Based Solutions (btop, htop, glances)

**Configuration**: Just use Tailscale hostname
```bash
ssh user@ubuntu-server.tail-scale.ts.net
```

**Benefits**:
- SSH already encrypted, Tailscale adds node authentication
- No additional configuration needed
- Works exactly like local SSH

**Optimization**: Add SSH config (`~/.ssh/config`):
```
Host ubuntu-server
    HostName ubuntu-server.tail-scale.ts.net
    User youruser
    IdentityFile ~/.ssh/id_ed25519
    ServerAliveInterval 60
```

Then: `ssh ubuntu-server` followed by `btop`

---

### Menu Bar Apps (ServerCat)

**Setup**:
1. In ServerCat, add server
2. Use Tailscale hostname: `ubuntu-server.tail-scale.ts.net`
3. SSH port 22 (default)
4. Add SSH key or password

**Security**: ServerCat stores credentials in macOS Keychain

---

### Client/Server Tools (glances)

**Bind glances to Tailscale interface**:

Find Tailscale IP:
```bash
ip addr show tailscale0
# Example: 100.64.1.5
```

Start glances bound to Tailscale:
```bash
glances -s -B 100.64.1.5
```

**Persistent service** (`/etc/systemd/system/glances.service`):
```ini
[Unit]
Description=Glances
After=network.target tailscaled.service

[Service]
ExecStart=/usr/bin/glances -s -B 100.64.1.5
Restart=on-failure
User=glances

[Install]
WantedBy=multi-user.target
```

Enable: `sudo systemctl enable --now glances`

**Mac client**:
```bash
glances -c ubuntu-server.tail-scale.ts.net
```

---

## 7. Recommendations Summary

### Top 3 Overall

1. **btop over SSH** - Best balance of features, performance, zero overhead
2. **ServerCat** - Best native macOS experience, worth the cost
3. **glances (client/server)** - Best for persistent monitoring with alerts

### Decision Tree

```
Do you want persistent monitoring?
├─ No → btop over SSH (casual checks)
└─ Yes → Do you want GUI/menu bar?
    ├─ Yes → ServerCat ($5.99)
    └─ No → glances client/server (free, TUI)
```

### Resource Usage Comparison

**Lightest**: btop (zero when not running, <20MB when viewing)
**Best value**: ServerCat (0 server, 50MB Mac)
**Most features**: glances (80MB persistent)

### Complementary Approach

**Recommended combo**:
- **Primary**: btop for interactive monitoring (SSH on demand)
- **Background**: Custom script with notifications for alerts
- **Optional**: ServerCat if you want menu bar visibility

This gives you zero persistent overhead, instant alerts, and deep-dive capability when needed.

---

## 8. Next Steps

1. **Try btop first**: `ssh server && sudo apt install btop && btop`
2. **Set up SSH config** for easy access
3. **Optional**: Add alert script if you want notifications
4. **Optional**: Buy ServerCat if you want menu bar integration

For web-based alternatives, see R1 report. For unified solutions, see R5 report.
