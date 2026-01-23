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
        // Consider None or 0.0 as a candidate for fetching (if we want to be sure)
        // Note: 0.0 is technically valid (sea level), but often means "missing" in router outputs.
        if let Some(alt) = p.alt {
            if alt != 0.0 {
                final_3d.push(vec![p.lon, p.lat, alt]);
                continue;
            }
        }
        
        final_3d.push(vec![p.lon, p.lat, 0.0]); // Placeholder
        missing_alt_indices.push(i);
        coords_for_fetch.push([p.lon, p.lat]);
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

    // Load smoothing settings
    let median_window = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.altitude_smoothing_median_window").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let avg_window = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.altitude_smoothing_avg_window").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let max_gradient = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.max_gradient_percent").and_then(|v| v.as_f64()).unwrap_or(40.0);

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

                let (track_points_3d_raw, warning) = prepare_points_3d(points_raw).await?;
                
                if let Some(w) = warning {
                    global_warning = Some(w);
                }

                // Apply altitude smoothing to the segment
                let track_points_3d = crate::gpx_processor::clean_altitude_data(&track_points_3d_raw, median_window, avg_window, max_gradient);

                // Write LineString file
                let linestring_filename = format!("lineString_{}_{}.json", request.metadata.id, suffix_full);
                let linestring_path = circuit_data_dir.join(&linestring_filename);
                let linestring_json = serde_json::json!({
                    "type": "LineString",
                    "coordinates": track_points_3d
                });
                fs::write(&linestring_path, serde_json::to_string_pretty(&linestring_json).unwrap()).map_err(|e| e.to_string())?;

                // Generate Tracking file
                let tracking_filename = format!("tracking_{}_{}.json", request.metadata.id, suffix_full);
                
                let segment_length = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(100.0);

                // Logic for nbrSegment according to user rules
                let (override_first, override_last) = match modification {
                    VariantModification::DepartDeporte { anchor_index_on_master, .. } => {
                        // Calculate geometric length to estimate nbr segments
                        let mut length_m = 0.0;
                        for i in 0..track_points_3d.len().saturating_sub(1) {
                            length_m += crate::gpx_processor::haversine_distance(
                                track_points_3d[i][1], track_points_3d[i][0],
                                track_points_3d[i+1][1], track_points_3d[i+1][0]
                            );
                        }
                        
                        let mut segments_count = (length_m / segment_length) as u32;
                        if (length_m % segment_length) > 1.0 {
                            segments_count += 1;
                        }

                        let mut first_ovr = serde_json::json!({
                            "nbrSegment": segments_count
                        });
                        
                        // Copy camera parameters from master anchor to variant start
                        if let Some(anchor_tp) = tracking_master.get(*anchor_index_on_master) {
                            if let Some(obj) = first_ovr.as_object_mut() {
                                for field in ["zoom", "pitch", "cap", "editedZoom", "editedPitch", "editedCap"] {
                                    if let Some(val) = anchor_tp.get(field) {
                                        obj.insert(field.to_string(), val.clone());
                                    }
                                }
                            }
                        }
                        
                        (Some(first_ovr), tracking_master.get(*anchor_index_on_master).cloned())
                    },
                    VariantModification::ArriveeReportee { anchor_index_on_master, .. } => {
                        let mut first_ovr = tracking_master.get(*anchor_index_on_master).cloned().unwrap_or(serde_json::json!({}));
                        // Arrivee: nbrSegment from anchor to master end
                        let nbr = (tracking_master.len() as u32).saturating_sub(*anchor_index_on_master as u32).saturating_sub(1);
                        first_ovr["nbrSegment"] = serde_json::json!(nbr);
                        (Some(first_ovr), None)
                    },
                    VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } => {
                        let mut first_ovr = tracking_master.get(anchor_start.index).cloned().unwrap_or(serde_json::json!({}));
                        
                        // Segment: nbrSegment from anchor_start to next pointDeControl on master
                        let mut next_cp_idx = anchor_start.index + 1;
                        while next_cp_idx < tracking_master.len() {
                            if tracking_master[next_cp_idx]["pointDeControl"].as_bool().unwrap_or(false) {
                                break;
                            }
                            next_cp_idx += 1;
                        }
                        let nbr = next_cp_idx.saturating_sub(anchor_start.index) as u32;
                        first_ovr["nbrSegment"] = serde_json::json!(nbr);
                        
                        (Some(first_ovr), tracking_master.get(anchor_end.index).cloned())
                    }
                };

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

    let total_tracking_pts = tracking_master.len();
    let total_high_res_pts = master_points_high_res.len();

    // Topological matching: Use the tracking index to hint where we should be in the high-res file
    // This prevents confusing start (idx 0) and end (idx N) of a loop which are geometrically identical.
    let find_corresponding_idx = |lon: f64, lat: f64, tracking_idx: usize, start_search_from: usize| -> usize {
        // 1. Calculate estimated position
        let ratio = tracking_idx as f64 / total_tracking_pts as f64;
        let estimated_idx = (ratio * total_high_res_pts as f64) as usize;

        // 2. Define search window (e.g. +/- 5% of total points, minimum 500 points)
        // We want to be generous but avoid wrapping around the loop
        let window_size = (total_high_res_pts / 20).max(500); 
        
        let min_search = estimated_idx.saturating_sub(window_size).max(start_search_from);
        let max_search = (estimated_idx + window_size).min(total_high_res_pts);

        let mut min_dist = f64::MAX;
        let mut best_idx = start_search_from;

        // Search in the topological window
        for i in min_search..max_search {
            let mp = &master_points_high_res[i];
            let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
            if d < min_dist { 
                min_dist = d; 
                best_idx = i; 
            }
        }
        
        // Safety Fallback: if "topological" search failed (e.g. very far deviation), 
        // try searching forward from start_search_from broadly (traditional method)
        if min_dist > 0.1 { // > 100m error
             // ... existing logic fallback could be here, but usually topological hint is better.
             // Let's stick to the best found in window.
        }

        best_idx
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
                current_master_idx = find_corresponding_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), *anchor_index_on_master, 0);
            },
            VariantModification::SegmentDeviation { anchor_start, anchor_end, full_geometry, .. } => {
                let start_idx = find_corresponding_idx(anchor_start.coords[0], anchor_start.coords[1], anchor_start.index, current_master_idx);
                
                // Copy points from master up to deviation start
                if start_idx >= current_master_idx {
                    for i in current_master_idx..=start_idx {
                        if i < master_points_high_res.len() {
                            final_points.push(master_points_high_res[i].clone());
                        }
                    }
                }
                current_master_idx = start_idx + 1; // Prepare for next

                // Insert deviation
                if let Some(geom) = full_geometry {
                    let (pts, w) = prepare_points_3d(geom).await?;
                    if let Some(msg) = w { global_warning = Some(msg); }
                    final_points.extend(pts);
                }
                
                // Find re-connection point
                let end_idx = find_corresponding_idx(anchor_end.coords[0], anchor_end.coords[1], anchor_end.index, current_master_idx);
                current_master_idx = end_idx; // Will resume from here
                
                // Safety: ensure we don't go backwards if end_idx < start_idx (should be caught by topological search + validation)
                if current_master_idx < start_idx { current_master_idx = start_idx + 1; }
            },
            VariantModification::ArriveeReportee { anchor_index_on_master, full_geometry, .. } => {
                has_arrivee_reportee = true;
                let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                let arrivee_idx = find_corresponding_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), *anchor_index_on_master, current_master_idx);
                
                 if arrivee_idx >= current_master_idx {
                    for i in current_master_idx..=arrivee_idx {
                        if i < master_points_high_res.len() {
                            final_points.push(master_points_high_res[i].clone());
                        }
                    }
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

    // ========== GÉNÉRATION DES FICHIERS PRÉCALCULÉS ==========
    
    // 1. Sauvegarder lineString_FULL.json (Géométrie haute résolution complète)
    let full_linestring_path = circuit_data_dir.join(format!("lineString_{}_FULL.json", metadata.id));
    let full_linestring = serde_json::json!({
        "type": "LineString",
        "coordinates": cleaned_points
    });
    fs::write(&full_linestring_path, serde_json::to_string_pretty(&full_linestring).unwrap())
        .map_err(|e| format!("Failed to write lineString_FULL: {}", e))?;
    
    // 2. Générer tracking_FULL.json (Tracking complet du variant)
    let full_tracking_filename = format!("tracking_{}_FULL.json", metadata.id);
    crate::tracking_processor::generate_tracking_file(
        &app_env_path,
        &request.circuit_id,
        &cleaned_points,
        &settings,
        Some(&full_tracking_filename),
        None, // Override first
        None  // Override last
    )?;
    
    // 3. Charger le tracking_FULL pour identifier les points d'ancrage
    let full_tracking_path = circuit_data_dir.join(&full_tracking_filename);
    let full_tracking_content = fs::read_to_string(&full_tracking_path)
        .map_err(|e| format!("Failed to read generated tracking_FULL: {}", e))?;
    let mut full_tracking_points: Vec<serde_json::Value> = serde_json::from_str(&full_tracking_content)
        .map_err(|e| format!("Failed to parse tracking_FULL: {}", e))?;
    
    // 🔴 NOUVEAU: Identifier et marquer les points d'ancrage
    // Collecter les coordonnées des points d'ancrage depuis les modifications
    let mut anchor_coords: Vec<(f64, f64)> = Vec::new();
    for modification in &sorted_mods {
        match modification {
            VariantModification::DepartDeporte { anchor_index_on_master, .. } => {
                let coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                anchor_coords.push((coords[0].as_f64().unwrap(), coords[1].as_f64().unwrap()));
            },
            VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } => {
                // Début de déviation
                anchor_coords.push((anchor_start.coords[0], anchor_start.coords[1]));
                // Fin de déviation
                anchor_coords.push((anchor_end.coords[0], anchor_end.coords[1]));
            },
            VariantModification::ArriveeReportee { anchor_index_on_master, .. } => {
                let coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                anchor_coords.push((coords[0].as_f64().unwrap(), coords[1].as_f64().unwrap()));
            }
        }
    }
    
    // 🔴 PASS 1: Trouver les indices des points d'ancrage
    let mut anchor_indices: Vec<usize> = Vec::new();
    for (i, point) in full_tracking_points.iter().enumerate() {
        let point_coords = point["coordonnee"].as_array().unwrap();
        let point_lon = point_coords[0].as_f64().unwrap();
        let point_lat = point_coords[1].as_f64().unwrap();
        
        for (anchor_lon, anchor_lat) in &anchor_coords {
            let dist = crate::gpx_processor::haversine_distance(point_lat, point_lon, *anchor_lat, *anchor_lon);
            if dist < 10.0 {
                anchor_indices.push(i);
                break;
            }
        }
    }
    
    // 🔴 PASS 2: Marquer les points d'ancrage
    for &i in &anchor_indices {
        full_tracking_points[i]["isAnchorPoint"] = serde_json::json!(true);
    }
    
    // Sauvegarder le tracking_FULL mis à jour avec les points d'ancrage
    fs::write(&full_tracking_path, serde_json::to_string_pretty(&full_tracking_points).unwrap())
        .map_err(|e| format!("Failed to write updated tracking_FULL: {}", e))?;
    
    // 4. Générer segments_metadata (Détection des overlaps)
    // 🔴 CORRECTION: Utiliser lineString_FULL (haute résolution) au lieu de tracking_FULL
    // Cela permet d'avoir des index de métadonnées cohérents avec la géométrie brute, comme pour la trace principale.
    let distance_threshold = crate::get_setting_value(&settings, "data.groupes.Visualisation.parametres.overlapDetectionThreshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(20.0); // 20 mètres par défaut

    let mut high_res_points: Vec<serde_json::Value> = Vec::new();
    let mut cumulative_dist = 0.0;
    
    // On extrait les coordonnées de full_linestring (déjà calculé plus haut)
    if let Some(coords) = full_linestring.get("coordinates").and_then(|c| c.as_array()) {
        for (i, coord) in coords.iter().enumerate() {
            let lat = coord[1].as_f64().unwrap_or(0.0);
            let lon = coord[0].as_f64().unwrap_or(0.0);
            
            if i > 0 {
                let prev = &coords[i-1];
                let dist_m = crate::segment_analyzer::haversine_distance(prev[1].as_f64().unwrap_or(0.0), prev[0].as_f64().unwrap_or(0.0), lat, lon);
                cumulative_dist += dist_m / 1000.0;
            }
            
            high_res_points.push(serde_json::json!({
                "coordonnee": [lon, lat],
                "distance": cumulative_dist
            }));
        }
    }

    let overlapping_zones = crate::segment_analyzer::detect_overlapping_segments(
        &high_res_points,
        distance_threshold
    )?;
    
    let segments_metadata = crate::segment_analyzer::SegmentMetadata {
        circuit_id: request.circuit_id.clone(),
        overlapping_zones,
        detection_threshold_meters: distance_threshold,
        total_points: high_res_points.len(),
        analysis_date: chrono::Utc::now().to_rfc3339(),
    };
    
    let metadata_path = circuit_data_dir.join(format!("segments_metadata_{}.json", metadata.id));
    fs::write(&metadata_path, serde_json::to_string_pretty(&segments_metadata).unwrap())
        .map_err(|e| format!("Failed to write segments_metadata: {}", e))?;
    
    // TODO Phase 2: Implémenter lissage d'altitude aux points d'ancrage

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

/// Charge le fichier lineString_FULL.json d'un variant
#[tauri::command]
pub async fn get_variant_full_linestring(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<serde_json::Value, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let linestring_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("lineString_{}_FULL.json", variant_id));

    if !linestring_path.exists() {
        return Err(format!("LineString FULL file not found for variant {}", variant_id));
    }

    let content = fs::read_to_string(&linestring_path).map_err(|e| e.to_string())?;
    let linestring: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(linestring)
}

