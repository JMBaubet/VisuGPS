// Ce fichier contient les structures et fonctions legacy qui sont encore utilisées par d'autres parties de l'application
// La logique serveur a été migrée vers remote_server.rs et remote_sse.rs

use tauri::{command, AppHandle, Manager, Emitter};
use serde::{Serialize, Deserialize};
use log::debug;
use std::sync::Mutex;
use crate::AppState;
use crate::remote_sse::SseMessage;

// === Structures de données conservées pour compatibilité ===

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteVariant {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteSegment {
    pub id: String,
    pub name: String,
    pub segment_type: String,
    pub start_distance: f64,
    pub end_distance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VisualizeViewState {
    pub is_controls_card_visible: bool,
    pub is_altitude_visible: bool,
    pub is_commune_widget_visible: bool,
    pub is_distance_display_visible: bool,
    pub is_static_weather_visible: bool,
    pub is_dynamic_weather_visible: bool,
    pub current_speed: f64,
    pub animation_state: String,
    pub has_variants: bool,
    pub variant_count: usize,
    pub variants: Vec<RemoteVariant>,
    pub segments: Vec<RemoteSegment>,
    pub current_segment_index: Option<usize>,
}

// === Commandes Tauri (UI → Backend) ===

#[command]
pub fn update_visualize_view_state(app_handle: AppHandle, state: VisualizeViewState) {
    let app_state = app_handle.state::<Mutex<AppState>>();
    {
        let app_state_lock = app_state.lock().unwrap();
        *app_state_lock.visualize_view_state.lock().unwrap() = Some(state.clone());
    }
    
    // Envoyer via SSE
    send_visualize_view_state_update(&app_handle, state);
}

#[tauri::command]
pub fn update_animation_speed(app_handle: AppHandle, speed: f32) {
    let app_state = app_handle.state::<Mutex<AppState>>();
    {
        let app_state_lock = app_state.lock().unwrap();
        *app_state_lock.animation_speed.lock().unwrap() = speed;
    }
    
    // Envoyer via SSE
    send_animation_speed_update(&app_handle, speed);
}

#[tauri::command]
pub fn notify_animation_progress(app_handle: AppHandle, current_distance: f64, current_segment_index: Option<usize>) {
    let app_state_mutex = app_handle.state::<Mutex<AppState>>();
    let lock = app_state_mutex.lock().unwrap();
    if let Some(sse_sender) = &lock.sse_sender {
        let sse_state = crate::remote_sse::SseState { tx: sse_sender.clone() };
        sse_state.send_animation_progress_update(current_distance, current_segment_index);
    }
}

#[tauri::command]
pub fn notify_pause_state_changed(paused: bool) {
    // TODO: Envoyer via SSE
    debug!("Pause state changed: {}", paused);
}

#[tauri::command]
pub fn remote_command_increase_speed(app_handle: AppHandle) {
    app_handle.emit("remote_command::increase_speed", ()).unwrap();
}

#[tauri::command]
pub fn remote_command_decrease_speed(app_handle: AppHandle) {
    app_handle.emit("remote_command::decrease_speed", ()).unwrap();
}

#[tauri::command]
pub fn update_speed_from_remote(app_handle: AppHandle, payload: serde_json::Value) {
    if let Some(speed) = payload["speed"].as_f64() {
        app_handle.emit("remote_command::update_speed", speed).unwrap();
    }
}

#[tauri::command]
pub fn set_speed_to_1x_from_remote(app_handle: AppHandle) {
    app_handle.emit("remote_command::set_speed_to_1x", ()).unwrap();
}

#[tauri::command]
pub fn approve_remote_client(app_handle: AppHandle, client_id: String) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let app_env_path = {
        let guard = state.lock().unwrap();
        guard.app_env_path.clone()
    };

    // 1. Add to authorized clients
    let _ = crate::remote_clients::add_authorized_client(&app_env_path, client_id.clone(), "Approved via UI".to_string());

    // 2. Remove from pending
    {
        let guard = state.lock().unwrap();
        guard.pending_clients.lock().unwrap().remove(&client_id);
    }

    // 3. Notify the client via SSE
    if let Ok(guard) = state.lock() {
        if let Some(sender) = &guard.sse_sender {
            // We broadcast to all, the client will check its ID
            let _ = sender.send(SseMessage {
                event_type: "pairing_approved".to_string(),
                data: serde_json::json!({ "clientId": client_id }),
            });
        }
    }

    // 4. Update Desktop UI icon (blue -> green)
    let _ = app_handle.emit("remote_control_status_changed", "connected");

    debug!("Client approved: {}", client_id);
    Ok(())
}

#[tauri::command]
pub fn refuse_remote_client(app_handle: AppHandle, client_id: String) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let app_env_path = {
        let guard = state.lock().unwrap();
        guard.app_env_path.clone()
    };
    
    // 1. Add to blacklist to prevent further prompts
    let _ = crate::remote_blacklist::add_to_blacklist(&app_env_path, client_id.clone(), "Refusé par l'utilisateur".to_string());

    // 2. Remove from pending
    {
        let guard = state.lock().unwrap();
        guard.pending_clients.lock().unwrap().remove(&client_id);
    }

    // 3. Notify via SSE
    if let Ok(guard) = state.lock() {
        if let Some(sender) = &guard.sse_sender {
            let _ = sender.send(SseMessage {
                event_type: "pairing_refused".to_string(),
                data: serde_json::json!({ "clientId": client_id, "reason": "Refusé par l'utilisateur" }),
            });
        }
    }

    debug!("Client refused: {}", client_id);
    Ok(())
}

