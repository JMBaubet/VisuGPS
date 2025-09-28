
use serde::Deserialize;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use crate::{AppState, read_circuits_file, write_circuits_file, get_setting_value};
use tokio::time::sleep; // Import tokio sleep


#[derive(Deserialize)]
struct MapboxFeature {
    text: String,
    place_type: Vec<String>,
}

#[derive(Deserialize)]
struct MapboxResponse {
    features: Vec<MapboxFeature>,
}

#[derive(Deserialize)]
struct OsmAddress {
    city: Option<String>,
    town: Option<String>,
    village: Option<String>,
    hamlet: Option<String>,
}

#[derive(Deserialize)]
struct OsmResponse {
    address: OsmAddress,
}


// Use a global variable for the cancellation token
lazy_static::lazy_static! {
    static ref CANCELLATION_TOKEN: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    static ref TASK_RUNNING: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    pub static ref IGN_ENABLED: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    pub static ref MAPBOX_ENABLED: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

#[tauri::command]
pub async fn start_communes_update(app_handle: AppHandle, circuit_id: String) -> Result<(), String> {
    if TASK_RUNNING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return Err("La mise à jour des communes est déjà en cours.".to_string());
    }

    CANCELLATION_TOKEN.store(false, Ordering::SeqCst);
    let token_clone = CANCELLATION_TOKEN.clone();
    let task_running_clone = TASK_RUNNING.clone();

    let app_env_path_clone;
    let mapbox_token_clone;

    {
        let state = app_handle.state::<Mutex<AppState>>();
        let mut app_state = state.lock().unwrap();
        app_state.updating_circuit_id = Some(circuit_id.clone());
        app_env_path_clone = app_state.app_env_path.clone();
        mapbox_token_clone = app_state.mapbox_token.clone();
    }
    
    let handle_clone = app_handle.clone();

    // Read settings to get timers
    let settings_path = app_env_path_clone.join("settings.json");
    let settings_content = std::fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let timer_ign = get_setting_value(&settings, "data.groupes.MajCommunes.groupes.Timers.parametres.timerIGN").and_then(|v| v.as_u64()).unwrap_or(200);
    let timer_mapbox = get_setting_value(&settings, "data.groupes.MajCommunes.groupes.Timers.parametres.timerMapbox").and_then(|v| v.as_u64()).unwrap_or(200);
    let timer_osm = get_setting_value(&settings, "data.groupes.MajCommunes.groupes.Timers.parametres.timerOSM").and_then(|v| v.as_u64()).unwrap_or(1000);

    tauri::async_runtime::spawn(async move {
        let _ = update_task_status(&app_env_path_clone, true, &circuit_id);
        let _ = handle_clone.emit("commune-update-status-changed", &true);

        let passes = [
            (16, 0), (16, 8), (8, 4), (4, 2), (2, 1),
        ];

        for (step, start_offset) in passes.iter() {
            if token_clone.load(Ordering::SeqCst) {
                break;
            }
            let _ = process_pass_async(&app_env_path_clone, &mapbox_token_clone, &circuit_id, *step, *start_offset, &token_clone, &handle_clone, timer_ign, timer_mapbox, timer_osm).await;
        }

        task_running_clone.store(false, Ordering::SeqCst);
        let _ = update_task_status(&app_env_path_clone, false, "");
        
        {
            let state = handle_clone.state::<Mutex<AppState>>();
            let mut app_state = state.lock().unwrap();
            app_state.updating_circuit_id = None;
        }

        let _ = handle_clone.emit("commune-update-status-changed", &false);
        let _ = handle_clone.emit("show-snackbar", format!("Mise à jour des communes terminée pour le circuit."));
    });

    Ok(())
}