/// Charge le fichier tracking_FULL.json d'un variant
#[tauri::command]
pub async fn get_variant_full_tracking(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<Vec<serde_json::Value>, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let tracking_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("tracking_{}_FULL.json", variant_id));

    if !tracking_path.exists() {
        return Err(format!("Tracking FULL file not found for variant {}", variant_id));
    }

    let content = fs::read_to_string(&tracking_path).map_err(|e| e.to_string())?;
    let tracking: Vec<serde_json::Value> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(tracking)
}

/// Charge le fichier segments_metadata.json d'un variant
#[tauri::command]
pub async fn get_variant_overlap_metadata(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<crate::segment_analyzer::SegmentMetadata, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let metadata_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("segments_metadata_{}.json", variant_id));

    if !metadata_path.exists() {
        return Err(format!("Segments metadata file not found for variant {}", variant_id));
    }

    let content = fs::read_to_string(&metadata_path).map_err(|e| e.to_string())?;
    let metadata: crate::segment_analyzer::SegmentMetadata = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(metadata)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteResult {
    pub geojson: String,
    pub warning: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
struct OrsResponse {
    features: Vec<OrsFeature>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrsFeature {
    geometry: OrsGeometry,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrsGeometry {
    coordinates: Vec<Vec<f64>>,
}

async fn call_graphhopper(api_key: &str, profile: &str, points: &Vec<[f64; 2]>) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("Clé API GraphHopper manquante.".to_string());
    }

    let max_points_per_request = 5;
    let mut all_coordinates: Vec<Vec<f64>> = Vec::new();

    let chunks: Vec<Vec<[f64; 2]>> = if points.len() > max_points_per_request {
        let mut result = Vec::new();
        let mut start_idx = 0;
        while start_idx < points.len() - 1 {
            let end_idx = (start_idx + max_points_per_request).min(points.len());
            if end_idx - start_idx < 2 { break; }
            result.push(points[start_idx..end_idx].to_vec());
            start_idx = end_idx - 1; 
        }
        result
    } else {
        vec![points.clone()]
    };

    for (i, chunk_points) in chunks.iter().enumerate() {
        if i > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        let client = reqwest::Client::new();
        let mut url = format!("https://graphhopper.com/api/1/route?key={}&profile={}&points_encoded=false&elevation=true", api_key, profile);
        
        for p in chunk_points {
            url.push_str(&format!("&point={},{}", p[1], p[0]));
        }

        let resp = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("GraphHopper Request failed: {}", e))?;

        if !resp.status().is_success() {
             return Err(format!("GraphHopper API Error: {}", resp.status()));
        }

        let gh_resp: GraphHopperResponse = resp.json().await.map_err(|e| format!("GH Parse error: {}", e))?;

        if let Some(path) = gh_resp.paths.first() {
            let mut coords = path.points.coordinates.clone();
            if i > 0 && !coords.is_empty() {
                coords.remove(0);
            }
            all_coordinates.extend(coords);
        } else {
             return Err("GraphHopper: Aucun chemin trouvé.".to_string());
        }
    }
    
    let geojson = serde_json::json!({
         "type": "LineString",
         "coordinates": all_coordinates
     });
     Ok(serde_json::to_string(&geojson).unwrap())
}

