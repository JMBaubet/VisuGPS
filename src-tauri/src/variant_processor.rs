use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantPoint {
    pub lat: f64,
    pub lon: f64,
    pub alt: Option<f64>,
    #[serde(rename = "type")]
    pub point_type: Option<String>, // "start_point", "end_point", "waypoint"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum VariantModification {
    #[serde(rename = "DEPART_DEPORTE", rename_all = "camelCase")]
    DepartDeporte {
        #[serde(alias = "anchor_index_on_master")]
        anchor_index_on_master: usize,
        points: Vec<VariantPoint>, // Raw editing points
        #[serde(alias = "full_geometry", skip_serializing_if = "Option::is_none")]
        full_geometry: Option<Vec<VariantPoint>>, // Detailed route
        longueur: f64,
        name: Option<String>,
    },
    #[serde(rename = "ARRIVEE_REPORTEE", rename_all = "camelCase")]
    ArriveeReportee {
        #[serde(alias = "anchor_index_on_master")]
        anchor_index_on_master: usize,
        points: Vec<VariantPoint>, // Raw editing points
        #[serde(alias = "full_geometry", skip_serializing_if = "Option::is_none")]
        full_geometry: Option<Vec<VariantPoint>>, // Detailed route
        longueur: f64,
        name: Option<String>,
    },
    #[serde(rename = "SEGMENT_DEVIATION", rename_all = "camelCase")]
    SegmentDeviation {
        #[serde(alias = "anchor_start")]
        anchor_start: AnchorPoint,
        #[serde(alias = "anchor_end")]
        anchor_end: AnchorPoint,
        waypoints: Vec<VariantPoint>, // Intermediate editing points
        #[serde(alias = "full_geometry", skip_serializing_if = "Option::is_none")]
        full_geometry: Option<Vec<VariantPoint>>, // Detailed route
        longueur: f64,
        name: Option<String>,
    },
}

