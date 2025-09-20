use tauri::{App, Manager, State, AppHandle};

use std::fs;
use std::path::PathBuf;
use reqwest;
use serde_json::Value;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

mod gpx_processor;

use chrono::prelude::*;
use gpx_processor::{Circuit, DraftCircuit};
use geo::{LineString as GeoLineString, Point, Coord};
use geo::prelude::*;
use polyline::encode_coordinates;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::collections::HashMap;
use once_cell::sync::Lazy;

const EMBEDDED_DEFAULT_SETTINGS: &str = include_str!("../settingsDefault.json");
const EMBEDDED_DEFAULT_CIRCUITS: &str = include_str!("../circuitsDefault.json");
const EMBEDDED_DEFAULT_ENV: &str = include_str!("../envDefault");

static VUETIFY_COLOR_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("red-lighten-5", "FFEBEE"); map.insert("red-lighten-4", "FFCDD2"); map.insert("red-lighten-3", "EF9A9A"); map.insert("red-lighten-2", "E57373"); map.insert("red-lighten-1", "EF5350"); map.insert("red", "F44336"); map.insert("red-darken-1", "E53935"); map.insert("red-darken-2", "D32F2F"); map.insert("red-darken-3", "C62828"); map.insert("red-darken-4", "B71C1C");
    map.insert("pink-lighten-5", "FCE4EC"); map.insert("pink-lighten-4", "F8BBD0"); map.insert("pink-lighten-3", "F48FB1"); map.insert("pink-lighten-2", "F06292"); map.insert("pink-lighten-1", "EC407A"); map.insert("pink", "E91E63"); map.insert("pink-darken-1", "D81B60"); map.insert("pink-darken-2", "C2185B"); map.insert("pink-darken-3", "AD1457"); map.insert("pink-darken-4", "880E4F");
    map.insert("purple-lighten-5", "F3E5F5"); map.insert("purple-lighten-4", "E1BEE7"); map.insert("purple-lighten-3", "CE93D8"); map.insert("purple-lighten-2", "BA68C8"); map.insert("purple-lighten-1", "AB47BC"); map.insert("purple", "9C27B0"); map.insert("purple-darken-1", "8E24AA"); map.insert("purple-darken-2", "7B1FA2"); map.insert("purple-darken-3", "6A1B9A"); map.insert("purple-darken-4", "4A148C");
    map.insert("deep-purple-lighten-5", "EDE7F6"); map.insert("deep-purple-lighten-4", "D1C4E9"); map.insert("deep-purple-lighten-3", "B39DDB"); map.insert("deep-purple-lighten-2", "9575CD"); map.insert("deep-purple-lighten-1", "7E57C2"); map.insert("deep-purple", "673AB7"); map.insert("deep-purple-darken-1", "5E35B1"); map.insert("deep-purple-darken-2", "512DA8"); map.insert("deep-purple-darken-3", "4527A0"); map.insert("deep-purple-darken-4", "311B92");
    map.insert("indigo-lighten-5", "E8EAF6"); map.insert("indigo-lighten-4", "C5CAE9"); map.insert("indigo-lighten-3", "9FA8DA"); map.insert("indigo-lighten-2", "7986CB"); map.insert("indigo-lighten-1", "5C6BC0"); map.insert("indigo", "3F51B5"); map.insert("indigo-darken-1", "3949AB"); map.insert("indigo-darken-2", "303F9F"); map.insert("indigo-darken-3", "283593"); map.insert("indigo-darken-4", "1A237E");
    map.insert("blue-lighten-5", "E3F2FD"); map.insert("blue-lighten-4", "BBDEFB"); map.insert("blue-lighten-3", "90CAF9"); map.insert("blue-lighten-2", "64B5F6"); map.insert("blue-lighten-1", "42A5F5"); map.insert("blue", "2196F3"); map.insert("blue-darken-1", "1E88E5"); map.insert("blue-darken-2", "1976D2"); map.insert("blue-darken-3", "1565C0"); map.insert("blue-darken-4", "0D47A1");
    map.insert("light-blue-lighten-5", "E1F5FE"); map.insert("light-blue-lighten-4", "B3E5FC"); map.insert("light-blue-lighten-3", "81D4FA"); map.insert("light-blue-lighten-2", "4FC3F7"); map.insert("light-blue-lighten-1", "29B6F6"); map.insert("light-blue", "03A9F4"); map.insert("light-blue-darken-1", "039BE5"); map.insert("light-blue-darken-2", "0288D1"); map.insert("light-blue-darken-3", "0277BD"); map.insert("light-blue-darken-4", "01579B");
    map.insert("cyan-lighten-5", "E0F7FA"); map.insert("cyan-lighten-4", "B2EBF2"); map.insert("cyan-lighten-3", "80DEEA"); map.insert("cyan-lighten-2", "4DD0E1"); map.insert("cyan-lighten-1", "26C6DA"); map.insert("cyan", "00BCD4"); map.insert("cyan-darken-1", "00ACC1"); map.insert("cyan-darken-2", "0097A7"); map.insert("cyan-darken-3", "00838F"); map.insert("cyan-darken-4", "006064");
    map.insert("teal-lighten-5", "E0F2F1"); map.insert("teal-lighten-4", "B2DFDB"); map.insert("teal-lighten-3", "80CBC4"); map.insert("teal-lighten-2", "4DB6AC"); map.insert("teal-lighten-1", "26A69A"); map.insert("teal", "009688"); map.insert("teal-darken-1", "00897B"); map.insert("teal-darken-2", "00796B"); map.insert("teal-darken-3", "00695C"); map.insert("teal-darken-4", "004D40");
    map.insert("green-lighten-5", "E8F5E9"); map.insert("green-lighten-4", "C8E6C9"); map.insert("green-lighten-3", "A5D6A7"); map.insert("green-lighten-2", "81C784"); map.insert("green-lighten-1", "66BB6A"); map.insert("green", "4CAF50"); map.insert("green-darken-1", "43A047"); map.insert("green-darken-2", "388E3C"); map.insert("green-darken-3", "2E7D32"); map.insert("green-darken-4", "1B5E20");
    map.insert("light-green-lighten-5", "F1F8E9"); map.insert("light-green-lighten-4", "DCEDC8"); map.insert("light-green-lighten-3", "C5E1A5"); map.insert("light-green-lighten-2", "AED581"); map.insert("light-green-lighten-1", "9CCC65"); map.insert("light-green", "8BC34A"); map.insert("light-green-darken-1", "7CB342"); map.insert("light-green-darken-2", "689F38"); map.insert("light-green-darken-3", "558B2F"); map.insert("light-green-darken-4", "33691E");
    map.insert("lime-lighten-5", "F9FBE7"); map.insert("lime-lighten-4", "F0F4C3"); map.insert("lime-lighten-3", "E6EE9C"); map.insert("lime-lighten-2", "DCE775"); map.insert("lime-lighten-1", "D4E157"); map.insert("lime", "CDDC39"); map.insert("lime-darken-1", "C0CA33"); map.insert("lime-darken-2", "AFB42B"); map.insert("lime-darken-3", "9E9D24"); map.insert("lime-darken-4", "827717");
    map.insert("yellow-lighten-5", "FFFDE7"); map.insert("yellow-lighten-4", "FFF9C4"); map.insert("yellow-lighten-3", "FFF59D"); map.insert("yellow-lighten-2", "FFF176"); map.insert("yellow-lighten-1", "FFEE58"); map.insert("yellow", "FFEB3B"); map.insert("yellow-darken-1", "FDD835"); map.insert("yellow-darken-2", "FBC02D"); map.insert("yellow-darken-3", "F9A825"); map.insert("yellow-darken-4", "F57F17");
    map.insert("amber-lighten-5", "FFF8E1"); map.insert("amber-lighten-4", "FFECB3"); map.insert("amber-lighten-3", "FFE082"); map.insert("amber-lighten-2", "FFD54F"); map.insert("amber-lighten-1", "FFCA28"); map.insert("amber", "FFC107"); map.insert("amber-darken-1", "FFB300"); map.insert("amber-darken-2", "FFA000"); map.insert("amber-darken-3", "FF8F00"); map.insert("amber-darken-4", "FF6F00");
    map.insert("orange-lighten-5", "FFF3E0"); map.insert("orange-lighten-4", "FFE0B2"); map.insert("orange-lighten-3", "FFCC80"); map.insert("orange-lighten-2", "FFB74D"); map.insert("orange-lighten-1", "FFA726"); map.insert("orange", "FF9800"); map.insert("orange-darken-1", "FB8C00"); map.insert("orange-darken-2", "F57C00"); map.insert("orange-darken-3", "EF6C00"); map.insert("orange-darken-4", "E65100");
    map.insert("deep-orange-lighten-5", "FBE9E7"); map.insert("deep-orange-lighten-4", "FFCCBC"); map.insert("deep-orange-lighten-3", "FFAB91"); map.insert("deep-orange-lighten-2", "FF8A65"); map.insert("deep-orange-lighten-1", "FF7043"); map.insert("deep-orange", "FF5722"); map.insert("deep-orange-darken-1", "F4511E"); map.insert("deep-orange-darken-2", "E64A19"); map.insert("deep-orange-darken-3", "D84315"); map.insert("deep-orange-darken-4", "BF360C");
    map.insert("brown-lighten-5", "EFEBE9"); map.insert("brown-lighten-4", "D7CCC8"); map.insert("brown-lighten-3", "BCAAA4"); map.insert("brown-lighten-2", "A1887F"); map.insert("brown-lighten-1", "8D6E63"); map.insert("brown", "795548"); map.insert("brown-darken-1", "6D4C41"); map.insert("brown-darken-2", "5D4037"); map.insert("brown-darken-3", "4E342E"); map.insert("brown-darken-4", "3E2723");
    map.insert("blue-grey-lighten-5", "ECEFF1"); map.insert("blue-grey-lighten-4", "CFD8DC"); map.insert("blue-grey-lighten-3", "B0BEC5"); map.insert("blue-grey-lighten-2", "90A4AE"); map.insert("blue-grey-lighten-1", "78909C"); map.insert("blue-grey", "607D8B"); map.insert("blue-grey-darken-1", "546E7A"); map.insert("blue-grey-darken-2", "455A64"); map.insert("blue-grey-darken-3", "37474F"); map.insert("blue-grey-darken-4", "263238");
    map.insert("grey-lighten-5", "FAFAFA"); map.insert("grey-lighten-4", "F5F5F5"); map.insert("grey-lighten-3", "EEEEEE"); map.insert("grey-lighten-2", "E0E0E0"); map.insert("grey-lighten-1", "BDBDBD"); map.insert("grey", "9E9E9E"); map.insert("grey-darken-1", "757575"); map.insert("grey-darken-2", "616161"); map.insert("grey-darken-3", "424242"); map.insert("grey-darken-4", "212121");
    map.insert("black", "000000");
    map.insert("white", "FFFFFF");
    map.insert("transparent", "FFFFFF00");
    map
});