async fn call_openrouteservice(api_key: &str, profile: &str, points: &Vec<[f64; 2]>) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("Clé API OpenRouteService manquante.".to_string());
    }

    // Map profiles: car->driving-car, racingbike->cycling-road, bike->cycling-mountain
    let ors_profile = match profile {
        "car" => "driving-car",
        "racingbike" => "cycling-road",
        "bike" => "cycling-mountain",
        _ => "driving-car"
    };

    let client = reqwest::Client::new();
    let url = format!("https://api.openrouteservice.org/v2/directions/{}/geojson", ors_profile);
    
    let body = serde_json::json!({
        "coordinates": points
    });

    let resp = client.post(&url)
        .header("Authorization", api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("ORS Request failed: {}", e))?;

     if !resp.status().is_success() {
         return Err(format!("OpenRouteService API Error: {}", resp.status()));
    }

    let ors_resp: OrsResponse = resp.json().await.map_err(|e| format!("ORS Parse error: {}", e))?;

    if let Some(feature) = ors_resp.features.first() {
         let geojson = serde_json::json!({
             "type": "LineString",
             "coordinates": feature.geometry.coordinates
         });
         Ok(serde_json::to_string(&geojson).unwrap())
    } else {
         Err("OpenRouteService: Aucun chemin trouvé.".to_string())
    }
}

