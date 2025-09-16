use tauri::{App, Manager, State, AppHandle};
use std::fs;
use std::path::PathBuf;
use reqwest;
use serde_json::Value;

use chrono::prelude::*;

const EMBEDDED_DEFAULT_SETTINGS: &str = include_str!("../settingsDefault.json");
const EMBEDDED_DEFAULT_CIRCUITS: &str = include_str!("../circuitsDefault.json");
const EMBEDDED_DEFAULT_ENV: &str = include_str!("../envDefault");


#[derive(serde::Serialize, Clone)]
pub struct MapboxStatusResult {
    success: bool,
    reason: Option<String>, // "unreachable", "invalid_token", etc.
}

#[tauri::command]
async fn check_internet_connectivity() -> bool {
    let client = reqwest::Client::new();
    let url = "http://connectivitycheck.gstatic.com/generate_204"; // Google's connectivity check endpoint
    match client.get(url).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[tauri::command]
async fn check_mapbox_status(token: String) -> MapboxStatusResult {
    let client = reqwest::Client::new();
    let url = format!("https://api.mapbox.com/geocoding/v5/mapbox.places/test.json?access_token={}", token);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let body: Value = response.json().await.unwrap_or_else(|_| serde_json::from_str("{}").unwrap());
                if body["message"].is_string() && body["message"].as_str().unwrap().contains("Not Authorized") {
                    MapboxStatusResult { success: false, reason: Some("invalid_token".to_string()) }
                } else if body["features"].is_array() {
                    MapboxStatusResult { success: true, reason: None }
                } else {
                    MapboxStatusResult { success: false, reason: Some("unexpected_response".to_string()) } // Handle unexpected valid responses
                }
            } else if response.status().as_u16() == 401 || response.status().as_u16() == 403 {
                MapboxStatusResult { success: false, reason: Some("invalid_token".to_string()) }
            } else {
                MapboxStatusResult { success: false, reason: Some(format!("server_error_{}", response.status().as_u16())) }
            }
        },
        Err(e) => {
            if e.is_connect() || e.is_timeout() || e.is_request() { // Added e.is_request()
                MapboxStatusResult { success: false, reason: Some("unreachable".to_string()) }
            } else {
                MapboxStatusResult { success: false, reason: Some(format!("network_error_{}", e)) }
            }
        }
    }
}

#[tauri::command]
fn get_app_state(state: State<AppState>) -> AppState {
    state.inner().clone()
}

#[tauri::command]
fn read_settings(state: State<AppState>) -> Result<Value, String> {
    let settings_path = state.app_env_path.join("settings.json");
    let file_content = fs::read_to_string(settings_path)
        .map_err(|e| e.to_string())?;
    let json_content: Value = serde_json::from_str(&file_content)
        .map_err(|e| e.to_string())?;
    Ok(json_content)
}

#[tauri::command]
fn list_gpx_files(state: State<AppState>) -> Result<Vec<String>, String> {
    let settings_path = state.app_env_path.join("settings.json");
    let file_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;

    let gpx_dir_setting = get_setting_value(&settings, "data.groupes.Importation.parametres.GPXFile")
        .ok_or_else(|| "GPXFile setting not found".to_string())?;

    let gpx_path = if gpx_dir_setting == "DEFAULT_DOWNLOADS" {
        dirs::download_dir().ok_or_else(|| "Could not find download directory".to_string())?
    } else {
        PathBuf::from(gpx_dir_setting)
    };

    if !gpx_path.is_dir() {
        return Err(format!("GPX directory not found: {}", gpx_path.display()));
    }

    let mut gpx_files = Vec::new();
    for entry in fs::read_dir(gpx_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                if extension.eq_ignore_ascii_case("gpx") {
                    gpx_files.push(path.file_name().unwrap().to_string_lossy().into_owned());
                }
            }
        }
    }

    gpx_files.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    Ok(gpx_files)
}

