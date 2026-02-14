# Syncing an Obsidian Vault Across Devices with Syncthing & Tailscale

A guide to making an Obsidian vault on a home server (Ubuntu Server) accessible and synced to a computer (macOS/Linux) and an iPhone — fully self-hosted, no cloud dependencies.

## Prerequisites

- A home server running **Ubuntu Server** with the vault folder
- A computer (macOS or Linux)
- An iPhone
- **Tailscale** installed and running on all three devices (all on the same Tailnet)
- Your server's Tailscale IP (e.g., `100.x.y.z`) — find it with `tailscale ip` on the server

---

## Step 1: Install Syncthing on the Server

SSH into your server and run:

```bash
sudo curl -o /usr/share/keyrings/syncthing-archive-keyring.gpg https://syncthing.net/release-key.gpg

echo "deb [signed-by=/usr/share/keyrings/syncthing-archive-keyring.gpg] https://apt.syncthing.net/ syncthing stable" | sudo tee /etc/apt/sources.list.d/syncthing.list

sudo apt update && sudo apt install syncthing
```

Run it once to generate config files:

```bash
syncthing
```

Press `Ctrl+C` after it finishes starting up, then enable it as a system service:

```bash
sudo systemctl enable --now syncthing@YOUR_USERNAME
```

Replace `YOUR_USERNAME` with your actual Linux username.

---

## Step 2: Access the Server's Syncthing Web UI

Since Ubuntu Server has no desktop environment, access the web UI from your computer using an SSH tunnel. Run in your computer's terminal:

```bash
ssh -L 8384:localhost:8384 YOUR_USERNAME@100.x.y.z
```
Where 8384 is the port of the Syncthing web UI and 100.x.y.z is the tailscale IP of the server.

Then open **http://localhost:8384** in your computer's browser.

### Alternative: Bind to Tailscale IP

If you prefer direct access without a tunnel:

1. Edit the config on the server:

```bash
nano ~/.local/state/syncthing/config.xml
```

2. In the `<gui>` section, change:

```xml
<address>127.0.0.1:8384</address>
```

to:

```xml
<address>0.0.0.0:8384</address>
```

3. Restart Syncthing:

```bash
sudo systemctl restart syncthing@YOUR_USERNAME
```

4. Access **http://100.x.y.z:8384** from any device on your Tailnet.
5. **Set a GUI password** when Syncthing prompts you — this is important since it's no longer localhost-only.

---

## Step 3: Install Syncthing on Your Computer

- **macOS:** Download from [syncthing.net](https://syncthing.net/) or install via Homebrew: `brew install syncthing`
- **Linux:** Same apt instructions as Step 1.

Start it and access the web UI at **http://localhost:8384**.

---

## Step 4: Install Syncthing on Your iPhone

Download **Möbius Sync** from the App Store. It is the main Syncthing client for iOS. It is free for one folder, or a one-time purchase of approximately $5 USD for unlimited folders.

---

## Step 5: Pair the Devices

Each Syncthing instance has a unique **Device ID**. You need to exchange these between all devices.

### Get Device IDs

- **Server & Computer:** In the web UI → **Actions** (top right) → **Show ID**
- **iPhone:** In Möbius Sync → tap your device name at the top to see the ID

### Add Remote Devices

On each device, add the other two:

1. Click/tap **Add Remote Device**
2. Paste the other device's ID
3. Give it a recognizable name (e.g., "Server", "MacBook", "iPhone")
4. Accept the incoming device request on the other side

Since all devices are on Tailscale, they should discover each other automatically.

---

## Step 6: Share the Vault Folder

### On the Server

1. In the Syncthing web UI, click **Add Folder**
2. Set **Folder Label** to something like `Obsidian Vault`
3. Set **Folder Path** to the full path of your vault (e.g., `/home/YOUR_USERNAME/obsidian/myvault`)
4. Go to the **Sharing** tab and check both your computer and iPhone
5. Click **Save**

### On Your Computer

1. A notification will appear in the web UI: "Server wants to share folder Obsidian Vault"
2. Click **Add** and choose a local path (e.g., `~/obsidian/myvault`)
3. The folder will begin syncing

### On Your iPhone (Möbius Sync)

1. A prompt will appear to accept the shared folder
2. Accept it — it will sync to a local directory on the phone

---

## Step 7: Create a .stignore File

To prevent sync conflicts from Obsidian's internal workspace state, create a `.stignore` file in the vault root **on the server**:

```bash
nano /home/YOUR_USERNAME/obsidian/myvault/.stignore
```

Add:

```
.obsidian/workspace.json
.obsidian/workspace-mobile.json
```

Save and exit. Syncthing will pick this up automatically. This file will not be synced to the other devices, so you need to create it manually on each device.

---

## Step 8: Open the Vault in Obsidian

### On Your Computer

1. Open Obsidian
2. Click **Open folder as vault**
3. Navigate to the synced folder (e.g., `~/obsidian/myvault`)

### On Your iPhone

1. Open Obsidian
2. Tap **Create new vault**
3. **Toggle "Store in iCloud" OFF**
4. Name the vault (e.g., `myvault`)
5. In Möbius Sync settings, configure the vault folder to sync directly into the Obsidian local storage path: **On My iPhone → Obsidian → myvault**

This way Möbius Sync writes directly into Obsidian's sandbox and you don't need to manually move files around.

---

## Verification

Once everything is set up:

1. Create a test note on one device
2. Wait a few seconds
3. Confirm it appears on the other devices

Syncthing syncs in near real-time over your Tailscale network. No internet required — it works entirely over your local Tailnet.

---

## Troubleshooting

- **Devices not connecting:** Verify Tailscale is running on all devices (`tailscale status`). Ensure Syncthing is running (`sudo systemctl status syncthing@YOUR_USERNAME`).
- **Sync conflicts:** If the same file is edited on two devices simultaneously, Syncthing creates a `.sync-conflict` copy. Manually merge and delete the duplicate.
- **Möbius Sync not syncing in background:** iOS aggressively suspends background apps. Open Möbius Sync periodically, or before opening Obsidian, to trigger a sync.