#[tauri::command]
pub async fn calculate_route(
    app_handle: tauri::AppHandle,
    service: String,
    profile: String,
    points: Vec<[f64; 2]>,
) -> Result<RouteResult, String> {
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

    let key_gh = crate::get_setting_value(&settings, "data.groupes.Variante.groupes.Parametres.parametres.apiKeyGraphHopper").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let key_ors = crate::get_setting_value(&settings, "data.groupes.Variante.groupes.Parametres.parametres.apiKeyOpenRouteService").and_then(|v| v.as_str()).unwrap_or("").to_string();

    // Define priority list based on user preference
    let mut attempt_order = Vec::new();
    if service == "GraphHopper" {
        attempt_order.push(("GraphHopper", &key_gh));
        attempt_order.push(("OpenRouteService", &key_ors));
    } else {
        attempt_order.push(("OpenRouteService", &key_ors));
        attempt_order.push(("GraphHopper", &key_gh));
    }

    let mut last_error = String::new();

    for (i, (svc_name, key)) in attempt_order.iter().enumerate() {
        let is_fallback = i > 0;
        
        // Retry logic for the current service (max 3 attempts)
        for attempt in 1..=3 {
            let result = if *svc_name == "GraphHopper" {
                call_graphhopper(key, &profile, &points).await
            } else {
                call_openrouteservice(key, &profile, &points).await
            };

            match result {
                Ok(geojson) => {
                    let warning = if is_fallback {
                        Some(format!("Service préférentiel indisponible. Bascule automatique sur {}.", svc_name))
                    } else {
                        None
                    };
                    return Ok(RouteResult { geojson, warning });
                },
                Err(e) => {
                    last_error = e.clone();
                    if attempt < 3 {
                        tokio::time::sleep(std::time::Duration::from_millis(500 * attempt)).await;
                    }
                }
            }
        }
    }

    Err(format!("Echec du routage sur tous les services. Dernière erreur: {}", last_error))
}

