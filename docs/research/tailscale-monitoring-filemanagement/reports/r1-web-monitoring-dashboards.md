# R1: Web-based Hardware Monitoring Solutions

**Research Question:** What are the best web-based dashboards for hardware monitoring on Ubuntu servers accessible via Tailscale?

**Priority:** P0
**Status:** Complete
**Completed:** 2026-01-21

---

## Executive Summary

Web-based monitoring solutions provide real-time hardware statistics through a browser interface, ideal for Tailscale's secure tunnel. This research evaluates five major solutions: Netdata, Glances, Grafana+Prometheus, Cockpit, and Glances Web Mode.

**Top Recommendations:**
1. **Netdata** - Best for ease of setup and comprehensive metrics
2. **Glances Web** - Best for lightweight, minimal setup
3. **Cockpit** - Best for unified system management (includes file management)

---

## Solution Analysis

### 1. Netdata

**Overview:** Real-time performance monitoring with thousands of metrics, auto-detection, and beautiful web UI.

**Pros:**
- Zero-configuration installation - works immediately after install
- Extremely comprehensive metrics (CPU, RAM, disk, network, processes, containers, etc.)
- Beautiful, responsive web interface
- Very low resource overhead (~1-3% CPU, 100-200MB RAM)
- Per-second granularity for real-time monitoring
- Built-in alarms and notifications
- No database required - uses memory for recent data, disk for historical
- Active community and regular updates

**Cons:**
- More resource-intensive than Glances (but still lightweight)
- Default setup exposes all metrics (may be too much information)
- Web UI can be overwhelming for simple needs
- Historical data retention limited without Netdata Cloud (optional paid service)

**Security Considerations:**
- Bind to Tailscale IP only: `[web] bind to = 100.x.x.x:19999`
- Tailscale encryption handles transport security
- Consider basic auth if desired (though Tailscale ACLs may be sufficient)

**Resource Usage:**
- CPU: 1-3% on average
- RAM: 100-200MB (depends on retention period)
- Disk: Minimal (logs and some state)

**Installation:**
```bash
# Ubuntu/Debian one-liner
wget -O /tmp/netdata-kickstart.sh https://get.netdata.cloud/kickstart.sh && sh /tmp/netdata-kickstart.sh --stable-channel --disable-telemetry

# Access at: http://[tailscale-ip]:19999
```

**Configuration for Tailscale:**
Edit `/etc/netdata/netdata.conf`:
```ini
[web]
    bind to = 100.x.x.x:19999
```

**Sources:**
- Official Documentation: https://learn.netdata.cloud/
- GitHub: https://github.com/netdata/netdata
- Resource usage benchmarks from community testing

---

### 2. Glances (Web Mode)

**Overview:** Cross-platform monitoring tool with a web server mode, written in Python.

**Pros:**
- Extremely lightweight (< 50MB RAM, <1% CPU)
- Simple, clean web interface
- Single Python package installation
- Excellent for resource-constrained servers
- Can export to various formats (JSON, CSV, etc.)
- Built-in API for custom integrations
- Responsive design works well on mobile

**Cons:**
- Less comprehensive than Netdata (focuses on essentials)
- Web UI is more utilitarian than beautiful
- Limited historical data (real-time focus)
- No built-in alerting
- Requires Python environment

**Security Considerations:**
- Bind to Tailscale IP: `glances -w --bind 100.x.x.x`
- Can add password protection: `glances -w --password`
- Already secure over Tailscale tunnel

**Resource Usage:**
- CPU: <1% on average
- RAM: 30-50MB
- Disk: Minimal

**Installation:**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install glances

# Or via pip for latest version
pip install glances[web]

# Run web server
glances -w --bind 100.x.x.x

# Access at: http://[tailscale-ip]:61208
```

**Best For:**
- Minimal resource footprint needed
- Simple, straightforward monitoring
- Servers with limited RAM (<2GB)

**Sources:**
- GitHub: https://github.com/nicolargo/glances
- Documentation: https://glances.readthedocs.io/

---

### 3. Grafana + Prometheus

**Overview:** Industry-standard monitoring stack with Prometheus for metrics collection and Grafana for visualization.

**Pros:**
- Extremely powerful and flexible
- Beautiful, customizable dashboards
- Excellent for complex setups and scaling
- Rich ecosystem of exporters for any service
- Advanced querying with PromQL
- Professional-grade alerting
- Long-term data retention with configurable resolution

**Cons:**
- Significant complexity for simple use case
- Higher resource usage (500MB-1GB RAM minimum)
- Requires multiple services (Prometheus + Node Exporter + Grafana)
- Steeper learning curve
- Overkill for single-server monitoring

**Security Considerations:**
- Bind each service to Tailscale IP
- Grafana has built-in authentication
- Prometheus should be firewalled (only Grafana needs access)

**Resource Usage:**
- CPU: 2-5% on average
- RAM: 500MB-1GB combined
- Disk: Depends on retention (few GB typical)

**Installation:**
```bash
# Install Prometheus
wget https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz
# ... setup as service

# Install Node Exporter
wget https://github.com/prometheus/node_exporter/releases/download/v1.6.1/node_exporter-1.6.1.linux-amd64.tar.gz
# ... setup as service

# Install Grafana
sudo apt-get install -y software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
sudo apt-get update
sudo apt-get install grafana

