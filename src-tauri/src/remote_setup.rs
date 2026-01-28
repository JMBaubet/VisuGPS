use tauri::App;
use std::path::PathBuf;
use crate::get_setting_value;
use crate::remote_server::start_axum_server;
use serde_json::Value;
use qrcode::QrCode;
use image::Luma;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
use tokio::sync::broadcast;
use crate::remote_sse::SseMessage; // Ensure this is accessible

pub fn init_remote_control(
    app: &mut App,
    _app_env_path: &PathBuf,
    settings: &Value,
    sse_sender: broadcast::Sender<SseMessage>
) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle_clone = app.handle().clone();
    let settings_clone = settings.clone();

    let remote_port = get_setting_value(settings, "data.groupes.Système.groupes.Télécommande.parametres.Port")
        .and_then(|v| v.as_i64())
        .map(|p| p as u16)
        .unwrap_or(9001); // Default to 9001 if not found or invalid

    // Spawn le serveur Axum dans une tâche async séparée
    tauri::async_runtime::spawn(async move {
        if let Err(e) = start_axum_server(app_handle_clone, remote_port, settings_clone, sse_sender).await {
            log::error!("Erreur lors du démarrage du serveur Remote Control: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn get_remote_control_status() -> String {
    // TODO: Implémenter avec la nouvelle architecture (vérifier les clients SSE connectés)
    "disconnected".to_string()
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


// Plugin initialization removed in favor of global commands
// pub fn init<R: Runtime>() -> TauriPlugin<R> { ... }
