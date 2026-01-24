use std::collections::HashMap;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CliStatus {
    backend_state: String,
    #[serde(rename = "Self")]
    self_node: CliPeer,
    #[serde(default)]
    peer: HashMap<String, CliPeer>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CliPeer {
    #[serde(default)]
    host_name: String,
    #[serde(default, rename = "DNSName")]
    dns_name: String,
    #[serde(default, rename = "OS")]
    os: String,
    #[serde(default, rename = "TailscaleIPs")]
    tailscale_ips: Vec<String>,
    #[serde(default)]
    online: bool,
    #[serde(default)]
    relay: String,
    #[serde(default)]
    last_seen: Option<String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TailscaleDevice {
    pub hostname: String,
    pub dns_name: String,
    pub os: String,
    pub ips: Vec<String>,
    pub online: bool,
    pub is_self: bool,
    pub relay: String,
    pub last_seen: Option<String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TailscaleStatus {
    pub connected: bool,
    pub devices: Vec<TailscaleDevice>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn get_tailscale_status() -> TailscaleStatus {
    let output = match Command::new("tailscale").args(["status", "--json"]).output() {
        Ok(o) => o,
        Err(e) => {
            return TailscaleStatus {
                connected: false,
                devices: vec![],
                error: Some(format!("Failed to run `tailscale status --json`: {}", e)),
            };
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return TailscaleStatus {
            connected: false,
            devices: vec![],
            error: Some(format!("tailscale exited with error: {}", stderr.trim())),
        };
    }

    let status: CliStatus = match serde_json::from_slice(&output.stdout) {
        Ok(s) => s,
        Err(e) => {
            return TailscaleStatus {
                connected: false,
                devices: vec![],
                error: Some(format!("Failed to parse tailscale output: {}", e)),
            };
        }
    };

    let connected = status.backend_state == "Running";

    let mut devices = Vec::new();

    devices.push(TailscaleDevice {
        hostname: status.self_node.host_name,
        dns_name: status.self_node.dns_name,
        os: status.self_node.os,
        ips: status.self_node.tailscale_ips,
        online: status.self_node.online,
        is_self: true,
        relay: status.self_node.relay,
        last_seen: status.self_node.last_seen,
    });

    for (_key, peer) in status.peer {
        devices.push(TailscaleDevice {
            hostname: peer.host_name,
            dns_name: peer.dns_name,
            os: peer.os,
            ips: peer.tailscale_ips,
            online: peer.online,
            is_self: false,
            relay: peer.relay,
            last_seen: peer.last_seen,
        });
    }

    devices.sort_by(|a, b| {
        b.is_self
            .cmp(&a.is_self)
            .then(b.online.cmp(&a.online))
            .then(a.hostname.cmp(&b.hostname))
    });

    TailscaleStatus {
        connected,
        devices,
        error: None,
    }
}

#[tauri::command]
pub async fn tailscale_up() -> Result<(), String> {
    let output = Command::new("tailscale")
        .args(["up"])
        .output()
        .map_err(|e| format!("Failed to run `tailscale up`: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("tailscale up failed: {}", stderr.trim()));
    }
    Ok(())
}

#[tauri::command]
pub async fn tailscale_down() -> Result<(), String> {
    let output = Command::new("tailscale")
        .args(["down"])
        .output()
        .map_err(|e| format!("Failed to run `tailscale down`: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("tailscale down failed: {}", stderr.trim()));
    }
    Ok(())
}
