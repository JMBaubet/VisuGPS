use tauri::{App, Manager, State, AppHandle, Wry};
use std::fs;
use std::path::PathBuf;
use reqwest; // Added reqwest
use serde_json::Value; // Added serde_json::Value
use tauri_plugin_dialog; // Added tauri_plugin_dialog
use tauri_plugin_process; // Added tauri_plugin_process

const EMBEDDED_DEFAULT_SETTINGS: &str = include_str!("../settingsDefault.json");
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

    let new_env_path = visugps_dir.join(format!(".env.{}", mode_name));
    let new_app_env_path = visugps_dir.join(&mode_name);

    if new_env_path.exists() || new_app_env_path.exists() {
        return Err("Execution mode already exists.".to_string());
    }

    // Create the .env file for the new mode
    let env_content = format!("APP_ENV={}\n", mode_name);
    fs::write(&new_env_path, env_content).map_err(|e| e.to_string())?;

    // Create the environment-specific directory
    fs::create_dir_all(&new_app_env_path).map_err(|e| e.to_string())?;

    // Copy settingsDefault.json into the new environment directory
    let settings_path = new_app_env_path.join("settings.json");
    if !settings_path.exists() {
        fs::write(settings_path, EMBEDDED_DEFAULT_SETTINGS).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn select_execution_mode(app: AppHandle, mode_name: String) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let visugps_dir = app_data_dir.join("VisuGPS");

    let main_env_path = visugps_dir.join(".env");
    let selected_env_path = if mode_name == "OPE" {
        visugps_dir.join(".env") // OPE uses the main .env file directly
    } else {
        visugps_dir.join(format!(".env.{}", mode_name))
    };

    if !selected_env_path.exists() {
        return Err(format!("Selected mode .env file does not exist: {}", selected_env_path.display()));
    }

    let env_content = fs::read_to_string(&selected_env_path).map_err(|e| e.to_string())?;
    fs::write(&main_env_path, env_content).map_err(|e| e.to_string())?;

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

// ... other code ...

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
    let mut mapbox_token = String::new(); // Initialize mapbox_token

    if let Ok(iter) = dotenvy::from_path_iter(&env_path) {
        for item in iter {
            if let Ok((key, val)) = item {
                match key.as_str() { // Use match for multiple keys
                    "APP_ENV" => app_env = val,
                    "MAPBOX_TOKEN" => mapbox_token = val, // Read MAPBOX_TOKEN
                    _ => {} // Ignore other keys
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
        fs::write(settings_path, EMBEDDED_DEFAULT_SETTINGS)?;
    }

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
            app.handle().plugin(tauri_plugin_dialog::init())?;
            app.handle().plugin(tauri_plugin_process::init())?;

            let state = setup_environment(app)?;
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_app_state, check_mapbox_status, check_internet_connectivity, read_settings, list_execution_modes, create_execution_mode, delete_execution_mode, select_execution_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}