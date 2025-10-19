use tauri::{App, Manager, State, AppHandle};

use std::fs;
use std::path::PathBuf;
use reqwest;
use serde_json::Value;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose, Engine as _};

mod gpx_processor;
pub mod colors;
pub mod thumbnail_generator;
pub mod tracking_processor;
pub mod communes_updater;
pub mod geo_processor;
pub mod event;
pub mod trace_style;

use chrono::prelude::*;use gpx_processor::{Circuit, DraftCircuit, CircuitSommet};
#[allow(unused_imports)]
use geo_processor::{TrackingPointJs, ProcessedTrackingPoint, ProcessedTrackingDataResult, process_tracking_data};


use std::sync::Mutex;

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
fn get_app_state(state: State<Mutex<AppState>>) -> AppState {
    state.lock().unwrap().clone()
}

#[tauri::command]
fn read_settings(state: State<Mutex<AppState>>) -> Result<Value, String> {
    let state = state.lock().unwrap();
    let settings_path = state.app_env_path.join("settings.json");
    let file_content = fs::read_to_string(settings_path)
        .map_err(|e| e.to_string())?;
    let json_content: Value = serde_json::from_str(&file_content)
        .map_err(|e| e.to_string())?;
    Ok(json_content)
}

#[tauri::command]
fn list_gpx_files(state: State<Mutex<AppState>>) -> Result<Vec<String>, String> {
    let state = state.lock().unwrap();
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
fn list_execution_modes(state: State<Mutex<AppState>>) -> Result<Vec<ExecutionMode>, String> {
    let state = state.lock().unwrap();
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
    let state_mutex = app.state::<Mutex<AppState>>();
    let current_app_state = state_mutex.lock().unwrap();
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
fn delete_execution_mode(app: AppHandle, state: State<Mutex<AppState>>, mode_name: String) -> Result<(), String> {
    let state = state.lock().unwrap();
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
    updating_circuit_id: Option<String>,
    pub updating_circuit_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ville {
    pub id: String,
    pub nom: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Editor {
    pub id: String,
    pub nom: String,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Traceur {
    pub id: String,
    pub nom: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CircuitsFile {
    pub version: String,
    pub description: String,
    pub auteur: String,
    pub commentaires: String,
    pub maj_communes: bool,
    pub circuit_id: String,
    pub villes: Vec<Ville>,
    pub traceurs: Vec<Traceur>,
    pub editeurs: Vec<Editor>,
    pub index_circuits: u32,
    pub circuits: Vec<Circuit>,
}

#[derive(serde::Serialize, Clone)]
struct DebugData {
    line_string: Value,
    tracking_points: Value,
}

#[tauri::command]
fn get_debug_data(state: State<Mutex<AppState>>, circuit_id: String) -> Result<DebugData, String> {
    let state = state.lock().unwrap();
    let data_dir = state.app_env_path.join("data").join(circuit_id);

    let line_string_path = data_dir.join("lineString.json");
    let tracking_points_path = data_dir.join("tracking.json");

    let line_string_content = fs::read_to_string(line_string_path).map_err(|e| e.to_string())?;
    let tracking_points_content = fs::read_to_string(tracking_points_path).map_err(|e| e.to_string())?;

    let line_string: Value = serde_json::from_str(&line_string_content).map_err(|e| e.to_string())?;
    let tracking_points: Value = serde_json::from_str(&tracking_points_content).map_err(|e| e.to_string())?;

    Ok(DebugData {
        line_string,
        tracking_points,
    })
}

#[tauri::command]
fn read_line_string_file(state: State<Mutex<AppState>>, circuit_id: String) -> Result<Value, String> {
    let state = state.lock().unwrap();
    let data_dir = state.app_env_path.join("data").join(circuit_id);
    let line_string_path = data_dir.join("lineString.json");
    let file_content = fs::read_to_string(line_string_path).map_err(|e| e.to_string())?;
    let json_content: Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;
    Ok(json_content)
}

#[tauri::command]
fn read_tracking_file(state: State<Mutex<AppState>>, circuit_id: String) -> Result<Value, String> {
    let state = state.lock().unwrap();
    let data_dir = state.app_env_path.join("data").join(circuit_id);
    let tracking_path = data_dir.join("tracking.json");
    let file_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let json_content: Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;
    Ok(json_content)
}

#[tauri::command]
fn convert_vuetify_color(color_name: String) -> String {
    let hex_color = colors::convert_vuetify_color_to_hex(&color_name);
    format!("#{}", hex_color)
}

#[tauri::command]
fn update_camera_position(
    state: State<Mutex<AppState>>,
    circuit_id: String,
    longitude: f64,
    latitude: f64,
    altitude: f64,
    zoom: f64,
    pitch: f64,
    bearing: f64,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    let data_dir = state.app_env_path.join("data").join(&circuit_id);
    let tracking_path = data_dir.join("tracking.json");

    let file_content = fs::read_to_string(&tracking_path).map_err(|e| e.to_string())?;
    let mut tracking_data: Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;

    if let Some(points) = tracking_data.as_array_mut() {
        if let Some(first_point) = points.get_mut(0) {
            first_point["coordonneeCamera"] = serde_json::to_value(vec![longitude, latitude]).map_err(|e| e.to_string())?;
            first_point["altitudeCamera"] = serde_json::to_value(altitude).map_err(|e| e.to_string())?;
            first_point["zoom"] = serde_json::to_value(zoom).map_err(|e| e.to_string())?;
            first_point["pitch"] = serde_json::to_value(pitch).map_err(|e| e.to_string())?;
            first_point["cap"] = serde_json::to_value(bearing).map_err(|e| e.to_string())?;
        } else {
            return Err("Tracking data is empty.".to_string());
        }
    } else {
        return Err("Tracking data is not an array.".to_string());
    }

    let new_content = serde_json::to_string_pretty(&tracking_data).map_err(|e| e.to_string())?;
    fs::write(&tracking_path, new_content).map_err(|e| e.to_string())?;

    Ok(())
}



#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CircuitForDisplay {
    circuit_id: String,
    nom: String,
    distance_km: f64,
    denivele_m: i32,
    ville_depart: String,
    ville_depart_id: String,
    sommet: Option<CircuitSommet>,
    traceur: String,
    traceur_id: String,
    tracking_km: f64,
    iso_date_time: DateTime<Utc>,
    avancement_communes: i32,
}

#[tauri::command]
fn get_circuits_for_display(state: State<Mutex<AppState>>) -> Result<Vec<CircuitForDisplay>, String> {
    let state = state.lock().unwrap();
    let circuits_file = read_circuits_file(&state.app_env_path)?;

    let mut circuits_for_display: Vec<CircuitForDisplay> = circuits_file.circuits.iter().map(|circuit| {
        let ville_depart = circuits_file.villes
            .iter()
            .find(|v| v.id == circuit.ville_depart_id)
            .map_or("Inconnue".to_string(), |v| v.nom.clone());
        
        let traceur = circuits_file.traceurs
            .iter()
            .find(|t| t.id == circuit.traceur_id)
            .map_or("Inconnu".to_string(), |t| t.nom.clone());

        CircuitForDisplay {
            circuit_id: circuit.circuit_id.clone(),
            nom: circuit.nom.clone(),
            distance_km: circuit.distance_km,
            denivele_m: circuit.denivele_m,
            ville_depart,
            ville_depart_id: circuit.ville_depart_id.clone(),
            sommet: Some(circuit.sommet.clone()),
            traceur,
            traceur_id: circuit.traceur_id.clone(),
            tracking_km: circuit.tracking_km,
            iso_date_time: circuit.iso_date_time,
            avancement_communes: circuit.avancement_communes,
        }
    }).collect();

    // Sort by date descending by default
    circuits_for_display.sort_by(|a, b| b.iso_date_time.cmp(&a.iso_date_time));

    Ok(circuits_for_display)
}

// Fonction pour lire le fichier circuits.json
pub fn read_circuits_file(app_env_path: &PathBuf) -> Result<CircuitsFile, String> {
    let circuits_path = app_env_path.join("circuits.json");
    let file_content = fs::read_to_string(&circuits_path)
        .map_err(|e| format!("Failed to read circuits.json: {}", e))?;
    serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse circuits.json: {}", e))
}

// Fonction pour écrire le fichier circuits.json
pub fn write_circuits_file(app_env_path: &PathBuf, circuits_data: &CircuitsFile) -> Result<(), String> {
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


pub fn get_setting_value<'a>(settings: &'a Value, path: &str) -> Option<&'a Value> {
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
fn list_traceurs(state: State<Mutex<AppState>>) -> Result<Vec<Traceur>, String> {
    let state = state.lock().unwrap();
    let circuits_file = read_circuits_file(&state.app_env_path)?;
    Ok(circuits_file.traceurs)
}

#[tauri::command]
fn add_traceur(state: State<Mutex<AppState>>, nom: String) -> Result<Traceur, String> {
    let state = state.lock().unwrap();
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
fn update_setting(state: State<Mutex<AppState>>, group_path: String, param_id: String, new_value: Value) -> Result<(), String> {
    let mut state = state.lock().unwrap();
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
            param["surcharge"] = new_value.clone(); // Clone new_value to use it later
        } else {
            return Err(format!("Parameter '{}' not found", param_id));
        }
    } else {
        return Err("No 'parametres' array found in the target group".to_string());
    }

    // Write back the updated settings
    let new_settings_content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(&settings_path, new_settings_content).map_err(|e| e.to_string())?;

    // If the mapbox token was updated, also update the in-memory state
    if group_path == "Système/Tokens" && param_id == "mapbox" {
        if let Some(token_str) = new_value.as_str() {
            state.mapbox_token = token_str.to_string();
        }
    }

    Ok(())
}



#[tauri::command]
async fn analyze_gpx_file(app_handle: tauri::AppHandle, filename: String) -> Result<DraftCircuit, String> {
    gpx_processor::analyze_gpx_file(&app_handle, &filename).await
}



#[tauri::command]
fn commit_new_circuit(
    app_handle: tauri::AppHandle,
    draft: DraftCircuit,
    traceur_id: String,
) -> Result<String, String> {
    gpx_processor::commit_new_circuit(&app_handle, draft, traceur_id)
}

#[tauri::command]
fn delete_circuit(state: State<Mutex<AppState>>, circuit_id: String) -> Result<(), String> {
    let state = state.lock().unwrap();
    let app_env_path = &state.app_env_path;

    // 1. Delete from circuits.json
    let mut circuits_file = read_circuits_file(app_env_path)?;

    let initial_len = circuits_file.circuits.len();
    circuits_file.circuits.retain(|c| c.circuit_id != circuit_id);

    if circuits_file.circuits.len() == initial_len {
        return Err(format!("Circuit with ID '{}' not found.", circuit_id));
    }

    write_circuits_file(app_env_path, &circuits_file)?;

    // 2. Delete the circuit's directory
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    if circuit_data_dir.exists() {
        fs::remove_dir_all(&circuit_data_dir)
            .map_err(|e| format!("Failed to delete circuit directory: {}", e))?;
    } else {
        println!("Circuit directory not found: {}", circuit_data_dir.display());
    }

    Ok(())
}

#[tauri::command]
fn get_thumbnail_as_base64(state: State<Mutex<AppState>>, circuit_id: String) -> Result<String, String> {
    let state = state.lock().unwrap();
    let thumbnail_path = state.app_env_path.join("data").join(circuit_id).join("vignette.png");

    if !thumbnail_path.exists() {
        return Err("Thumbnail not found.".to_string());
    }

    let image_bytes = fs::read(&thumbnail_path)
        .map_err(|e| format!("Failed to read thumbnail file: {}", e))?;

    let base64_str = general_purpose::STANDARD.encode(&image_bytes);

    Ok(format!("data:image/png;base64,{}", base64_str))
}

#[tauri::command]
fn save_tracking_file(state: State<Mutex<AppState>>, circuit_id: String, tracking_data: Value) -> Result<(), String> {
    let state = state.lock().unwrap();
    let tracking_path = state.app_env_path.join("data").join(circuit_id).join("tracking.json");

    let new_content = serde_json::to_string_pretty(&tracking_data)
        .map_err(|e| format!("Failed to serialize tracking data: {}", e))?;

    fs::write(&tracking_path, new_content).map_err(|e| e.to_string())?;

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
        mapbox_token,
        updating_circuit_id: None, // Initialize to None
        updating_circuit_name: None,
    })
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilterData {
    min_distance: f64,
    max_distance: f64,
    min_denivele: i32,
    max_denivele: i32,
    villes: Vec<Ville>,
    traceurs: Vec<Traceur>,
}

#[tauri::command]
fn get_filter_data(state: State<Mutex<AppState>>) -> Result<FilterData, String> {
    let state = state.lock().unwrap();
    let circuits_file = read_circuits_file(&state.app_env_path)?;

    let mut min_distance = f64::MAX;
    let mut max_distance = f64::MIN;
    let mut min_denivele = i32::MAX;
    let mut max_denivele = i32::MIN;

    if circuits_file.circuits.is_empty() {
        min_distance = 0.0;
        max_distance = 100.0; // Default value if no circuits
        min_denivele = 0;
        max_denivele = 1000; // Default value if no circuits
    } else {
        for circuit in &circuits_file.circuits {
            if circuit.distance_km < min_distance {
                min_distance = circuit.distance_km;
            }
            if circuit.distance_km > max_distance {
                max_distance = circuit.distance_km;
            }
            if circuit.denivele_m < min_denivele {
                min_denivele = circuit.denivele_m;
            }
            if circuit.denivele_m > max_denivele {
                max_denivele = circuit.denivele_m;
            }
        }
    }

    Ok(FilterData {
        min_distance: min_distance.floor(),
        max_distance: max_distance.ceil(),
        min_denivele,
        max_denivele,
        villes: circuits_file.villes,
        traceurs: circuits_file.traceurs,
    })
}

#[tauri::command]
fn update_tracking_km(state: State<Mutex<AppState>>, circuit_id: String, tracking_km: f64) -> Result<(), String> {
    let state = state.lock().unwrap();
    let app_env_path = &state.app_env_path;

    let mut circuits_file = read_circuits_file(app_env_path)?;

    if let Some(circuit) = circuits_file.circuits.iter_mut().find(|c| c.circuit_id == circuit_id) {
        circuit.tracking_km = (tracking_km * 10.0).round() / 10.0;
    } else {
        return Err(format!("Circuit with ID '{}' not found.", circuit_id));
    }

    write_circuits_file(app_env_path, &circuits_file)?;

    Ok(())
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
            app.manage(Mutex::new(state.clone()));

            // Check if a commune update was running
            let circuits_file = read_circuits_file(&state.app_env_path);
            if let Ok(file) = circuits_file {
                if file.maj_communes && !file.circuit_id.is_empty() {
                    let app_handle = app.handle().clone();
                    let circuit_id = file.circuit_id.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = communes_updater::start_communes_update(app_handle, circuit_id).await;
                    });
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_app_state, check_mapbox_status, check_internet_connectivity, read_settings, list_execution_modes, create_execution_mode, delete_execution_mode, select_execution_mode, update_setting, list_gpx_files, analyze_gpx_file, commit_new_circuit, list_traceurs, add_traceur, thumbnail_generator::generate_gpx_thumbnail, get_circuits_for_display, get_debug_data, delete_circuit, get_thumbnail_as_base64, read_line_string_file, read_tracking_file, save_tracking_file, convert_vuetify_color, update_camera_position, geo_processor::process_tracking_data, get_filter_data, update_tracking_km, communes_updater::start_communes_update, communes_updater::interrupt_communes_update, communes_updater::get_current_commune_task_info, communes_updater::toggle_ign_api,             communes_updater::toggle_mapbox_api, 
            communes_updater::get_ign_status, communes_updater::get_mapbox_status, communes_updater::get_commune_update_progress,
        event::get_events,
        event::add_pause_event,
        event::delete_pause_event,
        event::add_flyto_event,
        event::delete_flyto_event,
        event::add_message_event,
        event::delete_message_event,
        event::get_known_message_texts,
        trace_style::get_slope_color_expression,
    ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