#[tauri::command]
fn list_execution_modes(state: State<AppState>) -> Result<Vec<ExecutionMode>, String> {
    let visugps_dir = state.app_env_path.parent().ok_or("Could not get parent directory".to_string())?;
    let mut modes = Vec::new();

    for entry in fs::read_dir(visugps_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let mode_type = get_execution_mode(name);
                modes.push(ExecutionMode { name: name.to_string(), mode_type });
            }
        }
    }
    Ok(modes)
}

#[tauri::command]
fn create_execution_mode(app: AppHandle, mode_name: String, description: String) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let visugps_dir = app_data_dir.join("VisuGPS");

    // Get current mapbox token
    let current_app_state = app.state::<AppState>();
    let old_settings_path = current_app_state.app_env_path.join("settings.json");
    let old_settings_content = fs::read_to_string(old_settings_path).map_err(|e| e.to_string())?;
    let old_settings: Value = serde_json::from_str(&old_settings_content).map_err(|e| e.to_string())?;
    let mapbox_token = get_setting_value(&old_settings, "data.groupes.Système.groupes.Tokens.parametres.mapbox").unwrap_or_else(|| "".to_string());

    let new_env_path = visugps_dir.join(format!(".env.{}", mode_name));
    let new_app_env_path = visugps_dir.join(&mode_name);

    if new_env_path.exists() || new_app_env_path.exists() {
        return Err("Execution mode already exists.".to_string());
    }

    // Create the .env file for the new mode
    let env_content = format!("APP_ENV={}
", mode_name);
    fs::write(&new_env_path, env_content).map_err(|e| e.to_string())?;

    // Create the environment-specific directory
    fs::create_dir_all(&new_app_env_path).map_err(|e| e.to_string())?;

    // Parse the default settings
    let mut settings: Value = serde_json::from_str(EMBEDDED_DEFAULT_SETTINGS)
        .map_err(|e| format!("Failed to parse default settings: {}", e))?;

    // Modify the settings
    if let Some(reference) = settings.get_mut("référence") {
        if let Some(obj) = reference.as_object_mut() {
            obj.insert("context".to_string(), Value::String(mode_name.clone()));
            obj.insert("description".to_string(), Value::String(description));
            let now = Utc::now();
            obj.insert("date_creation".to_string(), Value::String(now.format("%Y-%m-%dT%H:%M:%S:00Z").to_string()));
        }
    }

    // Set the mapbox token in the new settings
    if let Some(system) = settings["data"]["groupes"].as_array_mut().and_then(|g| g.iter_mut().find(|g| g["libelle"] == "Système")) {
        if let Some(tokens) = system["groupes"].as_array_mut().and_then(|g| g.iter_mut().find(|g| g["libelle"] == "Tokens")) {
            if let Some(mapbox) = tokens["parametres"].as_array_mut().and_then(|p| p.iter_mut().find(|p| p["identifiant"] == "mapbox")) {
                mapbox["surcharge"] = Value::String(mapbox_token);
            }
        }
    }

    // Write the modified settings to the new environment's settings.json
    let settings_path = new_app_env_path.join("settings.json");
    let new_settings_content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(settings_path, new_settings_content)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;


    Ok(())
}