fn convert_vuetify_color_to_hex(color_name: &str) -> String {
    let hex = VUETIFY_COLOR_MAP.get(color_name).unwrap_or(&"000000"); // Default to black if not found

    // Only strip the suffix "00" if the hex code is 8 characters long (has an alpha channel)
    if hex.len() == 8 && hex.ends_with("00") {
        hex[..6].to_string() // Take only the first 6 characters
    } else {
        hex.to_string() // Otherwise, return the hex code as is
    }
}


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
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
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
    let mapbox_token = get_setting_value(&old_settings, "data.groupes.Système.groupes.Tokens.parametres.mapbox")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "".to_string());

    let new_app_env_path = visugps_dir.join(&mode_name);

    if new_app_env_path.exists() {
        return Err("Execution mode already exists.".to_string());
    }

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

    let env_var_to_update = if cfg!(debug_assertions) {
        "APP_ENV_DEV"
    } else {
        "APP_ENV_PROD"
    };

    let mut current_main_env_content = fs::read_to_string(&main_env_path).unwrap_or_default();

    let pattern = format!("(?m)^{}=.*$", env_var_to_update);
    let app_env_re = regex::Regex::new(&pattern).unwrap();
    let new_app_env_line = format!("{0}={1}", env_var_to_update, mode_name);

    if app_env_re.is_match(&current_main_env_content) {
        current_main_env_content = app_env_re.replace(&current_main_env_content, new_app_env_line).to_string();
    } else {
        current_main_env_content.push_str("
");
        current_main_env_content.push_str(&new_app_env_line);
    }

    fs::write(&main_env_path, current_main_env_content).map_err(|e| e.to_string())?;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionMode {
    pub name: String,
    #[serde(rename = "modeType")]
    pub mode_type: String,
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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Traceur {
    pub id: String,
    pub nom: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CircuitsFile {
    pub version: String,
    pub description: String,
    pub auteur: String,
    pub commentaires: String,
    pub villes: Vec<serde_json::Value>, // Placeholder for now
    pub traceurs: Vec<Traceur>,
    pub editeurs: Vec<serde_json::Value>, // Placeholder for now
    #[serde(rename = "indexCircuits")]
    pub index_circuits: u32,
    pub circuits: Vec<Circuit>, // Maintenant Vec<Circuit>
}

// Fonction pour lire le fichier circuits.json
fn read_circuits_file(app_env_path: &PathBuf) -> Result<CircuitsFile, String> {
    let circuits_path = app_env_path.join("circuits.json");
    let file_content = fs::read_to_string(&circuits_path)
        .map_err(|e| format!("Failed to read circuits.json: {}", e))?;
    serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse circuits.json: {}", e))
}

// Fonction pour écrire le fichier circuits.json
fn write_circuits_file(app_env_path: &PathBuf, circuits_data: &CircuitsFile) -> Result<(), String> {
    let circuits_path = app_env_path.join("circuits.json");
    let new_content = serde_json::to_string_pretty(circuits_data)
        .map_err(|e| format!("Failed to serialize circuits.json: {}", e))?;
    fs::write(&circuits_path, new_content)
        .map_err(|e| format!("Failed to write circuits.json: {}", e))
}




#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitsJson {
    pub version: String,
    pub description: String,
    pub auteur: String,
    pub commentaires: String,
    pub villes: Vec<serde_json::Value>, // Placeholder for now
    pub traceurs: Vec<Traceur>,
    pub editeurs: Vec<serde_json::Value>, // Placeholder for now
    #[serde(rename = "indexCircuits")]
    pub index_circuits: u32,
    pub circuits: Vec<serde_json::Value>, // Placeholder for now
}


fn get_setting_value<'a>(settings: &'a Value, path: &str) -> Option<&'a Value> {
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

    let surcharge = current.get("surcharge");
    if let Some(s) = surcharge {
        if !s.is_null() {
            if let Some(s_str) = s.as_str() {
                if !s_str.is_empty() {
                    return Some(s);
                }
            } else {
                return Some(s); // Not a string, so just return it if it's not null
            }
        }
    }
    
    current.get("defaut")
}


