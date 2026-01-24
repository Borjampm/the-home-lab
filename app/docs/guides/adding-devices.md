# Adding Devices to the Home Lab

This guide covers how to add a new machine to the home lab network so it's accessible from the Control Center's "Take control" feature without password prompts.

## Overview

The home lab uses **Tailscale SSH** for authentication. Instead of managing SSH keys across machines, Tailscale handles identity verification using your Tailscale account. When you click "Take control" on a device in the dashboard, you're dropped directly into a remote shell — no passwords, no key copying.

## Prerequisites

- A Tailscale account with an existing tailnet
- The new device must run **Linux** (Tailscale SSH acts as the SSH server)
- Admin access to your [Tailscale admin console](https://login.tailscale.com/admin)

## Step 1: Install Tailscale on the New Device

```bash
# Debian/Ubuntu
curl -fsSL https://tailscale.com/install.sh | sh

# Or follow the instructions for your distro:
# https://tailscale.com/download/linux
```

## Step 2: Join the Tailnet with SSH Enabled

```bash
sudo tailscale up --ssh
```

The `--ssh` flag tells Tailscale to accept SSH connections authenticated via Tailscale identity. This replaces the need for `authorized_keys` or password auth.

The device will appear in your Tailscale admin console and in the Control Center dashboard after a refresh.

## Step 3: Configure ACLs (First Time Only)

In your [Tailscale ACL policy](https://login.tailscale.com/admin/acls), ensure SSH access is allowed. Add an SSH rule if one doesn't exist:

```json
{
  "ssh": [
    {
      "action": "accept",
      "src": ["autogroup:member"],
      "dst": ["autogroup:self"],
      "users": ["autogroup:nonroot"]
    }
  ]
}
```

This allows any member of your tailnet to SSH into any device as any non-root user. Adjust `src`, `dst`, and `users` to match your security preferences.

**Note**: You only need to configure ACLs once. New devices added with `--ssh` will inherit the existing policy.

## Step 4: Verify from the Control Center

1. Open the Control Center
2. Click **Refresh** — the new device should appear in the device list
3. Click **Take control** on the device
4. You should land in a remote shell with no password prompt

## How It Works

When you click "Take control", the app runs `ssh <tailscale_ip>` in a PTY. On the remote device, Tailscale's SSH server intercepts the connection and verifies the client's Tailscale identity instead of checking SSH keys or passwords. This means:

- No SSH keys to generate or distribute
- No `~/.ssh/authorized_keys` to maintain
- New devices work immediately after `tailscale up --ssh`
- Revoking access is done from the Tailscale admin console

## Removing a Device

To remove a device from the network:

1. On the device: `sudo tailscale down && sudo tailscale logout`
2. In the [admin console](https://login.tailscale.com/admin/machines): remove the machine

## Troubleshooting

### "Take control" still asks for a password

- Verify the remote device was started with `--ssh`: run `tailscale status` on it and check that SSH is enabled
- If it was started without `--ssh`, re-run: `sudo tailscale up --ssh`
- Check that your ACL policy includes an `ssh` rule

### Device appears offline in dashboard

- Confirm Tailscale is running on the device: `tailscale status`
- Check network connectivity on the device
- Click **Refresh** in the Control Center

### Permission denied (non-root user)

- The ACL rule above allows `autogroup:nonroot` users. If you need to connect as a specific user, update the `users` field in the ACL or use `ssh user@<ip>` by adjusting the command format

## See Also

- [components/tailscale.md](../components/tailscale.md) — Tailscale integration architecture
- [components/terminal.md](../components/terminal.md) — Terminal and PTY system
- [Tailscale SSH documentation](https://tailscale.com/kb/1193/tailscale-ssh)