#[tauri::command]
fn select_execution_mode(app: AppHandle, mode_name: String) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let visugps_dir = app_data_dir.join("VisuGPS");

    let main_env_path = visugps_dir.join(".env");
    let selected_env_path = if mode_name == "OPE" {
        let current_env_content = fs::read_to_string(&main_env_path).map_err(|e| e.to_string())?;
        if current_env_content.trim() != "APP_ENV=OPE" {
            fs::write(&main_env_path, "APP_ENV=OPE").map_err(|e| e.to_string())?;
        }
        return Ok(()); // No further action needed for OPE if already set or just set
    } else {
        visugps_dir.join(format!(".env.{}", mode_name))
    };

    if !selected_env_path.exists() {
        // If the .env.MODE_NAME file doesn't exist, create it on the fly
        let env_content = format!("APP_ENV={}
", mode_name);
        fs::write(&selected_env_path, env_content).map_err(|e| e.to_string())?;
    }

    let mut current_main_env_content = fs::read_to_string(&main_env_path).unwrap_or_default();

    // Update APP_ENV line
    let app_env_re = regex::Regex::new(r"(?m)^APP_ENV=.*$").unwrap();
    let new_app_env_line = format!("APP_ENV={}
", mode_name);

    if app_env_re.is_match(&current_main_env_content) {
        current_main_env_content = app_env_re.replace(&current_main_env_content, new_app_env_line).to_string();
    } else {
        current_main_env_content.push_str(&new_app_env_line);
    }

    fs::write(&main_env_path, current_main_env_content).map_err(|e| e.to_string())?;

    // Delete the temporary .env.mode_name file
    if mode_name != "OPE" && selected_env_path.exists() {
        fs::remove_file(&selected_env_path).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn delete_execution_mode(app: AppHandle, state: State<AppState>, mode_name: String) -> Result<(), String> {
    if mode_name == "OPE" {
        return Err("Cannot delete OPE mode.".to_string());
    }

    if mode_name == state.app_env {
        return Err("Cannot delete the currently active mode.".to_string());
    }

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let visugps_dir = app_data_dir.join("VisuGPS");

    let env_file_path = visugps_dir.join(format!(".env.{}", mode_name));
    let mode_dir_path = visugps_dir.join(&mode_name);

    if env_file_path.exists() {
        fs::remove_file(&env_file_path).map_err(|e| e.to_string())?;
    }

    if mode_dir_path.exists() {
        fs::remove_dir_all(&mode_dir_path).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn get_execution_mode(app_env: &str) -> String {
    let lowercased_app_env = app_env.to_lowercase();
    if lowercased_app_env.starts_with("eval_") {
        "EVAL".to_string()
    } else if lowercased_app_env.starts_with("test_") {
        "TEST".to_string()
    } else {
        "OPE".to_string()
    }
}

#[derive(serde::Serialize, Clone)]
pub struct AppState {
    app_env: String,
    execution_mode: String,
    app_env_path: PathBuf,
    mapbox_token: String, // Added mapbox_token
}

#[derive(serde::Serialize, Clone)]
pub struct ExecutionMode {
    name: String,
    mode_type: String, // OPE, EVAL, TEST
}


fn get_setting_value(settings: &Value, path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = settings;

    for part in parts {
        if let Some(c) = current.get(part) {
            current = c;
        } else if let Some(arr) = current.as_array() {
            let mut found = false;
            for item in arr {
                if let Some(val) = item.get(part) {
                    current = val;
                    found = true;
                    break;
                } else if item.get("libelle").and_then(|v| v.as_str()) == Some(part) || item.get("identifiant").and_then(|v| v.as_str()) == Some(part) {
                    current = item;
                    found = true;
                    break;
                }
            }
            if !found {
                return None;
            }
        } else {
            return None;
        }
    }

    if let Some(value) = current.get("surcharge").and_then(|v| v.as_str()) {
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }
    if let Some(value) = current.get("defaut").and_then(|v| v.as_str()) {
        return Some(value.to_string());
    }

    None
}


#[tauri::command]
fn update_setting(state: State<AppState>, group_path: String, param_id: String, new_value: Value) -> Result<(), String> {
    let settings_path = state.app_env_path.join("settings.json");
    let file_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let mut settings: Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;

    // Update revision date
    if let Some(reference) = settings.get_mut("référence") {
        if let Some(obj) = reference.as_object_mut() {
            let now = Utc::now();
            obj.insert("date_revision".to_string(), Value::String(now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()));
        }
    }

    // Navigate to the correct group
    let path_parts: Vec<&str> = group_path.split('/').filter(|s| !s.is_empty()).collect();
    let mut target_group = &mut settings["data"];

    for part in path_parts {
        target_group = target_group
            .get_mut("groupes")
            .and_then(|g| g.as_array_mut())
            .and_then(|groups| {
                groups
                    .iter_mut()
                    .find(|g| g.get("libelle").and_then(|l| l.as_str()) == Some(part))
            })
            .ok_or_else(|| format!("Group not found in path: {}", part))?;
    }

    // Find and update the parameter in the target group
    if let Some(params) = target_group.get_mut("parametres").and_then(|p| p.as_array_mut()) {
        if let Some(param) = params.iter_mut().find(|p| p.get("identifiant").and_then(|i| i.as_str()) == Some(&param_id)) {
            param["surcharge"] = new_value;
        } else {
            return Err(format!("Parameter '{}' not found", param_id));
        }
    } else {
        return Err("No 'parametres' array found in the target group".to_string());
    }

    // Write back the updated settings
    let new_settings_content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(&settings_path, new_settings_content).map_err(|e| e.to_string())?;

    Ok(())
}


fn setup_environment(app: &mut App) -> Result<AppState, Box<dyn std::error::Error>> {
    let app_data_dir = app.path().app_data_dir()?;
    let visugps_dir = app_data_dir.join("VisuGPS");

    if !visugps_dir.exists() {
        fs::create_dir_all(&visugps_dir)?;
    }

    let env_path = visugps_dir.join(".env");

    if !env_path.exists() {
        fs::write(&env_path, EMBEDDED_DEFAULT_ENV)?;
    }

    let mut app_env = "OPE".to_string();

    

    if let Ok(iter) = dotenvy::from_path_iter(&env_path) {
        for item in iter {
            if let Ok((key, val)) = item {
                if key == "APP_ENV" {
                    app_env = val;
                    
                }
            }
        }
    }

    
    
    

    let execution_mode = get_execution_mode(&app_env);
    
    // Create the environment-specific directory
    let app_env_path = visugps_dir.join(&app_env);
    if !app_env_path.exists() {
        fs::create_dir_all(&app_env_path)?;
    }

    // Manage settings.json file
    let settings_path = app_env_path.join("settings.json");
    if !settings_path.exists() {
        // Parse the default settings
        let mut settings: Value = serde_json::from_str(EMBEDDED_DEFAULT_SETTINGS)
            .map_err(|e| format!("Failed to parse default settings: {}", e))?;

        // Modify the settings
        if let Some(reference) = settings.get_mut("référence") {
            if let Some(obj) = reference.as_object_mut() {
                obj.insert("context".to_string(), Value::String(app_env.clone())); // Use app_env for context
                let now = Utc::now();
                obj.insert("date_creation".to_string(), Value::String(now.format("%Y-%m-%dT%H:%M:%S:00Z").to_string()));
            }
        }

        // Write the modified settings to the new environment's settings.json
        let new_settings_content = serde_json::to_string_pretty(&settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;
        
        fs::write(&settings_path, new_settings_content)
            .map_err(|e| format!("Failed to write settings file: {}", e))?;
    }

    // Manage circuits.json file
    let circuits_path = app_env_path.join("circuits.json");
    if !circuits_path.exists() {
        fs::write(&circuits_path, EMBEDDED_DEFAULT_CIRCUITS)
            .map_err(|e| format!("Failed to write circuits file: {}", e))?;
    }

    // Read settings and extract mapbox token
    let settings_content = fs::read_to_string(&settings_path)?;
    let settings: Value = serde_json::from_str(&settings_content)?;
    let mapbox_token = get_setting_value(&settings, "data.groupes.Système.groupes.Tokens.parametres.mapbox")
        .unwrap_or_else(|| "".to_string());


    Ok(AppState {
        app_env,
        execution_mode,
        app_env_path,
        mapbox_token, // Populate mapbox_token
    })
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // Removed unused plugins
            // tauri::plugin::dialog::init();
            // tauri::plugin::process::init();

            let state = setup_environment(app)?;
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_app_state, check_mapbox_status, check_internet_connectivity, read_settings, list_execution_modes, create_execution_mode, delete_execution_mode, select_execution_mode, update_setting, list_gpx_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

