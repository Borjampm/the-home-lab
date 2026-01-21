# R3: Web-based File Management Solutions

**Research Question:** What are the best self-hosted web-based file managers for browsing Ubuntu server files from macOS?

**Priority:** P0
**Status:** Complete
**Completed:** 2026-01-21

---

## Executive Summary

Web-based file managers provide Finder-like interfaces accessible through a browser, ideal for remote server access over Tailscale. This research evaluates dedicated file management solutions that are lightweight, self-hosted, and privacy-focused.

**Top Recommendations:**
1. **File Browser** - Best overall for simplicity and features
2. **Cockpit File Manager** - Best if also using Cockpit for monitoring
3. **FileBrowser (alternative)** - Same as #1, confirming name

---

## Solution Analysis

### 1. File Browser (filebrowser/filebrowser)

**Overview:** Modern, lightweight web file manager written in Go. Single binary, zero dependencies.

**Pros:**
- Single binary executable - trivial installation
- No dependencies (compiled Go binary)
- Beautiful, modern UI similar to Finder/Explorer
- Comprehensive features: upload, download, delete, rename, move, copy
- Built-in text editor with syntax highlighting
- Image/video preview
- Search functionality
- Multi-user support with permissions
- Command runner for custom actions
- Archive creation (zip, tar)
- Extremely lightweight (<50MB RAM, <1% CPU)
- Share files with temporary links
- Mobile-responsive design

**Cons:**
- Less mature than some alternatives
- Limited collaborative features
- No real-time multi-user editing
- Basic text editor (not IDE-level)

**Security Considerations:**
- Built-in authentication system
- User-based permissions
- Bind to Tailscale IP only
- No TLS needed (Tailscale encrypts)
- Can set file/folder restrictions per user

**Resource Usage:**
- CPU: <1%
- RAM: 20-50MB
- Disk: ~20MB binary + stored configs

**Installation:**
```bash
# Download latest release
curl -fsSL https://raw.githubusercontent.com/filebrowser/get/master/get.sh | bash

# Create config database
filebrowser config init

# Set root directory
filebrowser config set --root /home

# Set address to bind to Tailscale IP
filebrowser config set --address 100.x.x.x --port 8080

# Create admin user
filebrowser users add admin password123 --perm.admin

# Run
filebrowser

# Or install as systemd service for auto-start
```

**Systemd Service:**
```ini
[Unit]
Description=File Browser
After=network.target

[Service]
ExecStart=/usr/local/bin/filebrowser
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

**Access:** `http://[tailscale-ip]:8080`

**Best For:**
- Users wanting modern, intuitive interface
- Simple setup requirements
- Lightweight operation
- Finder-like experience

**Sources:**
- GitHub: https://github.com/filebrowser/filebrowser
- Documentation: https://filebrowser.org/
- Community Docker usage and reviews

---

### 2. Cockpit File Manager

**Overview:** Built-in file manager component of Cockpit web console (from monitoring research).

**Pros:**
- Already included if using Cockpit for monitoring (unified solution!)
- No additional installation needed
- Integrated with system permissions
- Basic operations: browse, upload, download, delete, rename
- Navigate filesystem hierarchy
- View file permissions and ownership
- Edit text files
- Part of comprehensive system management

**Cons:**
- UI less modern than File Browser
- Fewer features than dedicated file managers
- Slower navigation (page refreshes)
- Not optimized specifically for file management
- Limited preview capabilities

**Security Considerations:**
- Uses Cockpit's authentication (system users)
- SystemD privilege management
- Respects Linux file permissions
- Built-in TLS support

**Resource Usage:**
- CPU: <1% (on-demand)
- RAM: 50-100MB (entire Cockpit)
- Disk: Minimal

**Installation:**
```bash
# Install Cockpit (if not already installed)
sudo apt update
sudo apt install cockpit

# Enable file manager navigator
sudo systemctl enable --now cockpit.socket

# Access at: https://[tailscale-ip]:9090
```