#[tauri::command]
pub async fn get_variant_geojson(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<String, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    // 1. Get Details (hydrated with full_geometry from lineString files)
    let archive = get_variant_details(app_handle.clone(), circuit_id.clone(), variant_id.clone()).await?;

    // 2. Load Master Trace
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    let master_ls_path = circuit_data_dir.join("lineString.json");
    let master_ls_content = fs::read_to_string(&master_ls_path).map_err(|e| format!("Failed to read lineString.json: {}", e))?;
    let master_ls: serde_json::Value = serde_json::from_str(&master_ls_content).map_err(|e| format!("Failed to parse lineString.json: {}", e))?;
    let master_coords = master_ls["coordinates"].as_array().ok_or("Invalid lineString format")?;
    let master_points_high_res: Vec<Vec<f64>> = master_coords.iter().map(|c| c.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();

    // 3. Load Master Tracking (needed for indexing logic)
    let tracking_master_path = circuit_data_dir.join("tracking.json");
    let tracking_master_content = fs::read_to_string(&tracking_master_path).map_err(|e| format!("Failed to read master tracking: {}", e))?;
    let tracking_master: Vec<serde_json::Value> = serde_json::from_str(&tracking_master_content).map_err(|e| format!("Failed to parse master tracking: {}", e))?;
    let _total_tracking_pts = tracking_master.len();
    let total_high_res_pts = master_points_high_res.len();


    // 4. Stitching Logic


    // 4. Stitching Logic
     let find_corresponding_idx = |lat: f64, lon: f64, hint_idx: usize, start_search_from: usize| -> (usize, f64) {
        // hint_idx is High Resolution index (from anchors). 
        // We use it directly as the bio-center for search.
        let estimated_idx = hint_idx;
        
        let window_size = (total_high_res_pts / 20).max(500); 
        let min_search = estimated_idx.saturating_sub(window_size).max(start_search_from);
        let max_search = (estimated_idx + window_size).min(total_high_res_pts);
        
        let mut min_dist = f64::MAX;
        let mut best_idx = start_search_from;
        
        // Safety Fallback if window invalid
        if min_search >= max_search {
             let fallback_max = (start_search_from + 2000).min(total_high_res_pts);
             for i in start_search_from..fallback_max {
                let mp = &master_points_high_res[i];
                let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
                if d < min_dist { min_dist = d; best_idx = i; }
             }
             return (best_idx, min_dist);
        }

        for i in min_search..max_search {
            let mp = &master_points_high_res[i];
             let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
            if d < min_dist { min_dist = d; best_idx = i; }
        }
        
        // Fallback: Global Search if local match is poor (> 50m)
        // This handles cases where indices might be corrupted or desynchronized
        if min_dist > 50.0 {
             let mut global_min = min_dist;
             for i in start_search_from..total_high_res_pts {
                let mp = &master_points_high_res[i];
                let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
                if d < global_min { global_min = d; best_idx = i; }
                if global_min < 1.0 { break; } // Optimization: found close match (< 1m)
             }
             min_dist = global_min;
        }
        
        (best_idx, min_dist)
    };

    let mut final_points: Vec<Vec<f64>> = Vec::new();
    let mut current_master_idx = 0;
    let mut sorted_mods = archive.modifications.clone();
    sorted_mods.sort_by_key(|m| m.get_start_anchor_index());
    let mut has_arrivee_reportee = false;
    
    eprintln!("🔍 RUST: Starting variant assembly. Master has {} high-res points", total_high_res_pts);

    for modification in &sorted_mods {
         match modification {
            VariantModification::DepartDeporte { anchor_index_on_master, full_geometry, .. } => {
                
                if let Some(geom) = full_geometry {
                     for p in geom {
                         final_points.push(vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]);
                     }
                }
                // Use master_points_high_res directly
                if *anchor_index_on_master < master_points_high_res.len() {
                    let anchor_coords = &master_points_high_res[*anchor_index_on_master];
                    // master_points_high_res: [lon, lat, alt], haversine needs (lat, lon)
                    let (idx, _dist) = find_corresponding_idx(anchor_coords[1], anchor_coords[0], *anchor_index_on_master, 0);
                    current_master_idx = idx;
                } else {
                    current_master_idx = 0; // Error fallback
                }
            },
             VariantModification::SegmentDeviation { anchor_start, anchor_end, full_geometry, .. } => {
                 // anchor_start.coords: [lon, lat], haversine needs (lat, lon)
                 let (start_idx, _start_dist) = find_corresponding_idx(anchor_start.coords[1], anchor_start.coords[0], anchor_start.index, current_master_idx);
                 
                 // Log what we actually found in master
                 if start_idx < master_points_high_res.len() {
                     let _found_point = &master_points_high_res[start_idx];
                 }
                 
                 if start_idx >= current_master_idx {
                    for i in current_master_idx..=start_idx {
                        if i < master_points_high_res.len() {
                            final_points.push(master_points_high_res[i].clone());
                        }
                    }
                }
                current_master_idx = start_idx + 1;
                
                if let Some(geom) = full_geometry {
                     for p in geom {
                         final_points.push(vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]);
                     }
                }
                
                // anchor_end.coords: [lon, lat], haversine needs (lat, lon)
                let (end_idx, _end_dist) = find_corresponding_idx(anchor_end.coords[1], anchor_end.coords[0], anchor_end.index, current_master_idx);
                
                // Log what we found
                if end_idx < master_points_high_res.len() {
                    let _found_point = &master_points_high_res[end_idx];
                }
                
                current_master_idx = end_idx;
                if current_master_idx < start_idx { 
                    current_master_idx = start_idx + 1; 
                }
             },
             VariantModification::ArriveeReportee { anchor_index_on_master, full_geometry, .. } => {
                 has_arrivee_reportee = true;
                 if *anchor_index_on_master < master_points_high_res.len() {
                    let anchor_coords = &master_points_high_res[*anchor_index_on_master];
                    // master_points_high_res: [lon, lat, alt], haversine needs (lat, lon)
                    let (arrivee_idx, _arrivee_dist) = find_corresponding_idx(anchor_coords[1], anchor_coords[0], *anchor_index_on_master, current_master_idx);
                    
                    if arrivee_idx >= current_master_idx {
                        for i in current_master_idx..=arrivee_idx {
                            if i < master_points_high_res.len() {
                                final_points.push(master_points_high_res[i].clone());
                            }
                        }
                    }
                 }
                 
                if let Some(geom) = full_geometry {
                      for p in geom {
                         final_points.push(vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]);
                     }
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

    // Convert to GeoJSON string
      let geojson = serde_json::json!({
        "type": "Feature",
        "geometry": {
            "type": "LineString",
            "coordinates": final_points
        },
        "properties": {
            "variant_id": variant_id
        }
    });

    Ok(serde_json::to_string(&geojson).map_err(|e| e.to_string())?)
}

#[tauri::command]
pub async fn get_variant_comparison_geojson(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<String, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    // 1. Get Details
    let archive = get_variant_details(app_handle.clone(), circuit_id.clone(), variant_id.clone()).await?;

    // 2. Load Master Trace
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    let master_ls_path = circuit_data_dir.join("lineString.json");
    let master_ls_content = fs::read_to_string(&master_ls_path).map_err(|e| format!("Failed to read lineString.json: {}", e))?;
    let master_ls: serde_json::Value = serde_json::from_str(&master_ls_content).map_err(|e| format!("Failed to parse lineString.json: {}", e))?;
    let master_coords = master_ls["coordinates"].as_array().ok_or("Invalid lineString format")?;
    let master_points_high_res: Vec<Vec<f64>> = master_coords.iter().map(|c| c.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();

    // 3. Load Master Tracking
    let tracking_master_path = circuit_data_dir.join("tracking.json");
    let tracking_master_content = fs::read_to_string(&tracking_master_path).map_err(|e| format!("Failed to read master tracking: {}", e))?;
    let tracking_master: Vec<serde_json::Value> = serde_json::from_str(&tracking_master_content).map_err(|e| format!("Failed to parse master tracking: {}", e))?;
    let total_tracking_pts = tracking_master.len();
    let total_high_res_pts = master_points_high_res.len();

    // 4. Indexing Helper
    let find_corresponding_idx = |lon: f64, lat: f64, tracking_idx: usize, start_search_from: usize| -> usize {
        let ratio = tracking_idx as f64 / total_tracking_pts as f64;
        let estimated_idx = (ratio * total_high_res_pts as f64) as usize;
        let window_size = (total_high_res_pts / 20).max(500); 
        let min_search = estimated_idx.saturating_sub(window_size).max(start_search_from);
        let max_search = (estimated_idx + window_size).min(total_high_res_pts);
        
        let mut min_dist = f64::MAX;
        let mut best_idx = start_search_from;
        
        for i in min_search..max_search {
            let mp = &master_points_high_res[i];
            let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
            if d < min_dist { min_dist = d; best_idx = i; }
        }
        best_idx
    };

    let mut features: Vec<serde_json::Value> = Vec::new();
    let mut current_master_idx = 0;
    
    // Sort modifications by start anchor index
    let mut sorted_mods = archive.modifications.clone();
    sorted_mods.sort_by_key(|m| m.get_start_anchor_index());
    let mut has_arrivee_reportee = false;

    // Helper to create Feature
    let create_feature = |points: Vec<Vec<f64>>, status: &str| -> serde_json::Value {
         serde_json::json!({
            "type": "Feature",
            "geometry": {
                "type": "LineString",
                "coordinates": points
            },
            "properties": {
                "status": status // "COMMON", "ABANDONED", "NEW"
            }
        })
    };

    for modification in &sorted_mods {
         match modification {
            VariantModification::DepartDeporte { anchor_index_on_master, full_geometry, .. } => {
                // Determine anchor position on master
                let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                let anchor_idx = find_corresponding_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), *anchor_index_on_master, 0);

                // ABANDONED segment from Start (0) to Anchor
                if anchor_idx > 0 {
                    let abandoned_pts: Vec<Vec<f64>> = master_points_high_res[0..=anchor_idx].to_vec();
                    features.push(create_feature(abandoned_pts, "ABANDONED"));
                }
                current_master_idx = anchor_idx;

                 // NEW segment (the variant part)
                if let Some(geom) = full_geometry {
                     let pts: Vec<Vec<f64>> = geom.iter().map(|p| vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]).collect();
                     features.push(create_feature(pts, "NEW"));
                }
            },
             VariantModification::SegmentDeviation { anchor_start, anchor_end, full_geometry, .. } => {
                 let start_idx = find_corresponding_idx(anchor_start.coords[0], anchor_start.coords[1], anchor_start.index, current_master_idx);
                 
                 // COMMON segment from current to start_idx
                 if start_idx > current_master_idx {
                     let common_pts: Vec<Vec<f64>> = master_points_high_res[current_master_idx..=start_idx].to_vec();
                     if !common_pts.is_empty() {
                         features.push(create_feature(common_pts, "COMMON"));
                     }
                 }
                 
                 // Find end_idx
                 let end_idx = find_corresponding_idx(anchor_end.coords[0], anchor_end.coords[1], anchor_end.index, start_idx);
                 
                 // ABANDONED segment from start_idx to end_idx
                 if end_idx > start_idx {
                      let abandoned_pts: Vec<Vec<f64>> = master_points_high_res[start_idx..=end_idx].to_vec();
                      if !abandoned_pts.is_empty() {
                         features.push(create_feature(abandoned_pts, "ABANDONED"));
                     }
                 }
                
                // NEW segment
                if let Some(geom) = full_geometry {
                     let pts: Vec<Vec<f64>> = geom.iter().map(|p| vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]).collect();
                     features.push(create_feature(pts, "NEW"));
                }
                
                current_master_idx = end_idx;
                if current_master_idx < start_idx { current_master_idx = start_idx; } // Safety
             },
             VariantModification::ArriveeReportee { anchor_index_on_master, full_geometry, .. } => {
                 has_arrivee_reportee = true;
                 let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                 let arrivee_idx = find_corresponding_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), *anchor_index_on_master, current_master_idx);
                 
                 // COMMON segment up to Arrivee Anchor
                 if arrivee_idx > current_master_idx {
                     let common_pts: Vec<Vec<f64>> = master_points_high_res[current_master_idx..=arrivee_idx].to_vec();
                      if !common_pts.is_empty() {
                         features.push(create_feature(common_pts, "COMMON"));
                     }
                 }
                 
                // ABANDONED segment from Arrivee Anchor to End
                if arrivee_idx < master_points_high_res.len() - 1 {
                    let abandoned_pts: Vec<Vec<f64>> = master_points_high_res[arrivee_idx..].to_vec();
                    features.push(create_feature(abandoned_pts, "ABANDONED"));
                }

                // NEW segment
                if let Some(geom) = full_geometry {
                     let pts: Vec<Vec<f64>> = geom.iter().map(|p| vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]).collect();
                     features.push(create_feature(pts, "NEW"));
                }
                current_master_idx = master_points_high_res.len();
             }
         }
    }

    // Remaining COMMON segment if any
    if !has_arrivee_reportee && current_master_idx < master_points_high_res.len() {
        let common_pts: Vec<Vec<f64>> = master_points_high_res[current_master_idx..].to_vec();
        features.push(create_feature(common_pts, "COMMON"));
    }

    let fc = serde_json::json!({
        "type": "FeatureCollection",
        "features": features
    });

    Ok(serde_json::to_string(&fc).map_err(|e| e.to_string())?)
}