impl VariantModification {
    pub fn get_start_anchor_index(&self) -> usize {
        match self {
            VariantModification::DepartDeporte { anchor_index_on_master: _, .. } => 0, // Virtual index 0 for start
            VariantModification::ArriveeReportee { anchor_index_on_master, .. } => *anchor_index_on_master,
            VariantModification::SegmentDeviation { anchor_start, .. } => anchor_start.index,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnchorPoint {
    index: usize,
    coords: [f64; 2],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantStats {
    pub total_distance: f64,
    pub total_ascent: f64,
    #[serde(alias = "master_distance", default)]
    pub master_distance: f64,
    #[serde(alias = "master_ascent", default)]
    pub master_ascent: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(alias = "creation_date")]
    pub creation_date: String,
    pub color: String,
    pub stats: VariantStats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Archive {
    metadata: VariantMetadata,
    modifications: Vec<VariantModification>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateVariantRequest {
    circuit_id: String,
    metadata: VariantMetadata,
    modifications: Vec<VariantModification>,
}

// Elevation fetching is now handled by crate::elevation_provider


async fn prepare_points_3d(points_raw: &Vec<VariantPoint>) -> Result<(Vec<Vec<f64>>, Option<String>), String> {
    let mut final_3d = Vec::new();
    let mut missing_alt_indices = Vec::new();
    let mut coords_for_fetch = Vec::new();

    for (i, p) in points_raw.iter().enumerate() {
        if let Some(alt) = p.alt {
            final_3d.push(vec![p.lon, p.lat, alt]);
        } else {
            final_3d.push(vec![p.lon, p.lat, 0.0]); // Placeholder
            missing_alt_indices.push(i);
            coords_for_fetch.push([p.lon, p.lat]);
        }
    }

    let mut warning = None;

    if !coords_for_fetch.is_empty() {
        match crate::elevation_provider::fetch_altitudes(&coords_for_fetch).await {
            Ok(fetched_alts) => {
                for (i, alt) in missing_alt_indices.iter().zip(fetched_alts.iter()) {
                    final_3d[*i][2] = *alt;
                }
            },
            Err(e) => {
                // FALLBACK: If elevation service is down, we proceed with 0.0 altitude to allow saving.
                let msg = format!("Attention: Échec de la récupération d'altitude ({}) -> 0.0 utilisé.", e);
                println!("{}", msg);
                warning = Some(msg);
                // We don't return an error, we just keep the 0.0 placeholders.
            }
        }
    }
    Ok((final_3d, warning))
}

#[tauri::command]
pub async fn create_variant_files(
    app_handle: tauri::AppHandle,
    request: CreateVariantRequest,
) -> Result<Option<String>, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let circuit_data_dir = app_env_path.join("data").join(&request.circuit_id);
    if !circuit_data_dir.exists() {
        return Err("Circuit directory not found".to_string());
    }

    // Load master tracking.json to copy anchor point data
    let tracking_master_path = circuit_data_dir.join("tracking.json");
    let tracking_master_content = fs::read_to_string(&tracking_master_path).map_err(|e| format!("Failed to read master tracking: {}", e))?;
    let tracking_master: Vec<serde_json::Value> = serde_json::from_str(&tracking_master_content).map_err(|e| format!("Failed to parse master tracking: {}", e))?;

    let mut global_warning: Option<String> = None;

    // Process each modification for individual files
    for (index, modification) in request.modifications.iter().enumerate() {
        // Validation: Backwards segments are not allowed
        if let VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } = modification {
            if anchor_end.index <= anchor_start.index {
                return Err(format!("Le segment {} est invalide : l'ancre de fin (index {}) doit être après l'ancre de départ (index {})", 
                    index + 1, anchor_end.index, anchor_start.index));
            }
        }

        let (suffix, points_raw_opt) = match modification {
            VariantModification::DepartDeporte { full_geometry, .. } => ("DEPART", full_geometry),
            VariantModification::ArriveeReportee { full_geometry, .. } => ("ARRIVEE", full_geometry),
            VariantModification::SegmentDeviation { full_geometry, .. } => ("SEGMENT", full_geometry)
        };

        if let Some(points_raw) = points_raw_opt {
            if !points_raw.is_empty() {
                let suffix_full = match modification {
                    VariantModification::SegmentDeviation { .. } => format!("{}_{}", suffix, index),
                    _ => suffix.to_string() 
                };

                let (track_points_3d, warning) = prepare_points_3d(points_raw).await?;
                
                if let Some(w) = warning {
                    global_warning = Some(w);
                }

                // Write LineString file
                let linestring_filename = format!("lineString_{}_{}.json", request.metadata.id, suffix_full);
                let linestring_path = circuit_data_dir.join(&linestring_filename);
                let linestring_json = serde_json::json!({
                    "type": "LineString",
                    "coordinates": track_points_3d
                });
                fs::write(&linestring_path, serde_json::to_string_pretty(&linestring_json).unwrap()).map_err(|e| e.to_string())?;

                // Overrides from master tracking
                let (override_first, override_last) = match modification {
                    VariantModification::DepartDeporte { anchor_index_on_master, .. } => (None, tracking_master.get(*anchor_index_on_master).cloned()),
                    VariantModification::ArriveeReportee { anchor_index_on_master, .. } => (tracking_master.get(*anchor_index_on_master).cloned(), None),
                    VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } => (tracking_master.get(anchor_start.index).cloned(), tracking_master.get(anchor_end.index).cloned())
                };

                // Generate Tracking file
                let tracking_filename = format!("tracking_{}_{}.json", request.metadata.id, suffix_full);
                crate::tracking_processor::generate_tracking_file(&app_env_path, &request.circuit_id, &track_points_3d, &settings, Some(&tracking_filename), override_first, override_last)?;
            }
        }
    }

    // --- Statistics Calculation (Full Trace Reconstruction) ---
    let master_ls_path = circuit_data_dir.join("lineString.json");
    let master_ls_content = fs::read_to_string(&master_ls_path).map_err(|e| format!("Failed to read lineString.json: {}", e))?;
    let master_ls: serde_json::Value = serde_json::from_str(&master_ls_content).map_err(|e| format!("Failed to parse lineString.json: {}", e))?;
    let master_coords = master_ls["coordinates"].as_array().ok_or("Invalid lineString format")?;
    let master_points_high_res: Vec<Vec<f64>> = master_coords.iter().map(|c| c.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();

    let find_closest_high_res_idx = |lon: f64, lat: f64, start_from: usize| -> usize {
        let mut min_dist = f64::MAX;
        let mut closest_idx = start_from;
        for i in start_from..master_points_high_res.len() {
            let mp = &master_points_high_res[i];
            let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
            if d < min_dist { min_dist = d; closest_idx = i; }
            if d < 1.0 && i > start_from + 1 {
                 let prev_d = crate::gpx_processor::haversine_distance(lat, lon, master_points_high_res[i-1][1], master_points_high_res[i-1][0]);
                 if d > prev_d { break; }
            }
        }
        closest_idx
    };

    let mut final_points: Vec<Vec<f64>> = Vec::new();
    let mut current_master_idx = 0;
    let mut sorted_mods = request.modifications.clone();
    sorted_mods.sort_by_key(|m| m.get_start_anchor_index());
    let mut has_arrivee_reportee = false;

    for modification in &sorted_mods {
        match modification {
            VariantModification::DepartDeporte { anchor_index_on_master, full_geometry, .. } => {
                if let Some(geom) = full_geometry {
                    let (pts, w) = prepare_points_3d(geom).await?;
                    if let Some(msg) = w { global_warning = Some(msg); }
                    final_points.extend(pts);
                }
                let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                current_master_idx = find_closest_high_res_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), 0);
            },
            VariantModification::SegmentDeviation { anchor_start, anchor_end, full_geometry, .. } => {
                let start_idx = find_closest_high_res_idx(anchor_start.coords[0], anchor_start.coords[1], current_master_idx);
                while current_master_idx <= start_idx && current_master_idx < master_points_high_res.len() {
                    final_points.push(master_points_high_res[current_master_idx].clone());
                    current_master_idx += 1;
                }
                if let Some(geom) = full_geometry {
                    let (pts, w) = prepare_points_3d(geom).await?;
                    if let Some(msg) = w { global_warning = Some(msg); }
                    final_points.extend(pts);
                }
                current_master_idx = find_closest_high_res_idx(anchor_end.coords[0], anchor_end.coords[1], start_idx);
                if current_master_idx <= start_idx { current_master_idx = start_idx + 1; }
            },
            VariantModification::ArriveeReportee { anchor_index_on_master, full_geometry, .. } => {
                has_arrivee_reportee = true;
                let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                let arrivee_idx = find_closest_high_res_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), current_master_idx);
                while current_master_idx <= arrivee_idx && current_master_idx < master_points_high_res.len() {
                    final_points.push(master_points_high_res[current_master_idx].clone());
                    current_master_idx += 1;
                }
                if let Some(geom) = full_geometry {
                    let (pts, w) = prepare_points_3d(geom).await?;
                    if let Some(msg) = w { global_warning = Some(msg); }
                    final_points.extend(pts);
                }
                current_master_idx = master_points_high_res.len();
            }
        }
    }

    if !has_arrivee_reportee {
        while current_master_idx < master_points_high_res.len() {
            final_points.push(master_points_high_res[current_master_idx].clone());
            current_master_idx += 1;
        }
    }

    let median_window = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.altitude_smoothing_median_window").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let avg_window = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.altitude_smoothing_avg_window").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let max_gradient = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.max_gradient_percent").and_then(|v| v.as_f64()).unwrap_or(40.0);
    let lissage_dist = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.denivele_lissage_distance").and_then(|v| v.as_f64()).unwrap_or(10.0);
    
    let cleaned_points = crate::gpx_processor::clean_altitude_data(&final_points, median_window, avg_window, max_gradient);
    let circuits_file = crate::read_circuits_file(&app_env_path)?;
    let master_circuit = circuits_file.circuits.iter().find(|c| c.circuit_id == request.circuit_id).ok_or_else(|| format!("Master circuit {} not found", request.circuit_id))?;
    let variant_stats = crate::gpx_processor::calculate_track_stats(&cleaned_points, lissage_dist);

    let mut metadata = request.metadata;
    metadata.stats.total_distance = variant_stats.total_distance_km;
    metadata.stats.total_ascent = variant_stats.positive_elevation_m as f64;
    metadata.stats.master_distance = master_circuit.distance_km;
    metadata.stats.master_ascent = master_circuit.denivele_m as f64;

    let archive_path = circuit_data_dir.join(format!("archive_{}.json", metadata.id));
    let mut modifications = request.modifications;
    for m in modifications.iter_mut() {
        match m {
            VariantModification::DepartDeporte { full_geometry, .. } | VariantModification::ArriveeReportee { full_geometry, .. } | VariantModification::SegmentDeviation { full_geometry, .. } => *full_geometry = None,
        }
    }
    fs::write(&archive_path, serde_json::to_string_pretty(&Archive { metadata, modifications }).unwrap()).map_err(|e| e.to_string())?;
    Ok(global_warning)
}