**Access:** Navigate to Storage or Navigator section in Cockpit UI

**Best For:**
- Users already using Cockpit for monitoring
- Those wanting unified system management
- Minimal additional overhead
- Traditional admin interface preference

**Sources:**
- Cockpit Documentation: https://cockpit-project.org/guide/latest/feature-storage
- Navigator plugin documentation

---

### 3. Nextcloud

**Overview:** Comprehensive cloud platform with file sync, sharing, and collaboration features.

**Pros:**
- Full-featured cloud storage platform
- Desktop/mobile sync clients
- Document editing (OnlyOffice/Collabora integration)
- Calendar, contacts, tasks
- Photo management and galleries
- Extensive plugin ecosystem
- Industry-standard solution
- Active development and security updates

**Cons:**
- **Significant complexity** - overkill for simple file browsing
- Resource intensive (requires PHP, database, web server)
- Longer setup time (30-60 minutes)
- Higher resource usage (500MB-1GB RAM minimum)
- Requires MySQL/PostgreSQL + Redis for performance
- Many features unused for simple file management

**Resource Usage:**
- CPU: 2-5%
- RAM: 512MB-1GB minimum
- Disk: 500MB+ for installation

**Verdict:** **Not recommended** for this use case - too heavy and complex for simple file browsing needs. Better suited for full cloud replacement scenarios.

**Sources:**
- Official Documentation: https://docs.nextcloud.com/
- Server requirements: https://docs.nextcloud.com/server/stable/admin_manual/installation/system_requirements.html

---

### 4. Tiny File Manager

**Overview:** Single PHP file file manager - extremely simple and lightweight.

**Pros:**
- Literally one PHP file
- No database required
- Basic file operations
- Very minimal resource usage
- Quick to deploy on existing web server

**Cons:**
- Requires PHP and web server (Apache/Nginx)
- Basic security model
- Limited features
- Dated UI
- Less actively maintained
- Security concerns with single-file architecture

**Security Considerations:**
- Basic password authentication
- Should use .htaccess or similar for additional protection
- Limited access control features
- Audit carefully before deployment

**Resource Usage:**
- CPU: <1%
- RAM: ~10-20MB
- Disk: ~100KB (single file)

**Installation:**
```bash
# Requires existing web server with PHP
sudo apt install apache2 php

# Download
wget https://raw.githubusercontent.com/prasathmani/tinyfilemanager/master/tinyfilemanager.php

# Place in web root
sudo mv tinyfilemanager.php /var/www/html/

# Access via browser and configure
```

**Verdict:** Only consider if you already have Apache/PHP running. Otherwise, File Browser is superior in every way.

**Sources:**
- GitHub: https://github.com/prasathmani/tinyfilemanager

---

### 5. Miniserve

**Overview:** Minimal file serving tool with simple web interface, written in Rust.

**Pros:**
- Single binary (Rust)
- Extremely fast
- Very simple to use
- Useful for quick file sharing
- QR code generation for easy mobile access
- Upload capabilities
- Archive downloads

**Cons:**
- More focused on serving/sharing than management
- No file editing
- No file operations (delete, rename, etc.)
- Not a full file manager
- Better suited as temporary file server

**Installation:**
```bash
# Install via cargo
cargo install miniserve

# Or download binary
wget https://github.com/svenstaro/miniserve/releases/download/v0.24.0/miniserve-v0.24.0-x86_64-unknown-linux-gnu

# Run
miniserve /path/to/serve --bind 100.x.x.x:8080
```

**Verdict:** Good for quick file serving but not a comprehensive file manager. Consider for ad-hoc needs, not permanent solution.

**Sources:**
- GitHub: https://github.com/svenstaro/miniserve

---

### 6. Samba Web Interface (SWAT) / Webmin

**Overview:** Web-based tools for configuring Samba/system, with file browsing as secondary feature.