// --- Helpers for Tracking Generation ---

fn simple_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1 = lat1.to_radians();
    let lon1 = lon1.to_radians();
    let lat2 = lat2.to_radians();
    let lon2 = lon2.to_radians();
    let d_lon = lon2 - lon1;
    let y = d_lon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * d_lon.cos();
    let mut brng = y.atan2(x).to_degrees();
    if brng < 0.0 { brng += 360.0; }
    (brng * 10.0).round() / 10.0
}

fn generate_interpolated_tracking(points: &Vec<VariantPoint>, segment_length: f64) -> Vec<serde_json::Value> {
    if points.is_empty() { return Vec::new(); }
    
    // 1. Convert to simple coordinates for calculation
    let coords_3d: Vec<Vec<f64>> = points.iter().map(|p| vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]).collect();
    
    // 2. Interpolate
    let mut calculated_points = Vec::new();
    calculated_points.push(coords_3d[0].clone());

    let mut distance_needed = segment_length;
    let mut distance_traversed = 0.0;

    for i in 0..coords_3d.len() - 1 {
        let p1 = &coords_3d[i];
        let p2 = &coords_3d[i+1];
        let dist = crate::gpx_processor::haversine_distance(p1[1], p1[0], p2[1], p2[0]);

        if dist > 0.0 {
            while distance_traversed + dist >= distance_needed {
                let fraction = (distance_needed - distance_traversed) / dist;
                let lon = p1[0] + (p2[0] - p1[0]) * fraction;
                let lat = p1[1] + (p2[1] - p1[1]) * fraction;
                let alt = p1[2] + (p2[2] - p1[2]) * fraction;
                calculated_points.push(vec![lon, lat, alt]);
                distance_needed += segment_length;
            }
        }
        distance_traversed += dist;
    }

    // 3. Add last point if needed
    if coords_3d.len() > 1 {
        let last = coords_3d.last().unwrap();
        let last_calc = calculated_points.last().unwrap();
        let d = crate::gpx_processor::haversine_distance(last[1], last[0], last_calc[1], last_calc[0]);
        if d > 1.0 {
            calculated_points.push(last.clone());
        }
    }

    // 4. Map to TrackingPoint JSON
    let count = calculated_points.len();
    let mut results: Vec<serde_json::Value> = Vec::new();
    for i in 0..count {
        let p = &calculated_points[i];
        let mut cap = 0.0;
        if i < count - 1 {
            let next = &calculated_points[i + 1];
            cap = simple_bearing(p[1], p[0], next[1], next[0]);
        } else if i > 0 {
            if let Some(prev) = results.last() {
                cap = prev["cap"].as_f64().unwrap_or(0.0);
            }
        }
        results.push(serde_json::json!({
            "increment": i,
            "coordonnee": [p[0], p[1]],
            "altitude": p[2],
            "distance": 0.0,
            "cap": cap,
            "zoom": 16.0,
            "pitch": 55.0,
            "pointDeControl": i == 0 || i == count - 1
        }));
    }
    results
}

