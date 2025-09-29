use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use crate::AppState;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Events {
    pub pause: Vec<u32>,
}

fn get_events_path(app_handle: &AppHandle, circuit_id: &str) -> Result<PathBuf, String> {
    let state_mutex = app_handle.state::<Mutex<AppState>>();
    let app_state = state_mutex.lock().unwrap();
    let app_env_path = app_state.app_env_path.clone();

    let data_dir = app_env_path.join("data").join(circuit_id);
    fs::create_dir_all(&data_dir).map_err(|e| format!("Failed to create data directory for circuit {}: {}", circuit_id, e))?;
    Ok(data_dir.join("evt.json"))
}

fn read_events(app_handle: &AppHandle, circuit_id: &str) -> Result<Events, String> {
    let path = get_events_path(app_handle, circuit_id)?;
    if !path.exists() {
        let default_events = Events::default();
        write_events(app_handle, circuit_id, &default_events)?;
        return Ok(default_events);
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read evt.json: {}", e))?;
    
    if content.trim().is_empty() {
        let default_events = Events::default();
        write_events(app_handle, circuit_id, &default_events)?;
        return Ok(default_events);
    }

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse evt.json: {}", e))
}

fn write_events(app_handle: &AppHandle, circuit_id: &str, events: &Events) -> Result<(), String> {
    let path = get_events_path(app_handle, circuit_id)?;
    let content = serde_json::to_string_pretty(events)
        .map_err(|e| format!("Failed to serialize events: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write evt.json: {}", e))
}

#[tauri::command]
pub fn get_events(app_handle: AppHandle, circuit_id: String) -> Result<Events, String> {
    read_events(&app_handle, &circuit_id)
}

#[tauri::command]
pub fn add_pause_event(app_handle: AppHandle, circuit_id: String, increment: u32) -> Result<Events, String> {
    let mut events = read_events(&app_handle, &circuit_id)?;
    if !events.pause.contains(&increment) {
        events.pause.push(increment);
        events.pause.sort();
        write_events(&app_handle, &circuit_id, &events)?;
    }
    Ok(events)
}

#[tauri::command]
pub fn delete_pause_event(app_handle: AppHandle, circuit_id: String, increment: u32) -> Result<Events, String> {
    let mut events = read_events(&app_handle, &circuit_id)?;
    events.pause.retain(|&p| p != increment);
    write_events(&app_handle, &circuit_id, &events)?;
    Ok(events)
}