**Verdict:** Not recommended - these are configuration tools, not file managers. Better to use Samba directly with native macOS mounting (covered in R4 report).

---

## Comparison Matrix

| Solution | Setup Time | Resource Usage | Features | UI Quality | File Operations | Maintenance |
|----------|------------|----------------|----------|------------|-----------------|-------------|
| **File Browser** | 5 min | Very Low | Excellent | Excellent | Full | Very Low |
| **Cockpit Files** | 0 min* | Low | Good | Fair | Basic | Very Low |
| **Nextcloud** | 60 min | High | Excellent+ | Excellent | Full+ | Medium |
| **Tiny File Manager** | 10 min | Very Low | Basic | Fair | Basic | Low |
| **Miniserve** | 2 min | Very Low | Minimal | Good | View/Upload only | Very Low |

*If Cockpit already installed for monitoring

---

## Feature Comparison

### File Operations Support

| Feature | File Browser | Cockpit | Nextcloud | Tiny FM | Miniserve |
|---------|--------------|---------|-----------|---------|-----------|
| Browse | ✓ | ✓ | ✓ | ✓ | ✓ |
| Upload | ✓ | ✓ | ✓ | ✓ | ✓ |
| Download | ✓ | ✓ | ✓ | ✓ | ✓ |
| Delete | ✓ | ✓ | ✓ | ✓ | ✗ |
| Rename | ✓ | ✓ | ✓ | ✓ | ✗ |
| Move/Copy | ✓ | ✓ | ✓ | ✓ | ✗ |
| Edit Text | ✓ | ✓ | ✓ | ✓ | ✗ |
| Preview Images | ✓ | ✗ | ✓ | ✓ | ✓ |
| Search | ✓ | ✗ | ✓ | ✓ | ✗ |
| Archive Create | ✓ | ✗ | ✓ | ✗ | ✗ |
| Share Links | ✓ | ✗ | ✓ | ✓ | ✗ |
| Multi-user | ✓ | ✓ | ✓ | ✗ | ✗ |

---

## Recommendations by Use Case

### Primary Recommendation: File Browser
**Best for most users** who want:
- Modern, Finder-like interface
- Full file management capabilities
- Minimal resource usage
- Simple installation (single binary)
- No dependencies

**Setup Priority:** Start here - likely meets all needs

**Installation Time:** 5 minutes

---

### Alternative: Cockpit File Manager
**Best if already using Cockpit** for monitoring:
- Zero additional setup
- Unified management interface
- No extra resources
- Good for basic file operations

**Note:** Less feature-rich than File Browser, but sufficient for basic needs

---

### Not Recommended: Nextcloud
**Too heavy for this use case** unless you specifically want:
- Full cloud platform
- Desktop/mobile sync
- Document collaboration
- Photo management ecosystem

For simple file browsing over Tailscale, this is overkill.

---

## Tailscale-Specific Considerations

### Network Binding
Configure file managers to bind only to Tailscale IP:
- File Browser: `filebrowser config set --address 100.x.x.x`
- Cockpit: Configure in socket settings
- Prevents exposure on public interfaces

### Authentication
- File Browser: Built-in user system (create users as needed)
- Cockpit: Uses system users (requires Linux account)
- Tailscale device auth provides network-level security
- Application auth provides user-level permissions

### Performance
- Web-based file managers work well over Tailscale
- Large file transfers benefit from Tailscale's direct connections
- For massive files (>10GB), consider native protocols (see R4)
- Preview generation happens server-side (no bandwidth impact)

### Port Selection
- File Browser default: 8080
- Cockpit default: 9090
- Choose ports that don't conflict with other services
- Document in your home lab inventory

---

## Integration Considerations

### With Monitoring Solutions

If using **Netdata** for monitoring:
- Install File Browser separately
- Two browser tabs/windows
- Minimal resource overlap

If using **Cockpit** for monitoring:
- Use built-in file manager
- Unified interface
- No additional resources