fn find_closest_tracking_idx(tracking: &Vec<serde_json::Value>, lat: f64, lon: f64, start_hint: usize) -> usize {
    let mut min_dist = f64::MAX;
    let mut best_idx = start_hint;
    let limit = tracking.len();
    // Search localized around hint first could be better but let's scan a reasonable window or all if small
    // Optimization: scan forward from start_hint
    for i in 0..limit {
        if let Some(coords) = tracking[i].get("coordonnee").and_then(|c| c.as_array()) {
            if coords.len() >= 2 {
                let clat = coords[1].as_f64().unwrap_or(0.0);
                let clon = coords[0].as_f64().unwrap_or(0.0);
                let d = (clat - lat).powi(2) + (clon - lon).powi(2);
                if d < min_dist {
                    min_dist = d;
                    best_idx = i;
                }
            }
        }
    }
    best_idx
}

// --- Commands ---

#[tauri::command]
pub async fn get_variant_tracking(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<String, String> {
    let tracking_data = get_variant_tracking_internal(app_handle, circuit_id, variant_id).await?;
    serde_json::to_string(&tracking_data).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_variant_slope_expression(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
    slope_colors: std::collections::HashMap<String, String>,
) -> Result<serde_json::Value, String> {
    // 1. Get Variant Tracking (Interpolated at 100m)
    let tracking_values = get_variant_tracking_internal(app_handle.clone(), circuit_id.clone(), variant_id).await?;
    
    // 2. Convert to TraceStyle TrackingPoint
    let tracking_points: Vec<crate::trace_style::TrackingPoint> = tracking_values.into_iter().map(|v| {
        let alt = v["altitude"].as_f64().unwrap_or(0.0);
        let coords = v["coordonnee"].as_array().unwrap();
        let lon = coords[0].as_f64().unwrap_or(0.0);
        let lat = coords[1].as_f64().unwrap_or(0.0);
        crate::trace_style::TrackingPoint {
            altitude: alt,
            coordonnee: [lon, lat],
        }
    }).collect();

    // 3. Get Settings for segment length
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };
    let settings_path = app_env_path.join("settings.json");
    let settings_content = std::fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;
    let segment_length = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);

     // 4. Generate Expression using TraceStyle (reusing existing logic)
     crate::trace_style::get_slope_color_expression(
         app_handle.state::<std::sync::Mutex<crate::AppState>>(),
         circuit_id,
         slope_colors,
         segment_length,
         None,
         Some(tracking_points)
     ).await
}




