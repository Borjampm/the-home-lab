use russh::client::{self, Handle};
use russh::keys::PrivateKeyWithHashAlg;
use russh_sftp::client::SftpSession;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read as StdRead;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::ipc::Channel;
use tauri::State;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

// ----- Error Types -----

#[derive(Error, Debug)]
pub enum SftpError {
    #[error("Connection failed: {0}")]
    Connection(String),
    #[error("Authentication failed: {0}")]
    Authentication(String),
    #[error("SSH agent error: {0}")]
    SshAgent(String),
    #[error("SFTP operation failed: {0}")]
    SftpOperation(String),
    #[error("Not connected: {0}")]
    NotConnected(String),
    #[error("Transfer cancelled")]
    TransferCancelled,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Timeout: {0}")]
    Timeout(String),
}

impl From<SftpError> for String {
    fn from(err: SftpError) -> String {
        err.to_string()
    }
}

// ----- Data Structures -----

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<u64>,
    pub permissions: u32,
    pub is_symlink: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryListing {
    pub path: String,
    pub entries: Vec<FileEntry>,
    pub parent_path: Option<String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum TransferEvent {
    Progress {
        bytes_transferred: u64,
        total_bytes: u64,
        filename: String,
    },
    Complete {
        filename: String,
    },
    Error {
        filename: String,
        message: String,
    },
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub id: String,
    pub name: String,
    pub device_ip: String,
    pub device_hostname: String,
    pub path: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkList {
    pub bookmarks: Vec<Bookmark>,
}

// ----- SSH Handler -----

struct SshHandler;

impl client::Handler for SshHandler {
    type Error = russh::Error;

    fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::ssh_key::PublicKey,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send {
        // Accept all host keys for home lab environment
        // In production, you'd want to verify against known_hosts
        async { Ok(true) }
    }
}

// ----- State Management -----

struct SftpConnection {
    #[allow(dead_code)]
    ssh_handle: Handle<SshHandler>,
    sftp: SftpSession,
    #[allow(dead_code)]
    device_ip: String,
    #[allow(dead_code)]
    username: String,
}

#[derive(Clone)]
pub enum TransferDirection {
    Download,
    Upload,
}

pub struct TransferJob {
    pub cancel_token: CancellationToken,
    #[allow(dead_code)]
    pub direction: TransferDirection,
}

pub struct SftpState {
    connections: Arc<RwLock<HashMap<String, SftpConnection>>>,
    transfers: Arc<RwLock<HashMap<String, TransferJob>>>,
}

impl Default for SftpState {
    fn default() -> Self {
        SftpState {
            connections: Arc::new(RwLock::new(HashMap::new())),
            transfers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// ----- Helper Functions -----

fn get_bookmarks_path() -> PathBuf {
    let proj_dirs = directories::ProjectDirs::from("com", "homelab", "control-center")
        .expect("Failed to get project directories");
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir).ok();
    data_dir.join("bookmarks.json")
}

fn get_parent_path(path: &str) -> Option<String> {
    let path = path.trim_end_matches('/');
    if path.is_empty() || path == "/" {
        return None;
    }
    let parent = std::path::Path::new(path)
        .parent()
        .map(|p| p.to_string_lossy().to_string());
    parent.map(|p| if p.is_empty() { "/".to_string() } else { p })
}

async fn establish_connection(
    device_ip: &str,
    username: &str,
) -> Result<(Handle<SshHandler>, SftpSession), SftpError> {
    // Configure SSH client
    let config = client::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(300)),
        keepalive_interval: Some(std::time::Duration::from_secs(30)),
        keepalive_max: 3,
        ..Default::default()
    };

    let addr = format!("{}:22", device_ip);

    // Connect with timeout
    let mut ssh_handle = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        client::connect(Arc::new(config), &addr, SshHandler),
    )
    .await
    .map_err(|_| SftpError::Timeout("SSH connection timed out".to_string()))?
    .map_err(|e| SftpError::Connection(e.to_string()))?;

    // Try SSH agent authentication first, then key file auth
    let authenticated = try_ssh_agent_auth(&mut ssh_handle, username).await
        || try_key_file_auth(&mut ssh_handle, username).await;

    if !authenticated {
        return Err(SftpError::Authentication(
            "Authentication failed. Ensure SSH agent is running with your keys loaded, or that ~/.ssh/id_ed25519 or ~/.ssh/id_rsa exists.".to_string(),
        ));
    }

    // Open channel and request SFTP subsystem
    let channel = ssh_handle
        .channel_open_session()
        .await
        .map_err(|e| SftpError::Connection(format!("Failed to open channel: {}", e)))?;

    channel
        .request_subsystem(true, "sftp")
        .await
        .map_err(|e| SftpError::Connection(format!("Failed to request SFTP subsystem: {}", e)))?;

    // Create SFTP session from channel
    let sftp = SftpSession::new(channel.into_stream())
        .await
        .map_err(|e| SftpError::SftpOperation(format!("Failed to create SFTP session: {}", e)))?;

    Ok((ssh_handle, sftp))
}

async fn try_ssh_agent_auth(handle: &mut Handle<SshHandler>, username: &str) -> bool {
    // Check if SSH_AUTH_SOCK is set
    let auth_sock = match std::env::var("SSH_AUTH_SOCK") {
        Ok(sock) => sock,
        Err(_) => return false,
    };

    // Try to connect to the SSH agent using russh's agent client
    let mut agent = match russh::keys::agent::client::AgentClient::connect_uds(&auth_sock).await {
        Ok(agent) => agent,
        Err(_) => return false,
    };

    // Get identities from agent
    let identities = match agent.request_identities().await {
        Ok(ids) => ids,
        Err(_) => return false,
    };

    // Try each identity with the agent as signer
    for identity in identities {
        match handle
            .authenticate_publickey_with(username, identity, None, &mut agent)
            .await
        {
            Ok(result) => {
                if result.success() {
                    return true;
                }
            }
            Err(_) => continue,
        }
    }

    false
}

async fn try_key_file_auth(handle: &mut Handle<SshHandler>, username: &str) -> bool {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let key_paths = [
        format!("{}/.ssh/id_ed25519", home),
        format!("{}/.ssh/id_rsa", home),
    ];

    for key_path in key_paths {
        let path = std::path::Path::new(&key_path);
        if !path.exists() {
            continue;
        }

        // Try to load the key (without passphrase first) using russh's key loader
        let key_pair = match russh::keys::load_secret_key(&key_path, None) {
            Ok(key) => key,
            Err(_) => continue,
        };

        // Create a key with the default hash algorithm for the key type
        let key_with_alg = PrivateKeyWithHashAlg::new(Arc::new(key_pair), None);

        match handle.authenticate_publickey(username, key_with_alg).await {
            Ok(result) => {
                if result.success() {
                    return true;
                }
            }
            Err(_) => continue,
        }
    }

    false
}

// ----- Tauri Commands -----

#[tauri::command]
pub async fn sftp_connect(
    state: State<'_, SftpState>,
    device_ip: String,
    username: Option<String>,
) -> Result<(), String> {
    // Check if already connected
    {
        let connections = state.connections.read().await;
        if connections.contains_key(&device_ip) {
            return Ok(());
        }
    }

    let user = username
        .unwrap_or_else(|| std::env::var("USER").unwrap_or_else(|_| "root".to_string()));

    let (ssh_handle, sftp) = establish_connection(&device_ip, &user).await?;

    let connection = SftpConnection {
        ssh_handle,
        sftp,
        device_ip: device_ip.clone(),
        username: user,
    };

    state.connections.write().await.insert(device_ip, connection);

    Ok(())
}

#[tauri::command]
pub async fn sftp_disconnect(state: State<'_, SftpState>, device_ip: String) -> Result<(), String> {
    state.connections.write().await.remove(&device_ip);
    Ok(())
}

#[tauri::command]
pub async fn sftp_list_dir(
    state: State<'_, SftpState>,
    device_ip: String,
    path: String,
) -> Result<DirectoryListing, String> {
    let connections = state.connections.read().await;
    let connection = connections
        .get(&device_ip)
        .ok_or_else(|| "Not connected to this device".to_string())?;

    let normalized_path = if path.is_empty() {
        "/".to_string()
    } else {
        path
    };

    let dir_entries = connection
        .sftp
        .read_dir(&normalized_path)
        .await
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut entries = Vec::new();

    for entry in dir_entries {
        let name = entry.file_name();

        // Skip . and ..
        if name == "." || name == ".." {
            continue;
        }

        let full_path = if normalized_path == "/" {
            format!("/{}", name)
        } else {
            format!("{}/{}", normalized_path.trim_end_matches('/'), name)
        };

        let metadata = entry.metadata();
        let file_type = entry.file_type();
        let is_dir = file_type.is_dir();
        let is_symlink = file_type.is_symlink();

        // Convert modified time: russh-sftp stores mtime as Option<u32>, convert to u64
        let modified = metadata.mtime.map(|t| t as u64);

        entries.push(FileEntry {
            name,
            path: full_path,
            is_dir,
            size: metadata.size.unwrap_or(0),
            modified,
            permissions: metadata.permissions.unwrap_or(0),
            is_symlink,
        });
    }

    // Sort: directories first, then alphabetically
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(DirectoryListing {
        path: normalized_path.clone(),
        entries,
        parent_path: get_parent_path(&normalized_path),
    })
}

#[tauri::command]
pub async fn sftp_mkdir(
    state: State<'_, SftpState>,
    device_ip: String,
    path: String,
) -> Result<(), String> {
    let connections = state.connections.read().await;
    let connection = connections
        .get(&device_ip)
        .ok_or_else(|| "Not connected to this device".to_string())?;

    connection
        .sftp
        .create_dir(&path)
        .await
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn sftp_read_file(
    state: State<'_, SftpState>,
    device_ip: String,
    path: String,
    max_bytes: Option<usize>,
) -> Result<String, String> {
    let connections = state.connections.read().await;
    let connection = connections
        .get(&device_ip)
        .ok_or_else(|| "Not connected to this device".to_string())?;

    let mut file = connection
        .sftp
        .open(&path)
        .await
        .map_err(|e| format!("Failed to open file: {}", e))?;

    let max = max_bytes.unwrap_or(1024 * 1024); // Default 1MB
    let mut buffer = vec![0u8; max];
    let bytes_read = file
        .read(&mut buffer)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;
    buffer.truncate(bytes_read);

    String::from_utf8(buffer)
        .map_err(|_| "File contains non-UTF8 content (binary file)".to_string())
}

#[tauri::command]
pub async fn sftp_delete(
    state: State<'_, SftpState>,
    device_ip: String,
    path: String,
    recursive: bool,
) -> Result<(), String> {
    let connections = state.connections.read().await;
    let connection = connections
        .get(&device_ip)
        .ok_or_else(|| "Not connected to this device".to_string())?;

    let metadata = connection
        .sftp
        .metadata(&path)
        .await
        .map_err(|e| format!("Failed to stat path: {}", e))?;

    if metadata.is_dir() {
        if recursive {
            // Recursively delete directory contents
            delete_recursive(&connection.sftp, &path).await?;
        } else {
            connection
                .sftp
                .remove_dir(&path)
                .await
                .map_err(|e| format!("Failed to remove directory (may not be empty): {}", e))?;
        }
    } else {
        connection
            .sftp
            .remove_file(&path)
            .await
            .map_err(|e| format!("Failed to delete file: {}", e))?;
    }

    Ok(())
}

async fn delete_recursive(sftp: &SftpSession, path: &str) -> Result<(), String> {
    let entries = sftp
        .read_dir(path)
        .await
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let name = entry.file_name();

        if name == "." || name == ".." {
            continue;
        }

        let full_path = format!("{}/{}", path.trim_end_matches('/'), name);
        let is_dir = entry.file_type().is_dir();

        if is_dir {
            Box::pin(delete_recursive(sftp, &full_path)).await?;
        } else {
            sftp.remove_file(&full_path)
                .await
                .map_err(|e| format!("Failed to delete file {}: {}", full_path, e))?;
        }
    }

    sftp.remove_dir(path)
        .await
        .map_err(|e| format!("Failed to remove directory: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn sftp_rename(
    state: State<'_, SftpState>,
    device_ip: String,
    old_path: String,
    new_path: String,
) -> Result<(), String> {
    let connections = state.connections.read().await;
    let connection = connections
        .get(&device_ip)
        .ok_or_else(|| "Not connected to this device".to_string())?;

    connection
        .sftp
        .rename(&old_path, &new_path)
        .await
        .map_err(|e| format!("Failed to rename: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn sftp_download(
    state: State<'_, SftpState>,
    device_ip: String,
    remote_path: String,
    local_path: String,
    transfer_id: String,
    on_progress: Channel<TransferEvent>,
) -> Result<(), String> {
    // Create cancellation token and register transfer
    let cancel_token = CancellationToken::new();
    {
        let mut transfers = state.transfers.write().await;
        transfers.insert(
            transfer_id.clone(),
            TransferJob {
                cancel_token: cancel_token.clone(),
                direction: TransferDirection::Download,
            },
        );
    }

    let filename = std::path::Path::new(&remote_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| remote_path.clone());

    let result = download_file(
        &state,
        &device_ip,
        &remote_path,
        &local_path,
        &filename,
        &cancel_token,
        &on_progress,
    )
    .await;

    // Clean up transfer tracking
    state.transfers.write().await.remove(&transfer_id);

    match result {
        Ok(_) => {
            let _ = on_progress.send(TransferEvent::Complete {
                filename: filename.clone(),
            });
            Ok(())
        }
        Err(e) => {
            let _ = on_progress.send(TransferEvent::Error {
                filename,
                message: e.clone(),
            });
            Err(e)
        }
    }
}

async fn download_file(
    state: &State<'_, SftpState>,
    device_ip: &str,
    remote_path: &str,
    local_path: &str,
    filename: &str,
    cancel_token: &CancellationToken,
    on_progress: &Channel<TransferEvent>,
) -> Result<(), String> {
    let connections = state.connections.read().await;
    let connection = connections
        .get(device_ip)
        .ok_or_else(|| "Not connected to this device".to_string())?;

    let metadata = connection
        .sftp
        .metadata(remote_path)
        .await
        .map_err(|e| format!("Failed to stat file: {}", e))?;

    let total_bytes = metadata.size.unwrap_or(0);

    let mut remote_file = connection
        .sftp
        .open(remote_path)
        .await
        .map_err(|e| format!("Failed to open remote file: {}", e))?;

    let mut local_file =
        tokio::fs::File::create(local_path)
            .await
            .map_err(|e| format!("Failed to create local file: {}", e))?;

    let mut buffer = vec![0u8; 65536]; // 64KB buffer
    let mut bytes_transferred: u64 = 0;

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => {
                drop(local_file);
                tokio::fs::remove_file(local_path).await.ok();
                return Err("Transfer cancelled".to_string());
            }
            read_result = remote_file.read(&mut buffer) => {
                let bytes_read = read_result
                    .map_err(|e| format!("Failed to read from remote: {}", e))?;

                if bytes_read == 0 {
                    break;
                }

                local_file.write_all(&buffer[..bytes_read])
                    .await
                    .map_err(|e| format!("Failed to write to local file: {}", e))?;

                bytes_transferred += bytes_read as u64;

                let _ = on_progress.send(TransferEvent::Progress {
                    bytes_transferred,
                    total_bytes,
                    filename: filename.to_string(),
                });
            }
        }
    }

    local_file
        .flush()
        .await
        .map_err(|e| format!("Failed to flush local file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn sftp_upload(
    state: State<'_, SftpState>,
    device_ip: String,
    local_path: String,
    remote_path: String,
    transfer_id: String,
    on_progress: Channel<TransferEvent>,
) -> Result<(), String> {
    // Create cancellation token and register transfer
    let cancel_token = CancellationToken::new();
    {
        let mut transfers = state.transfers.write().await;
        transfers.insert(
            transfer_id.clone(),
            TransferJob {
                cancel_token: cancel_token.clone(),
                direction: TransferDirection::Upload,
            },
        );
    }

    let filename = std::path::Path::new(&local_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| local_path.clone());

    let result = upload_file(
        &state,
        &device_ip,
        &local_path,
        &remote_path,
        &filename,
        &cancel_token,
        &on_progress,
    )
    .await;

    // Clean up transfer tracking
    state.transfers.write().await.remove(&transfer_id);

    match result {
        Ok(_) => {
            let _ = on_progress.send(TransferEvent::Complete {
                filename: filename.clone(),
            });
            Ok(())
        }
        Err(e) => {
            let _ = on_progress.send(TransferEvent::Error {
                filename,
                message: e.clone(),
            });
            Err(e)
        }
    }
}

async fn upload_file(
    state: &State<'_, SftpState>,
    device_ip: &str,
    local_path: &str,
    remote_path: &str,
    filename: &str,
    cancel_token: &CancellationToken,
    on_progress: &Channel<TransferEvent>,
) -> Result<(), String> {
    let metadata = tokio::fs::metadata(local_path)
        .await
        .map_err(|e| format!("Failed to get local file info: {}", e))?;

    let total_bytes = metadata.len();

    let mut local_file = tokio::fs::File::open(local_path)
        .await
        .map_err(|e| format!("Failed to open local file: {}", e))?;

    let connections = state.connections.read().await;
    let connection = connections
        .get(device_ip)
        .ok_or_else(|| "Not connected to this device".to_string())?;

    let mut remote_file = connection
        .sftp
        .create(remote_path)
        .await
        .map_err(|e| format!("Failed to create remote file: {}", e))?;

    let mut buffer = vec![0u8; 65536]; // 64KB buffer
    let mut bytes_transferred: u64 = 0;

    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => {
                drop(remote_file);
                // Try to delete partial remote file
                connection.sftp.remove_file(remote_path).await.ok();
                return Err("Transfer cancelled".to_string());
            }
            read_result = local_file.read(&mut buffer) => {
                let bytes_read = read_result
                    .map_err(|e| format!("Failed to read from local file: {}", e))?;

                if bytes_read == 0 {
                    break;
                }

                remote_file.write_all(&buffer[..bytes_read])
                    .await
                    .map_err(|e| format!("Failed to write to remote file: {}", e))?;

                bytes_transferred += bytes_read as u64;

                let _ = on_progress.send(TransferEvent::Progress {
                    bytes_transferred,
                    total_bytes,
                    filename: filename.to_string(),
                });
            }
        }
    }

    remote_file
        .flush()
        .await
        .map_err(|e| format!("Failed to flush remote file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn sftp_cancel_transfer(
    state: State<'_, SftpState>,
    transfer_id: String,
) -> Result<(), String> {
    let transfers = state.transfers.read().await;
    if let Some(job) = transfers.get(&transfer_id) {
        job.cancel_token.cancel();
    }
    Ok(())
}

// ----- Bookmark Commands -----

#[tauri::command]
pub async fn sftp_get_bookmarks() -> Result<BookmarkList, String> {
    let path = get_bookmarks_path();

    if !path.exists() {
        return Ok(BookmarkList { bookmarks: vec![] });
    }

    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read bookmarks: {}", e))?;

    let bookmarks: Vec<Bookmark> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse bookmarks: {}", e))?;

    Ok(BookmarkList { bookmarks })
}

#[tauri::command]
pub async fn sftp_save_bookmark(bookmark: Bookmark) -> Result<(), String> {
    let path = get_bookmarks_path();

    let mut bookmarks: Vec<Bookmark> = if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        vec![]
    };

    // Update existing or add new
    if let Some(existing) = bookmarks.iter_mut().find(|b| b.id == bookmark.id) {
        *existing = bookmark;
    } else {
        bookmarks.push(bookmark);
    }

    let content = serde_json::to_string_pretty(&bookmarks)
        .map_err(|e| format!("Failed to serialize bookmarks: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write bookmarks: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn sftp_delete_bookmark(bookmark_id: String) -> Result<(), String> {
    let path = get_bookmarks_path();

    if !path.exists() {
        return Ok(());
    }

    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read bookmarks: {}", e))?;

    let mut bookmarks: Vec<Bookmark> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse bookmarks: {}", e))?;

    bookmarks.retain(|b| b.id != bookmark_id);

    let content = serde_json::to_string_pretty(&bookmarks)
        .map_err(|e| format!("Failed to serialize bookmarks: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("Failed to write bookmarks: {}", e))?;

    Ok(())
}

// ----- Local File System Commands (for self device) -----

#[tauri::command]
pub async fn local_list_dir(path: String) -> Result<DirectoryListing, String> {
    let normalized_path = if path.is_empty() || path == "~" {
        std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        format!("{}{}", home, &path[1..])
    } else {
        path
    };

    let read_dir =
        fs::read_dir(&normalized_path).map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut entries = Vec::new();

    for entry in read_dir {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let metadata = entry
            .metadata()
            .map_err(|e| format!("Failed to get metadata: {}", e))?;
        let name = entry.file_name().to_string_lossy().to_string();

        let full_path = entry.path().to_string_lossy().to_string();

        entries.push(FileEntry {
            name,
            path: full_path,
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified: metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs()),
            permissions: 0o755, // Simplified for local
            is_symlink: metadata.file_type().is_symlink(),
        });
    }

    // Sort: directories first, then alphabetically
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(DirectoryListing {
        path: normalized_path.clone(),
        entries,
        parent_path: get_parent_path(&normalized_path),
    })
}

#[tauri::command]
pub async fn local_read_file(path: String, max_bytes: Option<usize>) -> Result<String, String> {
    let max = max_bytes.unwrap_or(1024 * 1024);
    let mut file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;

    let mut buffer = vec![0u8; max];
    let bytes_read = file
        .read(&mut buffer)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    buffer.truncate(bytes_read);

    String::from_utf8(buffer)
        .map_err(|_| "File contains non-UTF8 content (binary file)".to_string())
}

#[tauri::command]
pub async fn local_mkdir(path: String) -> Result<(), String> {
    fs::create_dir(&path).map_err(|e| format!("Failed to create directory: {}", e))
}

#[tauri::command]
pub async fn local_delete(path: String, recursive: bool) -> Result<(), String> {
    let metadata = fs::metadata(&path).map_err(|e| format!("Failed to get file info: {}", e))?;

    if metadata.is_dir() {
        if recursive {
            fs::remove_dir_all(&path).map_err(|e| format!("Failed to delete directory: {}", e))?;
        } else {
            fs::remove_dir(&path).map_err(|e| format!("Failed to delete directory: {}", e))?;
        }
    } else {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn local_rename(old_path: String, new_path: String) -> Result<(), String> {
    fs::rename(&old_path, &new_path).map_err(|e| format!("Failed to rename: {}", e))
}
