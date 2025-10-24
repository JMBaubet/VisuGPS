use tauri::{App, State};
use std::sync::Mutex;
use crate::{AppState, get_setting_value};
use crate::remote_control::{start_remote_server, PENDING_PAIRING_REQUESTS, ACTIVE_CLIENT_ID};
use serde_json::Value;

use std::path::PathBuf;

#[tauri::command]
pub fn get_remote_control_status() -> String {
    if ACTIVE_CLIENT_ID.lock().unwrap().is_some() {
        "connected".to_string()
    } else {
        "disconnected".to_string()
    }
}

pub fn init_remote_control(app: &mut App, _app_env_path: &PathBuf, settings: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle_clone = app.handle().clone();

    let remote_port = get_setting_value(settings, "data.groupes.Système.groupes.Télécommande.parametres.Port")
        .and_then(|v| v.as_i64())
        .map(|p| p as u16)
        .unwrap_or(9001); // Default to 9001 if not found or invalid

    // Spawn the WebSocket server in a separate async task
    tauri::async_runtime::spawn(async move {
        start_remote_server(app_handle_clone, remote_port).await;
    });

    Ok(())
}

#[tauri::command]
pub fn reply_to_pairing_request(
    _state: State<Mutex<AppState>>, // _state is unused here, but needed for AppState context
    client_id: String,
    accepted: bool,
    _client_name: Option<String>, // _client_name is unused for now
) -> Result<(), String> {
    let mut pending_requests = PENDING_PAIRING_REQUESTS.lock().unwrap();
    if let Some((_pairing_code, tx)) = pending_requests.remove(&client_id) {
        tx.send(accepted).map_err(|_| "Failed to send pairing decision".to_string())?;
    } else {
        return Err(format!("Pairing request for client {} not found.", client_id));
    }
    Ok(())
}
