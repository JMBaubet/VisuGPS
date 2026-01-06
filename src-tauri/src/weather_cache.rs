use tauri::{State, command};
use std::sync::Mutex;
use crate::AppState;
use std::fs;
use chrono::prelude::*;

fn clean_old_weather_files(cache_dir: &std::path::Path) {
    if let Ok(entries) = fs::read_dir(cache_dir) {
        let today = Local::now().date_naive();
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                // Filename format: YYYYMMDD-HH-to-HH.json
                if filename.len() >= 8 {
                    if let Ok(file_date) = NaiveDate::parse_from_str(&filename[0..8], "%Y%m%d") {
                        if file_date < today {
                            let _ = fs::remove_file(path);
                        }
                    }
                }
            }
        }
    }
}

#[command]
pub fn check_weather_cache_metadata(state: State<Mutex<AppState>>, circuit_id: String, filename: String) -> Result<Option<String>, String> {
    let state_guard = state.lock().map_err(|e| e.to_string())?;
    let cache_dir = state_guard.app_env_path.join("data").join(&circuit_id).join("meteo");
    
    clean_old_weather_files(&cache_dir);

    let cache_path = cache_dir.join(&filename);

    if cache_path.exists() {
        let metadata = fs::metadata(cache_path).map_err(|e| e.to_string())?;
        if let Ok(modified) = metadata.modified() {
            let dt: DateTime<Local> = modified.into();
            Ok(Some(dt.to_rfc3339()))
        } else {
             Ok(Some("Date inconnue".to_string()))
        }
    } else {
        Ok(None)
    }
}

#[command]
pub fn check_weather_cache(state: State<Mutex<AppState>>, circuit_id: String, filename: String) -> Result<Option<String>, String> {
    let state_guard = state.lock().map_err(|e| e.to_string())?;
    // Target: data/circuit_id/json/filename
    let cache_dir = state_guard.app_env_path.join("data").join(&circuit_id).join("meteo");
    
    clean_old_weather_files(&cache_dir);

    let cache_path = cache_dir.join(&filename);

    if cache_path.exists() {
        let content = fs::read_to_string(cache_path).map_err(|e| e.to_string())?;
        Ok(Some(content))
    } else {
        Ok(None)
    }
}

#[command]
pub fn save_weather_cache(state: State<Mutex<AppState>>, circuit_id: String, filename: String, content: String) -> Result<(), String> {
    let state_guard = state.lock().map_err(|e| e.to_string())?;
    let cache_dir = state_guard.app_env_path.join("data").join(&circuit_id).join("meteo");

    // Ensure directory exists
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }

    let cache_path = cache_dir.join(&filename);
    fs::write(cache_path, content).map_err(|e| e.to_string())?;
    
    Ok(())
}