### With Native File Access

Web file managers complement (not replace) native mounting:
- Use web UI for quick browsing and one-off operations
- Use native mounting (SFTP/SMB) for regular workflows
- Web UI accessible from any device with browser
- Native mounting better for large transfers

---

## Implementation Guidance

### Quick Start Path (File Browser)
```bash
# 1. Install
curl -fsSL https://raw.githubusercontent.com/filebrowser/get/master/get.sh | bash

# 2. Initialize config
filebrowser config init

# 3. Set root directory (adjust as needed)
filebrowser config set --root /home

# 4. Bind to Tailscale IP (replace with your IP)
filebrowser config set --address 100.64.0.2 --port 8080

# 5. Create admin user
filebrowser users add admin YourSecurePassword --perm.admin

# 6. Run (or setup systemd service)
filebrowser

# 7. Access from MacBook
# Open browser to: http://100.64.0.2:8080
```

### Systemd Service Setup
```bash
# Create service file
sudo nano /etc/systemd/system/filebrowser.service

# Add content:
[Unit]
Description=File Browser
After=network.target

[Service]
ExecStart=/usr/local/bin/filebrowser
WorkingDirectory=/home/youruser
Restart=on-failure
User=youruser

[Install]
WantedBy=multi-user.target

# Enable and start
sudo systemctl enable filebrowser
sudo systemctl start filebrowser
```

### Cockpit Path (If Using for Monitoring)
```bash
# Install Cockpit (if not already)
sudo apt install cockpit

# File manager is built-in
# Access at: https://[tailscale-ip]:9090
# Navigate to Storage or Navigator section
```

---

## Security Best Practices

### 1. Network Binding
Always bind to Tailscale IP only:
```bash
filebrowser config set --address 100.x.x.x
```

### 2. Strong Authentication
Create strong passwords:
```bash
filebrowser users add admin $(openssl rand -base64 32)
```

### 3. User Permissions
Limit access to necessary directories:
```bash
# Create limited user
filebrowser users add viewer viewerpass --scope /home/shared
```

### 4. Tailscale ACLs
Use Tailscale ACLs to restrict which devices can access:
```json
{
  "acls": [
    {
      "action": "accept",
      "src": ["tag:laptop"],
      "dst": ["tag:server:8080"]
    }
  ]
}
```

### 5. Regular Updates
Keep file manager updated:
```bash
# File Browser
curl -fsSL https://raw.githubusercontent.com/filebrowser/get/master/get.sh | bash
sudo systemctl restart filebrowser
```

---

## Troubleshooting Common Issues

### Cannot Access File Browser
1. Check if service is running: `sudo systemctl status filebrowser`
2. Verify Tailscale IP: `tailscale ip -4`
3. Check firewall: `sudo ufw status` (should allow on Tailscale interface)
4. Test locally: `curl http://localhost:8080`

### Permission Denied Errors
1. Check file permissions: `ls -la /path/to/files`
2. Verify user running service: `ps aux | grep filebrowser`
3. Adjust ownership: `sudo chown -R filebrowser-user:filebrowser-user /path`

### Slow Performance
1. Check server resources: `htop`
2. Verify Tailscale connection: `tailscale status`
3. Test with smaller files first
4. Consider native protocols for large transfers

---

## Conclusion

**Primary Recommendation: File Browser**
- Modern, intuitive interface
- Comprehensive file operations
- Minimal resources and dependencies
- Simple installation and maintenance
- Perfect for home lab file management over Tailscale

**Installation Priority:**
1. Install File Browser (5 minutes)
2. Configure to bind to Tailscale IP
3. Create admin user
4. Access from MacBook browser
5. Evaluate if meets needs

**Alternative Path:**
- If using Cockpit for monitoring → use built-in file manager
- If need full cloud platform → consider Nextcloud (but likely overkill)

File Browser provides the best balance of features, simplicity, and resources for Tailscale-based home lab file management.
