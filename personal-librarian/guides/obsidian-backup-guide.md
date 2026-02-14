# Encrypted Obsidian Vault Backup System

A guide to building an automated, encrypted backup system for an Obsidian vault on an Ubuntu Server. Backups are pushed to an external HDD (mounted only during backup) and cloud storage via rclone.

## Prerequisites

- Ubuntu Server with your Obsidian vault
- An external HDD (will be formatted — all data on it will be lost)
- A Backblaze B2 account (free tier) or Google Drive account

---

## Part 1: Setting Up the External HDD

### 1.1 Identify the Drive

Plug the HDD into the server and find it:

```bash
lsblk
```

Look for the new drive — it will be something like `/dev/sdb`. Verify the size matches your HDD to make sure you don't format the wrong drive.

For extra confirmation:

```bash
sudo fdisk -l /dev/sdb
```

> **WARNING:** Double-check the device name. Formatting the wrong drive will destroy data.

### 1.2 Partition the Drive

```bash
sudo fdisk /dev/sdb
```

Inside fdisk:

1. Press `g` to create a new GPT partition table
2. Press `n` to create a new partition (accept all defaults for a single full-disk partition)
3. Press `w` to write and exit

### 1.3 Format the Partition

Use ext4 — reliable, well-supported, and handles power loss gracefully:

```bash
sudo mkfs.ext4 -L "backups" /dev/sdb1
```

The `-L "backups"` sets a label so you can mount by label instead of device name (device names can shift between reboots).

### 1.4 Create the Mount Point

```bash
sudo mkdir -p /mnt/backup-hdd
```

### 1.5 Test the Mount

```bash
sudo mount /dev/sdb1 /mnt/backup-hdd
ls /mnt/backup-hdd  # Should show lost+found
sudo umount /mnt/backup-hdd
```

### 1.6 Set Up Mount-by-Label

Get the UUID of the partition:

```bash
sudo blkid /dev/sdb1
```

Add this line to `/etc/fstab` so the system knows about the drive, but does NOT mount it automatically:

```bash
echo 'LABEL=backups /mnt/backup-hdd ext4 defaults,noauto,nofail 0 2' | sudo tee -a /etc/fstab
```

Key flags:
- `noauto` — does not mount at boot (mount only when needed)
- `nofail` — server still boots if the drive is disconnected

Now you can mount and unmount cleanly:

```bash
sudo mount /mnt/backup-hdd    # Mount
sudo umount /mnt/backup-hdd   # Unmount
```

### 1.7 Set Ownership

```bash
sudo mount /mnt/backup-hdd
sudo mkdir -p /mnt/backup-hdd/obsidian-backups
sudo chown YOUR_USERNAME:YOUR_USERNAME /mnt/backup-hdd/obsidian-backups
sudo umount /mnt/backup-hdd
```

---

## Part 2: Install Backup Tools

### 2.1 Install age (Encryption)

age is a modern, simple encryption tool. No key management headaches — just a passphrase.

```bash
sudo apt install age
```

### 2.2 Install rclone (Cloud Sync)

```bash
sudo apt install rclone
```

### 2.3 Configure rclone

#### Option A: Backblaze B2

