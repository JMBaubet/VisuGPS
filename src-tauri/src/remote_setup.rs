use tauri::{App, Manager, Emitter};
use std::path::PathBuf;
use crate::get_setting_value;
use crate::remote_server::start_axum_server;
use serde_json::Value;
use qrcode::QrCode;
use image::Luma;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
use tokio::sync::broadcast;
use crate::remote_sse::SseMessage;
use crate::HeartbeatState;

pub fn init_remote_control(
    app: &mut App,
    _app_env_path: &PathBuf,
    settings: &Value,
    sse_sender: broadcast::Sender<SseMessage>
) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle_clone = app.handle().clone();
    let settings_clone = settings.clone();

    // Nettoyage des anciennes télécommandes au démarrage
    prune_remotes(_app_env_path, settings);

    let remote_port = get_setting_value(settings, "data.groupes.Système.groupes.Télécommande.parametres.Port")
        .and_then(|v| v.as_i64())
        .map(|p| p as u16)
        .unwrap_or(9001);

    // Spawn le serveur Axum dans une tâche async séparée
    tauri::async_runtime::spawn(async move {
        if let Err(e) = start_axum_server(app_handle_clone, remote_port, settings_clone, sse_sender).await {
            log::error!("Erreur lors du démarrage du serveur Remote Control: {}", e);
        }
    });

    // Surveillance du heartbeat pour détecter les déconnexions (fermeture onglet, etc.)
    let app_handle_for_monitor = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            if let Some(hb_state) = app_handle_for_monitor.try_state::<HeartbeatState>() {
                let now = chrono::Utc::now().timestamp();
                let mut active_clients = hb_state.active_clients.lock().unwrap();
                let prev_count = active_clients.len();
                
                // Nettoyer les clients expirés (> 10s)
                active_clients.retain(|_, &mut t| (now - t) < 10);
                
                let new_count = active_clients.len();
                if prev_count > 0 && new_count == 0 {
                    let _ = app_handle_for_monitor.emit("remote_control_status_changed", "disconnected");
                    log::debug!("Plus aucune télécommande connectée (timeout)");
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn get_remote_control_status(hb_state: tauri::State<HeartbeatState>) -> String {
    let now = chrono::Utc::now().timestamp();
    let active_clients = hb_state.active_clients.lock().unwrap();
    
    let has_active = active_clients.values().any(|&t| (now - t) < 10);
    
    if has_active {
        "connected".to_string()
    } else {
        "disconnected".to_string()
    }
}

#[tauri::command]
pub fn reply_to_pairing_request(
    _client_id: String,
    _accepted: bool,
    _client_name: Option<String>,
) -> Result<(), String> {
    // TODO: Implémenter avec la nouvelle architecture
    Ok(())
}

#[tauri::command]
pub fn get_network_interfaces() -> Vec<(String, String)> {
    crate::network_utils::get_available_interfaces()
}

#[tauri::command]
pub fn generate_qrcode_base64(url: String) -> Result<String, String> {
    let code = QrCode::new(url.as_bytes()).map_err(|e| e.to_string())?;
    // Rendu en image Luma8 (niveau de gris)
    let image = code.render::<Luma<u8>>().build();
    
    let mut buffer = Cursor::new(Vec::new());
    // image::ImageFormat::Png est utilisé pour écrire en PNG
    image.write_to(&mut buffer, image::ImageFormat::Png).map_err(|e| e.to_string())?;

    let encoded = general_purpose::STANDARD.encode(buffer.get_ref());
    Ok(format!("data:image/png;base64,{}", encoded))
}

pub fn prune_remotes(app_env_path: &PathBuf, settings: &Value) {
    let auth_days = get_setting_value(settings, "data.groupes.Système.groupes.Télécommande.groupes.Rétention.parametres.autorisees_retention_days")
        .and_then(|v| v.as_i64())
        .unwrap_or(7);
        
    let black_days = get_setting_value(settings, "data.groupes.Système.groupes.Télécommande.groupes.Rétention.parametres.interdites_retention_days")
        .and_then(|v| v.as_i64())
        .unwrap_or(14);

    let _ = crate::remote_clients::prune_authorized_clients(app_env_path, auth_days);
    let _ = crate::remote_blacklist::prune_blacklisted_clients(app_env_path, black_days);
}


// Plugin initialization removed in favor of global commands
// pub fn init<R: Runtime>() -> TauriPlugin<R> { ... }
