use serde::{Serialize, Deserialize};
use std::fs;
use chrono::{Utc, DateTime};
use tauri::State;
use std::sync::Mutex;
use crate::AppState; // Pour accéder à app_env_path

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEntry {
    pub error_type: String,
    pub message_id: String,
    pub circuit_id: String,
    pub anchor_increment: u32,
    pub timestamp: DateTime<Utc>,
    pub description: String,
}

#[tauri::command]
pub fn save_error_event(
    state: State<Mutex<AppState>>,
    circuit_id: String,
    new_errors: Vec<ErrorEntry>,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    let errors_dir = state.app_env_path.join("data").join(&circuit_id);
    let errors_file_path = errors_dir.join("errors.json");

    // Ensure the directory exists
    fs::create_dir_all(&errors_dir)
        .map_err(|e| format!("Failed to create errors directory: {}", e))?;

    let mut existing_errors: Vec<ErrorEntry> = if errors_file_path.exists() {
        let content = fs::read_to_string(&errors_file_path)
            .map_err(|e| format!("Failed to read existing errors file: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse existing errors file: {}", e))?
    } else {
        Vec::new()
    };

    // Add new errors, avoiding duplicates based on messageId, circuitId, and anchorIncrement
    for new_err in new_errors {
        if !existing_errors.iter().any(|e| 
            e.message_id == new_err.message_id && 
            e.circuit_id == new_err.circuit_id && 
            e.anchor_increment == new_err.anchor_increment
        ) {
            existing_errors.push(new_err);
        }
    }

    let updated_content = serde_json::to_string_pretty(&existing_errors)
        .map_err(|e| format!("Failed to serialize updated errors: {}", e))?;

    fs::write(&errors_file_path, updated_content)
        .map_err(|e| format!("Failed to write errors file: {}", e))?;

    Ok(())
}