#[tauri::command]
pub fn interrupt_communes_update() -> Result<(), String> {
    CANCELLATION_TOKEN.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
pub fn get_communes_update_status() -> bool {
    TASK_RUNNING.load(Ordering::SeqCst)
}

#[tauri::command]
pub fn get_ign_status() -> bool {
    IGN_ENABLED.load(Ordering::SeqCst)
}

#[tauri::command]
pub fn get_mapbox_status() -> bool {
    MAPBOX_ENABLED.load(Ordering::SeqCst)
}

#[tauri::command]
pub fn toggle_ign_api(enable: bool) {
    IGN_ENABLED.store(enable, Ordering::SeqCst);
}

#[tauri::command]
pub fn toggle_mapbox_api(enable: bool) {
    MAPBOX_ENABLED.store(enable, Ordering::SeqCst);
}


fn update_task_status(app_env_path: &std::path::Path, is_running: bool, circuit_id: &str) -> Result<(), String> {
    let mut circuits_file = read_circuits_file(&app_env_path.to_path_buf())?;
    circuits_file.maj_communes = is_running;
    circuits_file.circuit_id = circuit_id.to_string();
    write_circuits_file(&app_env_path.to_path_buf(), &circuits_file)
}

async fn process_pass_async(app_env_path: &std::path::Path, mapbox_token: &str, circuit_id: &str, step: usize, start_offset: usize, token: &Arc<AtomicBool>, app_handle: &AppHandle, timer_ign: u64, timer_mapbox: u64, timer_osm: u64) -> Result<(), String> {
    let tracking_path = app_env_path.join("data").join(circuit_id).join("tracking.json");
    let tracking_content = std::fs::read_to_string(&tracking_path).map_err(|e| e.to_string())?;
    let mut tracking_points: Vec<serde_json::Value> = serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?;

    let total_points = tracking_points.len();

    for i in (start_offset..total_points).step_by(step) {
        if token.load(Ordering::SeqCst) {
            break;
        }

        if let Some(point) = tracking_points.get_mut(i) {
            if point["commune"].is_null() {
                if let Some(coords) = point["coordonnee"].as_array() {
                    if coords.len() == 2 {
                        let lon = coords[0].as_f64().unwrap_or(0.0);
                        let lat = coords[1].as_f64().unwrap_or(0.0);

                        let commune_name = fetch_commune_name(lon, lat, mapbox_token, i, timer_ign, timer_mapbox, timer_osm).await;

                        if let Ok(name) = commune_name {
                            point["commune"] = serde_json::Value::String(name.clone());

                            let new_content = serde_json::to_string_pretty(&tracking_points).map_err(|e| e.to_string())?;
                            std::fs::write(&tracking_path, new_content).map_err(|e| e.to_string())?;

                            let mut circuits_file = read_circuits_file(&app_env_path.to_path_buf())?;
                            if let Some(circuit) = circuits_file.circuits.iter_mut().find(|c| c.circuit_id == circuit_id) {
                                let processed_count = tracking_points.iter().filter(|p| !p["commune"].is_null()).count();
                                circuit.avancement_communes = ((processed_count as f32 / total_points as f32) * 100.0) as i32;
                                let _ = app_handle.emit(
                                    "commune-progress-changed",
                                    (circuit_id.to_string(), circuit.avancement_communes)
                                );
                                write_circuits_file(&app_env_path.to_path_buf(), &circuits_file)?;
                            }
                        } else {
                            // Error already logged in fetch_commune_name
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

async fn fetch_commune_name(lon: f64, lat: f64, mapbox_token: &str, point_index: usize, timer_ign: u64, timer_mapbox: u64, timer_osm: u64) -> Result<String, String> {
    let client = reqwest::Client::new();

    // 1. IGN
    if IGN_ENABLED.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(timer_ign)).await;
        let ign_url = format!("https://api-adresse.data.gouv.fr/reverse/?lon={}&lat={}", lon, lat);
        if let Ok(resp) = client.get(&ign_url).send().await {
            if resp.status().is_success() {
                 if let Ok(data) = resp.json::<serde_json::Value>().await {
                    if let Some(features) = data["features"].as_array() {
                        if !features.is_empty() {
                             if let Some(city) = features[0]["properties"]["city"].as_str() {
                                return Ok(city.to_string());
                            }
                        }
                    }
                }
            }
        }
        println!("[API] Échec IGN pour le point {}.", point_index);
    }

    // 2. Mapbox
    if MAPBOX_ENABLED.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(timer_mapbox)).await;
        let mapbox_url = format!("https://api.mapbox.com/geocoding/v5/mapbox.places/{},{}.json?access_token={}", lon, lat, mapbox_token);
         if let Ok(resp) = client.get(&mapbox_url).send().await {
            match resp.status() {
                reqwest::StatusCode::OK => {
                    if let Ok(data) = resp.json::<MapboxResponse>().await {
                        if let Some(feature) = data.features.iter().find(|f| f.place_type.contains(&"place".to_string())) {
                            return Ok(feature.text.clone());
                        }
                    }
                },
                _ => {}
            }
        }
        println!("[API] Échec Mapbox pour le point {}.", point_index);
    }

    // 3. OpenStreetMap
    sleep(Duration::from_millis(timer_osm)).await;
    let osm_url = format!("https://nominatim.openstreetmap.org/reverse?format=json&lat={}&lon={}&zoom=10&addressdetails=1", lat, lon);
    if let Ok(resp) = client.get(&osm_url).header("User-Agent", "VisuGPS/0.1").send().await {
        if resp.status().is_success() {
            if let Ok(data) = resp.json::<OsmResponse>().await {
                let city = data.address.city
                    .or(data.address.town)
                    .or(data.address.village)
                    .or(data.address.hamlet)
                    .unwrap_or_else(|| "Inconnue".to_string());
                return Ok(city);
            }
        }
    }
    println!("[API] Échec OSM pour le point {}.", point_index);

    Err(format!("Toutes les API de géocodage ont échoué pour le point {}.", point_index))
}