#[tauri::command]
pub async fn get_variants(
    app_handle: tauri::AppHandle,
    circuit_id: String,
) -> Result<Vec<VariantMetadata>, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    if !circuit_data_dir.exists() {
        return Ok(Vec::new());
    }

    let mut variants = Vec::new();
    let entries = fs::read_dir(circuit_data_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if filename.starts_with("archive_var_") {
                let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                let archive: Archive = serde_json::from_str(&content).map_err(|e| e.to_string())?;
                variants.push(archive.metadata);
            }
        }
    }

    // Sort by creation date DESC
    variants.sort_by(|a, b| b.creation_date.cmp(&a.creation_date));

    Ok(variants)
}

#[tauri::command]
pub async fn get_variant_details(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<Archive, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let archive_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("archive_{}.json", variant_id));

    if !archive_path.exists() {
        return Err("Archive non trouvé".to_string());
    }

    let content = fs::read_to_string(&archive_path).map_err(|e| e.to_string())?;
    let mut archive: Archive = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    // Re-hydrate full_geometry from LineString files
    for (index, modification) in archive.modifications.iter_mut().enumerate() {
        let suffix = match modification {
            VariantModification::DepartDeporte { .. } => "DEPART".to_string(),
            VariantModification::ArriveeReportee { .. } => "ARRIVEE".to_string(),
            VariantModification::SegmentDeviation { .. } => format!("SEGMENT_{}", index),
        };

        let linestring_filename = format!("lineString_{}_{}.json", archive.metadata.id, suffix);
        let linestring_path = app_env_path.join("data").join(&circuit_id).join(&linestring_filename);

        if linestring_path.exists() {
            if let Ok(ls_content) = fs::read_to_string(&linestring_path) {
                if let Ok(ls_json) = serde_json::from_str::<serde_json::Value>(&ls_content) {
                    if let Some(coords) = ls_json.get("coordinates").and_then(|c| c.as_array()) {
                        let mut full_geom = Vec::new();
                        for coord in coords {
                            if let Some(arr) = coord.as_array() {
                                if arr.len() >= 2 {
                                    full_geom.push(VariantPoint {
                                        lat: arr[1].as_f64().unwrap_or(0.0),
                                        lon: arr[0].as_f64().unwrap_or(0.0),
                                        alt: (arr.len() >= 3).then(|| arr[2].as_f64().unwrap_or(0.0)),
                                        point_type: None,
                                    });
                                }
                            }
                        }
                        match modification {
                            VariantModification::DepartDeporte { full_geometry, .. } => *full_geometry = Some(full_geom),
                            VariantModification::ArriveeReportee { full_geometry, .. } => *full_geometry = Some(full_geom),
                            VariantModification::SegmentDeviation { full_geometry, .. } => *full_geometry = Some(full_geom),
                        }
                    }
                }
            }
        }
    }

    Ok(archive)
}