#[tauri::command]
fn list_traceurs(state: State<AppState>) -> Result<Vec<Traceur>, String> {
    let circuits_file = read_circuits_file(&state.app_env_path)?;
    Ok(circuits_file.traceurs)
}

#[tauri::command]
fn add_traceur(state: State<AppState>, nom: String) -> Result<Traceur, String> {
    let mut circuits_file = read_circuits_file(&state.app_env_path)?;

    // Vérifier si le traceur existe déjà (insensible à la casse)
    if circuits_file.traceurs.iter().any(|t| t.nom.eq_ignore_ascii_case(&nom)) {
        return Err(format!("Le traceur '{}' existe déjà.", nom));
    }

    let new_traceur = Traceur {
        id: Uuid::new_v4().to_string(),
        nom: nom.clone(),
    };
    circuits_file.traceurs.push(new_traceur.clone());

    write_circuits_file(&state.app_env_path, &circuits_file)?;

    Ok(new_traceur)
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



#[tauri::command]
async fn analyze_gpx_file(app_handle: tauri::AppHandle, filename: String) -> Result<DraftCircuit, String> {
    gpx_processor::analyze_gpx_file(&app_handle, &filename).await
}

#[tauri::command]
async fn generate_gpx_thumbnail(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    line_string_path: String,
    settings: Value, // Les paramètres de la vignette
) -> Result<String, String> {
    let app_state = app_handle.state::<AppState>();
    let mapbox_token = app_state.mapbox_token.clone();

    // 1. Lire le lineString.json
    let line_string_content = fs::read_to_string(&line_string_path)
        .map_err(|e| format!("Failed to read lineString.json: {}", e))?;
    let line_string_data: Value = serde_json::from_str(&line_string_content)
        .map_err(|e| format!("Failed to parse lineString.json: {}", e))?;

    let coordinates = line_string_data["coordinates"]
        .as_array()
        .ok_or("Coordinates not found in lineString.json")?;

    if coordinates.is_empty() {
        return Err("No coordinates found in lineString.json".to_string());
    }

    // Extraire les coordonnées de départ et d'arrivée
    let start_coord = coordinates[0].as_array().ok_or("Invalid start coordinate format")?;
    let end_coord = coordinates[coordinates.len() - 1].as_array().ok_or("Invalid end coordinate format")?;

    let start_lon = start_coord[0].as_f64().ok_or("Invalid start longitude")?;
    let start_lat = start_coord[1].as_f64().ok_or("Invalid start latitude")?;
    let end_lon = end_coord[0].as_f64().ok_or("Invalid end longitude")?;
    let end_lat = end_coord[1].as_f64().ok_or("Invalid end latitude")?;




    // 2. Extraire les paramètres de la vignette
    let style_vignette = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.styleVignette")
        .and_then(|v| v.as_str()).unwrap_or("mapbox://styles/mapbox/streets-v12");
    let color_gpx_vignette = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.colorGPXVignette")
        .and_then(|v| v.as_str()).unwrap_or("orange-darken-4");
    let largeur = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.largeur")
        .and_then(|v| v.as_u64()).unwrap_or(400) as u32;
    let format_str = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.format")
        .and_then(|v| v.as_str()).unwrap_or("1/1");
    let presence_distance = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.presenceDistance")
        .and_then(|v| v.as_bool()).unwrap_or(false);
    let distance_interval = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.Distance")
        .and_then(|v| v.as_u64()).unwrap_or(5) as f64; // en km
    let direction = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.direction")
        .and_then(|v| v.as_bool()).unwrap_or(false);
    let show_markers = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.Vignettes")
        .and_then(|v| v.as_bool()).unwrap_or(true);
    let couleur_depart = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.couleurDépart")
        .and_then(|v| v.as_str()).unwrap_or("green-darken-2");
    let couleur_arrivee = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.couleurArrivée")
        .and_then(|v| v.as_str()).unwrap_or("red-darken-2");
    let couleur_depart_arrivee = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.couleurDépartArrivée")
        .and_then(|v| v.as_str()).unwrap_or("blue-darken-2");
    let distance_max_close = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.distanceMax")
        .and_then(|v| v.as_u64()).unwrap_or(100) as f64; // en mètres

    // Convertir la couleur Vuetify en hex
    let gpx_color_hex = convert_vuetify_color_to_hex(color_gpx_vignette);

    // Calculer la hauteur en fonction du format
    let hauteur = match format_str {
        "1/1" => largeur,
        "4/3" => (largeur as f32 * 0.75) as u32,
        "16/9" => (largeur as f32 * 0.5625) as u32,
        _ => largeur, // Par défaut 1/1
    };

    // Construire la chaîne de coordonnées pour le chemin Mapbox
    // Convertir les coordonnées en GeoLineString pour la simplification
    let geo_points: Vec<Point<f64>> = coordinates.iter().map(|c| {
        let lon = c[0].as_f64().unwrap_or_default();
        let lat = c[1].as_f64().unwrap_or_default();
        Point::new(lon, lat)
    }).collect();
    let geo_line = GeoLineString::from(geo_points);

    let nb_pts = coordinates.len();
    let simplified_line = if nb_pts > 2000 { // Plus de 2000 points
        geo_line.simplify(&0.0005)
    } else if nb_pts > 500 { // Entre 500 et 2000 points
        geo_line.simplify(&0.0001)
    } else if nb_pts > 100 { // Entre 100 et 500 points
        geo_line.simplify(&0.00005)
    } else { // Moins de 100 points, ou très petite trace
        geo_line.simplify(&0.00001) // Appliquer une petite simplification par défaut
    };

    let nb_pts_simplified = simplified_line.points().count();
    println!("Nombre de points avant simplification: {}", coordinates.len());
    println!("Nombre de points après simplification: {}", nb_pts_simplified);

    let encoded_polyline = encode_coordinates(simplified_line.points().map(|p| Coord { x: p.x(), y: p.y() }), 5)
        .map_err(|e| format!("Failed to encode polyline: {:?}", e))?;
    // Remplacer uniquement les barres obliques inverses, comme requis par l'API Mapbox et la documentation du crate polyline.
    // Un encodage excessif peut entraîner des erreurs.
    let path_string = encoded_polyline.replace('\\', "%5C").replace('?', "%3F").replace('@', "%40").replace('[', "%5B").replace(']', "%5D");
    println!("Polyligne encodée: {}", encoded_polyline);
    println!("Polyligne encodée pour URL: {}", path_string);

    let largeur_trace = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.largeurTrace")
        .and_then(|v| v.as_u64()).unwrap_or(5) as u32;

    // Construire l'overlay pour la trace
    let mut overlay_parts = vec![];
    overlay_parts.push(format!("path-{}+{}({})", largeur_trace, gpx_color_hex, path_string)); // Largeur variable, couleur hex

    // Ajouter les marqueurs si activés
    if show_markers {
        // Calculer la distance entre le départ et l'arrivée pour déterminer si elles sont proches
        let dist_meters = gpx_processor::haversine_distance(start_lat, start_lon, end_lat, end_lon);

        if dist_meters <= distance_max_close {
            // Départ et arrivée proches, utiliser un seul marqueur
            let color_hex = convert_vuetify_color_to_hex(couleur_depart_arrivee);
            overlay_parts.push(format!("pin-s-circle+{}({},{})", color_hex, start_lon, start_lat));
        } else {
            // Départ et arrivée distincts
            let color_start_hex = convert_vuetify_color_to_hex(couleur_depart);
            let color_end_hex = convert_vuetify_color_to_hex(couleur_arrivee);
            overlay_parts.push(format!("pin-s+{}({:.4},{:.4})", color_start_hex, start_lon, start_lat));
            overlay_parts.push(format!("pin-s+{}({:.4},{:.4})", color_end_hex, end_lon, end_lat));
        }
    }

    // Calculer les distances cumulées et ajouter les marqueurs de distance/direction
    let mut cumulative_distance_km = 0.0;
    let mut last_marker_distance_km = 0.0;
    let mut distance_marker_count = 0;

    for i in 0..coordinates.len() {
        let current_coord = coordinates[i].as_array().ok_or("Invalid coordinate format")?;
        let current_lon = current_coord[0].as_f64().ok_or("Invalid longitude")?;
        let current_lat = current_coord[1].as_f64().ok_or("Invalid latitude")?;

        if i > 0 {
            let prev_coord = coordinates[i-1].as_array().ok_or("Invalid coordinate format")?;
            let prev_lon = prev_coord[0].as_f64().ok_or("Invalid longitude")?;
            let prev_lat = prev_coord[1].as_f64().ok_or("Invalid latitude")?;
            cumulative_distance_km += gpx_processor::haversine_distance(prev_lat, prev_lon, current_lat, current_lon) / 1000.0;
        }

        if (presence_distance || direction) && cumulative_distance_km >= (last_marker_distance_km + distance_interval) {
            distance_marker_count += 1;
            // Ajouter un marqueur de distance
            if presence_distance {
                let base_color = get_setting_value(&settings, "data.groupes.Importation.groupes.Mapbox.parametres.couleurPinDistance")
                    .and_then(|v| v.as_str()).unwrap_or("red");

                let decade = distance_marker_count / 10;
                let intensity_suffix = match decade {
                    0 => "lighten-3", // 1-9
                    1 => "lighten-2", // 10-19
                    2 => "lighten-1", // 20-29
                    3 => "darken-1",  // 30-39
                    4 => "darken-2",  // 40-49
                    5 => "darken-3",  // 50-59
                    _ => ""
                };

                let marker_color_hex = if !intensity_suffix.is_empty() {
                    let full_color_name = format!("{}-{}", base_color, intensity_suffix);
                    convert_vuetify_color_to_hex(&full_color_name)
                } else {
                    "000000".to_string() // Default to black for pins >= 60
                };

                let pin_label = distance_marker_count % 10;
                overlay_parts.push(format!("pin-s-{}+{}({},{})", pin_label, &marker_color_hex, current_lon, current_lat));
            }

            // Ajouter un marqueur de direction (flèche)
            if direction && i < coordinates.len() - 1 {
                let next_coord = coordinates[i+1].as_array().ok_or("Invalid coordinate format")?;
                let next_lon = next_coord[0].as_f64().ok_or("Invalid longitude")?;
                let next_lat = next_coord[1].as_f64().ok_or("Invalid latitude")?;

                // Calculer l'angle de la flèche (bearing)
                let _bearing = gpx_processor::calculate_bearing(current_lat, current_lon, next_lat, next_lon);
                // Mapbox Static Images API ne supporte pas directement la rotation des marqueurs.
                // On peut utiliser un marqueur personnalisé si on veut une flèche orientée.
                // Pour l'instant, on va juste mettre un marqueur générique.
                overlay_parts.push(format!("pin-s-arrow+{}({},{})", "000000", current_lon, current_lat)); // Flèche noire
            }
            last_marker_distance_km = cumulative_distance_km;
        }
    }

    let overlay_string = overlay_parts.join(",");

    // 3. Construire l'URL de l'API Mapbox Static Images
    let mapbox_url = format!(
        "https://api.mapbox.com/styles/v1/{}/static/{}/auto/{}x{}{}{}",
        style_vignette.replace("mapbox://styles/", ""), // Supprimer le préfixe
        overlay_string,
        largeur,
        hauteur,
        "@2x", // Pour une meilleure qualité
        format!("?access_token={}", mapbox_token)
    );
    println!("URL Mapbox générée: {}", mapbox_url);
    // 4. Effectuer la requête HTTP
    let client = reqwest::Client::new();
    let response = client.get(&mapbox_url).send().await
        .map_err(|e| format!("Failed to fetch Mapbox static image: {}", e))?;

    let status = response.status(); // Store the status before consuming response
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Mapbox API returned an error: {} - {}", status, error_text));
    }

    // 5. Sauvegarder l'image
    let image_bytes = response.bytes().await
        .map_err(|e| format!("Failed to read image bytes: {}", e))?;

    let data_dir = app_state.app_env_path.join("data");
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;

    let circuit_data_dir = data_dir.join(&circuit_id);
    fs::create_dir_all(&circuit_data_dir)
        .map_err(|e| format!("Failed to create circuit data directory: {}", e))?;

    let thumbnail_path = circuit_data_dir.join("vignette.png");
    fs::write(&thumbnail_path, image_bytes)
        .map_err(|e| format!("Failed to write thumbnail file: {}", e))?;

    Ok(thumbnail_path.to_string_lossy().into_owned())
}

#[tauri::command]
fn commit_new_circuit(
    app_handle: tauri::AppHandle,
    draft: DraftCircuit,
    traceur_id: String,
) -> Result<String, String> {
    gpx_processor::commit_new_circuit(&app_handle, draft, traceur_id)
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

    let env_var_to_read = if cfg!(debug_assertions) {
        "APP_ENV_DEV"
    } else {
        "APP_ENV_PROD"
    };

    if let Ok(iter) = dotenvy::from_path_iter(&env_path) {
        for item in iter {
            if let Ok((key, val)) = item {
                if key == env_var_to_read {
                    app_env = val;
                    break; 
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
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
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
        .plugin(tauri_plugin_process::init())
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
        .invoke_handler(tauri::generate_handler![get_app_state, check_mapbox_status, check_internet_connectivity, read_settings, list_execution_modes, create_execution_mode, delete_execution_mode, select_execution_mode, update_setting, list_gpx_files, analyze_gpx_file, commit_new_circuit, list_traceurs, add_traceur, generate_gpx_thumbnail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}