#[tauri::command]
pub fn abandon_remote_client(app_handle: AppHandle, client_id: String) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    
    // 1. Remove from pending list
    if let Ok(guard) = state.lock() {
        let mut pending = guard.pending_clients.lock().unwrap();
        pending.remove(&client_id);
    }

    // 2. Notify via SSE
    if let Ok(guard) = state.lock() {
        if let Some(sender) = &guard.sse_sender {
            let _ = sender.send(SseMessage {
                event_type: "pairing_abandoned".to_string(),
                data: serde_json::json!({ "clientId": client_id }),
            });
        }
    }

    debug!("Client abandoned: {}", client_id);
    Ok(())
}

#[tauri::command]
pub fn disconnect_active_remote_client(app_handle: AppHandle) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    
    // Notify clients via SSE to close their connections
    if let Ok(guard) = state.lock() {
        if let Some(sender) = &guard.sse_sender {
            let _ = sender.send(SseMessage {
                event_type: "remote_disconnect".to_string(),
                data: serde_json::json!({ "reason": "Déconnecté par l'utilisateur" }),
            });
        }
    }

    debug!("Disconnect request received - notifying clients");
    app_handle.emit("remote_control_status_changed", "disconnected").unwrap();
    Ok(())
}

pub fn send_app_state_update(app_handle: &AppHandle, new_state: &str) {
    let state = app_handle.state::<Mutex<AppState>>();
    if let Ok(guard) = state.lock() {
        if let Some(sender) = &guard.sse_sender {
            let _ = sender.send(SseMessage {
                event_type: "app_state_update".to_string(),
                data: serde_json::json!({ "appState": new_state }),
            });
        }
    };
}

pub fn send_visualize_view_state_update(app_handle: &AppHandle, state_view: VisualizeViewState) {
    let state = app_handle.state::<Mutex<AppState>>();
    if let Ok(guard) = state.lock() {
        if let Some(sender) = &guard.sse_sender {
            let _ = sender.send(SseMessage {
                event_type: "visualize_view_state_update".to_string(),
                data: serde_json::to_value(state_view).unwrap_or_default(),
            });
        }
    };
}

pub fn send_animation_state_update(app_handle: &AppHandle, state_val: &str, current_segment_index: Option<usize>) {
    let state = app_handle.state::<Mutex<AppState>>();
    if let Ok(guard) = state.lock() {
        if let Some(sender) = &guard.sse_sender {
            let _ = sender.send(SseMessage {
                event_type: "animation_state_update".to_string(),
                data: serde_json::json!({ 
                    "animationState": state_val,
                    "currentSegmentIndex": current_segment_index
                }),
            });
        }
    };
}

pub fn send_animation_speed_update(app_handle: &AppHandle, speed: f32) {
    let state = app_handle.state::<Mutex<AppState>>();
    if let Ok(guard) = state.lock() {
        if let Some(sender) = &guard.sse_sender {
            let _ = sender.send(SseMessage {
                event_type: "animation_speed_update".to_string(),
                data: serde_json::json!({ "speed": speed }),
            });
        }
    };
}
