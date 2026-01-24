mod tailscale;

use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use serde::Serialize;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tauri::ipc::Channel;
use tauri::State;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum PtyEvent {
    Data(String),
    Exit(i32),
}

struct PtySession {
    writer: Box<dyn Write + Send>,
    master: Box<dyn MasterPty + Send>,
}

pub struct PtyState {
    sessions: Arc<Mutex<HashMap<String, PtySession>>>,
}

#[tauri::command]
async fn spawn_shell(
    state: State<'_, PtyState>,
    on_data: Channel<PtyEvent>,
    rows: u16,
    cols: u16,
    terminal_id: String,
    command: Option<String>,
) -> Result<(), String> {
    let pty_system = native_pty_system();

    let pair = pty_system
        .openpty(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
        .map_err(|e| e.to_string())?;

    let mut cmd = if let Some(ref user_cmd) = command {
        let mut c = CommandBuilder::new("sh");
        c.args(["-c", user_cmd]);
        c
    } else {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
        CommandBuilder::new(&shell)
    };
    cmd.env("TERM", "xterm-256color");

    let mut child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    drop(pair.slave);

    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
    let reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;

    let session = PtySession {
        writer,
        master: pair.master,
    };

    state.sessions.lock().unwrap().insert(terminal_id.clone(), session);

    let sessions = Arc::clone(&state.sessions);
    let tid = terminal_id.clone();
    std::thread::spawn(move || {
        let mut buf = [0u8; 4096];
        let mut reader = reader;
        loop {
            match std::io::Read::read(&mut reader, &mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = on_data.send(PtyEvent::Data(text));
                }
            }
        }
        if let Ok(status) = child.wait() {
            let _ = on_data.send(PtyEvent::Exit(if status.success() { 0 } else { 1 }));
        }
        sessions.lock().unwrap().remove(&tid);
    });

    Ok(())
}

#[tauri::command]
async fn write_to_pty(state: State<'_, PtyState>, data: String, terminal_id: String) -> Result<(), String> {
    if let Some(session) = state.sessions.lock().unwrap().get_mut(&terminal_id) {
        session.writer.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
        session.writer.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn resize_pty(state: State<'_, PtyState>, rows: u16, cols: u16, terminal_id: String) -> Result<(), String> {
    if let Some(session) = state.sessions.lock().unwrap().get(&terminal_id) {
        session.master.resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn kill_pty(state: State<'_, PtyState>, terminal_id: String) -> Result<(), String> {
    state.sessions.lock().unwrap().remove(&terminal_id);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(PtyState {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        })
        .invoke_handler(tauri::generate_handler![
            spawn_shell,
            write_to_pty,
            resize_pty,
            kill_pty,
            tailscale::get_tailscale_status,
            tailscale::tailscale_up,
            tailscale::tailscale_down
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