1. Create a free account at [backblaze.com](https://www.backblaze.com/b2/cloud-storage.html)
2. Create a bucket named something like `obsidian-backups-YOURNAME` (bucket names are globally unique)
3. Create an application key (save the Key ID and Application Key)
4. Run:

```bash
rclone config
```

- Choose `n` for new remote
- Name: `b2`
- Storage type: `b2` (Backblaze B2)
- Enter your Key ID and Application Key
- Accept defaults for the rest

#### Option B: Google Drive

```bash
rclone config
```

- Choose `n` for new remote
- Name: `gdrive`
- Storage type: `drive` (Google Drive)
- Follow the OAuth flow (you'll need a browser — use SSH port forwarding if needed: `ssh -L 53682:localhost:53682 user@server`)
- Choose the scope you want (option 1 for full access, or option 3 for file-only access)

Test it works:

```bash
# B2
rclone ls b2:obsidian-backups-YOURNAME

# Google Drive
rclone ls gdrive:
```

---

## Part 3: The Backup Script

### 3.1 Create the Passphrase File

Store the encryption passphrase in a file readable only by your user:

```bash
echo "YOUR_STRONG_PASSPHRASE_HERE" > ~/.backup-passphrase
chmod 600 ~/.backup-passphrase
```

Use a strong passphrase and **store it somewhere safe outside the server** (password manager, printed copy, etc.). If you lose this passphrase, your backups are unrecoverable.

### 3.2 Create the Script

```bash
mkdir -p ~/scripts
nano ~/scripts/backup-vault.sh
```

Paste the following:

```bash
#!/usr/bin/env bash
set -euo pipefail

# =============================================================================
# Configuration
# =============================================================================
VAULT_PATH="$HOME/obsidian/myvault"          # Path to your vault
PASSPHRASE_FILE="$HOME/.backup-passphrase"    # age encryption passphrase
HDD_MOUNT="/mnt/backup-hdd"                   # External HDD mount point
HDD_BACKUP_DIR="$HDD_MOUNT/obsidian-backups"  # Backup dir on HDD
CLOUD_REMOTE="b2:obsidian-backups-YOURNAME"   # rclone remote (b2: or gdrive:path)
LOCAL_STAGING="$HOME/.backup-staging"          # Temp dir for building backups
LOG_FILE="$HOME/.backup.log"                  # Log file

# Retention policy
KEEP_DAILY=14     # Keep last 14 daily backups
KEEP_WEEKLY=8     # Keep last 8 weekly backups (every Sunday)
KEEP_MONTHLY=6    # Keep last 6 monthly backups (1st of month)

# =============================================================================
# Functions
# =============================================================================
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

cleanup() {
    log "Cleaning up staging area..."
    rm -rf "$LOCAL_STAGING"

    # Unmount HDD if we mounted it
    if mountpoint -q "$HDD_MOUNT" 2>/dev/null; then
        log "Unmounting external HDD..."
        sudo umount "$HDD_MOUNT"
    fi
}

# Always clean up, even on failure
trap cleanup EXIT

# =============================================================================
# Main
# =============================================================================
log "========== Backup started =========="

# Validate
if [ ! -d "$VAULT_PATH" ]; then
    log "ERROR: Vault not found at $VAULT_PATH"
    exit 1
fi

if [ ! -f "$PASSPHRASE_FILE" ]; then
    log "ERROR: Passphrase file not found at $PASSPHRASE_FILE"
    exit 1
fi

# Create staging area
mkdir -p "$LOCAL_STAGING"

# Build the archive filename
TIMESTAMP=$(date '+%Y%m%d-%H%M%S')
DAY_OF_WEEK=$(date '+%u')  # 1=Monday, 7=Sunday
DAY_OF_MONTH=$(date '+%d')
ARCHIVE_NAME="vault-${TIMESTAMP}.tar.gz.age"
CHECKSUM_NAME="vault-${TIMESTAMP}.sha256"

# --- Step 1: Compress ---
log "Compressing vault..."
tar -czf "$LOCAL_STAGING/vault-${TIMESTAMP}.tar.gz" \
    -C "$(dirname "$VAULT_PATH")" \
    "$(basename "$VAULT_PATH")"

# --- Step 2: Encrypt ---
log "Encrypting archive..."
age --passphrase -i "$PASSPHRASE_FILE" \
    -o "$LOCAL_STAGING/$ARCHIVE_NAME" \
    "$LOCAL_STAGING/vault-${TIMESTAMP}.tar.gz"

# Remove unencrypted archive immediately
rm -f "$LOCAL_STAGING/vault-${TIMESTAMP}.tar.gz"

# --- Step 3: Checksum ---
log "Generating checksum..."
cd "$LOCAL_STAGING"
sha256sum "$ARCHIVE_NAME" > "$CHECKSUM_NAME"
cd -

FILESIZE=$(du -h "$LOCAL_STAGING/$ARCHIVE_NAME" | cut -f1)
log "Archive created: $ARCHIVE_NAME ($FILESIZE)"

# --- Step 4: Copy to external HDD ---
log "Mounting external HDD..."
if ! mountpoint -q "$HDD_MOUNT" 2>/dev/null; then
    sudo mount "$HDD_MOUNT" || {
        log "WARNING: Could not mount HDD. Skipping HDD backup."
        HDD_FAILED=true
    }
fi

if [ "${HDD_FAILED:-false}" = false ]; then
    mkdir -p "$HDD_BACKUP_DIR"
    cp "$LOCAL_STAGING/$ARCHIVE_NAME" "$HDD_BACKUP_DIR/"
    cp "$LOCAL_STAGING/$CHECKSUM_NAME" "$HDD_BACKUP_DIR/"
    log "Copied to external HDD."
fi

# --- Step 5: Push to cloud ---
log "Uploading to cloud storage..."
rclone copy "$LOCAL_STAGING/$ARCHIVE_NAME" "$CLOUD_REMOTE/" --progress || {
    log "WARNING: Cloud upload failed."
}
rclone copy "$LOCAL_STAGING/$CHECKSUM_NAME" "$CLOUD_REMOTE/" --progress || {
    log "WARNING: Cloud checksum upload failed."
}
log "Cloud upload complete."

# --- Step 6: Rotate old backups ---
log "Rotating old backups..."

rotate_backups() {
    local backup_dir="$1"
    local is_remote="$2"

    if [ "$is_remote" = true ]; then
        # List remote files, sort by name (which sorts by date due to naming)
        files=$(rclone ls "$backup_dir" | grep '\.tar\.gz\.age$' | awk '{print $2}' | sort)
    else
        files=$(find "$backup_dir" -name '*.tar.gz.age' -printf '%f\n' 2>/dev/null | sort)
    fi

    total=$(echo "$files" | grep -c . || true)

    if [ "$total" -le "$KEEP_DAILY" ]; then
        log "  Only $total backups found, nothing to rotate in $backup_dir"
        return
    fi

    # Keep the most recent KEEP_DAILY, delete the rest
    # (Simple approach — for weekly/monthly, a more sophisticated script
    #  would tag and preserve specific backups)
    to_delete=$(echo "$files" | head -n -"$KEEP_DAILY")
    delete_count=$(echo "$to_delete" | grep -c . || true)

    if [ "$delete_count" -gt 0 ]; then
        echo "$to_delete" | while read -r f; do
            checksum_f="${f%.tar.gz.age}.sha256"
            if [ "$is_remote" = true ]; then
                rclone deletefile "$backup_dir/$f" 2>/dev/null
                rclone deletefile "$backup_dir/$checksum_f" 2>/dev/null
            else
                rm -f "$backup_dir/$f"
                rm -f "$backup_dir/$checksum_f"
            fi
        done
        log "  Deleted $delete_count old backups from $backup_dir"
    fi
}

# Rotate on HDD
if [ "${HDD_FAILED:-false}" = false ]; then
    rotate_backups "$HDD_BACKUP_DIR" false
fi

# Rotate on cloud
rotate_backups "$CLOUD_REMOTE" true

# --- Done ---
log "========== Backup completed successfully =========="
```

### 3.3 Make It Executable

```bash
chmod +x ~/scripts/backup-vault.sh
```

### 3.4 Allow Passwordless Mount/Unmount

The script needs sudo for mount/umount. Add a sudoers rule so it doesn't require a password prompt:

```bash
sudo visudo -f /etc/sudoers.d/backup-mount
```

Add:

```
YOUR_USERNAME ALL=(ALL) NOPASSWD: /usr/bin/mount /mnt/backup-hdd, /usr/bin/umount /mnt/backup-hdd
```

---

## Part 4: Schedule with Cron

Open your crontab:

```bash
crontab -e
```

Add entries for twice-daily backups (e.g., 8 AM and 8 PM):

```
0 8 * * * /home/YOUR_USERNAME/scripts/backup-vault.sh >> /home/YOUR_USERNAME/.backup.log 2>&1
0 20 * * * /home/YOUR_USERNAME/scripts/backup-vault.sh >> /home/YOUR_USERNAME/.backup.log 2>&1
```

---

## Part 5: Integrity Verification Script

Create a script to test that a backup can actually be decrypted and restored:

```bash
nano ~/scripts/verify-backup.sh
```

```bash
#!/usr/bin/env bash
set -euo pipefail

PASSPHRASE_FILE="$HOME/.backup-passphrase"
HDD_MOUNT="/mnt/backup-hdd"
HDD_BACKUP_DIR="$HDD_MOUNT/obsidian-backups"
VERIFY_DIR=$(mktemp -d)

echo "=== Backup Integrity Verification ==="

# Mount HDD
sudo mount "$HDD_MOUNT" 2>/dev/null || true

# Pick the latest backup
LATEST=$(find "$HDD_BACKUP_DIR" -name '*.tar.gz.age' | sort | tail -1)

if [ -z "$LATEST" ]; then
    echo "ERROR: No backups found."
    sudo umount "$HDD_MOUNT" 2>/dev/null || true
    exit 1
fi

BASENAME=$(basename "$LATEST")
echo "Testing: $BASENAME"

# Verify checksum
CHECKSUM_FILE="${LATEST%.tar.gz.age}.sha256"
if [ -f "$CHECKSUM_FILE" ]; then
    cd "$(dirname "$LATEST")"
    if sha256sum -c "$(basename "$CHECKSUM_FILE")" --quiet 2>/dev/null; then
        echo "[PASS] Checksum valid"
    else
        echo "[FAIL] Checksum mismatch!"
        sudo umount "$HDD_MOUNT" 2>/dev/null || true
        exit 1
    fi
    cd -
else
    echo "[WARN] No checksum file found, skipping checksum verification"
fi

# Decrypt
echo "Decrypting..."
age --decrypt -i "$PASSPHRASE_FILE" -o "$VERIFY_DIR/archive.tar.gz" "$LATEST"
echo "[PASS] Decryption successful"

# Extract
echo "Extracting..."
tar -xzf "$VERIFY_DIR/archive.tar.gz" -C "$VERIFY_DIR/"
echo "[PASS] Extraction successful"

# Count files
FILE_COUNT=$(find "$VERIFY_DIR" -type f | wc -l)
echo "[INFO] Backup contains $FILE_COUNT files"

# Cleanup
rm -rf "$VERIFY_DIR"
sudo umount "$HDD_MOUNT" 2>/dev/null || true

echo "=== Verification complete ==="
```

```bash
chmod +x ~/scripts/verify-backup.sh
```

Schedule a weekly verification (e.g., every Sunday at noon):

```bash
crontab -e
```

Add:

```
0 12 * * 0 /home/YOUR_USERNAME/scripts/verify-backup.sh >> /home/YOUR_USERNAME/.backup.log 2>&1
```

---

## Part 6: Manual Restore

If you ever need to restore from a backup:

```bash
# Mount the HDD
sudo mount /mnt/backup-hdd

# Pick the backup you want
ls /mnt/backup-hdd/obsidian-backups/

# Decrypt
age --decrypt -i ~/.backup-passphrase \
    -o ~/restore.tar.gz \
    /mnt/backup-hdd/obsidian-backups/vault-YYYYMMDD-HHMMSS.tar.gz.age

# Extract
tar -xzf ~/restore.tar.gz -C ~/

# Clean up
rm ~/restore.tar.gz
sudo umount /mnt/backup-hdd
```

To restore from cloud instead:

```bash
# Download
rclone copy b2:obsidian-backups-YOURNAME/vault-YYYYMMDD-HHMMSS.tar.gz.age ~/

# Then decrypt and extract as above
```

---

## Summary

| Component          | Tool         | Purpose                              |
|--------------------|--------------|--------------------------------------|
| Compression        | `tar`        | Bundle vault into single archive     |
| Encryption         | `age`        | Passphrase-based encryption          |
| Integrity          | `sha256sum`  | Verify backups aren't corrupted      |
| Local backup       | External HDD | Fast recovery, mounted only on use   |
| Cloud backup       | `rclone`     | Off-site redundancy (B2 or GDrive)   |
| Scheduling         | `cron`       | Twice-daily automatic backups        |
| Verification       | Custom script| Weekly automated restore test        |

---

## Important Reminders

- **Store your passphrase outside the server.** Password manager, printed copy, wherever — if the server dies and you only had the passphrase on it, your backups are useless.
- **Test a full restore at least once** after setting everything up. Don't wait for an emergency to find out something is broken.
- **Monitor the log file** periodically: `tail -50 ~/.backup.log`
- If using Backblaze B2, enable **Object Lock** on the bucket for ransomware protection (prevents deletion for a set period).