# Access Grafana at: http://[tailscale-ip]:3000
```

**Best For:**
- Users who want to learn industry-standard monitoring
- Plans to monitor multiple servers/services
- Need for advanced alerting and dashboards
- Long-term metrics storage and analysis

**Sources:**
- Prometheus Documentation: https://prometheus.io/docs/
- Grafana Documentation: https://grafana.com/docs/
- Node Exporter: https://github.com/prometheus/node_exporter

---

### 4. Cockpit

**Overview:** Web-based server management interface built by Red Hat, included in many Linux distributions.

**Pros:**
- Comprehensive system management (not just monitoring)
- Built-in file manager (addresses both requirements!)
- Terminal access via web
- Service management (systemd)
- Software updates via web UI
- Network configuration
- User management
- Very low resource usage
- Native Ubuntu package

**Cons:**
- UI less modern than Netdata
- Monitoring less detailed than dedicated tools
- More focused on administration than pure monitoring
- Real-time updates slower (several seconds refresh)

**Security Considerations:**
- Built-in authentication required
- Uses systemd for privilege management
- Bind to Tailscale IP in configuration
- TLS certificate recommendations

**Resource Usage:**
- CPU: <1% (on-demand)
- RAM: 50-100MB
- Disk: Minimal

**Installation:**
```bash
# Ubuntu
sudo apt update
sudo apt install cockpit

# Enable and start
sudo systemctl enable --now cockpit.socket

# Access at: https://[tailscale-ip]:9090
```

**Configuration for Tailscale:**
Edit `/etc/systemd/system/cockpit.socket.d/listen.conf`:
```ini
[Socket]
ListenStream=
ListenStream=100.x.x.x:9090
```

**Best For:**
- Users wanting both monitoring AND file management
- System administration tasks beyond monitoring
- Those comfortable with more traditional admin UIs

**Sources:**
- Official Documentation: https://cockpit-project.org/documentation.html
- GitHub: https://github.com/cockpit-project/cockpit

---

### 5. Uptime Kuma

**Overview:** Self-hosted monitoring tool focused on uptime and availability rather than detailed metrics.

**Pros:**
- Beautiful, modern UI
- Easy to set up
- Good for monitoring service availability
- Status pages
- Multi-user support

**Cons:**
- Not designed for hardware metrics (wrong tool for this job)
- Focused on HTTP/TCP/ping monitoring
- Would need additional exporters for system metrics

**Verdict:** Not recommended for this use case - better suited for monitoring web services.

---

## Comparison Matrix

| Solution | Setup Time | Resource Usage | Features | UI Quality | Maintenance |
|----------|------------|----------------|----------|------------|-------------|
| **Netdata** | 5 min | Medium-Low | Excellent | Excellent | Very Low |
| **Glances** | 2 min | Very Low | Good | Good | Very Low |
| **Grafana+Prometheus** | 30-60 min | Medium-High | Excellent+ | Excellent | Medium |
| **Cockpit** | 5 min | Low | Good+ | Good | Very Low |

---

## Recommendations by Use Case

### Recommended: Netdata
**Best for most users** who want:
- Comprehensive metrics out of the box
- Modern, intuitive interface
- Minimal setup and maintenance
- Balance of features and resources

**Setup Priority:** Install first, evaluate if it meets needs

---

### Alternative: Glances Web
**Best for minimal servers** (<2GB RAM) who want:
- Absolute minimum resource usage
- Simple, focused monitoring
- Quick setup
- Just the essentials

---

### Alternative: Cockpit
**Best for unified solution** if you want:
- Both monitoring AND file management
- System administration capabilities
- Traditional admin interface
- One tool for multiple needs

---

### Not Recommended (for this use case): Grafana+Prometheus
Only consider if:
- You specifically want to learn these enterprise tools
- Planning to expand to multi-server monitoring
- Need advanced alerting and long-term analytics
- Don't mind the complexity overhead

---

## Tailscale-Specific Considerations

### Network Binding
Always bind web services to your Tailscale IP to prevent exposure on public interfaces:
- Netdata: Edit `/etc/netdata/netdata.conf`
- Glances: Use `--bind` flag
- Grafana: Edit `/etc/grafana/grafana.ini`
- Cockpit: Edit socket configuration

### Authentication
Given Tailscale's strong device authentication:
- Basic/password auth is optional for single-user setups
- Tailscale ACLs provide network-level access control
- Still recommended for services like Grafana with built-in auth
- Cockpit requires authentication (good security practice)

### Performance
- Tailscale's WireGuard encryption has minimal overhead
- Web UIs work well over Tailscale (local network speeds)
- No special configuration needed for performance

---

## Implementation Guidance

### Quick Start Path
1. Install Netdata (5 minutes)
2. Configure to bind to Tailscale IP
3. Access from MacBook browser
4. Evaluate if it meets your needs
5. If too heavy, try Glances instead

### Unified Solution Path
1. Install Cockpit (5 minutes)
2. Access via HTTPS on port 9090
3. Use built-in monitoring + file manager
4. Evaluate if sufficient or if dedicated tools needed

### Learning/Enterprise Path
1. Set up Prometheus + Node Exporter
2. Install Grafana
3. Configure data sources
4. Import pre-built dashboards
5. Customize as needed

---

## Conclusion

**Primary Recommendation: Netdata**
- Optimal balance of features, ease of use, and resource efficiency
- Zero-configuration monitoring
- Perfect for home lab use cases
- Works beautifully over Tailscale

**Secondary Recommendation: Glances**
- If Netdata feels too heavy
- Minimum viable monitoring solution
- Extremely lightweight

**Unified Option: Cockpit**
- Consider if you want one tool for monitoring + file management
- Trade-off: Less detailed monitoring but broader functionality

All three solutions work excellently over Tailscale with minimal configuration.