#[tauri::command]
pub async fn delete_variant(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<(), String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    if !circuit_data_dir.exists() {
        return Err("Circuit directory not found".to_string());
    }

    let entries = fs::read_dir(&circuit_data_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let filename = entry.file_name().into_string().unwrap_or_default();
        
        // Delete archive, lineString and tracking files containing the variant_id
        if filename.contains(&variant_id) {
            fs::remove_file(entry.path()).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn rename_variant(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
    new_name: String,
) -> Result<(), String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let archive_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("archive_{}.json", variant_id));

    if !archive_path.exists() {
        return Err("Archive non trouvé".to_string());
    }

    let content = fs::read_to_string(&archive_path).map_err(|e| e.to_string())?;
    let mut archive: Archive = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    archive.metadata.name = new_name;
    
    let updated_content = serde_json::to_string_pretty(&archive).map_err(|e| e.to_string())?;
    fs::write(&archive_path, updated_content).map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphHopperResponse {
    paths: Vec<GraphHopperPath>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphHopperPath {
    points: GraphHopperPoints,
    distance: f64,
    ascend: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphHopperPoints {
    coordinates: Vec<Vec<f64>>,
}

#[tauri::command]
pub async fn calculate_route(
    app_handle: tauri::AppHandle,
    service: String,
    profile: String,
    points: Vec<[f64; 2]>,
) -> Result<String, String> {
    if points.len() < 2 {
        return Err("Il faut au moins 2 points pour calculer un itinéraire.".to_string());
    }

    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let api_key = crate::get_setting_value(
        &settings,
        "data.groupes.Variante.parametres.routingApiKey",
    )
    .and_then(|v| v.as_str())
    .unwrap_or("")
    .to_string();

    if service == "GraphHopper" {
        if api_key.is_empty() {
             return Err("Clé API GraphHopper manquante dans les paramètres.".to_string());
        }

        let max_points_per_request = 5;
        let mut all_coordinates: Vec<Vec<f64>> = Vec::new();
        
        // Chunking logic
        let chunks: Vec<Vec<[f64; 2]>> = if points.len() > max_points_per_request {
            let mut result = Vec::new();
            let mut start_idx = 0;
            while start_idx < points.len() - 1 {
                let end_idx = (start_idx + max_points_per_request).min(points.len());
                // Ensure we have at least 2 points
                if end_idx - start_idx < 2 {
                     // Should not happen with well-formed overlapping, but safety check
                     break; 
                }
                result.push(points[start_idx..end_idx].to_vec());
                start_idx = end_idx - 1; // Overlap: last point becomes first of next chunk
            }
            result
        } else {
            vec![points]
        };

        for (i, chunk_points) in chunks.iter().enumerate() {
            if i > 0 {
                // Add a small delay to avoid rate limiting
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }

            let client = reqwest::Client::new();
            let mut url = format!("https://graphhopper.com/api/1/route?key={}&profile={}&points_encoded=false&elevation=true", api_key, profile);
            
            for p in chunk_points {
                url.push_str(&format!("&point={},{}", p[1], p[0])); // Lat,Lon
            }

            let resp = client.get(&url)
                .send()
                .await
                .map_err(|e| format!("Request failed for chunk {}: {}", i, e))?;

            if !resp.status().is_success() {
                 return Err(format!("GraphHopper API Error on chunk {}: {}", i, resp.status()));
            }

            let gh_resp: GraphHopperResponse = resp.json().await.map_err(|e| format!("Parse error: {}", e))?;

            if let Some(path) = gh_resp.paths.first() {
                // If it's not the first chunk, remove the first point to avoid duplicate with previous chunk's last point
                let mut coords = path.points.coordinates.clone();
                if i > 0 && !coords.is_empty() {
                    coords.remove(0);
                }
                all_coordinates.extend(coords);
            } else {
                 return Err(format!("Aucun chemin trouvé pour le tronçon {}.", i+1));
            }
        }
        
        let geojson = serde_json::json!({
             "type": "LineString",
             "coordinates": all_coordinates
         });
         return Ok(serde_json::to_string(&geojson).unwrap());

    } else if service == "OpenRouteService" {
        return Err("OpenRouteService non implémenté pour le moment.".to_string());
    }

    Err("Service de routage inconnu.".to_string())
}