pub async fn get_variant_tracking_internal(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<Vec<serde_json::Value>, String> {
     let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let segment_length = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);

    // 1. Load Master Tracking
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    let tracking_path = circuit_data_dir.join("tracking.json");
    if !tracking_path.exists() { return Err("Master tracking.json missing".to_string()); }
    
    let content = fs::read_to_string(&tracking_path).map_err(|e| e.to_string())?;
    let master_tracking: Vec<serde_json::Value> = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    // 2. Load Variant Data
    // Use get_variant_details which returns Archive
    let archive = get_variant_details(app_handle.clone(), circuit_id.clone(), variant_id.clone()).await?;

    // 3. Assemble
    let mut final_tracking: Vec<serde_json::Value> = Vec::new();
    let mut current_master_idx = 0;
    
    // Check for DepartDeporte first as it overrides start
    // Sort logic similar to comparison? Yes.
    let mut sorted_mods = archive.modifications.clone();
    sorted_mods.sort_by_key(|m| m.get_start_anchor_index());

    for modification in sorted_mods {
        match modification {
            VariantModification::DepartDeporte { full_geometry, .. } => {
                 if let Some(geom) = full_geometry {
                      // Find where it connects on Master
                      let last = geom.last().unwrap();
                      let connect_idx = find_closest_tracking_idx(&master_tracking, last.lat, last.lon, 0);
                      
                      let mut variant_tracking = generate_interpolated_tracking(&geom, segment_length);

                      // Copy camera params from anchor to first point
                      if let Some(anchor_tp) = master_tracking.get(connect_idx) {
                          if let Some(first_tp) = variant_tracking.get_mut(0).and_then(|v| v.as_object_mut()) {
                              for field in ["zoom", "pitch", "cap", "editedZoom", "editedPitch", "editedCap"] {
                                  if let Some(val) = anchor_tp.get(field) {
                                      first_tp.insert(field.to_string(), val.clone());
                                  }
                              }
                          }
                      }
                      
                      // Update LAST point of variant to point to master_tracking[connect_idx + 1]
                      if let (Some(last_tp), Some(next_master_tp)) = (variant_tracking.last_mut(), master_tracking.get(connect_idx + 1)) {
                          if let (Some(last_obj), Some(next_obj)) = (last_tp.as_object_mut(), next_master_tp.as_object()) {
                              let l_coords = last_obj["coordonnee"].as_array().unwrap();
                              let n_coords = next_obj["coordonnee"].as_array().unwrap();
                              let new_cap = simple_bearing(l_coords[1].as_f64().unwrap(), l_coords[0].as_f64().unwrap(),
                                                         n_coords[1].as_f64().unwrap(), n_coords[0].as_f64().unwrap());
                              last_obj.insert("cap".to_string(), serde_json::json!(new_cap));
                              if last_obj.contains_key("editedCap") { last_obj.insert("editedCap".to_string(), serde_json::json!(new_cap)); }
                          }
                      }

                      final_tracking.extend(variant_tracking);
                      current_master_idx = connect_idx + 1;
                 }
            },
            VariantModification::ArriveeReportee { full_geometry, .. } => {
                 if let Some(geom) = full_geometry {
                     if let Some(first) = geom.first() {
                         let connect_idx = find_closest_tracking_idx(&master_tracking, first.lat, first.lon, current_master_idx);
                         
                         // Add Master until connection
                         for i in current_master_idx..=connect_idx {
                             if i < master_tracking.len() {
                                 final_tracking.push(master_tracking[i].clone());
                             }
                         }
                         
                         // Add New Segment (skip first point as it's the anchor in master)
                         let var_tracking = generate_interpolated_tracking(&geom, segment_length);
                         // Update master anchor's cap to point to the FIRST NEW point of variant
                         if let (Some(anchor_tp), Some(next_var_tp)) = (final_tracking.last_mut(), var_tracking.get(1)) {
                             if let (Some(anchor_obj), Some(next_obj)) = (anchor_tp.as_object_mut(), next_var_tp.as_object()) {
                                 let a_coords = anchor_obj["coordonnee"].as_array().unwrap();
                                 let n_coords = next_obj["coordonnee"].as_array().unwrap();
                                 let new_cap = simple_bearing(a_coords[1].as_f64().unwrap(), a_coords[0].as_f64().unwrap(),
                                                            n_coords[1].as_f64().unwrap(), n_coords[0].as_f64().unwrap());
                                 anchor_obj.insert("cap".to_string(), serde_json::json!(new_cap));
                                 if anchor_obj.contains_key("editedCap") { anchor_obj.insert("editedCap".to_string(), serde_json::json!(new_cap)); }
                             }
                         }
                         final_tracking.extend(var_tracking.into_iter().skip(1));
                         
                         current_master_idx = master_tracking.len(); // End
                     }
                 }
            },
            VariantModification::SegmentDeviation { full_geometry, .. } => {
                if let Some(geom) = full_geometry {
                    if let Some(first) = geom.first() {
                        let start_idx = find_closest_tracking_idx(&master_tracking, first.lat, first.lon, current_master_idx);
                        
                         // Add Master until start
                         for i in current_master_idx..=start_idx {
                             if i < master_tracking.len() {
                                 final_tracking.push(master_tracking[i].clone());
                             }
                         }
                         
                         // Add New Segment (skip first point as it's the anchor in master)
                         let mut var_tracking = generate_interpolated_tracking(&geom, segment_length);
                         
                         // 1. Update master anchor to point to variant[1]
                         if let (Some(anchor_tp), Some(next_var_tp)) = (final_tracking.last_mut(), var_tracking.get(1)) {
                              if let (Some(anchor_obj), Some(next_obj)) = (anchor_tp.as_object_mut(), next_var_tp.as_object()) {
                                 let a_coords = anchor_obj["coordonnee"].as_array().unwrap();
                                 let n_coords = next_obj["coordonnee"].as_array().unwrap();
                                 let new_cap = simple_bearing(a_coords[1].as_f64().unwrap(), a_coords[0].as_f64().unwrap(),
                                                            n_coords[1].as_f64().unwrap(), n_coords[0].as_f64().unwrap());
                                 anchor_obj.insert("cap".to_string(), serde_json::json!(new_cap));
                                 if anchor_obj.contains_key("editedCap") { anchor_obj.insert("editedCap".to_string(), serde_json::json!(new_cap)); }
                             }
                         }
                         
                         // 2. Update last point of variant to point to master_tracking[end_idx + 1]
                         if let Some(last) = geom.last() {
                             let end_idx = find_closest_tracking_idx(&master_tracking, last.lat, last.lon, start_idx);
                             
                             if let (Some(last_tp), Some(next_master_tp)) = (var_tracking.last_mut(), master_tracking.get(end_idx + 1)) {
                                  if let (Some(last_obj), Some(next_obj)) = (last_tp.as_object_mut(), next_master_tp.as_object()) {
                                     let l_coords = last_obj["coordonnee"].as_array().unwrap();
                                     let n_coords = next_obj["coordonnee"].as_array().unwrap();
                                     let new_cap = simple_bearing(l_coords[1].as_f64().unwrap(), l_coords[0].as_f64().unwrap(),
                                                                n_coords[1].as_f64().unwrap(), n_coords[0].as_f64().unwrap());
                                     last_obj.insert("cap".to_string(), serde_json::json!(new_cap));
                                     if last_obj.contains_key("editedCap") { last_obj.insert("editedCap".to_string(), serde_json::json!(new_cap)); }
                                 }
                             }
                             
                             final_tracking.extend(var_tracking.into_iter().skip(1));
                             current_master_idx = end_idx + 1;
                         } else {
                             final_tracking.extend(var_tracking.into_iter().skip(1));
                         }
                    }
                }
            }
        }
    }
    
    // Add remaining master
    for i in current_master_idx..master_tracking.len() {
        final_tracking.push(master_tracking[i].clone());
    }
    
    // 4. Post-Process: Recalculate Increments
    for (i, val) in final_tracking.iter_mut().enumerate() {
        if let Some(obj) = val.as_object_mut() {
            obj.insert("increment".to_string(), serde_json::json!(i));
        }
    }
    
    Ok(final_tracking)
}
